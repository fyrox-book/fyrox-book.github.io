use fyrox::{
    core::{
        math,
        pool::Handle,
        reflect::prelude::*,
        uuid::{uuid, Uuid},
        variable::InheritableVariable,
        visitor::prelude::*,
        TypeUuidProvider,
    },
    event::Event,
    impl_component_provider,
    resource::{model::ModelResource, model::ModelResourceExtension},
    scene::node::Node,
    script::{
        ScriptContext, ScriptDeinitContext, ScriptMessageContext, ScriptMessagePayload, ScriptTrait,
    },
};

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct Weapon {
    // ANCHOR: projectile_field
    #[visit(optional)]
    projectile: InheritableVariable<Option<ModelResource>>,
    // ANCHOR_END: projectile_field

    // ANCHOR: shot_point
    #[visit(optional)]
    shot_point: InheritableVariable<Handle<Node>>, // ANCHOR_END: shot_point
}

// ANCHOR: shoot_message
pub struct ShootWeaponMessage {}
// ANCHOR_END: shoot_message

impl_component_provider!(Weapon);

impl TypeUuidProvider for Weapon {
    fn type_uuid() -> Uuid {
        uuid!("e8adde73-1e96-471c-8531-5c0d16f0c29a")
    }
}

impl ScriptTrait for Weapon {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
    }

    // ANCHOR: on_start
    fn on_start(&mut self, context: &mut ScriptContext) {
        context
            .message_dispatcher
            .subscribe_to::<ShootWeaponMessage>(context.handle);
    }
    // ANCHOR_END: on_start

    fn on_deinit(&mut self, context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        // Respond to OS events here.
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        // Put object logic here.
    }

    // ANCHOR: on_message
    fn on_message(
        &mut self,
        message: &mut dyn ScriptMessagePayload,
        ctx: &mut ScriptMessageContext,
    ) {
        // Check if we've received an appropriate message. This is needed because message channel is
        // common across all scripts.
        if message.downcast_ref::<ShootWeaponMessage>().is_some() {
            if let Some(projectile_prefab) = self.projectile.as_ref() {
                // Try to get the position of the shooting point.
                if let Some(shot_point) = ctx
                    .scene
                    .graph
                    .try_get(*self.shot_point)
                    .map(|point| point.global_position())
                {
                    // Shooting direction is just a direction of the weapon (its look vector)
                    let direction = ctx.scene.graph[ctx.handle].look_vector();

                    // Finally instantiate our projectile at the position and direction.
                    projectile_prefab.instantiate_at(
                        ctx.scene,
                        shot_point,
                        math::vector_to_quat(direction),
                    );
                }
            }
        }
    }
    // ANCHOR_END: on_message

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
