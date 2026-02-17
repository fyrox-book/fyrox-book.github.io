use fyrox::plugin::error::GameResult;
use fyrox::plugin::SceneLoaderResult;
use fyrox::{
    core::{pool::Handle, reflect::prelude::*, visitor::prelude::*},
    plugin::{Plugin, PluginContext},
    scene::Scene,
};
use std::path::Path;

// ANCHOR: save
#[derive(Visit, Clone, Reflect, Debug, Default)]
struct MyData {
    foo: String,
    bar: u32,
}

#[derive(Visit, Clone, Reflect, Debug, Default)]
struct MyGame {
    scene: Handle<Scene>,
    data: MyData,
}

impl MyGame {
    fn load_scene(&mut self, path: &str, ctx: &mut PluginContext) -> GameResult {
        if self.scene.is_some() {
            ctx.scenes.remove(self.scene);
        }
        ctx.load_scene(path, false, |result, game: &mut MyGame, ctx| {
            game.on_scene_loading_result(result, ctx)
        });
        Ok(())
    }

    fn on_scene_loading_result(
        &mut self,
        result: SceneLoaderResult,
        ctx: &mut PluginContext,
    ) -> GameResult {
        self.scene = ctx.scenes.add(result?.payload);
        Ok(())
    }

    fn save_game(&mut self, context: &mut PluginContext) {
        let mut visitor = Visitor::new();
        // Serialize the current scene.
        context.scenes[self.scene]
            .save("Scene", &mut visitor)
            .unwrap();
        // Save it to a file.
        visitor.save_binary_to_file(Path::new("save.rgs")).unwrap()
    }

    pub fn load_game(&mut self, ctx: &mut PluginContext) {
        // Loading of a saved game is very easy - just ask the engine to load your scene.
        // Note the difference with `Game::new` - here we use `request_raw` instead of
        // `request` method. The main difference is that `request` creates a derived scene
        // from a source scene, but `request_raw` loads the scene without any modifications.
        self.load_scene("save.rgs", ctx).unwrap();
    }
}

impl Plugin for MyGame {
    fn init(&mut self, scene_path: Option<&str>, mut context: PluginContext) -> GameResult {
        self.load_scene(scene_path.unwrap_or("data/scene.rgs"), &mut context)
    }
}

// ANCHOR_END: save
