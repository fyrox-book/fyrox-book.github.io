use fyrox::plugin::error::GameResult;
use fyrox::{
    core::{
        color::Color, pool::Handle, reflect::prelude::*, type_traits::prelude::*,
        visitor::prelude::*, ComponentProvider, TypeUuidProvider,
    },
    graph::SceneGraphNode,
    plugin::{Plugin, PluginContext},
    scene::{navmesh::NavigationalMesh, node::NodeTrait, Scene},
    script::{ScriptContext, ScriptTrait},
};

// ANCHOR: debug_drawing
#[derive(Visit, Default, Reflect, Debug, Clone, ComponentProvider, TypeUuidProvider)]
#[type_uuid(id = "efc71c98-ecf1-4ec3-a08d-116e1656611b")]
struct MyScript {}

impl ScriptTrait for MyScript {
    fn on_update(&mut self, ctx: &mut ScriptContext) -> GameResult {
        let self_position = ctx.scene.graph[ctx.handle].global_position();

        ctx.scene
            .drawing_context
            .draw_sphere(self_position, 16, 16, 0.1, Color::GREEN);
        Ok(())
    }
}
// ANCHOR_END: debug_drawing

// ANCHOR: update_begin
#[derive(Visit, Reflect, Debug)]
struct Game {
    scene: Handle<Scene>,
}

impl Plugin for Game {
    fn update(&mut self, context: &mut PluginContext) -> GameResult {
        // ANCHOR_END: update_begin

        // ANCHOR: clear
        context.scenes[self.scene].drawing_context.clear_lines();
        // ANCHOR_END: clear

        // ANCHOR: physics_drawing
        let scene = &mut context.scenes[self.scene];
        scene.graph.physics.draw(&mut scene.drawing_context);
        // ANCHOR_END: physics_drawing

        // ANCHOR: node_debug_drawing_all
        let scene = &mut context.scenes[self.scene];
        for node in scene.graph.linear_iter() {
            node.debug_draw(&mut scene.drawing_context);
        }
        // ANCHOR_END: node_debug_drawing_all

        // ANCHOR: node_debug_drawing_specific
        let scene = &mut context.scenes[self.scene];
        for node in scene.graph.linear_iter() {
            if let Some(navmesh) = node.component_ref::<NavigationalMesh>() {
                navmesh.debug_draw(&mut scene.drawing_context);
            }
        }
        // ANCHOR_END: node_debug_drawing_specific

        // ANCHOR: update_end

        Ok(())
    }
}
// ANCHOR_END: update_end
