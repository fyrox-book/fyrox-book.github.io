use fyrox::plugin::error::GameResult;
use fyrox::{
    core::{algebra::Vector3, reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*},
    event::{ElementState, Event, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    script::{ScriptContext, ScriptTrait},
};

// ANCHOR: keyboard
#[derive(Clone, Debug, Reflect, Visit, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "abbad54c-e267-4d7e-a3cd-e125a7e87ff0")]
#[visit(optional)]
pub struct Player {
    move_left: bool,
    move_right: bool,
}

impl ScriptTrait for Player {
    fn on_os_event(&mut self, event: &Event<()>, _ctx: &mut ScriptContext) -> GameResult {
        // Listen to keyboard events, that comes to the main window.
        if let Event::WindowEvent {
            event: WindowEvent::KeyboardInput { event, .. },
            ..
        } = event
        {
            let pressed = event.state == ElementState::Pressed;
            if let PhysicalKey::Code(code) = event.physical_key {
                // Check which key was pressed and remember this state for further usage.
                match code {
                    KeyCode::KeyA => {
                        self.move_left = pressed;
                    }
                    KeyCode::KeyD => {
                        self.move_right = pressed;
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    }

    fn on_update(&mut self, ctx: &mut ScriptContext) -> GameResult {
        let node = &mut ctx.scene.graph[ctx.handle];
        let transform = node.local_transform_mut();
        if self.move_left {
            transform.offset(Vector3::new(-1.0, 0.0, 0.0));
        }
        if self.move_right {
            transform.offset(Vector3::new(1.0, 0.0, 0.0));
        }
        Ok(())
    }
}
// ANCHOR_END: keyboard
