use crate::Game;
use fyrox::{
    core::{
        algebra::Vector2, pool::Handle, reflect::prelude::*, type_traits::prelude::*,
        variable::InheritableVariable, visitor::prelude::*,
    },
    graph::BaseSceneGraph,
    scene::{
        animation::spritesheet::SpriteSheetAnimation,
        dim2::{physics::RayCastOptions, rectangle::Rectangle},
        node::Node,
    },
    script::{ScriptContext, ScriptTrait},
};

#[derive(Visit, Reflect, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "d2786d36-a0af-4e67-916a-438af62f818b")]
#[visit(optional)]
pub struct Bot {
    // ANCHOR: ground_probe_fields
    ground_probe: InheritableVariable<Handle<Node>>,
    ground_probe_distance: InheritableVariable<f32>,
    // ANCHOR_END: ground_probe_fields

    // ANCHOR: movement_fields
    speed: InheritableVariable<f32>,

    #[visit(skip)]
    #[reflect(hidden)]
    direction: f32,
    // ANCHOR_END: movement_fields

    // ANCHOR: target_fields
    #[visit(skip)]
    #[reflect(hidden)]
    target: Handle<Node>,
    // ANCHOR_END: target_fields

    // ANCHOR: animation_fields
    rectangle: InheritableVariable<Handle<Node>>,
    animations: Vec<SpriteSheetAnimation>,
    current_animation: InheritableVariable<u32>,
    // ANCHOR_END: animation_fields
}

// ANCHOR: bot_defaults
impl Default for Bot {
    fn default() -> Self {
        Self {
            ground_probe: Default::default(),
            ground_probe_distance: 2.0.into(),
            speed: 1.0.into(),
            direction: 1.0,
            target: Default::default(),
            rectangle: Default::default(),
            animations: Default::default(),
            current_animation: Default::default(),
        }
    }
}
// ANCHOR_END: bot_defaults

// ANCHOR: has_ground_in_front
impl Bot {
    fn has_ground_in_front(&self, ctx: &ScriptContext) -> bool {
        // Do ground check using ray casting from the ground probe position down at some distance.
        let Some(ground_probe) = ctx.scene.graph.try_get(*self.ground_probe) else {
            return false;
        };

        let ground_probe_position = ground_probe.global_position().xy();

        let mut intersections = Vec::new();
        ctx.scene.graph.physics2d.cast_ray(
            RayCastOptions {
                ray_origin: ground_probe_position.into(),
                // Cast the ray
                ray_direction: Vector2::new(0.0, -*self.ground_probe_distance),
                max_len: *self.ground_probe_distance,
                groups: Default::default(),
                // Make sure the closest intersection will be first in the list of intersections.
                sort_results: true,
            },
            &mut intersections,
        );

        if let Some(first_intersection) = intersections.first() {
            if first_intersection
                .position
                .coords
                .metric_distance(&ground_probe_position)
                <= *self.ground_probe_distance
            {
                return true;
            }
        }

        false
    }
    // ANCHOR_END: has_ground_in_front

    // ANCHOR: search_target
    fn search_target(&mut self, ctx: &mut ScriptContext) {
        let game = ctx.plugins.get::<Game>();

        let self_position = ctx.scene.graph[ctx.handle].global_position();

        let Some(player) = ctx.scene.graph.try_get(game.player) else {
            return;
        };

        let player_position = player.global_position();

        let signed_distance = player_position.x - self_position.x;
        if signed_distance.abs() < 3.0 && signed_distance.signum() == self.direction.signum() {
            self.target = game.player;
        } else {
            self.target = Handle::NONE;
        }
    }
    // ANCHOR_END: search_target
}

impl ScriptTrait for Bot {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
    }

    fn on_start(&mut self, context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
    }

    // ANCHOR: search_target_call
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        self.search_target(ctx);
        // ANCHOR_END: search_target_call

        // ANCHOR: applying_animation
        if let Some(current_animation) = self.animations.get_mut(*self.current_animation as usize) {
            current_animation.update(ctx.dt);

            if let Some(sprite) = ctx
                .scene
                .graph
                .try_get_mut(*self.rectangle)
                .and_then(|n| n.cast_mut::<Rectangle>())
            {
                // Set new frame to the sprite.
                sprite
                    .material()
                    .data_ref()
                    .set_texture(&"diffuseTexture".into(), current_animation.texture())
                    .unwrap();
                sprite.set_uv_rect(
                    current_animation
                        .current_frame_uv_rect()
                        .unwrap_or_default(),
                );
            }
        }
        // ANCHOR_END: applying_animation
    }
}
