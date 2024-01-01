# Font

Font is used to store graphical representation of unicode characters. The engine supports TTF and OTF fonts, 
you can load pretty much any font from the internet and use it as is.

## Create New Font

There are two ways to create font instance - either load font from file, or load it directly from memory.

### Loading Font From File

Since every font in the engine is a resource, you can load fonts using standard resource manager like so:

```rust ,no_run,edition2018
# extern crate fyrox;
# use fyrox::{asset::Resource, gui::font::Font, plugin::PluginContext};
# 
fn load_font_from_file(ctx: &PluginContext) -> Resource<Font> {
    ctx.resource_manager.request::<Font>("path/to/my/font")
}
```

### Creating Font From Memory

This option could be useful, if you already have your font loaded into memory. Loading font from data buffer is 
very simple:

```rust ,no_run
# extern crate fyrox;
# use fyrox::{asset::untyped::ResourceKind, asset::Resource, gui::font::Font};
# 
fn load_font_from_memory(data: Vec<u8>) -> Resource<Font> {
    Resource::new_ok(
        ResourceKind::Embedded,
        Font::from_memory(data, 1024).unwrap(),
    )
}
```

`data` input parameter could be a buffer that contains any valid TTF/OTF font. For example, you can load TTF file into
a data buffer and create font using the data buffer:

```rust ,no_run
# extern crate fyrox;
# use fyrox::{asset::untyped::ResourceKind, asset::Resource, gui::font::Font};
# use std::{fs::File, io::Read};
# 
fn load_font_from_memory() -> Resource<Font> {
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
```

## Default Font

User interface provides its own font of fixed size, it is enough to cover most of the use cases, but the default font
includes only ASCII characters, if you need extended character set, you can replace default font using the following
code snippet:

```rust ,no_run,edition2018
# extern crate fyrox;
# use fyrox::gui::{ttf::Font, UserInterface};
async fn set_default_font(ui: &mut UserInterface) {
    // Select character set.
    let character_set = Font::korean_char_set();
    // Load font.
    let new_font = Font::from_file("path/to/your/font.ttf", 20.0, character_set)
        .await
        .unwrap();
    // Set as default font.
    ui.default_font.set(new_font)
}
```

## How to Change Font Size

All you need to do is to set font size in your Text or TextBox widgets like so: 

```rust ,no_run
use fyrox::{
    core::pool::Handle,
    gui::{text::TextBuilder, widget::WidgetBuilder, BuildContext, UiNode},
};

fn text(ctx: &mut BuildContext) -> Handle<UiNode> {
    TextBuilder::new(WidgetBuilder::new())
        .with_text("Some text")
        .with_font_size(30.0) // Sets the desired font size.
        .build(ctx)
}
```

You can also change the font size at runtime using `TextMessage::FontSize` message like so:

```rust ,no_run
# use fyrox::{
#     core::pool::Handle,
#     gui::{message::MessageDirection, text::TextMessage, UiNode, UserInterface},
# };
# 
fn set_font_size(text: Handle<UiNode>, ui: &UserInterface, new_font_size: f32) {
    ui.send_message(TextMessage::font_size(
        text,
        MessageDirection::ToWidget,
        new_font_size,
    ))
}
```

## Important notes

Internally, font may use a number of texture atlases to pack all glyphs into a single texture. Font system creates
a new atlas for every glyph size. Each atlas could be split into multiple pages, which essentially just a texture
of a fixed size. Such paging is needed, because there's a hardware limit of maximum texture size on video cards and
instead of praying that everything fits into a single page, the engine automatically adds a new page if none of the
previous cannot fit a new character.

Keep in mind, that when you create a font, its atlases are empty. They're filled on demand when you try to use
a character that wasn't used previously. 