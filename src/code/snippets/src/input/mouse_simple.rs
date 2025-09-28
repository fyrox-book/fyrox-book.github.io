use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        reflect::prelude::*,
        type_traits::prelude::*,
        visitor::prelude::*,
    },
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
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        let mouse_speed = ctx.input_state.mouse_speed();
        let limit = std::f32::consts::FRAC_PI_2;

        self.pitch = (self.pitch + mouse_speed.y).clamp(-limit, limit);
        self.yaw += mouse_speed.x;

        let node = &mut ctx.scene.graph[ctx.handle];
        let transform = node.local_transform_mut();
        transform.set_rotation(
            UnitQuaternion::from_axis_angle(&Vector3::x_axis(), self.pitch)
                * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.yaw),
        );
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
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        if ctx.input_state.is_left_mouse_button_pressed() {
            self.counter += 1;
        } else if ctx.input_state.is_left_mouse_button_released() {
            self.counter -= 1;
        }
    }
}
// ANCHOR_END: clicker
