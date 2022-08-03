# Ray Casting

Ray casting allows you to query intersections of a ray with rigid bodies in a scene. Typical usage for ray casting is
hit-scan weapons (weapons that shoots high-speed projectiles), AI collision avoidance, etc. To query intersections,
use physics world instance of a scene graph:

```rust
# extern crate fyrox;
# use fyrox::{
#     core::algebra::{Point3, Vector3},
#     scene::graph::{
#         physics::{Intersection, RayCastOptions},
#         Graph,
#     },
# };
# 
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
```

The function above will return a collection of intersections that are sorted by intersection distance (a distance from
beginning of the ray to an intersection point). Each intersection is represented by the following structure:

```rust
pub struct Intersection {
    pub collider: Handle<Node>,
    pub normal: Vector3<f32>,
    pub position: Point3<f32>,
    pub feature: FeatureId,
    pub toi: f32,
}
```

- `collider` - a handle of the collider with which intersection was detected. To obtain a handle to rigid body, borrow
the `collider` and fetch its `parent` field: `graph[collider].parent()`.
- `normal` - a normal at the intersection position in world coordinates.
- `position` - a position of the intersection in world coordinates.
- `feature` - additional data that contains a kind of the feature with which intersection was detected as well as its
index. FeatureId::Face might have index that is greater than amount of triangles in a triangle mesh, this means that 
intersection was detected from "back" side of a face. To "fix" that index, simply subtract amount of triangles of a 
triangle mesh from the value.
- `toi` - (`time of impact`) a distance from ray's origin to `position`.

## Avoiding unnecessary allocations

As you might've noticed, the function above return `Vec<Intersection>` which allocates intersections on heap. This is
relatively slow and could be sped up a lot by using static array on stack:

```rust
# extern crate fyrox;
# use fyrox::{
#     core::{
#         algebra::{Point3, Vector3},
#         arrayvec::ArrayVec,
#     },
#     scene::graph::{
#         physics::{Intersection, RayCastOptions},
#         Graph,
#     },
# };
# 
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
```

`usage_example` shows how to use the `do_static_ray_cast` function - all you need to do is to specify maximum amount of
intersections you're interested in as a generic parameter.