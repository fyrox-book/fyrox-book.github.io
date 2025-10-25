//! Executor with your game connected to it as a plugin.
use fps::Game;
use fyrox::core::wasm_bindgen::{self, prelude::*};
use fyrox::engine::executor::Executor;
use fyrox::event_loop::EventLoop;

#[wasm_bindgen]
pub fn main() {
    set_panic_hook();
    let mut executor = Executor::new(EventLoop::new().ok());
    executor.add_plugin(Game::default());
    executor.run()
}
