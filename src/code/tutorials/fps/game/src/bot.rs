use fyrox::{
    core::{
        algebra::{Matrix4, Point3, Vector3},
        math::frustum::Frustum,
        reflect::prelude::*,
        type_traits::prelude::*,
        visitor::prelude::*,
    },
    event::Event,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "40c225ca-9657-4f4e-af67-48d6482a7aeb")]
pub struct Bot {
    // ANCHOR: frustum
    #[visit(skip)]
    #[reflect(hidden)]
    frustum: Frustum,
    // ANCHOR_END: frustum
}

impl Bot {
    // ANCHOR: frustum_update
    fn update_frustum(
        &mut self,
        position: Vector3<f32>,
        look_vector: Vector3<f32>,
        up_vector: Vector3<f32>,
        max_observing_distance: f32,
    ) {
        // Calculate an average head position.
        let head_pos = position + Vector3::new(0.0, 0.4, 0.0);
        let look_at = head_pos + look_vector;

        // View matrix is constructed using three parameters - observer position, target point,
        // and an up vector (usually it is just (0,1,0) vector).
        let view_matrix =
            Matrix4::look_at_rh(&Point3::from(head_pos), &Point3::from(look_at), &up_vector);

        // Build the perspective projection matrix.
        let projection_matrix = Matrix4::new_perspective(
            // Aspect ratio
            16.0 / 9.0,
            // Field of view of the bot
            90.0f32.to_radians(),
            0.1,
            max_observing_distance,
        );
        self.frustum =
            Frustum::from_view_projection_matrix(projection_matrix * view_matrix).unwrap();
    }
    // ANCHOR_END: frustum_update
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

    // ANCHOR: on_update
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        if let Some(rigid_body) = ctx.scene.graph.try_get_mut(ctx.handle) {
            let position = rigid_body.global_position();
            let up_vector = rigid_body.up_vector();
            let look_vector = rigid_body.look_vector();

            // Update the viewing frustum.
            self.update_frustum(position, look_vector, up_vector, 20.0);
        }
    }
    // ANCHOR_END: on_update
}
