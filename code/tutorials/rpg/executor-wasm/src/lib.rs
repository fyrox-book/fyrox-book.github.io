//! Executor with your game connected to it as a plugin.
use fyrox::core::wasm_bindgen::{self, prelude::*};
use fyrox::dpi::LogicalSize;
use fyrox::engine::executor::Executor;
use fyrox::engine::GraphicsContextParams;
use fyrox::event_loop::EventLoop;
use fyrox::window::WindowAttributes;
use rpg::Game;

#[wasm_bindgen]
pub fn main() {
    let mut executor = Executor::from_params(
        EventLoop::new().ok(),
        GraphicsContextParams {
            window_attributes: WindowAttributes::default()
                .with_inner_size(LogicalSize::new(800, 600)),
            vsync: false,
            msaa_sample_count: None,
            graphics_server_constructor: Default::default(),
            named_objects: false,
        },
    );
    executor.add_plugin(Game::default());
    executor.run()
}
