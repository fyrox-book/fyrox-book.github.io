use fyrox::engine::executor::Executor;
use fyrox::engine::GraphicsContextParams;
use fyrox::event_loop::EventLoop;
use fyrox::window::WindowAttributes;

// ANCHOR: executor_window_title
fn main() {
    let executor = Executor::from_params(
        EventLoop::new().ok(),
        GraphicsContextParams {
            window_attributes: WindowAttributes::default().with_title("My Game"),
            vsync: true,
            msaa_sample_count: None,
            graphics_server_constructor: Default::default(),
            named_objects: false,
        },
    );
    // ...
}
// ANCHOR_END: executor_window_title
