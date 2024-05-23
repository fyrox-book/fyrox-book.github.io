//! Editor with your game connected to it as a plugin.
use fps::Game;
use fyrox::event_loop::EventLoop;
use fyroxed_base::{Editor, StartupData};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut editor = Editor::new(Some(StartupData {
        working_directory: Default::default(),
        scenes: vec!["data/scene.rgs".into()],
    }));
    editor.add_game_plugin(Game::default());
    editor.run(event_loop)
}
