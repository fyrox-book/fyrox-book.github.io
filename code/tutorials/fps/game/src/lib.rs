// ANCHOR: player_mod_reg
//! Game project.
use crate::player::Player;
use fyrox::{
    core::pool::Handle,
    event::Event,
    gui::message::UiMessage,
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    scene::Scene,
};
use std::path::Path;

// Add this line
pub mod player;
// ANCHOR_END: player_mod_reg

pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    // ANCHOR: player_script_reg
    fn register(&self, context: PluginRegistrationContext) {
        context
            .serialization_context
            .script_constructors
            .add::<Player>("Player");
    }
    // ANCHOR_END: player_script_reg

    fn create_instance(&self, scene_path: Option<&str>, context: PluginContext) -> Box<dyn Plugin> {
        Box::new(Game::new(scene_path, context))
    }
}

pub struct Game {
    scene: Handle<Scene>,
}

impl Game {
    pub fn new(scene_path: Option<&str>, context: PluginContext) -> Self {
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));

        Self {
            scene: Handle::NONE,
        }
    }
}

impl Plugin for Game {
    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, _context: &mut PluginContext) {
        // Add your global update code here.
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: PluginContext) {
        // Do something on OS event here.
    }

    fn on_ui_message(&mut self, _context: &mut PluginContext, _message: &UiMessage) {
        // Handle UI events here.
    }

    fn on_scene_begin_loading(&mut self, path: &Path, ctx: &mut PluginContext) {
        if self.scene.is_some() {
            ctx.scenes.remove(self.scene);
        }
    }

    fn on_scene_loaded(
        &mut self,
        path: &Path,
        scene: Handle<Scene>,
        data: &[u8],
        context: &mut PluginContext,
    ) {
        self.scene = scene;
    }
}
