use fyrox::plugin::error::GameResult;
use fyrox::plugin::{Plugin, PluginRegistrationContext};
use fyrox::scene::node::Node;
use fyrox::script::{ScriptMessageContext, ScriptMessagePayload};
use fyrox::{
    core::{reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*},
    event::Event,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

// ANCHOR: example_script
#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "bf0f9804-56cb-4a2e-beba-93d75371a568")]
#[visit(optional)]
struct MyScript {
    // Add fields here.
}

impl ScriptTrait for MyScript {
    fn on_init(&mut self, context: &mut ScriptContext) -> GameResult {
        // Put initialization logic here.
        Ok(())
    }

    fn on_start(&mut self, context: &mut ScriptContext) -> GameResult {
        // Put start logic - it is called when every other script is already initialized.
        Ok(())
    }

    fn on_deinit(&mut self, context: &mut ScriptDeinitContext) -> GameResult {
        // Put de-initialization logic here.
        Ok(())
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) -> GameResult {
        // Respond to OS events here.
        Ok(())
    }

    fn on_update(&mut self, context: &mut ScriptContext) -> GameResult {
        // Put object logic here.
        Ok(())
    }

    fn on_message(
        &mut self,
        message: &mut dyn ScriptMessagePayload,
        ctx: &mut ScriptMessageContext,
    ) -> GameResult {
        // See "message passing" section below.
        Ok(())
    }
}
// ANCHOR: example_script

// ANCHOR: register
#[derive(Visit, Clone, Reflect, Debug)]
struct MyPlugin;

impl Plugin for MyPlugin {
    fn register(&self, context: PluginRegistrationContext) -> GameResult {
        context
            .serialization_context
            .script_constructors
            .add::<MyScript>("My Script");
        Ok(())
    }
}
// ANCHOR_END: register

// ANCHOR: add_my_script
fn add_my_script(node: &mut Node) {
    node.add_script(MyScript::default())
}
// ANCHOR_END: add_my_script
