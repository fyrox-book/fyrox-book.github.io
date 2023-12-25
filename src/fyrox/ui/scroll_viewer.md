# Scroll viewer

![scroll viewer](scroll_bar.gif)

Scroll viewer is a scrollable region with two scroll bars for each axis. It is used to wrap a content of unknown
size to ensure that all of it will be accessible in its parent widget bounds. For example, it could be used in a
[Window](window.md) widget to allow a content of the window to be accessible, even if the window is smaller than 
the content.

## Example

A scroll viewer widget could be created using `ScrollViewerBuilder`:

```rust
# extern crate fyrox;
# use fyrox::gui::{
#     button::ButtonBuilder, core::pool::Handle, scroll_viewer::ScrollViewerBuilder,
#     stack_panel::StackPanelBuilder, text::TextBuilder, widget::WidgetBuilder, BuildContext,
#     UiNode,
# };
#
fn create_scroll_viewer(ctx: &mut BuildContext) -> Handle<UiNode> {
    ScrollViewerBuilder::new(WidgetBuilder::new())
        .with_content(
            StackPanelBuilder::new(
                WidgetBuilder::new()
                    .with_child(
                        ButtonBuilder::new(WidgetBuilder::new())
                            .with_text("Click Me!")
                            .build(ctx),
                    )
                    .with_child(
                        TextBuilder::new(WidgetBuilder::new())
                            .with_text("Some\nlong\ntext")
                            .build(ctx),
                    ),
            )
            .build(ctx),
        )
        .build(ctx)
}
```

Keep in mind, that you can change the content of a scroll viewer at runtime using `ScrollViewerMessage::Content` message.

## Scrolling Speed and Controls

Scroll viewer can have an arbitrary scrolling speed for each axis. Scrolling is performed via mouse wheel and by default it
scrolls vertical axis, which can be changed by holding `Shift` key. Scrolling speed can be set during the build phase:

```rust
# extern crate fyrox;
# use fyrox::gui::{
#     core::pool::Handle, scroll_viewer::ScrollViewerBuilder, widget::WidgetBuilder,
#     BuildContext, UiNode,
# };
#
fn create_scroll_viewer(ctx: &mut BuildContext) -> Handle<UiNode> {
    ScrollViewerBuilder::new(WidgetBuilder::new())
        // Set vertical scrolling speed twice as fast as default scrolling speed.
        .with_v_scroll_speed(60.0)
        // Set horizontal scrolling speed slightly lower than the default value (30.0).
        .with_h_scroll_speed(20.0)
        .build(ctx)
}
```

Also, it could be set using `ScrollViewerMessage::HScrollSpeed` or `ScrollViewerMessage::VScrollSpeed` messages.

## Bringing a child into view

Calculates the scroll values to bring a desired child into view, it can be used for automatic navigation:

```rust
# extern crate fyrox;
# use fyrox::gui::{
#     core::pool::Handle, message::MessageDirection, scroll_viewer::ScrollViewerMessage, UiNode,
#     UserInterface,
# };
fn bring_child_into_view(
    scroll_viewer: Handle<UiNode>,
    child: Handle<UiNode>,
    ui: &UserInterface,
) {
    ui.send_message(ScrollViewerMessage::bring_into_view(
        scroll_viewer,
        MessageDirection::ToWidget,
        child,
    ))
}
```