use fyrox::{
    core::{reflect::prelude::*, visitor::prelude::*},
    event::{Event, WindowEvent},
    plugin::{error::GameResult, Plugin, PluginContext},
};

// ANCHOR: raw_text_input
#[derive(Visit, Clone, Reflect, Debug)]
struct MyPlugin;

impl Plugin for MyPlugin {
    fn on_os_event(&mut self, event: &Event<()>, context: PluginContext) -> GameResult {
        if let Event::WindowEvent { event, .. } = event {
            if let WindowEvent::KeyboardInput { event, .. } = event {
                if let Some(ref text) = event.text {
                    println!("{}", text);
                }
            }
        }
        Ok(())
    }
}
// ANCHOR_END: raw_text_input
