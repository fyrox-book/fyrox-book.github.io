use fyrox::asset::manager::ResourceManager;
use fyrox::core::algebra::{Matrix4, Point3, Vector2, Vector3};
use fyrox::core::futures::executor::block_on;
use fyrox::core::math::ray::Ray;
use fyrox::core::math::{TriangleDefinition, Vector3Ext};
use fyrox::core::pool::Handle;
use fyrox::renderer::Renderer;
use fyrox::resource::texture::{Texture, TextureWrapMode};
use fyrox::scene::base::BaseBuilder;
use fyrox::scene::camera::{
    Camera, CameraBuilder, ColorGradingLut, OrthographicProjection, PerspectiveProjection,
    Projection, SkyBox, SkyBoxBuilder,
};
use fyrox::scene::graph::Graph;
use fyrox::scene::mesh::buffer::{VertexAttributeUsage, VertexReadTrait};
use fyrox::scene::mesh::surface::SurfaceData;
use fyrox::scene::mesh::Mesh;
use fyrox::scene::node::Node;
use fyrox::scene::Scene;

// ANCHOR: create_camera
fn create_camera(scene: &mut Scene) -> Handle<Node> {
    CameraBuilder::new(BaseBuilder::new())
        // Set some properties.
        .with_fov(80.0f32.to_radians())
        .with_z_far(256.0)
        .build(&mut scene.graph)
}
// ANCHOR_END: create_camera

// ANCHOR: create_perspective_camera
fn create_perspective_camera(graph: &mut Graph) -> Handle<Node> {
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
fn create_orthographic_camera(graph: &mut Graph) -> Handle<Node> {
    CameraBuilder::new(BaseBuilder::new())
        .with_projection(Projection::Orthographic(OrthographicProjection {
            vertical_size: 5.0,
            z_near: 0.025,
            z_far: 1024.0,
        }))
        .build(graph)
}
// ANCHOR_END: create_orthographic_camera

// ANCHOR: create_camera_with_skybox
async fn create_skybox(resource_manager: ResourceManager) -> SkyBox {
    // Load skybox textures in parallel.
    let (front, back, left, right, top, bottom) = fyrox::core::futures::join!(
        resource_manager.request::<Texture>("path/to/front.jpg"),
        resource_manager.request::<Texture>("path/to/back.jpg"),
        resource_manager.request::<Texture>("path/to/left.jpg"),
        resource_manager.request::<Texture>("path/to/right.jpg"),
        resource_manager.request::<Texture>("path/to/up.jpg"),
        resource_manager.request::<Texture>("path/to/down.jpg")
    );

    // Unwrap everything.
    let skybox = SkyBoxBuilder {
        front: Some(front.unwrap()),
        back: Some(back.unwrap()),
        left: Some(left.unwrap()),
        right: Some(right.unwrap()),
        top: Some(top.unwrap()),
        bottom: Some(bottom.unwrap()),
    }
    .build()
    .unwrap();

    // Set S and T coordinate wrap mode, ClampToEdge will remove any possible seams on edges
    // of the skybox.
    let skybox_texture = skybox.cubemap().unwrap();
    let mut data = skybox_texture.data_ref();
    data.set_s_wrap_mode(TextureWrapMode::ClampToEdge);
    data.set_t_wrap_mode(TextureWrapMode::ClampToEdge);

    skybox
}

fn create_camera_with_skybox(scene: &mut Scene, resource_manager: ResourceManager) -> Handle<Node> {
    CameraBuilder::new(BaseBuilder::new())
        .with_skybox(block_on(create_skybox(resource_manager)))
        .build(&mut scene.graph)
}
// ANCHOR_END: create_camera_with_skybox

// ANCHOR: create_camera_with_lut
fn create_camera_with_lut(scene: &mut Scene, resource_manager: ResourceManager) -> Handle<Node> {
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

    if let Some(mesh) = node.query_component_ref::<Mesh>() {
        let transform = mesh.global_transform();

        for surface in mesh.surfaces().iter() {
            let data = surface.data();
            let data = data.lock();

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
