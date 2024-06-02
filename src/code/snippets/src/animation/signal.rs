use crate::scripting::context::ScriptContext;
use fyrox::generic_animation::machine::node::AnimationEventCollectionStrategy;
use fyrox::generic_animation::AnimationSignal;
use fyrox::graph::SceneGraph;
use fyrox::scene::animation::absm::{
    AnimationBlendingStateMachine, LayerAnimationEventsCollection,
};
use fyrox::{
    core::{pool::Handle, uuid::uuid},
    scene::{animation::AnimationPlayer, graph::Graph, node::Node},
};

// ANCHOR: add_signal
fn add_signal(
    animation_player: Handle<Node>,
    animation_name: &str,
    signal_name: &str,
    graph: &mut Graph,
) {
    if let Some(animation_player) = graph.try_get_mut_of_type::<AnimationPlayer>(animation_player) {
        let animations = animation_player.animations_mut().get_value_mut_silent();
        if let Some((_, animation)) = animations.find_by_name_mut(animation_name) {
            // This uuid should be unique, you could also use Uuid::new_v4() method, but it
            // will generate random uuid on every call. This uuid does not used by the engine,
            // it is used only for searching and useful when you have multiple signals with the
            // same name, but with different uuid.
            let uuid = uuid!("6d472c99-e1d3-44fd-81fd-5eb83bbafdf7");

            animation.add_signal(AnimationSignal::new(uuid, signal_name, 0.5));
        }
    }
}
// ANCHOR_END: add_signal

// ANCHOR: react_to_signal_events
fn react_to_signal_events(
    animation_player: Handle<Node>,
    animation_name: &str,
    signal_name: &str,
    graph: &mut Graph,
) {
    if let Some(animation_player) = graph.try_get_mut_of_type::<AnimationPlayer>(animation_player) {
        let animations = animation_player.animations_mut().get_value_mut_silent();

        // Ideally, animation fetching should be done via its handle (the first argument of the
        // tuple returned by find_by_name_mut/ref), but for the sake of simplicity we'll do
        // this by name.
        if let Some((_, animation)) = animations.find_by_name_mut(animation_name) {
            // Pop every event one-by-one and do something.
            while let Some(signal) = animation.pop_event() {
                // We're interested only in signals with specific name.
                if signal.name == signal_name {
                    println!("Signal event {} has occurred!", signal.name);
                }
            }
        }
    }
}
// ANCHOR_END: react_to_signal_events

// ANCHOR: collect_events_from_absm
fn collect_events_from_absm(
    absm: Handle<Node>,
    strategy: AnimationEventCollectionStrategy,
    ctx: &mut ScriptContext,
) -> LayerAnimationEventsCollection {
    if let Some(absm) = ctx
        .scene
        .graph
        .try_get_of_type::<AnimationBlendingStateMachine>(absm)
    {
        if let Some(animation_player) = ctx
            .scene
            .graph
            .try_get_of_type::<AnimationPlayer>(absm.animation_player())
        {
            // Fetch a layer first, it could be any layer of the ABMS, but for simplicity
            // we'll use the first layer.
            if let Some(layer) = absm.machine().layers().first() {
                return layer.collect_active_animations_events(
                    absm.machine().parameters(),
                    animation_player.animations(),
                    strategy,
                );
            }
        }
    }

    Default::default()
}
// ANCHOR_END: collect_events_from_absm
