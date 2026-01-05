pub mod message_passing;
pub mod without_message_passing;

use fyrox::plugin::error::GameResult;
use fyrox::{
    core::{
         pool::Handle, reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*,
        TypeUuidProvider,
    },
    scene::{animation::absm::prelude::*, node::Node},
    script::{ScriptContext, ScriptTrait},
};

// ANCHOR: synthetic_example
#[derive(Clone, Debug, Reflect, Visit, Default, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "a9fb15ad-ab56-4be6-8a06-73e73d8b1f49")]
#[visit(optional)]
struct MyScript {
    some_node: Handle<Node>,
    some_other_node: Handle<Node>,
    yet_another_node: Handle<Node>,
}

impl ScriptTrait for MyScript {
    fn on_update(&mut self, ctx: &mut ScriptContext) -> GameResult {
        // Begin multiple borrowing.
        let mbc = ctx.scene.graph.begin_multi_borrow();

        // Borrow immutably.
        let some_node_ref_1 = mbc.try_get(self.some_node)?;

        // Then borrow other nodes mutably.
        let some_other_node_ref = mbc.try_get_mut(self.some_other_node)?;
        let yet_another_node_ref = mbc.try_get_mut(self.yet_another_node)?;

        // We can borrow the same node immutably pretty much infinite number of times, if it wasn't
        // borrowed mutably.
        let some_node_ref_2 = mbc.try_get(self.some_node)?;

        Ok(())
    }
}
// ANCHOR_END: synthetic_example

// ANCHOR: bot_example
#[derive(Clone, Debug, Reflect, Visit, Default, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "a9fb15ad-ab56-4be6-8a06-73e73d8b1f49")]
#[visit(optional)]
struct Bot {
    target: Handle<Node>,
    absm: Handle<Node>,
}

impl ScriptTrait for Bot {
    fn on_update(&mut self, ctx: &mut ScriptContext) -> GameResult {
        // Begin multiple borrowing.
        let mbc = ctx.scene.graph.begin_multi_borrow();

        // At first, borrow a node on which this script is running on.
        let this = mbc.try_get_mut(ctx.handle)?;

        // Try to borrow the target. It can fail in two cases:
        // 1) `self.target` is invalid or unassigned handle.
        // 2) A node is already borrowed, this could only happen if the bot has itself as the target.
        let target = mbc.try_get_mut(self.target)?;

        // Check if we are close enough to target.
        let close_enough = target
            .global_position()
            .metric_distance(&this.global_position())
            < 1.0;

        // Switch animations accordingly.
        let mut absm =
            mbc.try_get_component_of_type_mut::<AnimationBlendingStateMachine>(self.absm)?;
        absm.machine_mut()
            .get_value_mut_silent()
            .set_parameter("Attack", Parameter::Rule(close_enough));
        Ok(())
    }
}
// ANCHOR_END: bot_example
