pub mod base;
pub mod camera;
pub mod collider;
pub mod custom;
pub mod debug;
pub mod decal;
pub mod graph;
pub mod inheritance;
pub mod joint;
pub mod light;
pub mod mesh;
pub mod particle_system;
pub mod probe;
pub mod ray;
pub mod rectangle;
pub mod rigid_body;
pub mod sound;
pub mod sprite;
pub mod terrain;
pub mod tilemap;

use fyrox::plugin::error::GameResult;
use fyrox::{
    core::{algebra::Vector3, pool::Handle, reflect::prelude::*, visitor::prelude::*},
    plugin::{Plugin, PluginContext},
    scene::{
        base::BaseBuilder,
        camera::CameraBuilder,
        mesh::{surface, surface::SurfaceBuilder, MeshBuilder},
        transform::TransformBuilder,
        Scene,
    },
};
use std::path::Path;

// ANCHOR: load_scene
#[derive(Visit, Clone, Reflect, Debug)]
struct MyGame {
    main_scene: Handle<Scene>,
}

impl Plugin for MyGame {
    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) -> GameResult {
        // Step 1. Kick off scene loading in a separate thread. This method could
        // be located in any place of your code.
        context.async_scene_loader.request("path/to/your/scene.rgs");
        Ok(())
    }

    fn on_scene_loaded(
        &mut self,
        path: &Path,
        scene: Handle<Scene>,
        data: &[u8],
        context: &mut PluginContext,
    ) -> GameResult {
        // Step 2.
        // This method is called once a scene was fully loaded.
        // You may want to remove the previous scene first.
        if self.main_scene.is_some() {
            context.scenes.remove(self.main_scene)
        }

        // Remember the new scene as main.
        self.main_scene = scene;

        Ok(())
    }

    fn on_scene_begin_loading(&mut self, path: &Path, context: &mut PluginContext) -> GameResult {
        // This method is called if a scene just began to load.
        Ok(())
    }

    fn on_scene_loading_failed(
        &mut self,
        path: &Path,
        error: &VisitError,
        context: &mut PluginContext,
    ) -> GameResult {
        // This method is called if a scene failed to load.
        Ok(())
    }

    // ...
    // ANCHOR_END: load_scene

    // ANCHOR: scene_borrowing
    fn update(&mut self, context: &mut PluginContext) -> GameResult {
        // Borrow a scene using its handle. `try_get` performs immutable borrow, to mutably borrow the scene
        // use `try_get_mut`.
        if let Ok(scene) = context.scenes.try_get(self.main_scene) {
            // Do something.
            println!("{:?}", scene.graph.performance_statistics);
        }
        Ok(())
    }
    // ANCHOR_END: scene_borrowing
}

// ANCHOR: create_scene
fn create_scene(ctx: &mut PluginContext) -> Handle<Scene> {
    let mut scene = Scene::new();

    // Use node builders, create sounds, add physics, etc. here to fill the scene.
    // The following code creates a simple cube and a camera to visualize it.
    CameraBuilder::new(BaseBuilder::new()).build(&mut scene.graph);

    MeshBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                .with_local_position(Vector3::new(0.0, 0.0, 3.0))
                .build(),
        ),
    )
    .with_surfaces(vec![
        SurfaceBuilder::new(surface::CUBE.resource.clone()).build()
    ])
    .build(&mut scene.graph);

    ctx.scenes.add(scene)
}
// ANCHOR_END: create_scene
