pub mod context;
pub mod example;
pub mod plugin;
pub mod tasks;

use fyrox::graph::SceneGraph;
use fyrox::plugin::error::GameResult;
use fyrox::plugin::Plugin;
use fyrox::script::{ScriptDeinitContext, ScriptMessageContext, ScriptMessagePayload};
use fyrox::{
    core::{
        log::Log, pool::Handle, reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*,
        TypeUuidProvider,
    },
    scene::node::Node,
    script::{ScriptContext, ScriptTrait},
};

// ANCHOR: access_other_1
#[derive(Clone, Debug, Reflect, Visit, Default, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "a9fb05ad-ab56-4be6-8a06-73e73d8b1f48")]
#[visit(optional)]
struct MyScript {
    second_node: Handle<Node>,
}

impl ScriptTrait for MyScript {
    // ANCHOR_END: access_other_1

    // ANCHOR: find_node
    fn on_start(&mut self, ctx: &mut ScriptContext) -> GameResult {
        self.second_node = ctx
            .scene
            .graph
            .find_by_name_from_root("SomeName")
            .map(|(handle, _)| handle)
            .unwrap_or_default();
        Ok(())
    }
    // ANCHOR_END: find_node

    // ANCHOR: access_other_2
    fn on_update(&mut self, ctx: &mut ScriptContext) -> GameResult {
        let second_nodes_script_ref = ctx
            .scene
            .graph
            .try_get_script_of::<MyOtherScript>(self.second_node)?;
        if second_nodes_script_ref.counter > 60.0 {
            Log::info("Done!");
        }

        // The code below is equivalent to the code above. The only difference is that
        // it borrows the node and then borrows the script from it, giving you access
        // to the node.
        let second_node_ref = ctx.scene.graph.try_get(self.second_node)?;
        if let Some(second_nodes_script_ref) = second_node_ref.try_get_script::<MyOtherScript>() {
            if second_nodes_script_ref.counter > 60.0 {
                Log::info("Done!");
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Reflect, Visit, Default, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "a9fb05ad-ab56-4be6-8a06-73e73d8b1f49")]
#[visit(optional)]
struct MyOtherScript {
    counter: f32,
}

impl ScriptTrait for MyOtherScript {
    fn on_update(&mut self, _ctx: &mut ScriptContext) -> GameResult {
        // Counting.
        self.counter += 1.0;
        Ok(())
    }
}
// ANCHOR_END: access_other_2

// ANCHOR: message_passing
#[derive(Debug, ScriptMessagePayload)]
enum Message {
    Damage {
        actor: Handle<Node>,
        attacker: Handle<Node>,
    },
}
impl ScriptMessagePayload for Message {}

#[derive(Default, Clone, Reflect, Visit, Debug, ComponentProvider, TypeUuidProvider)]
#[type_uuid(id = "eb3c6354-eaf5-4e43-827d-0bb10d6d966b")]
#[visit(optional)]
struct Projectile;

impl ScriptTrait for Projectile {
    fn on_update(&mut self, ctx: &mut ScriptContext) -> GameResult {
        // Broadcast the message globally.
        ctx.message_sender.send_global(Message::Damage {
            actor: Default::default(),
            attacker: ctx.handle,
        });
        Ok(())
    }
}

#[derive(Default, Clone, Reflect, Visit, Debug, ComponentProvider, TypeUuidProvider)]
#[type_uuid(id = "ede36945-5cba-41a1-9ef9-9b33b0f0db36")]
#[visit(optional)]
struct LaserSight;

impl ScriptTrait for LaserSight {
    fn on_start(&mut self, ctx: &mut ScriptContext) -> GameResult {
        // Subscript to messages.
        ctx.message_dispatcher.subscribe_to::<Message>(ctx.handle);
        Ok(())
    }

    fn on_message(
        &mut self,
        message: &mut dyn ScriptMessagePayload,
        _ctx: &mut ScriptMessageContext,
    ) -> GameResult {
        // React to message.
        if let Some(Message::Damage { actor, attacker }) = message.downcast_ref::<Message>() {
            Log::info(format!("{actor} damaged {attacker}",))
        }
        Ok(())
    }
}
// ANCHOR_END: message_passing

// ANCHOR: access_plugin
#[derive(Default, Debug, Clone, Reflect, Visit)]
struct GamePlugin {
    bots: Vec<Handle<Node>>,
}

impl Plugin for GamePlugin {
    // ..
}

#[derive(Clone, Debug, Default, Visit, Reflect, ComponentProvider, TypeUuidProvider)]
#[type_uuid(id = "460cd09f-8768-4f38-8799-5e9c0c08b8fd")]
struct Bot {
    // ..
}

impl ScriptTrait for Bot {
    fn on_start(&mut self, ctx: &mut ScriptContext) -> GameResult {
        // Get a reference to the plugin.
        let plugin = ctx.plugins.get_mut::<GamePlugin>();
        // Register self in the "global" list of bots.
        plugin.bots.push(ctx.handle);
        Ok(())
    }

    fn on_deinit(&mut self, ctx: &mut ScriptDeinitContext) -> GameResult {
        let plugin = ctx.plugins.get_mut::<GamePlugin>();
        // Unregister the bot from the list.
        if let Some(index) = plugin
            .bots
            .iter()
            .position(|handle| *handle == ctx.node_handle)
        {
            plugin.bots.remove(index);
        }
        Ok(())
    }

    fn on_update(&mut self, ctx: &mut ScriptContext) -> GameResult {
        let plugin = ctx.plugins.get::<GamePlugin>();
        for bot in plugin.bots.iter() {
            if *bot != ctx.handle {
                // Search for target.
            }
        }
        Ok(())
    }
}
// ANCHOR_END: access_plugin
