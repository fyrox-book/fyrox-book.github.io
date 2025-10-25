// ANCHOR: player_mod_reg
use crate::{bot::Bot, player::Player, projectile::Projectile, weapon::Weapon};
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

impl Plugin for Game {
    fn register(&self, context: PluginRegistrationContext) {
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
    }

    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));
    }

    fn on_scene_begin_loading(&mut self, _path: &Path, ctx: &mut PluginContext) {
        if self.scene.is_some() {
            ctx.scenes.remove(self.scene);
        }
    }

    fn on_scene_loaded(
        &mut self,
        _path: &Path,
        scene: Handle<Scene>,
        _data: &[u8],
        _context: &mut PluginContext,
    ) {
        self.scene = scene;
    }
}
