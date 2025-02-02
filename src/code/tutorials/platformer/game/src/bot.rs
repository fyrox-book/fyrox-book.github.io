// ANCHOR: imports
use crate::Game;
use fyrox::{
    core::{
        algebra::{Vector2, Vector3},
        pool::Handle,
        reflect::prelude::*,
        type_traits::prelude::*,
        variable::InheritableVariable,
        visitor::prelude::*,
    },
    graph::{BaseSceneGraph, SceneGraph},
    scene::{
        animation::spritesheet::SpriteSheetAnimation,
        dim2::{
            collider::Collider, physics::RayCastOptions, rectangle::Rectangle, rigidbody::RigidBody,
        },
        node::Node,
        rigidbody::RigidBodyType,
    },
    script::{ScriptContext, ScriptTrait},
};
// ANCHOR_END: imports

#[derive(Visit, Reflect, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "d2786d36-a0af-4e67-916a-438af62f818b")]
#[visit(optional)]
pub struct Bot {
    // ANCHOR: visual_fields
    rectangle: InheritableVariable<Handle<Node>>,
    // ANCHOR_END: visual_fields

    // ANCHOR: ground_probe_fields
    ground_probe: InheritableVariable<Handle<Node>>,
    ground_probe_distance: InheritableVariable<f32>,
    ground_probe_timeout: f32,
    // ANCHOR_END: ground_probe_fields

    // ANCHOR: movement_fields
    speed: InheritableVariable<f32>,
    direction: f32,
    front_obstacle_sensor: InheritableVariable<Handle<Node>>,
    back_obstacle_sensor: InheritableVariable<Handle<Node>>,
    // ANCHOR_END: movement_fields

    // ANCHOR: target_fields
    #[visit(skip)]
    #[reflect(hidden)]
    target: Handle<Node>,
    // ANCHOR_END: target_fields

    // ANCHOR: animation_fields
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
            ground_probe_timeout: 0.0,
            speed: 1.0.into(),
            direction: 1.0,
            front_obstacle_sensor: Default::default(),
            back_obstacle_sensor: Default::default(),
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

        for intersection in intersections {
            let Some(collider) = ctx.scene.graph.try_get(intersection.collider) else {
                continue;
            };

            let Some(rigid_body) = ctx
                .scene
                .graph
                .try_get_of_type::<RigidBody>(collider.parent())
            else {
                continue;
            };

            if rigid_body.body_type() == RigidBodyType::Static
                && intersection
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
        if signed_distance.abs() < 3.0 && signed_distance.signum() != self.direction.signum() {
            self.target = game.player;
        }
    }
    // ANCHOR_END: search_target

    // ANCHOR: do_move
    fn do_move(&mut self, ctx: &mut ScriptContext) {
        let Some(rigid_body) = ctx.scene.graph.try_get_mut_of_type::<RigidBody>(ctx.handle) else {
            return;
        };

        let y_vel = rigid_body.lin_vel().y;

        rigid_body.set_lin_vel(Vector2::new(-*self.speed * self.direction, y_vel));

        // Also, inverse the sprite along the X axis.
        let Some(rectangle) = ctx.scene.graph.try_get_mut(*self.rectangle) else {
            return;
        };

        rectangle.local_transform_mut().set_scale(Vector3::new(
            2.0 * self.direction.signum(),
            2.0,
            1.0,
        ));
    }
    // ANCHOR_END: do_move

    // ANCHOR: has_obstacles
    fn has_obstacles(&mut self, ctx: &mut ScriptContext) -> bool {
        let graph = &ctx.scene.graph;

        // Select the sensor using current walking direction.
        let sensor_handle = if self.direction < 0.0 {
            *self.back_obstacle_sensor
        } else {
            *self.front_obstacle_sensor
        };

        // Check if it intersects something.
        let Some(obstacle_sensor) = graph.try_get_of_type::<Collider>(sensor_handle) else {
            return false;
        };

        for intersection in obstacle_sensor
            .intersects(&ctx.scene.graph.physics2d)
            .filter(|i| i.has_any_active_contact)
        {
            for collider_handle in [intersection.collider1, intersection.collider2] {
                let Some(other_collider) = graph.try_get_of_type::<Collider>(collider_handle)
                else {
                    continue;
                };

                let Some(rigid_body) = graph.try_get_of_type::<RigidBody>(other_collider.parent())
                else {
                    continue;
                };

                if rigid_body.body_type() == RigidBodyType::Static {
                    return true;
                }
            }
        }

        false
    }
    // ANCHOR_END: has_obstacles
}

impl ScriptTrait for Bot {
    // ANCHOR: search_target_call
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        self.search_target(ctx);
        // ANCHOR_END: search_target_call

        // ANCHOR: check_for_obstacles
        if self.has_obstacles(ctx) {
            self.direction = -self.direction;
        }
        // ANCHOR_END: check_for_obstacles

        // ANCHOR: ground_checks
        self.ground_probe_timeout -= ctx.dt;
        if self.ground_probe_timeout <= 0.0 {
            if !self.has_ground_in_front(ctx) {
                self.direction = -self.direction;
            }
            self.ground_probe_timeout = 0.3;
        }
        // ANCHOR_END: ground_checks

        // ANCHOR: move_to_target
        if self.target.is_some() {
            let target_position = ctx.scene.graph[self.target].global_position();
            let self_position = ctx.scene.graph[ctx.handle].global_position();
            self.direction = (self_position.x - target_position.x).signum();

            // Stand still while attacking.
            if target_position.metric_distance(&self_position) > 1.1 {
                self.speed.set_value_and_mark_modified(1.2);
            } else {
                self.speed.set_value_and_mark_modified(0.0);
            }
        }
        // ANCHOR_END: move_to_target

        // ANCHOR: do_move_call
        self.do_move(ctx);
        // ANCHOR_END: do_move_call

        // ANCHOR: animation_switching
        if self.direction != 0.0 {
            self.current_animation.set_value_and_mark_modified(2);
        }
        if self.target.is_some() {
            let target_position = ctx.scene.graph[self.target].global_position();
            let self_position = ctx.scene.graph[ctx.handle].global_position();
            if target_position.metric_distance(&self_position) < 1.1 {
                self.current_animation.set_value_and_mark_modified(0);
            }
        }
        // ANCHOR_END: animation_switching

        // ANCHOR: applying_animation
        if let Some(current_animation) = self.animations.get_mut(*self.current_animation as usize) {
            current_animation.update(ctx.dt);

            if let Some(sprite) = ctx
                .scene
                .graph
                .try_get_mut_of_type::<Rectangle>(*self.rectangle)
            {
                // Set new frame to the sprite.
                sprite
                    .material()
                    .data_ref()
                    .bind("diffuseTexture", current_animation.texture());
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
