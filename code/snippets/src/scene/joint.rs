use fyrox::scene::joint::Joint;
use fyrox::scene::rigidbody::RigidBody;
use fyrox::{
    core::pool::Handle,
    scene::{
        base::BaseBuilder,
        graph::Graph,
        joint::{BallJoint, JointBuilder, JointParams},
    },
};

// ANCHOR: create_joint
fn create_joint(
    graph: &mut Graph,
    body1: Handle<RigidBody>,
    body2: Handle<RigidBody>,
) -> Handle<Joint> {
    JointBuilder::new(BaseBuilder::new())
        .with_body1(body1)
        .with_body2(body2)
        .with_params(JointParams::BallJoint(BallJoint {
            x_limits_enabled: false,
            x_limits_angles: Default::default(),
            y_limits_enabled: false,
            y_limits_angles: Default::default(),
            z_limits_enabled: false,
            z_limits_angles: Default::default(),
        }))
        .build(graph)
}
// ANCHOR_END: create_joint
