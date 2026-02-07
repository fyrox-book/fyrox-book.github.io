use fyrox::core::{ComponentProvider, TypeUuidProvider};
use fyrox::graph::SceneGraph;
use fyrox::plugin::error::GameResult;
use fyrox::plugin::{Plugin, PluginContext};
use fyrox::{
    core::{
        algebra::Vector3, log::Log, pool::Handle, reflect::prelude::*, type_traits::prelude::*,
        visitor::prelude::*,
    },
    scene::{debug::Line, navmesh::NavigationalMesh, node::Node},
    script::{ScriptContext, ScriptTrait},
};
use std::fs::File;
use std::io::Read;

// ANCHOR: script_task
#[derive(Visit, Default, Reflect, Debug, Clone, ComponentProvider, TypeUuidProvider)]
#[type_uuid(id = "efc71c98-ecf1-4ec3-a08d-116e1656611b")]
struct MyScript {
    navmesh: Handle<Node>,
    path: Option<Vec<Vector3<f32>>>,
}

impl ScriptTrait for MyScript {
    fn on_start(&mut self, ctx: &mut ScriptContext) -> GameResult {
        // Borrow a navigational mesh scene node first.
        let navmesh_node = ctx
            .scene
            .graph
            .try_get_of_type::<NavigationalMesh>(self.navmesh)?;

        // Take a shared reference to the internal navigational mesh.
        let shared_navmesh = navmesh_node.navmesh();

        // Spawn a task, that will calculate a long path.
        ctx.task_pool.spawn_script_task(
            ctx.scene_handle,
            ctx.handle,
            ctx.script_index,
            async move {
                let navmesh = shared_navmesh.read();

                if let Some((_, begin_index)) = navmesh.query_closest(Vector3::new(1.0, 0.0, 3.0)) {
                    if let Some((_, end_index)) =
                        navmesh.query_closest(Vector3::new(500.0, 0.0, 800.0))
                    {
                        let mut path = Vec::new();
                        if navmesh
                            .build_path(begin_index, end_index, &mut path)
                            .is_ok()
                        {
                            return Some(path);
                        }
                    }
                }

                None
            },
            |path, this: &mut MyScript, _ctx| {
                this.path = path;

                Log::info("Path is calculated!");

                Ok(())
            },
        );

        Ok(())
    }

    fn on_update(&mut self, ctx: &mut ScriptContext) {
        // Draw the computed path.
        if let Some(path) = self.path.as_ref() {
            for segment in path.windows(2) {
                ctx.scene.drawing_context.add_line(Line {
                    begin: segment[0],
                    end: segment[1],
                    color: Default::default(),
                })
            }
        }
    }
}
// ANCHOR_END: script_task

// ANCHOR: plugin_task
#[derive(Debug, Visit, Reflect)]
struct MyGame {
    data: Option<Vec<u8>>,
}

impl MyGame {
    pub fn new(context: PluginContext) -> Self {
        context.task_pool.spawn_plugin_task(
            // Emulate heavy task by reading a potentially large file. The game will be fully
            // responsive while it runs.
            async move {
                let mut file = File::open("some/file.txt").unwrap();
                let mut data = Vec::new();
                file.read_to_end(&mut data).unwrap();
                data
            },
            // This closure is called when the future above has finished, but not immediately - on
            // the next update iteration.
            |data, game: &mut MyGame, _context| {
                // Store the data in the game instance.
                game.data = Some(data);
            },
        );

        // Immediately return the new game instance with empty data.
        Self { data: None }
    }
}

impl Plugin for MyGame {
    fn update(&mut self, _context: &mut PluginContext) -> GameResult {
        // Do something with the data.
        if let Some(data) = self.data.take() {
            println!("The data is: {:?}", data);
        }
        Ok(())
    }
}
// ANCHOR_END: plugin_task
