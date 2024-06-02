use fyrox::core::pool::Handle;
use fyrox::generic_animation::machine::PoseWeight;
use fyrox::scene::animation::absm::{
    BlendAnimations, BlendPose, Machine, PlayAnimation, PoseNode, State, Transition,
};

// ANCHOR: create_absm
fn create_absm() -> Machine {
    // Assume that these are correct handles.
    let idle_animation = Handle::default();
    let walk_animation = Handle::default();
    let aim_animation = Handle::default();

    let mut machine = Machine::new();

    let root_layer = machine.layers_mut().first_mut().unwrap();

    let aim = root_layer.add_node(PoseNode::PlayAnimation(PlayAnimation::new(aim_animation)));
    let walk = root_layer.add_node(PoseNode::PlayAnimation(PlayAnimation::new(walk_animation)));

    // Blend two animations together
    let blend_aim_walk =
        root_layer.add_node(PoseNode::BlendAnimations(BlendAnimations::new(vec![
            BlendPose::new(PoseWeight::Constant(0.75), aim),
            BlendPose::new(PoseWeight::Constant(0.25), walk),
        ])));

    let walk_state = root_layer.add_state(State::new("Walk", blend_aim_walk));

    let idle = root_layer.add_node(PoseNode::PlayAnimation(PlayAnimation::new(idle_animation)));
    let idle_state = root_layer.add_state(State::new("Idle", idle));

    root_layer.add_transition(Transition::new(
        "Walk->Idle",
        walk_state,
        idle_state,
        1.0,
        "WalkToIdle",
    ));
    root_layer.add_transition(Transition::new(
        "Idle->Walk",
        idle_state,
        walk_state,
        1.0,
        "IdleToWalk",
    ));

    machine
}
// ANCHOR_END: create_absm
