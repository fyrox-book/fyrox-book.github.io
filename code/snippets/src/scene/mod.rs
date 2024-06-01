pub mod base;
pub mod camera;
pub mod custom;
pub mod decal;
pub mod graph;
pub mod inheritance;
pub mod light;
pub mod mesh;
pub mod particle_system;
pub mod rectangle;
pub mod sprite;

use fyrox::core::color::Color;
use fyrox::core::pool::Handle;
use fyrox::plugin::{Plugin, PluginContext};
use fyrox::{core::reflect::prelude::*, core::visitor::prelude::*, scene::Scene};
use std::path::Path;

// ANCHOR: load_scene
#[derive(Visit, Reflect, Debug)]
struct MyGame {
    main_scene: Handle<Scene>,
}

impl Plugin for MyGame {
    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        // Step 1. Kick off scene loading in a separate thread. This method could
        // be located in any place of your code.
        context.async_scene_loader.request("path/to/your/scene.rgs")
    }

    fn on_scene_loaded(
        &mut self,
        path: &Path,
        scene: Handle<Scene>,
        data: &[u8],
        context: &mut PluginContext,
    ) {
        // Step 2.
        // This method is called once a scene was fully loaded.
        // You may want to remove previous scene first.
        if self.main_scene.is_some() {
            context.scenes.remove(self.main_scene)
        }

        // Remember new scene as main.
        self.main_scene = scene;
    }

    fn on_scene_begin_loading(&mut self, path: &Path, context: &mut PluginContext) {
        // This method is called if a scene just began to load.
    }

    fn on_scene_loading_failed(
        &mut self,
        path: &Path,
        error: &VisitError,
        context: &mut PluginContext,
    ) {
        // This method is called if a scene failed to load.
    }

    // ...
    // ANCHOR_END: load_scene

    // ANCHOR: scene_borrowing
    fn update(&mut self, context: &mut PluginContext) {
        // Borrow a scene using its handle. `try_get` performs immutable borrow, to mutably borrow the scene
        // use `try_get_mut`.
        if let Some(scene) = context.scenes.try_get(self.main_scene) {
            // Do something.
            println!("{:?}", scene.graph.performance_statistics);
        }
    }
    // ANCHOR_END: scene_borrowing
}

// ANCHOR: create_scene
fn create_scene(ctx: &mut PluginContext) -> Handle<Scene> {
    let scene = Scene::new();

    // Use node builders, create sounds, add physics, etc. here to fill the scene.

    ctx.scenes.add(scene)
}
// ANCHOR_END: create_scene

// ANCHOR: set_ambient_lighting
fn set_ambient_lighting(scene: &mut Scene) {
    scene.rendering_options.ambient_lighting_color = Color::opaque(30, 30, 30);
} // ANCHOR_END: set_ambient_lighting
