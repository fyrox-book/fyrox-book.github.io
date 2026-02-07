use fyrox::{
    asset::manager::ResourceManager,
    core::algebra::{Matrix4, Point3, Vector2, Vector3},
    core::futures::executor::block_on,
    core::math::ray::Ray,
    core::math::{TriangleDefinition, Vector3Ext},
    core::pool::Handle,
    graph::SceneGraphNode,
    renderer::Renderer,
    resource::texture::Texture,
    scene::base::BaseBuilder,
    scene::camera::{
        Camera, CameraBuilder, ColorGradingLut, OrthographicProjection, PerspectiveProjection,
        Projection,
    },
    scene::graph::Graph,
    scene::mesh::buffer::{VertexAttributeUsage, VertexReadTrait},
    scene::mesh::surface::SurfaceData,
    scene::mesh::Mesh,
    scene::node::Node,
    scene::Scene,
};

// ANCHOR: create_camera
fn create_camera(scene: &mut Scene) -> Handle<Camera> {
    CameraBuilder::new(BaseBuilder::new())
        // Set some properties.
        .with_fov(80.0f32.to_radians())
        .with_z_far(256.0)
        .build(&mut scene.graph)
}
// ANCHOR_END: create_camera

// ANCHOR: create_perspective_camera
fn create_perspective_camera(graph: &mut Graph) -> Handle<Camera> {
    CameraBuilder::new(BaseBuilder::new())
        .with_projection(Projection::Perspective(PerspectiveProjection {
            // Keep in mind that field of view expressed in radians!
            fov: 60.0f32.to_radians(),
            z_near: 0.025,
            z_far: 1024.0,
        }))
        .build(graph)
}
// ANCHOR_END: create_perspective_camera

// ANCHOR: create_orthographic_camera
fn create_orthographic_camera(graph: &mut Graph) -> Handle<Camera> {
    CameraBuilder::new(BaseBuilder::new())
        .with_projection(Projection::Orthographic(OrthographicProjection {
            vertical_size: 5.0,
            z_near: 0.025,
            z_far: 1024.0,
        }))
        .build(graph)
}
// ANCHOR_END: create_orthographic_camera

// ANCHOR: create_camera_with_lut
fn create_camera_with_lut(scene: &mut Scene, resource_manager: ResourceManager) -> Handle<Camera> {
    CameraBuilder::new(BaseBuilder::new())
        .with_color_grading_enabled(true)
        .with_color_grading_lut(
            block_on(ColorGradingLut::new(
                resource_manager.request::<Texture>("path/to/lut.jpg"),
            ))
            .unwrap(),
        )
        .build(&mut scene.graph)
}
// ANCHOR_END: create_camera_with_lut

// ANCHOR: make_picking_ray
fn make_picking_ray(camera: &Camera, point: Vector2<f32>, renderer: &Renderer) -> Ray {
    camera.make_ray(point, renderer.get_frame_bounds())
}
// ANCHOR_END: make_picking_ray

// ANCHOR: precise_ray_test
fn read_vertex_position(data: &SurfaceData, i: u32) -> Option<Vector3<f32>> {
    data.vertex_buffer
        .get(i as usize)
        .and_then(|v| v.read_3_f32(VertexAttributeUsage::Position).ok())
}

fn transform_vertex(vertex: Vector3<f32>, transform: &Matrix4<f32>) -> Vector3<f32> {
    transform.transform_point(&Point3::from(vertex)).coords
}

fn read_triangle(
    data: &SurfaceData,
    triangle: &TriangleDefinition,
    transform: &Matrix4<f32>,
) -> Option<[Vector3<f32>; 3]> {
    let a = transform_vertex(read_vertex_position(data, triangle[0])?, transform);
    let b = transform_vertex(read_vertex_position(data, triangle[1])?, transform);
    let c = transform_vertex(read_vertex_position(data, triangle[2])?, transform);
    Some([a, b, c])
}

pub fn precise_ray_test(
    node: &Node,
    ray: &Ray,
    ignore_back_faces: bool,
) -> Option<(f32, Vector3<f32>)> {
    let mut closest_distance = f32::MAX;
    let mut closest_point = None;

    if let Some(mesh) = node.component_ref::<Mesh>() {
        let transform = mesh.global_transform();

        for surface in mesh.surfaces().iter() {
            let data = surface.data();
            let data = data.data_ref();

            for triangle in data
                .geometry_buffer
                .iter()
                .filter_map(|t| read_triangle(&data, t, &transform))
            {
                if ignore_back_faces {
                    // If normal of the triangle is facing in the same direction as ray's direction,
                    // then we skip such triangle.
                    let normal = (triangle[1] - triangle[0]).cross(&(triangle[2] - triangle[0]));
                    if normal.dot(&ray.dir) >= 0.0 {
                        continue;
                    }
                }

                if let Some(pt) = ray.triangle_intersection_point(&triangle) {
                    let distance = ray.origin.sqr_distance(&pt);

                    if distance < closest_distance {
                        closest_distance = distance;
                        closest_point = Some(pt);
                    }
                }
            }
        }
    }

    closest_point.map(|pt| (closest_distance, pt))
}
// ANCHOR_END: precise_ray_test
