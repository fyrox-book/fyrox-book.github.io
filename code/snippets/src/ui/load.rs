use fyrox::core::log::Log;
use fyrox::core::pool::Handle;
use fyrox::gui::{UiNode, UserInterface};
use fyrox::plugin::{Plugin, PluginContext};
use fyrox::{core::reflect::prelude::*, core::visitor::prelude::*};

// ANCHOR: load_ui
#[derive(Visit, Reflect, Debug)]
struct MyPlugin {
    button: Handle<UiNode>,
}

impl Plugin for MyPlugin {
    fn init(&mut self, scene_path: Option<&str>, ctx: PluginContext) {
        ctx.task_pool.spawn_plugin_task(
            UserInterface::load_from_file("data/my_ui_asset.ui", ctx.resource_manager.clone()),
            |result, game: &mut MyPlugin, ctx| match result {
                Ok(ui) => {
                    *ctx.user_interfaces.first_mut() = ui;
                }
                Err(e) => Log::err(format!("Unable to load a user interface! Reason: {:?}", e)),
            },
        );
    }
}
// ANCHOR_END: load_ui
