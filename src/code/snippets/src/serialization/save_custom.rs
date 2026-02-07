use fyrox::plugin::error::GameResult;
use fyrox::{
    core::{pool::Handle, reflect::prelude::*, visitor::prelude::*},
    plugin::{Plugin, PluginContext},
    scene::Scene,
};
use std::path::Path;

// ANCHOR: save
#[derive(Visit, Reflect, Debug, Default)]
struct MyData {
    foo: String,
    bar: u32,
}

#[derive(Visit, Reflect, Debug, Default)]
struct MyGame {
    scene: Handle<Scene>,
    data: MyData,
}

impl MyGame {
    fn new(scene_path: Option<&str>, context: PluginContext) -> Self {
        // Load the scene as usual.
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));

        Self {
            scene: Handle::NONE,
            data: Default::default(),
        }
    }

    fn save_game(&mut self, context: &mut PluginContext) {
        let mut visitor = Visitor::new();

        // Serialize the current scene.
        context.scenes[self.scene]
            .save("Scene", &mut visitor)
            .unwrap();

        // Write additional data.
        self.data.visit("Data", &mut visitor).unwrap();

        // Save it to a file.
        visitor.save_binary_to_file(Path::new("save.rgs")).unwrap()
    }

    pub fn load_game(&mut self, context: &mut PluginContext) {
        // Loading of a saved game is very easy - just ask the engine to load your scene.
        // Note the difference with `Game::new` - here we use `request_raw` instead of
        // `request` method. The main difference is that `request` creates a derived scene
        // from a source scene, but `request_raw` loads the scene without any modifications.
        context.async_scene_loader.request_raw("save.rgs");
    }
}

impl Plugin for MyGame {
    fn on_scene_begin_loading(&mut self, _path: &Path, context: &mut PluginContext) -> GameResult {
        if self.scene.is_some() {
            context.scenes.remove(self.scene);
        }
        Ok(())
    }

    fn on_scene_loaded(
        &mut self,
        _path: &Path,
        scene: Handle<Scene>,
        data: &[u8],
        _context: &mut PluginContext,
    ) -> GameResult {
        self.scene = scene;

        // Restore the data when the scene was loaded.
        if let Ok(mut visitor) = Visitor::load_from_memory(data) {
            self.data.visit("Data", &mut visitor).unwrap();
        }

        Ok(())
    }
}
// ANCHOR_END: save
