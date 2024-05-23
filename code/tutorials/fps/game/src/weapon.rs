use fyrox::graph::{BaseSceneGraph, SceneGraph};
use fyrox::{
    core::{
        math, pool::Handle, reflect::prelude::*, type_traits::prelude::*,
        variable::InheritableVariable, visitor::prelude::*,
    },
    resource::{model::ModelResource, model::ModelResourceExtension},
    scene::{animation::AnimationPlayer, node::Node},
    script::{ScriptContext, ScriptMessageContext, ScriptMessagePayload, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "e8adde73-1e96-471c-8531-5c0d16f0c29a")]
#[visit(optional)]
pub struct Weapon {
    // ANCHOR: projectile_field
    projectile: InheritableVariable<Option<ModelResource>>,
    // ANCHOR_END: projectile_field

    // ANCHOR: shot_point
    shot_point: InheritableVariable<Handle<Node>>,
    // ANCHOR_END: shot_point

    // ANCHOR: shot_timer
    shot_interval: InheritableVariable<f32>,

    #[reflect(hidden)]
    shot_timer: f32,
    // ANCHOR_END: shot_timer

    // ANCHOR: animation_player
    animation_player: InheritableVariable<Handle<Node>>,
    // ANCHOR_END: animation_player
}

// ANCHOR: shoot_message
#[derive(Debug)]
pub struct ShootWeaponMessage {}
// ANCHOR_END: shoot_message

impl ScriptTrait for Weapon {
    // ANCHOR: on_start
    fn on_start(&mut self, context: &mut ScriptContext) {
        context
            .message_dispatcher
            .subscribe_to::<ShootWeaponMessage>(context.handle);
    }
    // ANCHOR_END: on_start

    // ANCHOR: on_update
    fn on_update(&mut self, context: &mut ScriptContext) {
        self.shot_timer -= context.dt;
    }
    // ANCHOR_END: on_update

    // ANCHOR: on_message_begin
    fn on_message(
        &mut self,
        message: &mut dyn ScriptMessagePayload,
        ctx: &mut ScriptMessageContext,
    ) {
        // Check if we've received an appropriate message. This is needed because message channel is
        // common across all scripts.
        if message.downcast_ref::<ShootWeaponMessage>().is_some() {
            // ANCHOR_END: on_message_begin

            // ANCHOR: shooting_condition
            if self.shot_timer >= 0.0 {
                return;
            }
            // Reset the timer, this way the next shot cannot be done earlier than the interval.
            self.shot_timer = *self.shot_interval;
            // ANCHOR_END: shooting_condition

            // ANCHOR: recoil_animation
            if let Some(animation_player) = ctx
                .scene
                .graph
                .try_get_mut_of_type::<AnimationPlayer>(*self.animation_player)
            {
                if let Some(animation) = animation_player
                    .animations_mut()
                    .get_value_mut_silent()
                    .iter_mut()
                    .next()
                {
                    animation.rewind();
                    animation.set_enabled(true);
                }
            }
            // ANCHOR_END: recoil_animation

            // ANCHOR: on_message_end
            if let Some(projectile_prefab) = self.projectile.as_ref() {
                // Try to get the position of the shooting point.
                if let Some(shot_point) = ctx
                    .scene
                    .graph
                    .try_get(*self.shot_point)
                    .map(|point| point.global_position())
                {
                    // Shooting direction is just a direction of the weapon (its look vector)
                    let direction = ctx.scene.graph[ctx.handle].look_vector();

                    // Finally instantiate our projectile at the position and direction.
                    projectile_prefab.instantiate_at(
                        ctx.scene,
                        shot_point,
                        math::vector_to_quat(direction),
                    );
                }
            }
        }
    }
    // ANCHOR_END: on_message_end
}
