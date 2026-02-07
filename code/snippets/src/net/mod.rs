mod client_server;

use fyrox::scene::pivot::Pivot;
use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        log::Log,
        pool::Handle,
        reflect::prelude::*,
        visitor::prelude::*,
        Uuid,
    },
    fxhash::FxHashMap,
    plugin::{Plugin, PluginContext},
    resource::model::{Model, ModelResourceExtension},
    scene::{
        base::{BaseBuilder, SceneNodeId},
        node::Node,
        pivot::PivotBuilder,
        Scene,
    },
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ANCHOR: create_node_with_id
pub fn create_node_with_id(scene: &mut Scene, id: Uuid) -> Handle<Pivot> {
    PivotBuilder::new(BaseBuilder::new().with_instance_id(SceneNodeId(id))).build(&mut scene.graph)
}
// ANCHOR_END: create_node_with_id

#[derive(Visit, Clone, Reflect, Debug)]
pub struct Game {
    scene: Handle<Scene>,
}

impl Plugin for Game {}

// ANCHOR: prefab_message
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InstantiatePrefabMessage {
    pub path: PathBuf,
    pub position: Vector3<f32>,
    pub rotation: UnitQuaternion<f32>,
    pub ids: FxHashMap<Handle<Node>, SceneNodeId>,
}
// ANCHOR_END: prefab_message

// ANCHOR: create_prefab_message
// This function is called on the server side.
pub fn create_prefab_message(ctx: &mut PluginContext) -> InstantiatePrefabMessage {
    let path = PathBuf::from("data/models/bot.rgs");

    let bot_prefab =
        fyrox::core::futures::executor::block_on(ctx.resource_manager.request::<Model>(&path))
            .unwrap();

    InstantiatePrefabMessage {
        path,
        position: Vector3::new(1.0, 2.0, 3.0),
        rotation: Default::default(),
        ids: bot_prefab.generate_ids(),
    }
}
// ANCHOR_END: create_prefab_message

// ANCHOR: on_prefab_message_received
// This function is called on the client side.
pub fn on_prefab_message_received(
    desc: InstantiatePrefabMessage,
    game: &mut Game,
    ctx: &mut PluginContext,
) {
    let result =
        fyrox::core::futures::executor::block_on(ctx.resource_manager.request::<Model>(&desc.path));

    match result {
        Ok(model) => {
            let scene = &mut ctx.scenes[game.scene];
            model
                .begin_instantiation(scene)
                .with_position(desc.position)
                .with_rotation(desc.rotation)
                .with_ids(&desc.ids)
                .finish();
        }
        Err(err) => {
            Log::err(format!(
                "Unable to instantiate {} prefab. Reason: {:?}",
                desc.path.display(),
                err
            ));
        }
    }
}
// ANCHOR_END: on_prefab_message_received

// ANCHOR: on_prefab_message_received_async
// This function is called on the client side.
pub fn on_prefab_message_received_async(desc: InstantiatePrefabMessage, ctx: &mut PluginContext) {
    ctx.task_pool.spawn_plugin_task(
        // The resource itself could be used as a task.
        ctx.resource_manager.request::<Model>(&desc.path),
        // This closure will be called when the task (resource loading) is completed.
        move |result, game: &mut Game, ctx| {
            let scene = &mut ctx.scenes[game.scene];
            result?
                .begin_instantiation(scene)
                .with_position(desc.position)
                .with_rotation(desc.rotation)
                .with_ids(&desc.ids)
                .finish();
            Ok(())
        },
    );
}
// ANCHOR_END: on_prefab_message_received_async
