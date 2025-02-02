use crate::player::Player;
use fyrox::core::algebra::UnitQuaternion;
use fyrox::graph::{BaseSceneGraph, SceneGraph, SceneGraphNode};
use fyrox::{
    core::{
        algebra::{Matrix4, Point3, Vector3},
        math::frustum::Frustum,
        pool::Handle,
        reflect::prelude::*,
        type_traits::prelude::*,
        variable::InheritableVariable,
        visitor::prelude::*,
    },
    scene::{animation::absm::prelude::*, node::Node, rigidbody::RigidBody},
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "40c225ca-9657-4f4e-af67-48d6482a7aeb")]
#[visit(optional)]
pub struct Bot {
    // ANCHOR: frustum
    #[visit(skip)]
    #[reflect(hidden)]
    frustum: Frustum,
    // ANCHOR_END: frustum

    // ANCHOR: absm_model_root_fields
    absm: InheritableVariable<Handle<Node>>,
    model_root: InheritableVariable<Handle<Node>>,
    // ANCHOR_END: absm_model_root_fields

    // ANCHOR: target_field
    #[visit(skip)]
    #[reflect(hidden)]
    target: Handle<Node>,
    // ANCHOR_END: target_field
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

    fn on_update(&mut self, ctx: &mut ScriptContext) {
        // ANCHOR: frustum_check
        // Look for targets only if we don't have one.
        if self.target.is_none() {
            for (handle, node) in ctx.scene.graph.pair_iter() {
                if node.has_script::<Player>()
                    && self.frustum.is_contains_point(node.global_position())
                {
                    self.target = handle;
                    break;
                }
            }
        }

        // A helper flag, that tells the bot that it is close enough to a target for melee
        // attack.
        let close_to_target = ctx
            .scene
            .graph
            .try_get(self.target)
            .map_or(false, |target| {
                target
                    .global_position()
                    .metric_distance(&ctx.scene.graph[ctx.handle].global_position())
                    < 1.25
            });
        // ANCHOR_END: frustum_check

        // ANCHOR: root_motion_1
        let model_transform = ctx
            .scene
            .graph
            .try_get(*self.model_root)
            .map(|model| model.global_transform())
            .unwrap_or_default();

        let mut velocity = Vector3::default();
        if let Some(state_machine) = ctx
            .scene
            .graph
            .try_get_mut(*self.absm)
            .and_then(|node| node.component_mut::<AnimationBlendingStateMachine>())
        {
            // ANCHOR_END: root_motion_1
            if let Some(root_motion) = state_machine.machine().pose().root_motion() {
                velocity = model_transform
                    .transform_vector(&root_motion.delta_position)
                    .scale(1.0 / ctx.dt);
            }

            // ANCHOR: absm_parameters
            state_machine
                .machine_mut()
                .get_value_mut_silent()
                .set_parameter("Run", Parameter::Rule(self.target.is_some()))
                .set_parameter("Attack", Parameter::Rule(close_to_target));
            // ANCHOR_END: absm_parameters

            // ANCHOR: root_motion_2
        }
        // ANCHOR_END: root_motion_2

        // ANCHOR: angle_calculation
        let angle_to_target = ctx.scene.graph.try_get(self.target).map(|target| {
            let self_position = ctx.scene.graph[ctx.handle].global_position();
            let look_dir = target.global_position() - self_position;
            look_dir.x.atan2(look_dir.z)
        });
        // ANCHOR_END: angle_calculation

        // ANCHOR: on_update_1
        if let Some(rigid_body) = ctx.scene.graph.try_get_mut_of_type::<RigidBody>(ctx.handle) {
            let position = rigid_body.global_position();
            let up_vector = rigid_body.up_vector();
            let look_vector = rigid_body.look_vector();

            // Update the viewing frustum.
            self.update_frustum(position, look_vector, up_vector, 20.0);
            // ANCHOR_END: on_update_1

            // ANCHOR: rigid_body_velocity
            let y_vel = rigid_body.lin_vel().y;
            rigid_body.set_lin_vel(Vector3::new(velocity.x, y_vel, velocity.z));
            // ANCHOR_END: rigid_body_velocity

            // ANCHOR: angle_usage
            if let Some(angle) = angle_to_target {
                rigid_body
                    .local_transform_mut()
                    .set_rotation(UnitQuaternion::from_axis_angle(&Vector3::y_axis(), angle));
            }
            // ANCHOR_END: angle_usage

            // ANCHOR: on_update_2
        }
        // ANCHOR_END: on_update_2
    }
}
