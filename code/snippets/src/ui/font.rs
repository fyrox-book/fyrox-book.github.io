use fyrox::asset::manager::ResourceManager;
use fyrox::asset::untyped::ResourceKind;
use fyrox::core::pool::Handle;
use fyrox::gui::message::MessageDirection;
use fyrox::gui::text::{TextBuilder, TextMessage};
use fyrox::gui::widget::WidgetBuilder;
use fyrox::gui::{BuildContext, UiNode, UserInterface};
use fyrox::{asset::Resource, gui::font::Font, plugin::PluginContext};
use std::fs::File;
use std::io::Read;

// ANCHOR: load_font_from_file
fn load_font_from_file(ctx: &PluginContext) -> Resource<Font> {
    ctx.resource_manager.request::<Font>("path/to/my/font")
}
// ANCHOR_END: load_font_from_file

// ANCHOR: load_font_from_memory
fn load_font_from_memory(data: Vec<u8>) -> Resource<Font> {
    Resource::new_ok(
        ResourceKind::Embedded,
        Font::from_memory(data, 1024).unwrap(),
    )
}
// ANCHOR_END: load_font_from_memory

// ANCHOR: load_font_from_file_memory
fn load_font_from_file_memory() -> Resource<Font> {
    let mut data = Vec::new();
    File::open("path/to/your/font.ttf")
        .unwrap()
        .read_to_end(&mut data)
        .unwrap();

    Resource::new_ok(
        ResourceKind::Embedded,
        Font::from_memory(data, 1024).unwrap(),
    )
}
// ANCHOR_END: load_font_from_file_memory

// ANCHOR: set_default_font
fn set_default_font(ui: &mut UserInterface, resource_manager: &ResourceManager) {
    ui.default_font = resource_manager.request::<Font>("path/to/my/font");
}
// ANCHOR_END: set_default_font

// ANCHOR: text
fn text(ctx: &mut BuildContext) -> Handle<UiNode> {
    TextBuilder::new(WidgetBuilder::new())
        .with_text("Some text")
        .with_font_size(30.0) // Sets the desired font size.
        .build(ctx)
}
// ANCHOR_END: text

// ANCHOR: set_font_size
fn set_font_size(text: Handle<UiNode>, ui: &UserInterface, new_font_size: f32) {
    ui.send_message(TextMessage::font_size(
        text,
        MessageDirection::ToWidget,
        new_font_size,
    ))
}
// ANCHOR_END: set_font_size
