//! Executor with your game connected to it as a plugin.
use fyrox::engine::executor::Executor;
use fyrox::event_loop::EventLoop;
use platformer::Game;

fn main() {
    let mut executor = Executor::new(EventLoop::new().ok());
    executor.add_plugin(Game::default());
    executor.run()
}
