use fyrox::plugin::error::GameResult;
use fyrox::{
    core::{
        pool::Handle, reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*,
        TypeUuidProvider,
    },
    scene::{graph::Graph, node::Node},
    script::{ScriptContext, ScriptMessageContext, ScriptMessagePayload, ScriptTrait},
};

// ANCHOR: message_passing
#[derive(Clone, Debug, Reflect, Visit, Default, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "a9fb15ad-ab56-4be6-8a06-73e73d8b1f49")]
#[visit(optional)]
struct Weapon {
    bullets: u32,
}

impl Weapon {
    fn shoot(&mut self, self_handle: Handle<Node>, graph: &mut Graph) {
        // -- This method is the same
    }
}

#[derive(Debug)]
pub struct ShootMessage;
impl ScriptMessagePayload for ShootMessage {}

impl ScriptTrait for Weapon {
    fn on_start(&mut self, ctx: &mut ScriptContext) -> GameResult {
        // Subscribe to shooting message.
        ctx.message_dispatcher
            .subscribe_to::<ShootMessage>(ctx.handle);
        Ok(())
    }

    fn on_message(
        &mut self,
        message: &mut dyn ScriptMessagePayload,
        ctx: &mut ScriptMessageContext,
    ) -> GameResult {
        // Receive shooting messages.
        if message.downcast_ref::<ShootMessage>().is_some() {
            self.shoot(ctx.handle, &mut ctx.scene.graph);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Reflect, Visit, Default, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "a9fb15ad-ab56-4be6-8a06-73e73d8b1f49")]
#[visit(optional)]
struct Bot {
    weapon: Handle<Node>,
    collider: Handle<Node>,
    health: f32,
}

impl ScriptTrait for Bot {
    fn on_update(&mut self, ctx: &mut ScriptContext) -> GameResult {
        // Note, that we know nothing about the weapon here - just its handle and a message that it
        // can accept and process.
        ctx.message_sender.send_to_target(self.weapon, ShootMessage);
        Ok(())
    }
}

// ANCHOR_END: message_passing
