//! Executor with your game connected to it as a plugin.
#![cfg(target_arch = "wasm32")]

use fyrox::core::wasm_bindgen::{self, prelude::*};
use fyrox::dpi::LogicalSize;
use fyrox::engine::executor::Executor;
use fyrox::engine::GraphicsContextParams;
use fyrox::event_loop::EventLoop;
use fyrox::window::WindowAttributes;
use platformer::Game;

#[wasm_bindgen]
pub fn main() {
    let mut window_attributes = WindowAttributes::default();
    window_attributes.inner_size = Some(LogicalSize::new(800, 600).into());
    window_attributes.resizable = true;
    window_attributes.title = "Platformer".to_string();
    let mut executor = Executor::from_params(
        EventLoop::new().ok(),
        GraphicsContextParams {
            window_attributes,
            vsync: true,
            msaa_sample_count: None,
            graphics_server_constructor: Default::default(),
            named_objects: false,
        },
    );
    executor.add_plugin(Game::default());
    executor.run()
}
