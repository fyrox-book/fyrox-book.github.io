use fyrox::plugin::error::GameResult;
use fyrox::{
    core::{algebra::Vector3, reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*},
    keyboard::KeyCode,
    script::{ScriptContext, ScriptTrait},
};

// ANCHOR: keyboard
#[derive(Clone, Debug, Reflect, Visit, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "abbad54c-e267-4d7e-a3cd-e125a7e87ff0")]
#[visit(optional)]
pub struct Player {}

impl ScriptTrait for Player {
    fn on_update(&mut self, ctx: &mut ScriptContext) -> GameResult {
        let node = &mut ctx.scene.graph[ctx.handle];
        let transform = node.local_transform_mut();

        // Check if the keys are down and move the player accordingly.
        if ctx.input_state.is_key_down(KeyCode::KeyA) {
            transform.offset(Vector3::new(-1.0, 0.0, 0.0));
        }
        if ctx.input_state.is_key_down(KeyCode::KeyD) {
            transform.offset(Vector3::new(1.0, 0.0, 0.0));
        }

        // It is also possible to check if a key was pressed or released on this frame. This
        // could be useful to do something only once per each press or release. For example,
        // jump could be performed on a key press.
        if ctx.input_state.is_key_pressed(KeyCode::Space) {
            transform.offset(Vector3::new(0.0, 1.0, 0.0));
        }
        Ok(())
    }
}
// ANCHOR_END: keyboard
