use fyrox::engine::executor::Executor;
use fyrox::engine::GraphicsContextParams;
use fyrox::event_loop::EventLoop;
use fyrox::window::WindowAttributes;

// ANCHOR: executor_named_objects
fn main() {
    let executor = Executor::from_params(
        EventLoop::new().ok(),
        GraphicsContextParams {
            // This option forces the engine to use meaningful names for
            // GPU objects (textures, buffers, shaders, etc.)
            named_objects: true,
            window_attributes: WindowAttributes::default(),
            vsync: true,
            msaa_sample_count: None,
            graphics_server_constructor: Default::default(),
        },
    );
    // ...
}
// ANCHOR_END: executor_named_objects
