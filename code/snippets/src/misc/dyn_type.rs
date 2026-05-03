use fyrox::{
    plugin::{
        error::GameResult,
        Plugin,
        PluginContext,
        PluginRegistrationContext
    },
    scene::rigidbody::RigidBody,
    core::{
        pool::Handle,
        visitor::prelude::*,
        reflect::prelude::*,
        type_traits::prelude::*
    },
    event::{DeviceEvent, Event, WindowEvent},
};

// ANCHOR: my_data
#[derive(Visit, Reflect, TypeUuidProvider, Clone, Default)]
#[type_uuid(id = "df479e17-1db1-449f-b91b-029d05332c9b")]
struct MyData {
    target_name: String,
    rigid_body: Handle<RigidBody>
}
// ANCHOR_END: my_data

#[derive(Reflect, Clone, Debug, Visit)]
struct MyGame {}

// ANCHOR: registration
impl Plugin for MyGame {
    fn register(&self, context: PluginRegistrationContext) -> GameResult {
        context.dyn_type_constructors.add::<MyData, Self>("My Data");
        Ok(())
    }
}
// ANCHOR_END: registration
