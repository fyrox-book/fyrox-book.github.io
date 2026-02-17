use fyrox::core::pool::Handle;
use fyrox::gui::UiNode;
use fyrox::plugin::error::GameResult;
use fyrox::plugin::{Plugin, PluginContext};
use fyrox::{core::reflect::prelude::*, core::visitor::prelude::*};

// ANCHOR: load_ui
#[derive(Visit, Clone, Reflect, Debug)]
struct MyPlugin {
    button: Handle<UiNode>,
}

impl Plugin for MyPlugin {
    fn init(&mut self, scene_path: Option<&str>, mut ctx: PluginContext) -> GameResult {
        ctx.load_ui("data/my_ui_asset.ui", |result, game: &mut MyPlugin, ctx| {
            ctx.user_interfaces.add(result?.payload);
            Ok(())
        });
        Ok(())
    }
}
// ANCHOR_END: load_ui
