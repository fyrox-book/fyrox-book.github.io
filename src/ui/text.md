# Text

Text is a simple widget that allows you to print text on screen. It has various options like word wrapping, text
alignment, and so on.

## How to create

An instance of the Text widget could be created like so:

```rust,no_run
{{#include ../code/snippets/src/ui/text.rs:create_text}}
```

## Text alignment and word wrapping

There are various text alignment options for both vertical and horizontal axes. Typical alignment values are:
`Left`, `Center`, `Right` for horizontal axis, and `Top`, `Center`, `Bottom` for vertical axis. An instance of 
centered text could be created like so:

```rust,no_run
{{#include ../code/snippets/src/ui/text.rs:create_centered_text}}
```

Long text is usually needs to wrap on available bounds, there are three possible options for word wrapping:
`NoWrap`, `Letter`, `Word`. An instance of text with word-based wrapping could be created like so:

```rust,no_run
{{#include ../code/snippets/src/ui/text.rs:create_text_with_word_wrap}}
```

## Background

If you need to have a text with some background, you should use [Border](./border.md) widget as a parent widget of your 
text. **Caveat:** `Widget::background` is ignored for `Text` widget!

```rust,no_run
{{#include ../code/snippets/src/ui/text.rs:create_text_with_background}}
```

Keep in mind that now the text widget is a child widget of the border, so if you need to position the text, you should
position the border, not the text.

## Fonts and colors

To set a color of the text just use `.with_foreground(..)` of the `WidgetBuilder` while building the text instance:

```rust,no_run
{{#include ../code/snippets/src/ui/text.rs:create_colored_text}}
```

By default, text is created with default font, however it is possible to set any custom font:

```rust,no_run
{{#include ../code/snippets/src/ui/text.rs:create_text_with_font}}
```

Please refer to [Font](font.md) chapter to learn more about fonts.

## Shadows

Text widget supports shadows effect to add contrast to your text, which could be useful to make text readable independent
on the background colors. This effect could be used for subtitles. Shadows are pretty easy to add, all you need to do
is to enable them, setup desired thickness, offset and brush (solid color or gradient).

```rust,no_run
{{#include ../code/snippets/src/ui/text.rs:create_red_text_with_black_shadows}}
```

## Messages

Text widget can accept the following list of messages at runtime (respective constructors are name with small letter - 
`TextMessage::Text -> TextMessage::text(widget_handle, direction, text)`):

- `TextMessage::Text` - sets new text for a `Text` widget.
- `TextMessage::Wrap` - sets new [wrapping mode](text.md#text-alignment-and-word-wrapping). 
- `TextMessage::Font` - sets new [font](text.md#fonts-and-colors) 
- `TextMessage::VerticalAlignment` and `TextMessage::HorizontalAlignment` sets 
[vertical and horizontal](text.md#text-alignment-and-word-wrapping) text alignment respectively.
- `TextMessage::Shadow` - enables or disables [shadow casting](text.md#shadows)
- `TextMessage::ShadowDilation` - sets "thickness" of the shadows under the tex.
- `TextMessage::ShadowBrush` - sets shadow brush (allows you to change color and even make shadow with color gradients).
- `TextMessage::ShadowOffset` - sets offset of the shadows.

An example of changing text at runtime could be something like this:

```rust,no_run
{{#include ../code/snippets/src/ui/text.rs:request_change_text}}
```

Please keep in mind, that like any other situation when you "changing" something via messages, you should remember
that the change is **not** immediate. The change will be applied on `ui.poll_message(..)` call somewhere in your
code (or will be done automatically if you're using scripts or Framework (obsolete)).