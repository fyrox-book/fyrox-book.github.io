use fyrox::core::arrayvec::ArrayVec;
use fyrox::{
    core::algebra::{Point3, Vector3},
    scene::graph::{
        physics::{Intersection, RayCastOptions},
        Graph,
    },
};

// ANCHOR: do_ray_cast
fn do_ray_cast(graph: &mut Graph, begin: Vector3<f32>, end: Vector3<f32>) -> Vec<Intersection> {
    let mut buffer = Vec::new();

    let ray_direction = end - begin;

    graph.physics.cast_ray(
        RayCastOptions {
            ray_origin: Point3::from(begin),
            ray_direction,
            max_len: ray_direction.norm(),
            groups: Default::default(),
            sort_results: true,
        },
        &mut buffer,
    );

    buffer
}
// ANCHOR_END: do_ray_cast

// ANCHOR: do_static_ray_cast
fn do_static_ray_cast<const N: usize>(
    graph: &mut Graph,
    begin: Vector3<f32>,
    end: Vector3<f32>,
) -> ArrayVec<Intersection, N> {
    let mut buffer = ArrayVec::<Intersection, N>::new();

    let ray_direction = end - begin;

    graph.physics.cast_ray(
        RayCastOptions {
            ray_origin: Point3::from(begin),
            ray_direction,
            max_len: ray_direction.norm(),
            groups: Default::default(),
            sort_results: true,
        },
        &mut buffer,
    );

    buffer
}

fn usage_example(graph: &mut Graph, begin: Vector3<f32>, end: Vector3<f32>) {
    // Fetch first 32 intersections.
    dbg!(do_static_ray_cast::<32>(graph, begin, end));
}
// ANCHOR_END: do_static_ray_cast
