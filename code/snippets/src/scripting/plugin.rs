use fyrox::gui::inspector::editors::PropertyEditorDefinitionContainer;
use fyrox::gui::UserInterface;
use fyrox::plugin::error::GameResult;
use fyrox::{
    core::{pool::Handle, reflect::prelude::*, visitor::prelude::*},
    event::Event,
    gui::message::UiMessage,
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::Scene,
};
use std::path::Path;
use std::sync::Arc;

// ANCHOR: plugin_structure
#[derive(Visit, Clone, Reflect, Debug)]
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
    fn register(&self, context: PluginRegistrationContext) -> GameResult {
        // Register scripts here.
        Ok(())
    }

    fn register_property_editors(&self, editors: Arc<PropertyEditorDefinitionContainer>) {
        // Register custom property editors for the editor here.
    }

    // ANCHOR: plugin_init
    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) -> GameResult {
        // Do initialization logic here. Usually it just requests a scene:
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));
        Ok(())
    }
    // ANCHOR_END: plugin_init

    fn on_loaded(&mut self, context: PluginContext) -> GameResult {
        // For hot reloading only! Only for development.
        // Re-initialize non-serializable data.
        Ok(())
    }

    fn on_deinit(&mut self, _context: PluginContext) -> GameResult {
        // Do a cleanup here.
        Ok(())
    }

    fn update(&mut self, _context: &mut PluginContext) -> GameResult {
        // Add your global update code here.
        Ok(())
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: PluginContext) -> GameResult {
        // Do something on OS event here.
        Ok(())
    }

    fn on_graphics_context_initialized(&mut self, context: PluginContext) -> GameResult {
        // Executed when graphics context was initialized.
        Ok(())
    }

    fn before_rendering(&mut self, context: PluginContext) -> GameResult {
        // Executed before rendering begins.
        Ok(())
    }

    fn on_graphics_context_destroyed(&mut self, context: PluginContext) -> GameResult {
        // Executed when graphics context was destroyed.
        Ok(())
    }

    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        _message: &UiMessage,
        ui_handle: Handle<UserInterface>,
    ) -> GameResult {
        // Handle UI events here.
        Ok(())
    }

    fn on_scene_begin_loading(&mut self, path: &Path, context: &mut PluginContext) -> GameResult {
        // Handle started scene loading here.
        Ok(())
    }

    fn on_scene_loaded(
        &mut self,
        _path: &Path,
        scene: Handle<Scene>,
        data: &[u8],
        context: &mut PluginContext,
    ) -> GameResult {
        if self.scene.is_some() {
            context.scenes.remove(self.scene);
        }

        self.scene = scene;

        Ok(())
    }

    fn on_scene_loading_failed(
        &mut self,
        path: &Path,
        error: &VisitError,
        context: &mut PluginContext,
    ) -> GameResult {
        // Handle failed scenes here.
        Ok(())
    }
}
// ANCHOR_END: plugin_structure
