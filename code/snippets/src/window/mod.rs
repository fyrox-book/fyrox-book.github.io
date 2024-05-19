use fyrox::{
    core::log::Log,
    core::visitor::prelude::*,
    core::reflect::prelude::*,
    engine::GraphicsContext,
    plugin::{Plugin, PluginContext},
    window::{CursorGrabMode, Fullscreen},
};

#[derive(Debug, Reflect, Visit)]
struct MyGame {
    fullscreen: bool,
}

impl Plugin for MyGame {
    fn update(&mut self, ctx: &mut PluginContext) {
        // ANCHOR: fullscreen
        if let GraphicsContext::Initialized(ref graphics_context) = ctx.graphics_context {
            // Option 1: Use borderless non-exclusive full screen mode.
            graphics_context
                .window
                .set_fullscreen(Some(Fullscreen::Borderless(None)));

            // Option 2: Use true exclusive full screen mode.
            if let Some(monitor) = graphics_context.window.current_monitor() {
                if let Some(first_avilable_video_mode) = monitor.video_modes().next() {
                    graphics_context
                        .window
                        .set_fullscreen(Some(Fullscreen::Exclusive(first_avilable_video_mode)));
                }
            }
        }
        // ANCHOR_END: fullscreen

        // ANCHOR: title
        if let GraphicsContext::Initialized(ref graphics_context) = ctx.graphics_context {
            graphics_context.window.set_title("My Awesome Game");
        }
        // ANCHOR_END: title

        // ANCHOR: hide_cursor
        // Hide the cursor if the window exists.
        if let GraphicsContext::Initialized(ref graphics_context) = ctx.graphics_context {
            graphics_context.window.set_cursor_visible(false);
        }
        // ANCHOR_END: hide_cursor

        // ANCHOR: lock_cursor
        if let GraphicsContext::Initialized(ref graphics_context) = ctx.graphics_context {
            Log::verify(graphics_context.window.set_cursor_grab(
                // Use one of the following here: None, Confined, Locked. See the API docs for
                // CursorGrabMode for more info.
                CursorGrabMode::Confined,
            ));
        }
        // ANCHOR_END: lock_cursor

        if let GraphicsContext::Initialized(ref graphics_context) = ctx.graphics_context {
            let window_size = graphics_context.window.inner_size();
        }
    }
}
