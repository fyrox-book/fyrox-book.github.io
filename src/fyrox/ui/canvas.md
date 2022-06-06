# Canvas

Canvas is a panel widget that allows you to explicitly set coordinates for children widgets. It is useful when you 
need to manually control position of children widgets. Root UI node is canvas, so any widgets that are not attached
to any other widgets can have explicit position.

## How to create

Use `CanvasBuilder` to create Canvas instance:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     gui::{canvas::CanvasBuilder, widget::WidgetBuilder, BuildContext, UiNode},
# };
# 
fn create_canvas(ctx: &mut BuildContext) -> Handle<UiNode> {
    CanvasBuilder::new(WidgetBuilder::new()).build(ctx)
}
```

Canvas does not have any specific options, so its creation is probably simplest of all widgets.

## How to position children nodes

Use `.with_desired_position` on children widgets to set specific position:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{algebra::Vector2, pool::Handle},
#     gui::{
#         button::ButtonBuilder, canvas::CanvasBuilder, text::TextBuilder, widget::WidgetBuilder,
#         BuildContext, UiNode,
#     },
# };
# 
fn create_canvas_with_children_widgets(ctx: &mut BuildContext) -> Handle<UiNode> {
    CanvasBuilder::new(
        WidgetBuilder::new()
            .with_child(
                TextBuilder::new(
                    WidgetBuilder::new().with_desired_position(Vector2::new(100.0, 200.0)),
                )
                .with_text("Simple Text at (100.0, 200.0)")
                .build(ctx),
            )
            .with_child(
                ButtonBuilder::new(
                    WidgetBuilder::new().with_desired_position(Vector2::new(200.0, 100.0)),
                )
                .with_text("Simple Button at (200.0, 100.0)")
                .build(ctx),
            ),
    )
    .build(ctx)
}
```

The code snippet will create a canvas with a text widget located at (100.0, 200.0) relative to top-left corner of the
canvas and a button located at (200.0, 100.0).

## Tips

Canvas provides infinite bounds for children widgets, this means that children nodes **will not** be stretched, instead
they'll shrink to fit their content. For example, a button with a text will take slightly bigger rectangle than the 
text bounds.