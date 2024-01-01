# Scroll panel 

Scroll panel widget does the same as [Scroll Viewer](scroll_viewer.md) widget, but it does not have any additional
widgets and does not have any graphics. It is a panel widget that provides basic scrolling functionality and 
[Scroll Viewer](scroll_viewer.md) is built on top of it. Strictly speaking, scroll panel widget is used to arrange its 
children widgets, so they can be offset by a certain amount of units from top-left corner. It is used to provide basic 
scrolling functionality.

## Examples

```rust
# extern crate fyrox;
# use fyrox_ui::{
#     button::ButtonBuilder,
#     core::{algebra::Vector2, pool::Handle},
#     grid::{Column, GridBuilder, Row},
#     scroll_panel::ScrollPanelBuilder,
#     widget::WidgetBuilder,
#     BuildContext, UiNode,
# };
#
fn create_scroll_panel(ctx: &mut BuildContext) -> Handle<UiNode> {
    ScrollPanelBuilder::new(
        WidgetBuilder::new().with_child(
            GridBuilder::new(
                WidgetBuilder::new()
                    .with_child(
                        ButtonBuilder::new(WidgetBuilder::new())
                            .with_text("Some Button")
                            .build(ctx),
                    )
                    .with_child(
                        ButtonBuilder::new(WidgetBuilder::new())
                            .with_text("Some Other Button")
                            .build(ctx),
                    ),
            )
            .add_row(Row::auto())
            .add_row(Row::auto())
            .add_column(Column::stretch())
            .build(ctx),
        ),
    )
    .with_scroll_value(Vector2::new(100.0, 200.0))
    .with_vertical_scroll_allowed(true)
    .with_horizontal_scroll_allowed(true)
    .build(ctx)
}
```

## Scrolling

Scrolling value for both axes can be set via `ScrollPanelMessage::VerticalScroll` and `ScrollPanelMessage::HorizontalScroll`:

```rust
use fyrox_ui::{
    core::pool::Handle, message::MessageDirection, scroll_panel::ScrollPanelMessage, UiNode,
    UserInterface,
};
fn set_scrolling_value(
    scroll_panel: Handle<UiNode>,
    horizontal: f32,
    vertical: f32,
    ui: &UserInterface,
) {
    ui.send_message(ScrollPanelMessage::horizontal_scroll(
        scroll_panel,
        MessageDirection::ToWidget,
        horizontal,
    ));
    ui.send_message(ScrollPanelMessage::vertical_scroll(
        scroll_panel,
        MessageDirection::ToWidget,
        vertical,
    ));
}
```

## Bringing child into view

Calculates the scroll values to bring a desired child into view, it can be used for automatic navigation:

```rust
# use fyrox_ui::{
#     core::pool::Handle, message::MessageDirection, scroll_panel::ScrollPanelMessage, UiNode,
#     UserInterface,
# };
fn bring_child_into_view(
    scroll_panel: Handle<UiNode>,
    child: Handle<UiNode>,
    ui: &UserInterface,
) {
    ui.send_message(ScrollPanelMessage::bring_into_view(
        scroll_panel,
        MessageDirection::ToWidget,
        child,
    ))
}
```