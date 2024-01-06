// ANCHOR: player_mod_reg
use crate::{player::Player, projectile::Projectile, weapon::Weapon};
use fyrox::{
    core::pool::Handle,
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    scene::Scene,
};
use std::path::Path;

// Add this line
pub mod player;
// ANCHOR_END: player_mod_reg
pub mod projectile;
pub mod weapon;

pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
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
    }

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
