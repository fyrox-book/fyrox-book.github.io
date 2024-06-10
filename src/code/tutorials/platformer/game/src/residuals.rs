#![allow(unused_variables)]

// Tutorial code that does not do anything specific and serves as an intermediate step.
// Ignore this code.

use fyrox::script::ScriptDeinitContext;
use fyrox::{
    core::{reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*},
    event::Event,
    script::{ScriptContext, ScriptTrait},
};

// ANCHOR: player_stub_script
#[derive(Visit, Reflect, Debug, Clone, Default, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "c5671d19-9f1a-4286-8486-add4ebaadaec")]
#[visit(optional)]
struct Player;

impl ScriptTrait for Player {
    // Called once at initialization.
    fn on_init(&mut self, context: &mut ScriptContext) {}

    // Put start logic - it is called when every other script is already initialized.
    fn on_start(&mut self, context: &mut ScriptContext) {}

    // Called whenever there is an event from OS (mouse click, keypress, etc.)
    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {}

    // Called every frame at fixed rate of 60 FPS.
    fn on_update(&mut self, context: &mut ScriptContext) {}
}
// ANCHOR_END: player_stub_script

// ANCHOR: bot_stub_script
#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "d2786d36-a0af-4e67-916a-438af62f818b")]
#[visit(optional)]
pub struct Bot {
    // Add fields here.
}

impl ScriptTrait for Bot {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
    }

    fn on_start(&mut self, context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
    }

    fn on_deinit(&mut self, context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        // Respond to OS events here.
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        // Put object logic here.
    }
}
// ANCHOR_END: bot_stub_script
