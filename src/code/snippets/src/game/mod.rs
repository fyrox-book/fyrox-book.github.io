use fyrox::{
    asset::manager::ResourceManager,
    core::{
        instant::Instant,
        log::{Log, MessageKind},
        task::TaskPool,
    },
    engine::{
        Engine, EngineInitParams, GraphicsContext, GraphicsContextParams,
        GraphicsServerConstructor, SerializationContext,
    },
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    gui::constructor::WidgetConstructorContainer,
    utils::translate_event,
    window::WindowAttributes,
};
use std::sync::Arc;

// ANCHOR: custom_game_loop
fn main() {
    let event_loop = EventLoop::new().unwrap();

    let mut window_attributes = WindowAttributes::default();
    window_attributes.resizable = true;
    window_attributes.title = "My Game".to_string();

    let serialization_context = Arc::new(SerializationContext::new());
    let task_pool = Arc::new(TaskPool::new());
    let mut engine = Engine::new(EngineInitParams {
        graphics_context_params: GraphicsContextParams {
            window_attributes,
            vsync: true,
            msaa_sample_count: None,
            graphics_server_constructor: GraphicsServerConstructor::default(),
        },
        resource_manager: ResourceManager::new(task_pool.clone()),
        serialization_context,
        task_pool,
        widget_constructors: Arc::new(WidgetConstructorContainer::default()),
    })
    .unwrap();

    let mut previous = Instant::now();
    let fixed_time_step = 1.0 / 60.0;
    let mut lag = 0.0;

    event_loop
        .run(move |event, window_target| {
            window_target.set_control_flow(ControlFlow::Wait);

            let scenes = engine
                .scenes
                .pair_iter()
                .map(|(s, _)| s)
                .collect::<Vec<_>>();

            match event {
                Event::Resumed => {
                    engine
                        .initialize_graphics_context(window_target)
                        .expect("Unable to initialize graphics context!");
                }
                Event::Suspended => {
                    engine
                        .destroy_graphics_context()
                        .expect("Unable to destroy graphics context!");
                }
                Event::AboutToWait => {
                    // This main game loop - it has fixed time step which means that game
                    // code will run at fixed speed even if renderer can't give you desired
                    // 60 fps.
                    let elapsed = previous.elapsed();
                    previous = Instant::now();
                    lag += elapsed.as_secs_f32();
                    while lag >= fixed_time_step {
                        lag -= fixed_time_step;

                        // ************************
                        // ************************
                        // Put your game logic here.
                        // ************************
                        // ************************

                        // It is very important to update the engine every frame!
                        engine.update(fixed_time_step, window_target, &mut lag, Default::default());
                    }

                    if let GraphicsContext::Initialized(ref ctx) = engine.graphics_context {
                        ctx.window.request_redraw();
                    }
                }
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => window_target.exit(),
                        WindowEvent::Resized(size) => {
                            if let Err(e) = engine.set_frame_size(size.into()) {
                                Log::writeln(
                                    MessageKind::Error,
                                    format!("Unable to set frame size: {:?}", e),
                                );
                            }
                        }
                        WindowEvent::RedrawRequested => {
                            engine.render().unwrap();
                        }
                        _ => (),
                    }

                    if let Some(os_event) = translate_event(&event) {
                        for ui in engine.user_interfaces.iter_mut() {
                            ui.process_os_event(&os_event);
                        }
                    }
                }
                _ => (),
            }
        })
        .unwrap();
}
// ANCHOR_END: custom_game_loop
