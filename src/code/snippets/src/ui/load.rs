
use fyrox::core::pool::Handle;
use fyrox::gui::{UiNode, UserInterface};
use fyrox::plugin::error::GameResult;
use fyrox::plugin::{Plugin, PluginContext};
use fyrox::{core::reflect::prelude::*, core::visitor::prelude::*};

// ANCHOR: load_ui
#[derive(Visit, Reflect, Debug)]
struct MyPlugin {
    button: Handle<UiNode>,
}

impl Plugin for MyPlugin {
    fn init(&mut self, scene_path: Option<&str>, ctx: PluginContext) -> GameResult {
        ctx.task_pool.spawn_plugin_task(
            UserInterface::load_from_file(
                "data/my_ui_asset.ui",
                ctx.widget_constructors.clone(),
                ctx.dyn_type_constructors.clone(),
                ctx.resource_manager.clone(),
            ),
            |result, game: &mut MyPlugin, ctx| {
                *ctx.user_interfaces.first_mut() = result?;
                Ok(())
            },
        );
        Ok(())
    }
}
// ANCHOR_END: load_ui
