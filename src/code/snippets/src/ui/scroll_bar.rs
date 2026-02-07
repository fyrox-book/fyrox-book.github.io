use fyrox::gui::message::UiMessage;
use fyrox::gui::scroll_bar::{ScrollBar, ScrollBarMessage};
use fyrox::gui::{
    core::pool::Handle,
    core::{reflect::prelude::*, visitor::prelude::*},
    scroll_bar::ScrollBarBuilder,
    widget::WidgetBuilder,
    BuildContext, UiNode, UserInterface,
};
use fyrox::plugin::error::GameResult;
use fyrox::plugin::{Plugin, PluginContext};

// ANCHOR: create_scroll_bar
fn create_scroll_bar(ctx: &mut BuildContext) -> Handle<ScrollBar> {
    ScrollBarBuilder::new(WidgetBuilder::new())
        .with_min(0.0)
        .with_max(200.0)
        .with_value(123.0)
        .build(ctx)
}
// ANCHOR_END: create_scroll_bar

// ANCHOR: usage_example
#[derive(Visit, Clone, Reflect, Debug, Default)]
struct Game {
    scroll_bar: Handle<UiNode>,
}

impl Plugin for Game {
    fn on_ui_message(
        &mut self,
        context: &mut PluginContext,
        message: &UiMessage,
        ui_handle: Handle<UserInterface>,
    ) -> GameResult {
        if let Some(ScrollBarMessage::Value(value)) = message.data_from(self.scroll_bar) {
            //
            // Insert handler code here.
            //
        }
        Ok(())
    }
}
// ANCHOR_END: usage_example
