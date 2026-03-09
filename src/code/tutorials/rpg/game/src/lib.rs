//! Game project.
use crate::player::Player;
use fyrox::plugin::error::GameResult;
use fyrox::plugin::SceneLoaderResult;
use fyrox::{
    core::pool::Handle,
    core::{reflect::prelude::*, visitor::prelude::*},
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::Scene,
};

mod player;

#[derive(Visit, Reflect, Clone, Default, Debug)]
pub struct Game {
    scene: Handle<Scene>,
}

impl Game {
    fn on_scene_loading_result(
        &mut self,
        result: SceneLoaderResult,
        ctx: &mut PluginContext,
    ) -> GameResult {
        if self.scene.is_some() {
            ctx.scenes.remove(self.scene);
        }
        self.scene = ctx.scenes.add(result?.payload);
        Ok(())
    }
}

// ANCHOR: register
impl Plugin for Game {
    fn register(&self, context: PluginRegistrationContext) -> GameResult {
        context
            .serialization_context
            .script_constructors
            .add::<Player>("Player");
        Ok(())
    }
    // ANCHOR_END: register

    fn init(&mut self, scene_path: Option<&str>, mut context: PluginContext) -> GameResult {
        context.load_scene(
            scene_path.unwrap_or("data/scene.rgs"),
            false,
            |result, game: &mut Game, ctx| game.on_scene_loading_result(result, ctx),
        );
        Ok(())
    }

}
