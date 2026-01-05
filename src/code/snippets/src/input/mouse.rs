use fyrox::plugin::error::GameResult;
use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        reflect::prelude::*,
        type_traits::prelude::*,
        visitor::prelude::*,
    },
    event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent},
    script::{ScriptContext, ScriptTrait},
};

// ANCHOR: mouse
#[derive(Clone, Debug, Reflect, Visit, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "abbad54c-e267-4d7e-a3cd-e125a7e87ff0")]
#[visit(optional)]
pub struct Player {
    yaw: f32,
    pitch: f32,
}

impl ScriptTrait for Player {
    fn on_os_event(&mut self, event: &Event<()>, _ctx: &mut ScriptContext) -> GameResult {
        // We'll listen to MouseMotion raw device event to rotate an object. It provides
        // offsets only.
        if let Event::DeviceEvent {
            event: DeviceEvent::MouseMotion {
                delta: (dx, dy), ..
            },
            ..
        } = event
        {
            let limit = std::f32::consts::FRAC_PI_2;
            self.pitch = (self.pitch + *dy as f32).clamp(-limit, limit);
            self.yaw += *dx as f32;
        }
        Ok(())
    }

    fn on_update(&mut self, ctx: &mut ScriptContext) -> GameResult {
        let node = &mut ctx.scene.graph[ctx.handle];
        let transform = node.local_transform_mut();
        transform.set_rotation(
            UnitQuaternion::from_axis_angle(&Vector3::x_axis(), self.pitch)
                * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.yaw),
        );
        Ok(())
    }
}
// ANCHOR_END: mouse

// ANCHOR: clicker
#[derive(Clone, Debug, Reflect, Visit, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "abbad54c-e267-4d7e-a3cd-e125a7e87ff1")]
#[visit(optional)]
pub struct Clicker {
    counter: i32,
}

impl ScriptTrait for Clicker {
    fn on_os_event(&mut self, event: &Event<()>, _ctx: &mut ScriptContext) -> GameResult {
        if let Event::WindowEvent {
            event: WindowEvent::MouseInput { button, state, .. },
            ..
        } = event
        {
            if *state == ElementState::Pressed {
                match *button {
                    MouseButton::Left => {
                        self.counter -= 1;
                    }
                    MouseButton::Right => {
                        self.counter += 1;
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    }
}
// ANCHOR_END: clicker
