// ANCHOR: player_mod_reg
use crate::{bot::Bot, player::Player, projectile::Projectile, weapon::Weapon};
use fyrox::plugin::error::GameResult;
use fyrox::plugin::SceneLoaderResult;
use fyrox::{
    core::pool::Handle,
    core::{reflect::prelude::*, visitor::prelude::*},
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::Scene,
};
use std::path::Path;

// Add this line
pub mod player;
// ANCHOR_END: player_mod_reg
pub mod bot;
pub mod projectile;
pub mod weapon;

#[derive(Visit, Reflect, Default, Debug, Clone)]
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

impl Plugin for Game {
    fn register(&self, context: PluginRegistrationContext) -> GameResult {
        // ANCHOR: player_script_reg
        context
            .serialization_context
            .script_constructors
            .add::<Player>("Player");
        // ANCHOR_END: player_script_reg

        // ANCHOR: weapon_script_reg
        context
            .serialization_context
            .script_constructors
            .add::<Weapon>("Weapon");
        // ANCHOR_END: weapon_script_reg

        // ANCHOR: projectile_script_reg
        context
            .serialization_context
            .script_constructors
            .add::<Projectile>("Projectile");
        // ANCHOR_END: projectile_script_reg

        // ANCHOR: bot_script_reg
        context
            .serialization_context
            .script_constructors
            .add::<Bot>("Bot");
        // ANCHOR_END: bot_script_reg
        Ok(())
    }

    fn init(&mut self, scene_path: Option<&str>, mut context: PluginContext) -> GameResult {
        context.load_scene(
            scene_path.unwrap_or("data/scene.rgs"),
            false,
            |result, game: &mut Game, ctx| game.on_scene_loading_result(result, ctx),
        );
        Ok(())
    }
}
