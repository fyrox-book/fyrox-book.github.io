# Text

Text is a simple widget that allows you to print text on screen. It has various options like word wrapping, text
alignment, and so on.

## How to create

An instance of the Text widget could be created like so:

```rust
# extern crate Fyrox;
# use fyrox::{
#     core::pool::Handle,
#     gui::{text::TextBuilder, widget::WidgetBuilder, UiNode, UserInterface},
# };
fn create_text(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
    TextBuilder::new(WidgetBuilder::new())
        .with_text(text)
        .build(&mut ui.build_ctx())
}
```

## Text alignment and word wrapping

There are various text alignment options for both vertical and horizontal axes. Typical alignment values are:
`Left`, `Center`, `Right` for horizontal axis, and `Top`, `Center`, `Bottom` for vertical axis. An instance of 
centered text could be created like so:

```rust
# extern crate Fyrox;
# use fyrox::{
#     core::pool::Handle,
#     gui::{
#         text::TextBuilder, widget::WidgetBuilder, HorizontalAlignment, UiNode, UserInterface,
#         VerticalAlignment,
#     },
# };
fn create_centered_text(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
    TextBuilder::new(WidgetBuilder::new())
        .with_horizontal_text_alignment(HorizontalAlignment::Center)
        .with_vertical_text_alignment(VerticalAlignment::Center)
    .with_text(text)
    .build(&mut ui.build_ctx())
}
```

Long text is usually needs to wrap on available bounds, there are three possible options for word wrapping:
`NoWrap`, `Letter`, `Word`. An instance of text with word-based wrapping could be created like so:

```rust
# extern crate Fyrox;
# use fyrox::{
#     core::pool::Handle,
#     gui::{
#         formatted_text::WrapMode, text::TextBuilder, widget::WidgetBuilder, UiNode,
#         UserInterface,
#     },
# };
fn create_text_with_word_wrap(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
    TextBuilder::new(WidgetBuilder::new())
        .with_wrap(WrapMode::Word)
        .with_text(text)
        .build(&mut ui.build_ctx())
}
```

## Fonts and colors

To set a color of the text just use `.with_foreground(..)` of the `WidgetBuilder` while building the text instance:

```rust
# extern crate Fyrox;
# use fyrox::{
#     core::{color::Color, pool::Handle},
#     gui::{brush::Brush, text::TextBuilder, widget::WidgetBuilder, UiNode, UserInterface},
# };
fn create_text(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
    //               vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
    TextBuilder::new(WidgetBuilder::new().with_foreground(Brush::Solid(Color::RED)))
        .with_text(text)
        .build(&mut ui.build_ctx())
}
```

By default, text is created with default font, however it is possible to set any custom font:

```rust
# extern crate Fyrox;
# use fyrox::{
#     core::{futures::executor::block_on, pool::Handle},
#     gui::{
#         text::TextBuilder,
#         ttf::{Font, SharedFont},
#         widget::WidgetBuilder,
#         UiNode, UserInterface,
#     },
# };

fn load_font() -> SharedFont {
    // Choose desired character set, default is Basic Latin + Latin Supplement.
    // Character set is a set of ranges with Unicode code points.
    let character_set = Font::default_char_set();

    // Normally `block_on` should be avoided.
    let font = block_on(Font::from_file(
        "path/to/your/font.ttf",
        24.0,
        character_set,
    ))
    .unwrap();

    SharedFont::new(font)
}

fn create_text(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
    TextBuilder::new(WidgetBuilder::new())
        .with_font(load_font())
        .with_text(text)
        .build(&mut ui.build_ctx())
}
```

### Font size

There is no way to change font size without changing the entire font used by Text, it is known issue and there is
[tracking issue](https://github.com/FyroxEngine/Fyrox/issues/74) for that.

## Messages

There are few message types that Text widget can produce, most common are `Text`, `Wrap`, `Font`, `VerticalAlignment`,
and `HorizontalAlignment`. An example of changing text at runtime could be something like this:

```rust
# extern crate Fyrox;
# use fyrox::{
#     core::pool::Handle,
#     gui::{
#         message::{MessageDirection},
#         UiNode, UserInterface,
#         text::TextMessage
#     },
# };
fn request_change_text(ui: &UserInterface, text_widget_handle: Handle<UiNode>, text: &str) {
    ui.send_message(TextMessage::text(
        text_widget_handle,
        MessageDirection::ToWidget,
        text.to_owned(),
    ))
}
```

Please keep in mind, that like any other situation when you "changing" something via messages, you should remember
that the change is **not** immediate. The change will be applied on `ui.poll_message(..)` call somewhere in your
code (or will be done automatically if you're using Framework).