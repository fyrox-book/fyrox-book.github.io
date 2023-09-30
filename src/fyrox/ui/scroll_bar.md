# Scroll bar 

![scroll bar](scroll_bar.gif)

Scroll bar is used to represent a value on a finite range. It has a thumb that shows the current value on
on the bar. Usually it is used in pair with `ScrollPanel` to create something like
`ScrollViewer` widget. However, it could also be used to create sliders to show some
value that lies within some range.

## Example

A simple example of how to create a new `ScrollBar` could be something like this:

```rust
# extern crate fyrox;
# use fyrox::gui::{
#     core::pool::Handle, scroll_bar::ScrollBarBuilder, widget::WidgetBuilder, BuildContext,
#     UiNode,
# };
fn create_scroll_bar(ctx: &mut BuildContext) -> Handle<UiNode> {
    ScrollBarBuilder::new(WidgetBuilder::new())
        .with_min(0.0)
        .with_max(200.0)
        .with_value(123.0)
        .build(ctx)
}
```

It creates a horizontal scroll bar with `123.0` value and a range of `[0.0..200.0]`. To fetch the new value
of the scroll bar, use `ScrollBarMessage::Value` message:

```rust
# extern crate fyrox;
# use fyrox::gui::{
#     core::pool::Handle,
#     message::{MessageDirection, UiMessage},
#     scroll_bar::ScrollBarMessage,
#     UiNode,
# };
# fn foo(scroll_bar: Handle<UiNode>, message: &mut UiMessage) {
if message.destination() == scroll_bar
    && message.direction() == MessageDirection::FromWidget
{
    if let Some(ScrollBarMessage::Value(value)) = message.data() {
        println!("{}", value);
    }
}
# }
```

Please note, that you need to explicitly filter messages by `MessageDirection::FromWidget`, because it's the only
direction that is used as an "indicator" that the value was accepted by the scroll bar.

## Orientation

Scroll bar could be either horizontal (default) or vertical. You can select the orientation when building
a scroll bar using `ScrollBarBuilder::with_orientation` method and provide a desired value from `Orientation`
enum there.

## Show values

By default, scroll bar does not show its actual value, you can turn it on using `ScrollBarBuilder::show_value`
method with `true` as the first argument. To change rounding of the value, use `ScrollBarBuilder::with_value_precision`
and provide the desired amount of decimal places there.

## Step

Scroll bar provides arrows to change the current value using a fixed step value. You can change it using
`ScrollBarBuilder::with_step` method.