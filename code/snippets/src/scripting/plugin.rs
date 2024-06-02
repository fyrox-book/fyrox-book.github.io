use fyrox::gui::inspector::editors::PropertyEditorDefinitionContainer;
use fyrox::{
    core::{pool::Handle, reflect::prelude::*, visitor::prelude::*},
    event::Event,
    gui::message::UiMessage,
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::Scene,
};
use std::path::Path;

// ANCHOR: plugin_structure
#[derive(Visit, Reflect, Debug)]
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
    fn register(&self, context: PluginRegistrationContext) {
        // Register scripts here.
    }

    fn register_property_editors(&self) -> PropertyEditorDefinitionContainer {
        // Register custom property editors for the editor here.
        PropertyEditorDefinitionContainer::empty()
    }

    // ANCHOR: plugin_init
    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        // Do initialization logic here. Usually it just requests a scene:
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));
    }
    // ANCHOR_END: plugin_init

    fn on_loaded(&mut self, context: PluginContext) {
        // For hot reloading only! Only for development.
        // Re-initialize non-serializable data.
    }

    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, _context: &mut PluginContext) {
        // Add your global update code here.
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: PluginContext) {
        // Do something on OS event here.
    }

    fn on_graphics_context_initialized(&mut self, context: PluginContext) {
        // Executed when graphics context was initialized.
    }

    fn before_rendering(&mut self, context: PluginContext) {
        // Executed before rendering begins.
    }

    fn on_graphics_context_destroyed(&mut self, context: PluginContext) {
        // Executed when graphics context was destroyed.
    }

    fn on_ui_message(&mut self, _context: &mut PluginContext, _message: &UiMessage) {
        // Handle UI events here.
    }

    fn on_scene_begin_loading(&mut self, path: &Path, context: &mut PluginContext) {
        // Handle started scene loading here.
    }

    fn on_scene_loaded(
        &mut self,
        _path: &Path,
        scene: Handle<Scene>,
        data: &[u8],
        context: &mut PluginContext,
    ) {
        if self.scene.is_some() {
            context.scenes.remove(self.scene);
        }

        self.scene = scene;
    }

    fn on_scene_loading_failed(
        &mut self,
        path: &Path,
        error: &VisitError,
        context: &mut PluginContext,
    ) {
        // Handle failed scenes here.
    }
}
// ANCHOR_END: plugin_structure
