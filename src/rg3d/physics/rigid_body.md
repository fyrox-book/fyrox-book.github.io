# Rigid body node

Rigid body node is the one of main physical entities in the engine. Rigid body nodes can be affected by gravity, 
external forces, other rigid bodies, etc. Use rigid body node everywhere you need natural physical behaviour for
your objects.

## How to create

Use RigidBodyBuilder to create a rigid body instance:

```rust
# extern crate rg3d;
# use rg3d::{
#     core::{algebra::Vector3, pool::Handle},
#     scene::{
#         base::BaseBuilder,
#         collider::{ColliderBuilder, ColliderShape},
#         graph::Graph,
#         node::Node,
#         rigidbody::RigidBodyBuilder,
#     },
# };
fn create_cube_rigid_body(graph: &mut Graph) -> Handle<Node> {
    RigidBodyBuilder::new(BaseBuilder::new().with_children(vec![
            // Rigid body must have at least one collider
            ColliderBuilder::new(BaseBuilder::new())
                .with_shape(ColliderShape::cuboid(0.5, 0.5, 0.5))
                .build(graph),
        ]))
    .with_mass(2.0)
    .with_lin_vel(Vector3::new(0.0, 3.0, 1.0))
    .build(graph)
}
```

## Colliders

Rigid body must have at least one collider to participate in simulation properly, multiple colliders can be used to
create complex shapes from simple shapes, you can create concave objects this way.

## Force and torque

You can apply forces and torque to any rigid body, but only dynamic bodies will be affected. There is two ways of
applying force to a rigid body: at center of mass or at particular point at the body:

```rust
# extern crate rg3d;
# use rg3d::{core::algebra::Vector3, scene::rigidbody::RigidBody};
fn apply_force_and_torque(rigid_body: &mut RigidBody) {
    // Push rigid body forward at the center of mass.
    rigid_body.apply_force(Vector3::new(0.0, 0.0, 1.0));

    // Kick rigid body at the side (this will also make it rotate)
    rigid_body.apply_force_at_point(Vector3::new(0.0, 0.0, 1.0), Vector3::new(1.0, 0.0, 0.0));

    // Turn rigid body around center of mass.
    rigid_body.apply_torque(Vector3::new(0.0, 3.0, 0.0));
}
```

## Kinematic rigid bodies

Sometimes you may want to have direct control over position/rotation of a rigid body and tell the physics engine to not
do simulation for the body. This can be achieved by making the rigid body _kinematic_:

```rust
# extern crate rg3d;
# use rg3d::{
#     core::{algebra::Vector3, pool::Handle},
#     scene::{
#         base::BaseBuilder,
#         collider::{ColliderBuilder, ColliderShape},
#         graph::Graph,
#         node::Node,
#         rigidbody::{RigidBodyBuilder, RigidBodyType},
#     },
# };

fn create_kinematic_rigid_body(graph: &mut Graph) -> Handle<Node> {
    RigidBodyBuilder::new(BaseBuilder::new().with_children(vec![
            // Rigid body must have at least one collider
            ColliderBuilder::new(BaseBuilder::new())
                .with_shape(ColliderShape::cuboid(0.5, 0.5, 0.5))
                .build(graph),
        ]))
    .with_body_type(RigidBodyType::KinematicPositionBased)
    .build(graph)
}
```

## Continuous collision detection

Fast-moving rigid bodies can "fly through" other objects (for example a bullet can completely ignore walls if it is 
moving too fast), this happens because of discrete calculation. This can be fixed by using continuous collision detection,
to enable it use either `.with_ccd_enabled(state)` of `RigidBodyBuilder` or `.set_ccd_enabled(state)` of `RigidBody`.

## 2D rigid bodies

2D rigid bodies have no difference with 3D, except the simulation happens in oXY plane and Z coordinate is ignored.