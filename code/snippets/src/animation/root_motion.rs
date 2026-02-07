use fyrox::core::algebra::Vector3;
use fyrox::generic_animation::RootMotionSettings;
use fyrox::graph::SceneGraph;
use fyrox::scene::animation::absm::AnimationBlendingStateMachine;
use fyrox::scene::rigidbody::RigidBody;
use fyrox::{
    core::pool::Handle,
    scene::animation::Animation,
    scene::{animation::AnimationPlayer, node::Node},
    script::ScriptContext,
};

// ANCHOR: setup_root_motion
fn setup_root_motion(
    animation_player: Handle<AnimationPlayer>,
    animation: Handle<Animation>,
    root_node: Handle<Node>,
    ctx: &mut ScriptContext,
) {
    if let Ok(animation_player) = ctx.scene.graph.try_get_mut(animation_player) {
        if let Ok(animation) = animation_player.animations_mut().try_get_mut(animation) {
            animation.set_root_motion_settings(Some(RootMotionSettings {
                node: root_node,
                ignore_x_movement: false,
                ignore_y_movement: true,
                ignore_z_movement: false,
                ignore_rotations: true,
            }))
        }
    }
}
// ANCHOR_END: setup_root_motion

// ANCHOR: fetch_and_apply_root_motion
fn fetch_and_apply_root_motion(
    absm: Handle<AnimationBlendingStateMachine>,
    rigid_body: Handle<RigidBody>,
    character_model: Handle<Node>,
    ctx: &mut ScriptContext,
) {
    // Step 1. Fetch the velocity vector from the animation blending state machine.
    let transform = ctx.scene.graph[character_model].global_transform();
    let mut velocity = Vector3::default();
    if let Ok(state_machine) = ctx.scene.graph.try_get(absm) {
        if let Some(root_motion) = state_machine.machine().pose().root_motion() {
            velocity = transform
                .transform_vector(&root_motion.delta_position)
                .scale(1.0 / ctx.dt);
        }
    }

    // Step 2. Apply the velocity to the rigid body and lock rotations.
    if let Ok(body) = ctx.scene.graph.try_get_mut(rigid_body) {
        body.set_ang_vel(Default::default());
        body.set_lin_vel(Vector3::new(velocity.x, body.lin_vel().y, velocity.z));
    }
}
// ANCHOR_END: fetch_and_apply_root_motion
