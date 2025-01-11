pub mod blending;
pub mod root_motion;
pub mod signal;

use fyrox::{
    asset::manager::ResourceManager,
    core::{
        math::curve::{Curve, CurveKey, CurveKeyKind},
        pool::Handle,
        reflect::prelude::*,
        type_traits::prelude::*,
        visitor::prelude::*,
    },
    generic_animation::{
        container::{TrackDataContainer, TrackValueKind},
        track::TrackBinding,
        value::ValueBinding,
    },
    graph::SceneGraph,
    resource::model::{Model, ModelResourceExtension},
    scene::{
        animation::{
            spritesheet::SpriteSheetAnimation, Animation, AnimationPlayer, AnimationPlayerBuilder,
            AnimationPoseExt, Track,
        },
        base::BaseBuilder,
        dim2::rectangle::Rectangle,
        graph::Graph,
        node::Node,
        pivot::PivotBuilder,
        Scene,
    },
    script::{ScriptContext, ScriptTrait},
};

// ANCHOR: animation
#[derive(Default, Clone, Debug, Reflect, Visit, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "aeebb95f-8e32-490e-971c-c22417bd19c5")]
#[visit(optional)]
struct Player {
    animation: SpriteSheetAnimation,
}

impl ScriptTrait for Player {
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        // Update the animation first, it will switch current frame automatically if needed.
        self.animation.update(ctx.dt);

        if let Some(sprite) = ctx
            .scene
            .graph
            .try_get_mut(ctx.handle)
            .and_then(|n| n.cast_mut::<Rectangle>())
        {
            // Assign the texture from the animation to the sprite first.
            sprite
                .material()
                .data_ref()
                .texture_mut("diffuseTexture")
                .unwrap()
                .value = self.animation.texture();

            // Set the current animation's UV rect to the sprite.
            sprite.set_uv_rect(self.animation.current_frame_uv_rect().unwrap_or_default());
        }
    }
}
// ANCHOR_END: animation

// ANCHOR: create_animation
fn create_animation(node: Handle<Node>) -> Animation {
    let mut frames_container = TrackDataContainer::new(TrackValueKind::Vector3);
    // We'll animate only X coordinate (at index 0).
    frames_container.curves_mut()[0] = Curve::from(vec![
        CurveKey::new(0.5, 2.0, CurveKeyKind::Linear),
        CurveKey::new(0.75, 1.0, CurveKeyKind::Linear),
        CurveKey::new(1.0, 3.0, CurveKeyKind::Linear),
    ]);
    // Create a track that will animated the node using the curve above.
    let track = Track::new(frames_container, ValueBinding::Position);
    // Finally create an animation and set its time slice and turn it on.
    let mut animation = Animation::default();
    animation.add_track_with_binding(TrackBinding::new(node), track);
    animation.set_time_slice(0.0..1.0);
    animation.set_enabled(true);
    animation
}

fn use_animation() {
    // Create a graph with a node.
    let mut graph = Graph::new();
    let some_node = PivotBuilder::new(BaseBuilder::new()).build(&mut graph);
    // Create the animation.
    let mut animation = create_animation(some_node);
    // Emulate some ticks (like it was updated from the main loop of your game).
    for _ in 0..10 {
        animation.tick(1.0 / 60.0);
        animation.pose().apply(&mut graph);
    }
}
// ANCHOR_END: create_animation

// ANCHOR: create_animated_character
async fn create_animated_character(
    scene: &mut Scene,
    resource_manager: &ResourceManager,
) -> (Handle<Node>, Handle<Node>) {
    // Load a character model first.
    let character_resource = resource_manager
        .request::<Model>("path/to/my/character.fbx")
        .await
        .unwrap();

    // Create its instance.
    let character_instance = character_resource.instantiate(scene);

    // Create a new animation player.
    let animation_player = AnimationPlayerBuilder::new(BaseBuilder::new()).build(&mut scene.graph);

    // Load an animation.
    let animation_resource = resource_manager
        .request::<Model>("path/to/my/animation.fbx")
        .await
        .unwrap();

    // "Instantiate" an animation from the animation resource to the animation player.
    // You can call this method multiple times with different animations, each time it
    // will create a new animation instance and put it in the animation player.
    let _animations = animation_resource.retarget_animations_to_player(
        character_instance,
        animation_player,
        &mut scene.graph,
    );

    (character_instance, animation_player)
}
// ANCHOR_END: create_animated_character

// ANCHOR: enable_animation
fn enable_animation(animation_player: Handle<Node>, graph: &mut Graph, name: &str, enabled: bool) {
    if let Some(animation_player) = graph.try_get_mut_of_type::<AnimationPlayer>(animation_player) {
        // `get_value_mut_silent` prevents marking the variable as modified (see Property Inheritance
        // chapter for more info).
        let animations = animation_player.animations_mut().get_value_mut_silent();

        // Find an animation with the given name.
        if let Some((_animation_handle, animation)) = animations.find_by_name_mut(name) {
            // You could also store _animation_handle somewhere and use  animations.get_mut/get(handle)
            // to fetch an animation faster.

            // Turn the animation on/off.
            animation.set_enabled(enabled);
        }
    }
}
// ANCHOR_END: enable_animation
