# Setting up

There is no framework for rg3d-ui itself. This makes it a much more involved part of the engine.

To start off we will use an already available program (This will not do anything)

```rust

use rg3d::utils::log::{Log, MessageKind};
use rg3d::{
    animation::Animation,
    core::{
        algebra::{UnitQuaternion, Vector2, Vector3},
        color::Color,
        pool::Handle,
    },
    engine::{resource_manager::{ResourceManager, MaterialSearchOptions}, Engine},
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    gui::{
        //! We will leave this empty for now. Except for this
        UiNode,
    },
    monitor::VideoMode,
    scene::{node::Node, Scene},
    utils::translate_event,
    window::Fullscreen,
};
use rg3d_ui::formatted_text::WrapMode;
use std::time::Instant;

struct Interface {}

fn UI(engine: &mut Engine) -> Interface {
    let window_width = engine.renderer.get_frame_size().0 as f32;

    // Gather all suitable video modes, we'll use them to fill combo box of
    // available resolutions.
    let video_modes = engine
        .get_window()
        .primary_monitor()
        .unwrap()
        .video_modes()
        .filter(|vm| {
            // Leave only modern video modes, we are not in 1998.
            vm.size().width > 800 && vm.size().height > 600 && vm.bit_depth() == 32
        })
        .collect::<Vec<_>>();

    let ctx = &mut engine.user_interface.build_ctx();

    WindowBuilder::new(
        WidgetBuilder::new()
            // We want the window to be anchored at right top corner at the beginning
            .with_desired_position(Vector2::new(window_width - 300.0, 0.0))
            .with_width(300.0),
    )
    .with_title(WindowTitle::text("Model Options"))
    .can_close(false)
    .build(ctx);

    Interface {}
}
fn main() {
    let event_loop = EventLoop::new();

    let window_builder = rg3d::window::WindowBuilder::new()
        .with_title("Example - User Interface")
        .with_resizable(true);

    let mut engine = Engine::new(window_builder, &event_loop, true).unwrap();

    // Create simple user interface that will show some useful info.
    let interface = UI(&mut engine);
    
    // Finally run our event loop which will respond to OS and window events and update
    // engine state accordingly. Engine lets you to decide which event should be handled,
    // this is minimal working example if how it should be.
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared => {
                // It is very important to "pump" messages from UI. This our main point where we communicate
                // with user interface. As you saw earlier, there is no callbacks on UI elements, instead we
                // use messages to get information from UI elements. This provides perfect decoupling of logic
                // from UI elements and works well with borrow checker.
                while let Some(ui_message) = engine.user_interface.poll_message() {
                    match ui_message.data() {
                             //! Leave blank for now
                        }
                        _ => (),
                    }
                }

                // Rendering must be explicitly requested and handled after RedrawRequested event is received.
                engine.get_window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Run renderer at max speed - it is not tied to game code.
                engine.render().unwrap();
            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => {
                        // It is very important to handle Resized event from window, because
                        // renderer knows nothing about window size - it must be notified
                        // directly when window size has changed.
                        if let Err(e) = engine.set_frame_size(size.into()) {
                            Log::writeln(
                                MessageKind::Error,
                                format!("Unable to set frame size: {:?}", e),
                            );
                        }
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(key_code) = input.virtual_keycode {
                            if input.state == ElementState::Pressed
                                && key_code == VirtualKeyCode::Escape
                            {
                                *control_flow = ControlFlow::Exit;
                            }
                        }
                    }
                    _ => (),
                }

                // It is very important to "feed" user interface (UI) with events coming
                // from main window, otherwise UI won't respond to mouse, keyboard, or any
                // other event.
                if let Some(os_event) = translate_event(&event) {
                    engine.user_interface.process_os_event(&os_event);
                }
            }
            Event::DeviceEvent { .. } => {
                // Handle key input events via `WindowEvent`, not via `DeviceEvent` (#32)
            }
            _ => *control_flow = ControlFlow::Poll,
        }
    });
}
```

What we have written here is the imports, some will get removed along the way but keeping them won't hurt,
the interface struct which we will add to, the UI function and what we have for this guide and the event loop
for the Program that watches for UI events and computer events.

This will be the basis for all the future tutorials.
