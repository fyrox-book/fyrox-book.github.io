use fyrox::{
    core::{
        algebra::{Vector2, Vector3},
        pool::Handle,
        reflect::prelude::*,
        type_traits::prelude::*,
        visitor::prelude::*,
    },
    engine::Engine,
    gui::{BuildContext, UiNode},
    scene::node::Node,
    script::ScriptTrait,
};
use fyroxed_base::{
    interaction::{gizmo::move_gizmo::MoveGizmo, make_interaction_mode_button, InteractionMode},
    plugin::EditorPlugin,
    scene::{controller::SceneController, GameScene, Selection},
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
                    let move_gizmo = MoveGizmo::new(game_scene, &mut editor.engine);

                    entry.interaction_modes.add(MyInteractionMode {
                        move_gizmo,
                        drag_context: None,
                    });
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

struct DragContext {
    point_index: usize,
    initial_position: Vector3<f32>,
}

// ANCHOR: interaction_mode
#[derive(TypeUuidProvider)]
#[type_uuid(id = "d7f56947-a106-408a-9c18-d0191ef89925")]
pub struct MyInteractionMode {
    move_gizmo: MoveGizmo,
    drag_context: Option<DragContext>,
}

impl MyInteractionMode {
    pub fn new(game_scene: &GameScene, engine: &mut Engine) -> Self {
        Self {
            move_gizmo: MoveGizmo::new(game_scene, engine),
            drag_context: None,
        }
    }
}

impl InteractionMode for MyInteractionMode {
    fn on_left_mouse_button_down(
        &mut self,
        editor_selection: &Selection,
        controller: &mut dyn SceneController,
        engine: &mut Engine,
        mouse_pos: Vector2<f32>,
        frame_size: Vector2<f32>,
        settings: &Settings,
    ) {
    }

    fn on_left_mouse_button_up(
        &mut self,
        editor_selection: &Selection,
        controller: &mut dyn SceneController,
        engine: &mut Engine,
        mouse_pos: Vector2<f32>,
        frame_size: Vector2<f32>,
        settings: &Settings,
    ) {
    }

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
    }

    fn deactivate(&mut self, controller: &dyn SceneController, engine: &mut Engine) {}

    fn make_button(&mut self, ctx: &mut BuildContext, selected: bool) -> Handle<UiNode> {
        make_interaction_mode_button(ctx, include_bytes!("icon.png"), "Line Edit Mode", selected)
    }

    fn uuid(&self) -> Uuid {
        Self::type_uuid()
    }
}
// ANCHOR_END: interaction_mode

fn add_my_plugin(editor: &mut Editor) {
    // ANCHOR: plugin_registration
    editor.add_editor_plugin(MyPlugin::default());
    // ANCHOR_END: plugin_registration
}
