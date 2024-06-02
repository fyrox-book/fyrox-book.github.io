use fyrox::scene::rigidbody::{RigidBody, RigidBodyType};
use fyrox::{
    core::{algebra::Vector3, pool::Handle},
    scene::{
        base::BaseBuilder,
        collider::{ColliderBuilder, ColliderShape},
        graph::Graph,
        node::Node,
        rigidbody::RigidBodyBuilder,
    },
};

// ANCHOR: create_cube_rigid_body
fn create_cube_rigid_body(graph: &mut Graph) -> Handle<Node> {
    RigidBodyBuilder::new(
        BaseBuilder::new().with_children(&[
            // Rigid body must have at least one collider
            ColliderBuilder::new(BaseBuilder::new())
                .with_shape(ColliderShape::cuboid(0.5, 0.5, 0.5))
                .build(graph),
        ]),
    )
    .with_mass(2.0)
    .with_lin_vel(Vector3::new(0.0, 3.0, 1.0))
    .build(graph)
}
// ANCHOR_END: create_cube_rigid_body

// ANCHOR: apply_force_and_torque
fn apply_force_and_torque(rigid_body: &mut RigidBody) {
    // Push rigid body forward at the center of mass.
    rigid_body.apply_force(Vector3::new(0.0, 0.0, 1.0));

    // Kick rigid body at the side (this will also make it rotate)
    rigid_body.apply_force_at_point(Vector3::new(0.0, 0.0, 1.0), Vector3::new(1.0, 0.0, 0.0));

    // Turn rigid body around center of mass.
    rigid_body.apply_torque(Vector3::new(0.0, 3.0, 0.0));
}
// ANCHOR_END: apply_force_and_torque

// ANCHOR: create_kinematic_rigid_body
fn create_kinematic_rigid_body(graph: &mut Graph) -> Handle<Node> {
    RigidBodyBuilder::new(
        BaseBuilder::new().with_children(&[
            // Rigid body must have at least one collider
            ColliderBuilder::new(BaseBuilder::new())
                .with_shape(ColliderShape::cuboid(0.5, 0.5, 0.5))
                .build(graph),
        ]),
    )
    .with_body_type(RigidBodyType::KinematicPositionBased)
    .build(graph)
}
// ANCHOR_END: create_kinematic_rigid_body
