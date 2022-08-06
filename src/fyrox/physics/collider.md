# Collider node

Collider is a geometrical shape that is used for collision detection, contact manifold generation, etc. Colliders are used
in pair with rigid bodies, they make rigid body participate in collisions.

## Shapes

Collider can have almost any shape, the engine offers the following shapes for 3D:

- Ball - dynamic sphere shape.
- Cylinder - dynamic cylinder shape.
- Cone - dynamic cone shape.
- Cuboid - dynamic box shape.
- Capsule - dynamic capsule shape.
- Segment - dynamic segment ("line") shape
- Triangle - simple dynamic triangle shape
- Triangle mesh - static concave shape, can be used together with any static level geometry (wall, floors, ceilings,
anything else)
- Height field - static height field shape, can be used together with terrains.
- Polyhedron - dynamic concave shape.

Also, there is a similar, but smaller set for 2D (because some shapes degenerate in 2D):

- Ball - dynamic circle shape.
- Cuboid - dynamic rectangle shape.
- Capsule - dynamic capsule shape.
- Segment - dynamic segment ("line") shape.
- Triangle - dynamic triangle shape.
- Trimesh - static triangle mesh shape.
- Heightfield - static height field shape.

_Dynamic_ in both lists means that such shapes can be used together with _dynamic_ rigid bodies, they'll correctly handle
all collisions and simulation will look as it should. _Static_ means that such shape should be used only with _static_
rigid bodies.

## How to create

Use ColliderBuilder to create an instance of collider from code with any shape you want.

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     scene::{
#         base::BaseBuilder,
#         collider::{ColliderBuilder, ColliderShape},
#         graph::Graph,
#         node::Node,
#     },
# };
fn create_capsule_collider(graph: &mut Graph) -> Handle<Node> {
    ColliderBuilder::new(BaseBuilder::new())
        .with_shape(ColliderShape::capsule_y(0.5, 0.2))
        .with_friction(1.0)
        .build(graph)
}
```

In the editor you can use `MainMenu -> Create -> Physics -> Collider`, or right-click on a node in `World Viewer` and
select `Add Child -> Physics -> Collider`. Collider must be direct child of a rigid body, colliders do nothing on
their own!