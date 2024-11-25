use fyrox::{asset::event::ResourceEvent, asset::manager::ResourceManager};
use std::sync::mpsc::channel;

// ANCHOR: subscribe_to_events
pub fn subscribe_to_events(resource_manager: &ResourceManager) {
    let (sender, receiver) = channel();
    resource_manager.state().event_broadcaster.add(sender);

    while let Ok(event) = receiver.try_recv() {
        match event {
            ResourceEvent::Loaded(_) => {}
            ResourceEvent::Reloaded(_) => {}
            ResourceEvent::Added(_) => {}
            ResourceEvent::Removed(_) => {}
        }
    }
}

// ANCHOR_END: subscribe_to_events
