use fyrox::{
    core::{
        algebra::{Vector2, Vector3},
        pool::Handle,
        reflect::prelude::*,
        type_traits::prelude::*,
        visitor::prelude::*,
    },
    engine::Engine,
    graph::BaseSceneGraph,
    gui::{BuildContext, UiNode},
    scene::{base::BaseBuilder, graph::Graph, node::Node, sprite::SpriteBuilder},
    script::ScriptTrait,
};
use fyroxed_base::{
    camera::PickingOptions,
    command::{CommandContext, CommandTrait},
    interaction::{
        gizmo::move_gizmo::MoveGizmo, make_interaction_mode_button, plane::PlaneKind,
        InteractionMode,
    },
    message::MessageSender,
    plugin::EditorPlugin,
    scene::{commands::GameSceneContext, controller::SceneController, GameScene, Selection},
    settings::Settings,
    Editor, Message,
};

// ANCHOR: my_script
#[derive(Clone, Debug, TypeUuidProvider, ComponentProvider, Reflect, Visit)]
#[type_uuid(id = "69302f1c-f3c7-4853-801c-552c566948d0")]
pub struct MyScript {
    points: Vec<Vector3<f32>>,
}

impl ScriptTrait for MyScript {}
// ANCHOR_END: my_script

// ANCHOR: plugin_definition
#[derive(Default)]
pub struct MyPlugin {
    node_handle: Handle<Node>,
}
// ANCHOR_END: plugin_definition

// ANCHOR: plugin_impl_1
impl EditorPlugin for MyPlugin {
    // ANCHOR_END: plugin_impl_1
    fn on_sync_to_model(&mut self, editor: &mut Editor) {}

    fn on_update(&mut self, editor: &mut Editor) {}

    // ANCHOR: selection_1
    fn on_message(&mut self, message: &Message, editor: &mut Editor) {
        // Fetch the active scene.
        let Some(entry) = editor.scenes.current_scene_entry_mut() else {
            return;
        };

        let Some(selection) = entry.selection.as_graph() else {
            return;
        };

        // Try to cast it to GameScene, it could also be UiScene for UI scene plugins.
        let Some(game_scene) = entry.controller.downcast_mut::<GameScene>() else {
            return;
        };

        let scene = &mut editor.engine.scenes[game_scene.scene];

        // When user clicks on some object in scene, the editor produces `SelectionChanged` message
        // which we can catch and check which object was selected.
        // ANCHOR: gizmo_destroy
        if let Message::SelectionChanged { .. } = message {
            // ANCHOR_END: selection_1

            if let Some(mode) = entry.interaction_modes.remove_typed::<MyInteractionMode>() {
                mode.move_gizmo.destroy(&mut scene.graph);
            }
            // ANCHOR_END: gizmo_destroy

            // ANCHOR: selection_2
            for node_handle in selection.nodes().iter() {
                // An object with our script was selected, remember the handle of it in the
                // plugin.
                if scene
                    .graph
                    .try_get_script_of::<MyScript>(*node_handle)
                    .is_some()
                {
                    self.node_handle = *node_handle;

                    // ANCHOR_END: selection_2

                    // ANCHOR: interaction_mode_create
                    entry.interaction_modes.add(MyInteractionMode::new(
                        game_scene,
                        &mut editor.engine,
                        editor.message_sender.clone(),
                        *node_handle,
                    ));
                    // ANCHOR_END: interaction_mode_create

                    // ANCHOR: selection_3
                    break;
                }
            }
        }
    }
    // ANCHOR_END: selection_3

    // ANCHOR: plugin_impl_2
}
// ANCHOR_END: plugin_impl_2

// ANCHOR: interaction_mode_definition
struct DragContext {
    point_index: usize,
    initial_position: Vector3<f32>,
    plane_kind: PlaneKind,
}

#[derive(TypeUuidProvider)]
#[type_uuid(id = "d7f56947-a106-408a-9c18-d0191ef89925")]
pub struct MyInteractionMode {
    move_gizmo: MoveGizmo,
    node_handle: Handle<Node>,
    drag_context: Option<DragContext>,
    message_sender: MessageSender,
    line_points_gizmo: LinePointsGizmo,
    selected_point_index: Option<usize>,
}

impl MyInteractionMode {
    pub fn new(
        game_scene: &GameScene,
        engine: &mut Engine,
        message_sender: MessageSender,
        node_handle: Handle<Node>,
    ) -> Self {
        Self {
            move_gizmo: MoveGizmo::new(game_scene, engine),
            node_handle,
            drag_context: None,
            message_sender,
            line_points_gizmo: LinePointsGizmo::default(),
            selected_point_index: None,
        }
    }
}
// ANCHOR_END: interaction_mode_definition

impl InteractionMode for MyInteractionMode {
    // ANCHOR: on_left_mouse_button_down
    fn on_left_mouse_button_down(
        &mut self,
        editor_selection: &Selection,
        controller: &mut dyn SceneController,
        engine: &mut Engine,
        mouse_pos: Vector2<f32>,
        frame_size: Vector2<f32>,
        settings: &Settings,
    ) {
        let Some(game_scene) = controller.downcast_mut::<GameScene>() else {
            return;
        };

        let scene = &mut engine.scenes[game_scene.scene];

        // Pick scene entity at the cursor position.
        if let Some(result) = game_scene.camera_controller.pick(
            &scene.graph,
            PickingOptions {
                cursor_pos: mouse_pos,
                editor_only: true,
                filter: Some(&mut |handle, _| handle != self.move_gizmo.origin),
                ..Default::default()
            },
        ) {
            // The gizmo needs to be fed with input events as well, so it can react to the cursor.
            if let Some(plane_kind) = self.move_gizmo.handle_pick(result.node, &mut scene.graph) {
                // Start point dragging if there's any point selected.
                if let Some(selected_point_index) = self.selected_point_index {
                    self.drag_context = Some(DragContext {
                        point_index: selected_point_index,
                        initial_position: scene.graph
                            [self.line_points_gizmo.point_nodes[selected_point_index]]
                            .global_position(),
                        plane_kind,
                    })
                }
            } else {
                // Handle point picking and remember a selected point.
                for (index, point_handle) in self.line_points_gizmo.point_nodes.iter().enumerate() {
                    if result.node == *point_handle {
                        self.selected_point_index = Some(index);
                    }
                }
            }
        }
    }
    // ANCHOR_END: on_left_mouse_button_down

    // ANCHOR: on_left_mouse_button_up
    fn on_left_mouse_button_up(
        &mut self,
        editor_selection: &Selection,
        controller: &mut dyn SceneController,
        engine: &mut Engine,
        mouse_pos: Vector2<f32>,
        frame_size: Vector2<f32>,
        settings: &Settings,
    ) {
        let Some(game_scene) = controller.downcast_mut::<GameScene>() else {
            return;
        };

        let scene = &mut engine.scenes[game_scene.scene];

        if let Some(drag_context) = self.drag_context.take() {
            if let Some(script) = scene
                .graph
                .try_get_script_of_mut::<MyScript>(self.node_handle)
            {
                // Restore the position of the point and use its new position as the value for
                // the command below.
                let new_position = std::mem::replace(
                    &mut script.points[drag_context.point_index],
                    drag_context.initial_position,
                );

                // Confirm the action by creating respective command.
                self.message_sender.do_command(SetPointPositionCommand {
                    node_handle: self.node_handle,
                    point_index: drag_context.point_index,
                    point_position: new_position,
                });
            }
        }
    }
    // ANCHOR_END: on_left_mouse_button_up

    // ANCHOR: on_mouse_move
    fn on_mouse_move(
        &mut self,
        mouse_offset: Vector2<f32>,
        mouse_position: Vector2<f32>,
        editor_selection: &Selection,
        controller: &mut dyn SceneController,
        engine: &mut Engine,
        frame_size: Vector2<f32>,
        settings: &Settings,
    ) {
        let Some(game_scene) = controller.downcast_mut::<GameScene>() else {
            return;
        };

        let scene = &mut engine.scenes[game_scene.scene];

        if let Some(drag_context) = self.drag_context.as_ref() {
            let global_offset = self.move_gizmo.calculate_offset(
                &scene.graph,
                game_scene.camera_controller.camera,
                mouse_offset,
                mouse_position,
                frame_size,
                drag_context.plane_kind,
            );

            if let Some(script) = scene
                .graph
                .try_get_script_of_mut::<MyScript>(self.node_handle)
            {
                script.points[drag_context.point_index] =
                    drag_context.initial_position + global_offset;
            }
        }
    }
    // ANCHOR_END: on_mouse_move

    // ANCHOR: update
    fn update(
        &mut self,
        editor_selection: &Selection,
        controller: &mut dyn SceneController,
        engine: &mut Engine,
        settings: &Settings,
    ) {
        let Some(game_scene) = controller.downcast_mut::<GameScene>() else {
            return;
        };

        let scene = &mut engine.scenes[game_scene.scene];

        self.line_points_gizmo
            .sync_to_model(self.node_handle, game_scene, &mut scene.graph);
    }
    // ANCHOR_END: update

    fn deactivate(&mut self, controller: &dyn SceneController, engine: &mut Engine) {}

    // ANCHOR: make_button
    fn make_button(&mut self, ctx: &mut BuildContext, selected: bool) -> Handle<UiNode> {
        make_interaction_mode_button(ctx, include_bytes!("icon.png"), "Line Edit Mode", selected)
    }
    // ANCHOR_END: make_button

    fn uuid(&self) -> Uuid {
        Self::type_uuid()
    }
}

fn add_my_plugin(editor: &mut Editor) {
    // ANCHOR: plugin_registration
    editor.add_editor_plugin(MyPlugin::default());
    // ANCHOR_END: plugin_registration
}

// ANCHOR: line_points_gizmo
#[derive(Default)]
struct LinePointsGizmo {
    point_nodes: Vec<Handle<Node>>,
}

impl LinePointsGizmo {
    fn sync_to_model(
        &mut self,
        node_handle: Handle<Node>,
        game_scene: &GameScene,
        graph: &mut Graph,
    ) {
        let Some(script) = graph.try_get_script_of::<MyScript>(node_handle) else {
            return;
        };
        let points = script.points.clone();

        if self.point_nodes.len() != points.len() {
            self.remove_points(graph);
            for point in points {
                // Point could be represented via sprite - it will always be facing towards editor's
                // camera.
                let point_node = SpriteBuilder::new(BaseBuilder::new())
                    .with_size(0.1)
                    .build(graph);

                self.point_nodes.push(point_node);

                // Link the sprite with the special scene node - the name of it should clearly state
                // its purpose.
                graph.link_nodes(point_node, game_scene.editor_objects_root);
            }
        }
    }

    fn remove_points(&mut self, graph: &mut Graph) {
        for handle in self.point_nodes.drain(..) {
            graph.remove_node(handle);
        }
    }
}

// ANCHOR_END: line_points_gizmo

// ANCHOR: command
#[derive(Debug)]
struct SetPointPositionCommand {
    node_handle: Handle<Node>,
    point_index: usize,
    point_position: Vector3<f32>,
}

impl SetPointPositionCommand {
    fn swap(&mut self, context: &mut dyn CommandContext) {
        // Get typed version of the context, it could also be UiSceneContext for
        // UI scenes.
        let context = context.get_mut::<GameSceneContext>();
        // Get a reference to the script instance.
        let script = context.scene.graph[self.node_handle]
            .try_get_script_mut::<MyScript>()
            .unwrap();
        // Swap the position of the point with the one stored in the command.
        std::mem::swap(
            &mut script.points[self.point_index],
            &mut self.point_position,
        );
    }
}

impl CommandTrait for SetPointPositionCommand {
    fn name(&mut self, context: &dyn CommandContext) -> String {
        "Set Point Position".to_owned()
    }

    fn execute(&mut self, context: &mut dyn CommandContext) {
        self.swap(context)
    }

    fn revert(&mut self, context: &mut dyn CommandContext) {
        self.swap(context)
    }
}
// ANCHOR_END: command
