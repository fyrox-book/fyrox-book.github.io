use fyrox::asset::manager::ResourceManager;
use fyrox::core::algebra::Vector2;
use fyrox::core::color::Color;
use fyrox::gui::border::{Border, BorderBuilder};
use fyrox::gui::brush::Brush;
use fyrox::gui::font::Font;
use fyrox::gui::formatted_text::WrapMode;
use fyrox::gui::text::{Text, TextMessage};
use fyrox::gui::{HorizontalAlignment, VerticalAlignment};
use fyrox::{
    core::pool::Handle,
    gui::{text::TextBuilder, widget::WidgetBuilder, UiNode, UserInterface},
};

// ANCHOR: create_text
fn create_text(ui: &mut UserInterface, text: &str) -> Handle<Text> {
    TextBuilder::new(WidgetBuilder::new())
        .with_text(text)
        .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_text

// ANCHOR: create_centered_text
fn create_centered_text(ui: &mut UserInterface, text: &str) -> Handle<Text> {
    TextBuilder::new(WidgetBuilder::new())
        .with_horizontal_text_alignment(HorizontalAlignment::Center)
        .with_vertical_text_alignment(VerticalAlignment::Center)
        .with_text(text)
        .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_centered_text

// ANCHOR: create_text_with_word_wrap
fn create_text_with_word_wrap(ui: &mut UserInterface, text: &str) -> Handle<Text> {
    TextBuilder::new(WidgetBuilder::new())
        .with_wrap(WrapMode::Word)
        .with_text(text)
        .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_text_with_word_wrap

// ANCHOR: create_text_with_background
fn create_text_with_background(ui: &mut UserInterface, text: &str) -> Handle<Border> {
    let text_widget =
        TextBuilder::new(WidgetBuilder::new().with_foreground(Brush::Solid(Color::RED).into()))
            .with_text(text)
            .build(&mut ui.build_ctx());
    BorderBuilder::new(
        WidgetBuilder::new()
            .with_child(text_widget) // <-- Text is now a child of the border
            .with_background(Brush::Solid(Color::opaque(50, 50, 50)).into()),
    )
    .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_text_with_background

// ANCHOR: create_colored_text
fn create_colored_text(ui: &mut UserInterface, text: &str) -> Handle<Text> {
    //               vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
    TextBuilder::new(WidgetBuilder::new().with_foreground(Brush::Solid(Color::RED).into()))
        .with_text(text)
        .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_colored_text

// ANCHOR: create_text_with_font
fn create_text_with_font(
    ui: &mut UserInterface,
    text: &str,
    resource_manager: &ResourceManager,
) -> Handle<Text> {
    TextBuilder::new(WidgetBuilder::new())
        .with_font(resource_manager.request::<Font>("path/to/your/font.ttf"))
        .with_text(text)
        // You can set any size as well.
        .with_font_size(24.0.into())
        .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_text_with_font

// ANCHOR: create_red_text_with_black_shadows
fn create_red_text_with_black_shadows(ui: &mut UserInterface, text: &str) -> Handle<Text> {
    TextBuilder::new(WidgetBuilder::new().with_foreground(Brush::Solid(Color::RED).into()))
        .with_text(text)
        // Enable shadows.
        .with_shadow(true)
        // Black shadows.
        .with_shadow_brush(Brush::Solid(Color::BLACK))
        // 1px thick.
        .with_shadow_dilation(1.0)
        // Offset the shadow slightly to the right-bottom.
        .with_shadow_offset(Vector2::new(1.0, 1.0))
        .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_red_text_with_black_shadows

// ANCHOR: request_change_text
fn request_change_text(ui: &UserInterface, text_widget_handle: Handle<UiNode>, text: &str) {
    ui.send(text_widget_handle, TextMessage::Text(text.to_owned()))
}
// ANCHOR_END: request_change_text
