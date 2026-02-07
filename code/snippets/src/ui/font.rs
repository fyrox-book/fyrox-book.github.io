use fyrox::asset::manager::ResourceManager;
use fyrox::asset::untyped::ResourceKind;
use fyrox::core::pool::Handle;
use fyrox::core::uuid;
use fyrox::gui::font::FontStyles;
use fyrox::gui::text::{Text, TextBuilder, TextMessage};
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
        uuid!("6e0816bd-4c90-4f0d-9827-d6ecdde8d39c"),
        ResourceKind::Embedded,
        Font::from_memory(data, 1024, Default::default(), Default::default()).unwrap(),
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
        uuid!("8e6e90be-e8af-4b7d-99e0-381868666ec0"),
        ResourceKind::Embedded,
        Font::from_memory(data, 1024, FontStyles::default(), Default::default()).unwrap(),
    )
}
// ANCHOR_END: load_font_from_file_memory

// ANCHOR: set_default_font
fn set_default_font(ui: &mut UserInterface, resource_manager: &ResourceManager) {
    ui.default_font = resource_manager.request::<Font>("path/to/my/font");
}
// ANCHOR_END: set_default_font

// ANCHOR: text
fn text(ctx: &mut BuildContext) -> Handle<Text> {
    TextBuilder::new(WidgetBuilder::new())
        .with_text("Some text")
        .with_font_size(30.0.into()) // Sets the desired font size.
        .build(ctx)
}
// ANCHOR_END: text

// ANCHOR: set_font_size
fn set_font_size(text: Handle<UiNode>, ui: &UserInterface, new_font_size: f32) {
    ui.send(text, TextMessage::FontSize(new_font_size.into()))
}
// ANCHOR_END: set_font_size
