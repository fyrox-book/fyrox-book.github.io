use fyrox::asset::manager::ResourceManager;
use fyrox::core::pool::Handle;
use fyrox::engine::task::TaskPoolHandler;
use fyrox::engine::{GraphicsContext, ScriptMessageDispatcher};
use fyrox::gui::UiContainer;
use fyrox::scene::node::Node;
use fyrox::scene::Scene;
use fyrox::script::{PluginsRefMut, ScriptMessageSender};

// ANCHOR: context
pub struct ScriptContext<'a, 'b, 'c> {
    pub dt: f32,
    pub elapsed_time: f32,
    pub plugins: PluginsRefMut<'a>,
    pub handle: Handle<Node>,
    pub scene: &'b mut Scene,
    pub scene_handle: Handle<Scene>,
    pub resource_manager: &'a ResourceManager,
    pub message_sender: &'c ScriptMessageSender,
    pub message_dispatcher: &'c mut ScriptMessageDispatcher,
    pub task_pool: &'a mut TaskPoolHandler,
    pub graphics_context: &'a mut GraphicsContext,
    pub user_interfaces: &'a mut UiContainer,
    pub script_index: usize,
}
// ANCHOR_END: context
