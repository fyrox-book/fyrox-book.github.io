use fyrox::{
    asset::{manager::ResourceManager, untyped::ResourceKind},
    core::{
        algebra::{Matrix4, Point3, Vector3},
        futures::executor::block_on,
        pool::Handle,
        ImmutableString,
    },
    graph::BaseSceneGraph,
    material::{shader::SamplerFallback, Material, MaterialResource, PropertyValue},
    resource::{
        model::{Model, ModelResourceExtension},
        texture::Texture,
    },
    scene::{
        base::BaseBuilder,
        graph::Graph,
        mesh::{
            buffer::{VertexAttributeUsage, VertexReadTrait},
            surface::{SurfaceBuilder, SurfaceData, SurfaceResource},
            Mesh, MeshBuilder,
        },
        node::Node,
        transform::TransformBuilder,
        Scene,
    },
};
use std::path::Path;

// ANCHOR: load_model_to_scene
fn load_model_to_scene(
    scene: &mut Scene,
    path: &Path,
    resource_manager: ResourceManager,
) -> Handle<Node> {
    // Request model resource and block until it loading.
    let model_resource = block_on(resource_manager.request::<Model>(path)).unwrap();

    // Create an instance of the resource in the scene.
    model_resource.instantiate(scene)
}
// ANCHOR_END: load_model_to_scene

// ANCHOR: create_procedural_mesh
fn create_procedural_mesh(scene: &mut Scene, resource_manager: ResourceManager) -> Handle<Node> {
    let mut material = Material::standard();

    // Material is completely optional, but here we'll demonstrate that it is possible to
    // create procedural meshes with any material you want.
    material
        .set_property(
            &ImmutableString::new("diffuseTexture"),
            PropertyValue::Sampler {
                value: Some(resource_manager.request::<Texture>("some_texture.jpg")),
                fallback: SamplerFallback::White,
            },
        )
        .unwrap();

    // Notice the MeshBuilder.
    MeshBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                .with_local_position(Vector3::new(0.0, -0.25, 0.0))
                .build(),
        ),
    )
    .with_surfaces(vec![SurfaceBuilder::new(SurfaceResource::new_ok(
        ResourceKind::Embedded,
        // Our procedural mesh will have a form of squashed cube.
        // A mesh can have unlimited number of surfaces.
        SurfaceData::make_cube(Matrix4::new_nonuniform_scaling(&Vector3::new(
            25.0, 0.25, 25.0,
        ))),
    ))
    .with_material(MaterialResource::new_ok(ResourceKind::Embedded, material))
    .build()])
    .build(&mut scene.graph)
}
// ANCHOR_END: create_procedural_mesh

// ANCHOR: extract_world_space_vertices
fn extract_world_space_vertices(mesh: &Mesh, graph: &Graph) -> Vec<Vector3<f32>> {
    let mut vertices = Vec::new();

    for surface in mesh.surfaces() {
        let guard = surface.data();
        let data = guard.data_ref();

        for vertex in data.vertex_buffer.iter() {
            let Ok(position) = vertex.read_3_f32(VertexAttributeUsage::Position) else {
                continue;
            };

            let Ok(weights) = vertex.read_4_f32(VertexAttributeUsage::BoneWeight) else {
                continue;
            };

            let Ok(indices) = vertex.read_4_u8(VertexAttributeUsage::BoneIndices) else {
                continue;
            };

            let mut world_space_vertex = Vector3::default();
            for (weight, index) in weights.iter().zip(indices.iter()) {
                if let Some(bone_node) = surface
                    .bones()
                    .get(*index as usize)
                    .and_then(|bone_handle| graph.try_get(*bone_handle))
                {
                    let bone_transform =
                        bone_node.global_transform() * bone_node.inv_bind_pose_transform();
                    world_space_vertex += bone_transform
                        .transform_point(&Point3::from(position))
                        .coords
                        .scale(*weight);
                }
            }

            vertices.push(world_space_vertex);
        }
    }

    vertices
}
// ANCHOR_END: extract_world_space_vertices
