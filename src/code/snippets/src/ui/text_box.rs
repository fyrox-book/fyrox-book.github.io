use fyrox::asset::manager::ResourceManager;
use fyrox::core::color::Color;
use fyrox::core::parking_lot::Mutex;
use fyrox::gui::brush::Brush;
use fyrox::gui::font::Font;
use fyrox::gui::formatted_text::WrapMode;
use fyrox::gui::message::MessageDirection;
use fyrox::gui::text::TextMessage;
use fyrox::gui::{HorizontalAlignment, VerticalAlignment};
use fyrox::{
    core::pool::Handle,
    gui::{text_box::TextBoxBuilder, widget::WidgetBuilder, UiNode, UserInterface},
};
use std::sync::Arc;

// ANCHOR: create_text_box
fn create_text_box(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
    TextBoxBuilder::new(WidgetBuilder::new())
        .with_text(text)
        .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_text_box

// ANCHOR: create_centered_text
fn create_centered_text(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
    TextBoxBuilder::new(WidgetBuilder::new())
        .with_horizontal_text_alignment(HorizontalAlignment::Center)
        .with_vertical_text_alignment(VerticalAlignment::Center)
        .with_text(text)
        .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_centered_text

// ANCHOR: create_text_with_word_wrap
fn create_text_with_word_wrap(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
    TextBoxBuilder::new(WidgetBuilder::new())
        .with_wrap(WrapMode::Word)
        .with_text(text)
        .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_text_with_word_wrap

// ANCHOR: create_colored_text_box
fn create_colored_text_box(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
    //                  vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
    TextBoxBuilder::new(WidgetBuilder::new().with_foreground(Brush::Solid(Color::RED)))
        .with_text(text)
        .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_colored_text_box

// ANCHOR: create_text_box_with_font
fn create_text_with_font(
    ui: &mut UserInterface,
    text: &str,
    resource_manager: &ResourceManager,
) -> Handle<UiNode> {
    TextBoxBuilder::new(WidgetBuilder::new())
        .with_font(resource_manager.request::<Font>("path/to/your/font.ttf"))
        .with_text(text)
        // You can set any size as well.
        .with_font_size(24.0)
        .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_text_box_with_font

// ANCHOR: request_change_text
fn request_change_text(ui: &UserInterface, text_box_widget_handle: Handle<UiNode>, text: &str) {
    ui.send_message(TextMessage::text(
        text_box_widget_handle,
        MessageDirection::ToWidget,
        text.to_owned(),
    ))
}
// ANCHOR_END: request_change_text

// ANCHOR: create_text_box_with_filter
fn create_text_box_with_filter(ui: &mut UserInterface) -> Handle<UiNode> {
    TextBoxBuilder::new(WidgetBuilder::new())
        // Specify a filter that will pass only digits.
        .with_filter(Arc::new(Mutex::new(|c: char| c.is_ascii_digit())))
        .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_text_box_with_filter
