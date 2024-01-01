# Signals

In some cases you may need to perform an action when at certain time of your animation. It could be a footstep sound,
when foot touches ground, grenade tossing, etc. This could be done via animation signals. Animation signal is just 
a named marker that has time position at an animation timeline. It will be emitted when animation playback time passes
it (left-to-right or right-to-left depending on the actual speed of your animation). All you need to do, is to catch
these signals in your game code and do the desired actions.

## How to add

As usual, there are two possible ways of adding animation signals - from the animation editor and from code. 

### From animation editor

To add a signal to some animation, select an animation player, open the animation editor, select some animation in 
it. Now all you need to do is to right-click on the timeline and press `Add Signal`.

![Add Signal](signal_add.png)

After the signal is added, you can select it and edit its properties in the inspector. Also, you can drag it on the 
timeline to adjust its position.

![Edit Signal](signal_edit.png)

Set a meaningful name to the signal, and it is pretty much done - all you need to do next is to write signal handling
code in your game. See [the next section](#reacting-to-signal-events) to learn how to do it.

### From code

A signal could also be added from code, this requires knowing a handle of your animation player and a name/handle of
your animation. Please note the comment about signal's uuid in the code below.

```rust ,no_run
# extern crate fyrox;
# use fyrox::{
#     animation::AnimationSignal,
#     core::{pool::Handle, uuid::uuid},
#     scene::{animation::AnimationPlayer, graph::Graph, node::Node},
# };
# 
fn add_signal(
    animation_player: Handle<Node>,
    animation_name: &str,
    signal_name: &str,
    graph: &mut Graph,
) {
    if let Some(animation_player) =
        graph.try_get_mut_of_type::<AnimationPlayer>(animation_player)
    {
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
```

## Reacting to signal events

When you have your signals ready for use, all you need to do is to react to the signals somehow. This is very simple:
just borrow your animation from the animation player and pop animation event one-by-one from internal queue:

```rust ,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     scene::{animation::AnimationPlayer, graph::Graph, node::Node},
# };
# 
fn react_to_signal_events(
    animation_player: Handle<Node>,
    animation_name: &str,
    signal_name: &str,
    graph: &mut Graph,
) {
    if let Some(animation_player) =
        graph.try_get_mut_of_type::<AnimationPlayer>(animation_player)
    {
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
```

You can do pretty much anything when reacting to signals. For example, this could be a prefab instantiation to
create smoke effect under the feet, playing a footstep sound, etc.

## Events from ABSM

Animation blending state machines are able to collect events from the currently playing animations using 
different strategies. This ability prevents you from tedious manual animation events collection from a bunch
of animations manually. 

```rust ,no_run
# extern crate fyrox;
# use fyrox::{
#     animation::machine::{
#         layer::LayerAnimationEventsCollection, node::AnimationEventCollectionStrategy,
#     },
#     core::pool::Handle,
#     scene::{
#         animation::{absm::AnimationBlendingStateMachine, AnimationPlayer},
#         node::Node,
#     },
#     script::ScriptContext,
# };
# 
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
```

This function collects all animation events from all active animations in the specified ABSM (in its first
layer). The arguments to it are the following:

- `absm` - a handle to an animation blending state machine node.
- `strategy` - event collection strategy, which includes all events collection, max and min weight. The 
last two may be used if you're getting a lot of events and want to get events from the animations with max or
min weights respectively.
- `ctx` - current script context, available in pretty much any script methods.