use fyrox::{
    event::{DeviceEvent, Event, WindowEvent},
    plugin::{Plugin, PluginContext},
};

// ANCHOR: events_example
struct MyGame {}

impl Plugin for MyGame {
    fn on_os_event(&mut self, event: &Event<()>, context: PluginContext) {
        match event {
            // This branch should be used for pre-processed events that comes from
            // the main window.
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(_) => {}
                WindowEvent::Moved(_) => {}
                WindowEvent::CloseRequested => {}
                WindowEvent::Destroyed => {}
                WindowEvent::DroppedFile(_) => {}
                WindowEvent::HoveredFile(_) => {}
                WindowEvent::HoveredFileCancelled => {}
                WindowEvent::Focused(_) => {}
                WindowEvent::KeyboardInput { .. } => {}
                WindowEvent::ModifiersChanged(_) => {}
                WindowEvent::Ime(_) => {}
                WindowEvent::CursorMoved { .. } => {}
                WindowEvent::CursorEntered { .. } => {}
                WindowEvent::CursorLeft { .. } => {}
                WindowEvent::MouseWheel { .. } => {}
                WindowEvent::MouseInput { .. } => {}
                WindowEvent::TouchpadPressure { .. } => {}
                WindowEvent::AxisMotion { .. } => {}
                WindowEvent::Touch(_) => {}
                WindowEvent::ScaleFactorChanged { .. } => {}
                WindowEvent::RedrawRequested => {}
                _ => (),
            },
            // This branch should be used for raw input events from various devices.
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::Added => {}
                DeviceEvent::Removed => {}
                DeviceEvent::MouseMotion { .. } => {}
                DeviceEvent::MouseWheel { .. } => {}
                DeviceEvent::Motion { .. } => {}
                DeviceEvent::Button { .. } => {}
                DeviceEvent::Key(_) => {}
            },
            _ => (),
        }
    }
}
// ANCHOR_END: events_example
