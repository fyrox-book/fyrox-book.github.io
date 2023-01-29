# Text Box

TextBox is a text widget that allows you to edit text and create specialized input fields. It has various options like 
word wrapping, text alignment, and so on.

## How to create

An instance of the TextBox widget could be created like so:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     gui::{text_box::TextBoxBuilder, widget::WidgetBuilder, UiNode, UserInterface},
# };
fn create_text_box(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
    TextBoxBuilder::new(WidgetBuilder::new())
        .with_text(text)
        .build(&mut ui.build_ctx())
}
```

## Text alignment and word wrapping

There are various text alignment options for both vertical and horizontal axes. Typical alignment values are:
`Left`, `Center`, `Right` for horizontal axis, and `Top`, `Center`, `Bottom` for vertical axis. An instance of
centered text could be created like so:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     gui::{
#         text_box::TextBoxBuilder, widget::WidgetBuilder, HorizontalAlignment, UiNode, UserInterface,
#         VerticalAlignment,
#     },
# };
fn create_centered_text(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
    TextBoxBuilder::new(WidgetBuilder::new())
        .with_horizontal_text_alignment(HorizontalAlignment::Center)
        .with_vertical_text_alignment(VerticalAlignment::Center)
    .with_text(text)
    .build(&mut ui.build_ctx())
}
```

Long text is usually needs to wrap on available bounds, there are three possible options for word wrapping:
`NoWrap`, `Letter`, `Word`. An instance of text with word-based wrapping could be created like so:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     gui::{
#         formatted_text::WrapMode, text_box::TextBoxBuilder, widget::WidgetBuilder, UiNode,
#         UserInterface,
#     },
# };
fn create_text_with_word_wrap(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
    TextBoxBuilder::new(WidgetBuilder::new())
        .with_wrap(WrapMode::Word)
        .with_text(text)
        .build(&mut ui.build_ctx())
}
```

## Fonts and colors

To set a color of the text just use `.with_foreground(..)` of the `WidgetBuilder` while building the text instance:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{color::Color, pool::Handle},
#     gui::{brush::Brush, text_box::TextBoxBuilder, widget::WidgetBuilder, UiNode, UserInterface},
# };
fn create_text(ui: &mut UserInterface, text: &str) -> Handle<UiNode> {
    //                  vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
    TextBoxBuilder::new(WidgetBuilder::new().with_foreground(Brush::Solid(Color::RED)))
        .with_text(text)
        .build(&mut ui.build_ctx())
}
```

By default, text is created with default font, however it is possible to set any custom font:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{futures::executor::block_on, pool::Handle},
#     gui::{
#         text_box::TextBoxBuilder,
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
    TextBoxBuilder::new(WidgetBuilder::new())
        .with_font(load_font())
        .with_text(text)
        .build(&mut ui.build_ctx())
}
```

Please refer to [Font](font.md) chapter to learn more about fonts.

### Font size

There is no way to change font size without changing the entire font used by Text, it is known issue and there is
[tracking issue](https://github.com/FyroxEngine/Fyrox/issues/74) for that. Check [Font](font.md) chapter to learn how
to create fonts.

## Messages

TextBox widget accepts the following list of messages:

- `TextBoxMessage::SelectionBrush` - change the brush that is used to highlight selection.
- `TextBoxMessage::CaretBrush` - changes the brush of the caret (small blinking vertical line).
- `TextBoxMessage::TextCommitMode` - changes the [text commit mode](text_box.md#text-commit-mode).
- `TextBoxMessage::Multiline` - makes the TextBox either multiline (`true`) or single line (`false`)
- `TextBoxMessage::Editable` - enables or disables editing of the text. 

**Important:** Please keep in mind, that TextBox widget also accepts `Text` [widget messages](text.md#messages). An 
example of changing text at runtime could be something like this:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     gui::{
#         message::{MessageDirection},
#         UiNode, UserInterface,
#         text::TextMessage
#     },
# };
fn request_change_text(ui: &UserInterface, text_box_widget_handle: Handle<UiNode>, text: &str) {
    ui.send_message(TextMessage::text(
        text_box_widget_handle,
        MessageDirection::ToWidget,
        text.to_owned(),
    ))
}
```

Please keep in mind, that like any other situation when you "changing" something via messages, you should remember
that the change is **not** immediate. The change will be applied on `ui.poll_message(..)` call somewhere in your
code (or will be done automatically if you're using scripts or Framework (obsolete)).

## Shortcuts

There are number of default shortcuts that can be used to speed up text editing:

- `Ctrl+A` - select all
- `Ctrl+C` - copy selected text
- `Ctrl+V` - paste text from clipboard
- `Ctrl+Home` - move caret to the beginning of the text
- `Ctrl+End` - move caret to the beginning of the text
- `Shift+Home` - select everything from current caret position until the beginning of current line
- `Shift+End` - select everything from current caret position until the end of current line
- `Arrows` - move caret accordingly
- `Delete` - deletes next character
- `Backspace` - deletes previous character
- `Enter` - new line (if multiline mode is set) or `commit` message

## Multiline Text Box

By default, text box will not add new line character to the text if you press `Enter` on keyboard. To enable this 
functionality use `.with_multiline(true)`

## Read-only Mode

You can enable or disable content editing by using read-only mode. Use `.with_readonly` at build stage.

## Mask Character

You can specify replacement character for every other characters, this is useful option for password fields. Use 
`.with_mask_char` at build stage. For example, you can set replacement character to asterisk `*` using 
`.with_mask_char(Some('*'))`

## Text Commit Mode

In many situations you don't need the text box to send `new text` message every new character, you either want this 
message if `Enter` key is pressed or TextBox has lost keyboard focus (or both). There is `with_text_commit_mode` on builder 
specifically for that purpose. Use one of the following modes:

- `TextCommitMode::Immediate` - text box will immediately send `Text` message after any change.
- `TextCommitMode::LostFocus` - text box will send `Text` message only when it loses focus.
- `TextCommitMode::LostFocusPlusEnter` - text box will send `Text` message when it loses focus or if Enter key was pressed. 
This is **default** behavior. In case of multiline text box hitting Enter key won't commit text!

## Filtering

It is possible specify custom input filter, it can be useful if you're creating special input fields like numerical or
phone number. A filter can be specified at build stage like so:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     gui::{text_box::TextBoxBuilder, widget::WidgetBuilder, UiNode, UserInterface},
# };
# use std::{cell::RefCell, rc::Rc};
fn create_text_box(ui: &mut UserInterface) -> Handle<UiNode> {
    TextBoxBuilder::new(WidgetBuilder::new())
        // Specify a filter that will pass only digits.
        .with_filter(Rc::new(RefCell::new(|c: char| c.is_ascii_digit())))
        .build(&mut ui.build_ctx())
}
```

## Style

You can change brush of caret by using `.with_caret_brush` and also selection brush by using `.with_selection_brush`,
it could be useful if you don't like default colors.