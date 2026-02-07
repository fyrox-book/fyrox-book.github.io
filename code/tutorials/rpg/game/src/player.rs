use fyrox::plugin::error::GameResult;
use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        impl_component_provider,
        math::SmoothAngle,
        pool::Handle,
        reflect::prelude::*,
        type_traits::prelude::*,
        variable::InheritableVariable,
        visitor::prelude::*,
        TypeUuidProvider,
    },
    event::{DeviceEvent, ElementState, Event, WindowEvent},
    graph::SceneGraph,
    keyboard::{KeyCode, PhysicalKey},
    scene::{animation::absm::prelude::*, node::Node, rigidbody::RigidBody},
    script::{ScriptContext, ScriptTrait},
};

// ANCHOR: player_struct
#[derive(Visit, Reflect, Default, Debug, Clone)]
#[visit(optional)]
pub struct Player {
    camera_pivot: InheritableVariable<Handle<Node>>,

    camera_hinge: InheritableVariable<Handle<Node>>,

    state_machine: InheritableVariable<Handle<AnimationBlendingStateMachine>>,

    model_pivot: InheritableVariable<Handle<Node>>,

    model: InheritableVariable<Handle<Node>>,

    model_yaw: InheritableVariable<SmoothAngle>,

    #[reflect(hidden)]
    #[visit(skip)]
    walk_forward: bool,

    #[reflect(hidden)]
    #[visit(skip)]
    walk_backward: bool,

    #[reflect(hidden)]
    #[visit(skip)]
    walk_left: bool,

    #[reflect(hidden)]
    #[visit(skip)]
    walk_right: bool,

    #[reflect(hidden)]
    #[visit(skip)]
    yaw: f32,

    #[reflect(hidden)]
    #[visit(skip)]
    pitch: f32,
}
// ANCHOR_END: player_struct

impl_component_provider!(Player);

impl TypeUuidProvider for Player {
    fn type_uuid() -> Uuid {
        uuid!("f30ccfe4-eff1-4f56-93a2-643e83d34f2e")
    }
}

impl ScriptTrait for Player {
    // ANCHOR: on_os_event
    fn on_os_event(&mut self, event: &Event<()>, ctx: &mut ScriptContext) -> GameResult {
        match event {
            Event::WindowEvent { event, .. } => {
                if let WindowEvent::KeyboardInput { event, .. } = event {
                    if let PhysicalKey::Code(code) = event.physical_key {
                        let pressed = event.state == ElementState::Pressed;
                        match code {
                            KeyCode::KeyW => self.walk_forward = pressed,
                            KeyCode::KeyS => self.walk_backward = pressed,
                            KeyCode::KeyA => self.walk_left = pressed,
                            KeyCode::KeyD => self.walk_right = pressed,
                            _ => (),
                        }
                    }
                }
            }
            Event::DeviceEvent { event, .. } => {
                if let DeviceEvent::MouseMotion { delta } = event {
                    let mouse_sens = 0.2 * ctx.dt;
                    self.yaw -= (delta.0 as f32) * mouse_sens;
                    self.pitch = (self.pitch + (delta.1 as f32) * mouse_sens)
                        .clamp(-90.0f32.to_radians(), 90.0f32.to_radians());
                }
            }
            _ => (),
        }
        Ok(())
    }
    // ANCHOR_END: on_os_event

    // ANCHOR: on_update
    fn on_update(&mut self, ctx: &mut ScriptContext) -> GameResult {
        // Step 1. Fetch the velocity vector from the animation blending state machine.
        let transform = ctx.scene.graph[*self.model].global_transform();
        let mut velocity = Vector3::default();
        let state_machine = ctx.scene.graph.try_get(*self.state_machine)?;
        if let Some(root_motion) = state_machine.machine().pose().root_motion() {
            velocity = transform
                .transform_vector(&root_motion.delta_position)
                .scale(1.0 / ctx.dt);
        }

        // Step 2. Apply the velocity to the rigid body and lock rotations.
        let body = ctx
            .scene
            .graph
            .try_get_mut_of_type::<RigidBody>(ctx.handle)?;
        body.set_ang_vel(Default::default());
        body.set_lin_vel(Vector3::new(velocity.x, body.lin_vel().y, velocity.z));

        // Step 3. Rotate the model pivot according to the movement direction.
        let quat_yaw = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.yaw);

        if velocity.norm_squared() > 0.0 {
            // Since we have free camera while not moving, we have to sync rotation of pivot
            // with rotation of camera so character will start moving in look direction.
            ctx.scene
                .graph
                .try_get_mut(*self.model_pivot)?
                .local_transform_mut()
                .set_rotation(quat_yaw);

            // Apply additional rotation to model - it will turn in front of walking direction.
            let angle: f32 = if self.walk_left {
                if self.walk_forward {
                    45.0
                } else if self.walk_backward {
                    135.0
                } else {
                    90.0
                }
            } else if self.walk_right {
                if self.walk_forward {
                    -45.0
                } else if self.walk_backward {
                    -135.0
                } else {
                    -90.0
                }
            } else if self.walk_backward {
                180.0
            } else {
                0.0
            };

            self.model_yaw.set_target(angle.to_radians()).update(ctx.dt);

            ctx.scene
                .graph
                .try_get_mut(*self.model)?
                .local_transform_mut()
                .set_rotation(UnitQuaternion::from_axis_angle(
                    &Vector3::y_axis(),
                    self.model_yaw.angle,
                ));
        }

        ctx.scene
            .graph
            .try_get_mut(*self.camera_pivot)?
            .local_transform_mut()
            .set_rotation(quat_yaw);

        // Rotate camera hinge - this will make camera move up and down while look at character
        // (well not exactly on character - on characters head)
        ctx.scene
            .graph
            .try_get_mut(*self.camera_hinge)?
            .local_transform_mut()
            .set_rotation(UnitQuaternion::from_axis_angle(
                &Vector3::x_axis(),
                self.pitch,
            ));

        // Step 4. Feed the animation blending state machine with the current state of the player.
        let state_machine = ctx.scene.graph.try_get_mut(*self.state_machine)?;
        let moving = self.walk_left || self.walk_right || self.walk_forward || self.walk_backward;
        state_machine
            .machine_mut()
            .get_value_mut_silent()
            .set_parameter("Running", Parameter::Rule(moving));

        Ok(())
    }
    // ANCHOR_END: on_update
}
