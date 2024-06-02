use fyrox::gui::check_box::CheckBoxMessage;
use fyrox::gui::message::UiMessage;
use fyrox::gui::text::TextBuilder;
use fyrox::plugin::{Plugin, PluginContext};
use fyrox::{
    core::pool::Handle,
    core::{reflect::prelude::*, visitor::prelude::*},
    gui::{check_box::CheckBoxBuilder, widget::WidgetBuilder, UiNode, UserInterface},
};

// ANCHOR: create_checkbox
fn create_checkbox(ui: &mut UserInterface) -> Handle<UiNode> {
    CheckBoxBuilder::new(WidgetBuilder::new())
        // A custom value can be set during initialization.
        .checked(Some(true))
        .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_checkbox

// ANCHOR: create_checkbox_with_text
fn create_checkbox_with_text(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();

    CheckBoxBuilder::new(WidgetBuilder::new())
        // A custom value can be set during initialization.
        .checked(Some(true))
        .with_content(
            TextBuilder::new(WidgetBuilder::new())
                .with_text("This is a checkbox")
                .build(ctx),
        )
        .build(ctx)
}
// ANCHOR_END: create_checkbox_with_text

// ANCHOR: usage_example
#[derive(Visit, Reflect, Debug, Default)]
struct Game {
    checkbox: Handle<UiNode>,
}

impl Plugin for Game {
    fn on_ui_message(&mut self, context: &mut PluginContext, message: &UiMessage) {
        if let Some(CheckBoxMessage::Check(value)) = message.data() {
            if message.destination() == self.checkbox {
                //
                // Insert your clicking handling code here.
                //
            }
        }
    }
}
// ANCHOR_END: usage_example
