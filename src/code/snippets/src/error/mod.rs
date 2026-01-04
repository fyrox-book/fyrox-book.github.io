use fyrox::{
    core::pool::Handle,
    core::{reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*},
    plugin::error::{enable_backtrace_capture, GameResult},
    plugin::{Plugin, PluginContext},
    scene::node::Node,
    script::{ScriptContext, ScriptTrait},
};
use fyrox::graph::SceneGraph;

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
