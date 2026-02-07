use fyrox::asset::untyped::ResourceKind;
use fyrox::core::uuid;
use fyrox::material::MaterialResource;
use fyrox::scene::terrain::Terrain;
use fyrox::{
    asset::manager::ResourceManager,
    core::{algebra::Vector2, algebra::Vector3, pool::Handle},
    material::Material,
    rand::{thread_rng, Rng},
    resource::texture::Texture,
    scene::{
        base::BaseBuilder,
        graph::Graph,
        terrain::{Brush, BrushContext, BrushMode, BrushShape, BrushTarget, Layer, TerrainBuilder},
    },
};

// ANCHOR: create_random_two_layer_terrain
fn setup_layer_material(
    material: &mut Material,
    resource_manager: ResourceManager,
    diffuse_texture: &str,
    normal_texture: &str,
) {
    material.bind(
        "diffuseTexture",
        Some(resource_manager.request::<Texture>(diffuse_texture)),
    );
    material.bind(
        "normalTexture",
        Some(resource_manager.request::<Texture>(normal_texture)),
    );
    material.set_property("texCoordScale", Vector2::new(10.0, 10.0));
}

pub fn create_random_two_layer_terrain(
    graph: &mut Graph,
    resource_manager: &ResourceManager,
) -> Handle<Terrain> {
    let terrain = TerrainBuilder::new(BaseBuilder::new())
        .with_layers(vec![
            Layer {
                material: {
                    let mut material = Material::standard_terrain();
                    setup_layer_material(
                        &mut material,
                        resource_manager.clone(),
                        "examples/data/Grass_DiffuseColor.jpg",
                        "examples/data/Grass_NormalColor.jpg",
                    );
                    MaterialResource::new_ok(
                        uuid!("4374fc82-ef19-4eea-a518-8a6f8afa7dba"),
                        ResourceKind::Embedded,
                        material,
                    )
                },
                ..Default::default()
            },
            Layer {
                material: {
                    let mut material = Material::standard_terrain();
                    setup_layer_material(
                        &mut material,
                        resource_manager.clone(),
                        "examples/data/Rock_DiffuseColor.jpg",
                        "examples/data/Rock_Normal.jpg",
                    );
                    MaterialResource::new_ok(
                        uuid!("19b0df85-201e-40c5-96d7-577ee6f75e7c"),
                        ResourceKind::Embedded,
                        material,
                    )
                },
                ..Default::default()
            },
        ])
        .build(graph);

    let terrain_ref = &mut graph[terrain];
    let mut context = BrushContext::default();

    // Draw something on the terrain.
    for _ in 0..60 {
        let x = thread_rng().gen_range(4.0..60.00);
        let z = thread_rng().gen_range(4.0..60.00);
        let radius = thread_rng().gen_range(2.0..4.0);
        let height = thread_rng().gen_range(1.0..3.0);
        let tail_x = thread_rng().gen_range(-5.0..=5.0);
        let tail_z = thread_rng().gen_range(-5.0..=5.0);

        // Pull terrain.
        context.start_stroke(
            terrain_ref,
            Brush {
                shape: BrushShape::Circle { radius },
                mode: BrushMode::Raise { amount: height },
                target: BrushTarget::HeightMap,
                hardness: 0.0,
                ..Brush::default()
            },
        );
        context.stamp(terrain_ref, Vector3::new(x, 0.0, z));
        *context.shape() = BrushShape::Circle {
            radius: radius * 0.5,
        };
        context.smear(
            terrain_ref,
            Vector3::new(x, 0.0, z),
            Vector3::new(x + tail_x, 0.0, z + tail_z),
        );
        context.end_stroke();

        // Draw rock texture on top.
        context.start_stroke(
            terrain_ref,
            Brush {
                shape: BrushShape::Circle { radius },
                mode: BrushMode::Assign { value: 1.0 },
                target: BrushTarget::LayerMask { layer: 1 },
                hardness: 0.0,
                ..Brush::default()
            },
        );
        context.stamp(terrain_ref, Vector3::new(x, 0.0, z));
        *context.shape() = BrushShape::Circle {
            radius: radius * 0.5,
        };
        context.smear(
            terrain_ref,
            Vector3::new(x, 0.0, z),
            Vector3::new(x + tail_x, 0.0, z + tail_z),
        );
        context.end_stroke();
    }

    terrain
}
// ANCHOR_END: create_random_two_layer_terrain
