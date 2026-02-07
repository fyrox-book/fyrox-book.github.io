//! Game project.
use crate::player::Player;
use fyrox::plugin::error::GameResult;
use fyrox::{
    core::pool::Handle,
    core::{reflect::prelude::*, visitor::prelude::*},
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::Scene,
};
use std::path::Path;

mod player;

#[derive(Visit, Reflect, Clone, Default, Debug)]
pub struct Game {
    scene: Handle<Scene>,
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

    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) -> GameResult {
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));
        Ok(())
    }

    fn on_scene_loaded(
        &mut self,
        _path: &Path,
        scene: Handle<Scene>,
        _data: &[u8],
        context: &mut PluginContext,
    ) -> GameResult {
        if self.scene.is_some() {
            context.scenes.remove(self.scene);
        }

        self.scene = scene;

        Ok(())
    }
}
