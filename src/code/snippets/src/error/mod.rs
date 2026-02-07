use fyrox::core::warn;
use fyrox::graph::SceneGraph;
use fyrox::plugin::error::{GameError, GameErrorKind};
use fyrox::{
    core::pool::Handle,
    core::{reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*},
    plugin::error::{enable_backtrace_capture, GameResult},
    plugin::{Plugin, PluginContext},
    scene::node::Node,
    script::{ScriptContext, ScriptTrait},
};
use std::fmt::{Display, Formatter};

// ANCHOR: error_handling
#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "bf0f9804-56cb-4a2e-beba-93d75371a568")]
#[visit(optional)]
struct MyScript {
    handle: Handle<Node>,
}

impl ScriptTrait for MyScript {
    fn on_update(&mut self, context: &mut ScriptContext) -> GameResult {
        let node = context.scene.graph.try_get(context.handle)?;
        println!("{}", node.name());
        Ok(())
    }
}
// ANCHOR: error_handling

// ANCHOR: enable_backtrace_capture
#[derive(Visit, Reflect, Debug)]
struct MyPlugin;

impl Plugin for MyPlugin {
    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) -> GameResult {
        // This method can be called at any point in your game, this way you can enable or disable
        // enhanced error data collection when needed.
        enable_backtrace_capture(true);
        Ok(())
    }
}
// ANCHOR_END: enable_backtrace_capture

// ANCHOR: error_handler
#[derive(Visit, Reflect, Debug)]
struct MyGame;

// Define an error type for your game first.
#[derive(Debug)]
pub enum MyError {
    NoScene,
}
impl std::error::Error for MyError {}
impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::NoScene => {
                write!(f, "The scene is not specified!")
            }
        }
    }
}

impl Plugin for MyGame {
    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) -> GameResult {
        match scene_path {
            Some(scene_path) => {
                context.async_scene_loader.request(scene_path);
                Ok(())
            }
            // Spawn an error.
            None => Err(GameError::user(MyError::NoScene)),
        }
    }

    fn on_game_error(&mut self, context: &mut PluginContext, error: &GameError) -> bool {
        if let Some(GameErrorKind::UserError(ref err)) = error.kind {
            if let Some(my_error) = err.downcast_ref::<MyError>() {
                // Do something useful, for example show a warning message box.
                // ...

                // Mark the error as handled.
                return true;
            }
        }

        // The rest is unhandled.
        false
    }
}
// ANCHOR_END: error_handler
