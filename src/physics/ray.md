# Ray Casting

Ray casting allows you to query intersections of a ray with rigid bodies in a scene. Typical usage for ray casting is
hit-scan weapons (weapons that shoots high-speed projectiles), AI collision avoidance, etc. To query intersections,
use physics world instance of a scene graph:

```rust,no_run
{{#include ../code/snippets/src/scene/ray.rs:do_ray_cast}}
```

The function above will return a collection of intersections that are sorted by intersection distance (a distance from
beginning of the ray to an intersection point). Each intersection is represented by the following structure:

```rust,no_run
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

```rust,no_run
{{#include ../code/snippets/src/scene/ray.rs:do_static_ray_cast}}
```

`usage_example` shows how to use the `do_static_ray_cast` function - all you need to do is to specify maximum amount of
intersections you're interested in as a generic parameter.