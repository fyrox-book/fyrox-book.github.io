use fyrox::core::{reflect::prelude::*, visitor::prelude::*};
use fyrox::gui::check_box::CheckBoxBuilder;
use fyrox::gui::expander::ExpanderMessage;
use fyrox::gui::image::ImageBuilder;
use fyrox::gui::message::{MessageDirection, UiMessage};
use fyrox::gui::{
    button::ButtonBuilder, core::pool::Handle, expander::ExpanderBuilder,
    stack_panel::StackPanelBuilder, text::TextBuilder, widget::WidgetBuilder, BuildContext, UiNode,
    UserInterface,
};
use fyrox::plugin::error::GameResult;
use fyrox::plugin::{Plugin, PluginContext};

// ANCHOR: create_expander
fn create_expander(ctx: &mut BuildContext) -> Handle<UiNode> {
    ExpanderBuilder::new(WidgetBuilder::new())
        // Header is visible all the time.
        .with_header(
            TextBuilder::new(WidgetBuilder::new())
                .with_text("Foobar")
                .build(ctx),
        )
        // Define a content of collapsible area.
        .with_content(
            StackPanelBuilder::new(
                WidgetBuilder::new()
                    .with_child(
                        ButtonBuilder::new(WidgetBuilder::new())
                            .with_text("Button 1")
                            .build(ctx),
                    )
                    .with_child(
                        ButtonBuilder::new(WidgetBuilder::new())
                            .with_text("Button 2")
                            .build(ctx),
                    ),
            )
            .build(ctx),
        )
        .build(ctx)
}
// ANCHOR_END: create_expander

// ANCHOR: create_expander_with_image
fn create_expander_with_image(ctx: &mut BuildContext) -> Handle<UiNode> {
    ExpanderBuilder::new(WidgetBuilder::new())
        .with_checkbox(
            CheckBoxBuilder::new(WidgetBuilder::new())
                .with_check_mark(
                    ImageBuilder::new(WidgetBuilder::new().with_height(16.0).with_height(16.0))
                        .with_opt_texture(None) // Set this to required image.
                        .build(ctx),
                )
                .with_uncheck_mark(
                    ImageBuilder::new(WidgetBuilder::new().with_height(16.0).with_height(16.0))
                        .with_opt_texture(None) // Set this to required image.
                        .build(ctx),
                )
                .build(ctx),
        )
        // The rest is omitted.
        .build(ctx)
}
// ANCHOR_END: create_expander_with_image

// ANCHOR: message_handling
#[derive(Visit, Reflect, Debug, Default)]
struct Game {
    expander: Handle<UiNode>,
}

impl Plugin for Game {
    fn on_ui_message(
        &mut self,
        context: &mut PluginContext,
        message: &UiMessage,
        ui_handle: Handle<UserInterface>,
    ) -> GameResult {
        if let Some(ExpanderMessage::Expand(expanded)) = message.data() {
            if message.destination() == self.expander
                && message.direction() == MessageDirection::FromWidget
            {
                println!(
                    "{} expander has changed its state to {}!",
                    message.destination(),
                    expanded
                );
            }
        }
        Ok(())
    }
}
// ANCHOR_END: message_handling
