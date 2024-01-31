use fyrox::scene::graph::physics::RayCastOptions;
use fyrox::scene::graph::Graph;
use fyrox::{
    core::{
        pool::Handle, reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*,
        TypeUuidProvider,
    },
    scene::node::Node,
    script::{ScriptContext, ScriptTrait},
};

// ANCHOR: without_message_passing
#[derive(Clone, Debug, Reflect, Visit, Default, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "a9fb15ad-ab56-4be6-8a06-73e73d8b1f49")]
#[visit(optional)]
struct Weapon {
    bullets: u32,
}

impl Weapon {
    fn shoot(&mut self, self_handle: Handle<Node>, graph: &mut Graph) {
        if self.bullets > 0 {
            let this = &graph[self_handle];
            let position = this.global_position();
            let direction = this.look_vector().scale(10.0);

            // Cast a ray in front of the weapon.
            let mut results = Vec::new();
            graph.physics.cast_ray(
                RayCastOptions {
                    ray_origin: position.into(),
                    ray_direction: direction,
                    max_len: 10.0,
                    groups: Default::default(),
                    sort_results: false,
                },
                &mut results,
            );

            // Try to damage all the bots that were hit by the ray.
            for result in results {
                for node in graph.linear_iter_mut() {
                    if let Some(bot) = node.try_get_script_mut::<Bot>() {
                        if bot.collider == result.collider {
                            bot.health -= 10.0;
                        }
                    }
                }
            }

            self.bullets -= 1;
        }
    }
}

impl ScriptTrait for Weapon {}

#[derive(Clone, Debug, Reflect, Visit, Default, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "a9fb15ad-ab56-4be6-8a06-73e73d8b1f49")]
#[visit(optional)]
struct Bot {
    weapon: Handle<Node>,
    collider: Handle<Node>,
    health: f32,
}

impl ScriptTrait for Bot {
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        // Try to shoot the weapon.
        if let Some(weapon) = ctx
            .scene
            .graph
            .try_get_script_component_of_mut::<Weapon>(self.weapon)
        {
            // !!! This will not compile, because it requires mutable access to the weapon and to
            // the script context at the same time. This is impossible to do safely, because we've
            // just borrowed the weapon from the context.

            // weapon.shoot(ctx.handle, &mut ctx.scene.graph);
        }
    }
}

// ANCHOR_END: without_message_passing
