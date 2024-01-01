use fyrox::{
    core::{algebra::Vector2, pool::Handle},
    gui::{
        button::ButtonBuilder, canvas::CanvasBuilder, text::TextBuilder, widget::WidgetBuilder,
        BuildContext, UiNode,
    },
};

// ANCHOR: create_canvas
fn create_canvas(ctx: &mut BuildContext) -> Handle<UiNode> {
    CanvasBuilder::new(WidgetBuilder::new()).build(ctx)
}
// ANCHOR_END: create_canvas

// ANCHOR: create_canvas_with_children_widgets
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
// ANCHOR_END: create_canvas_with_children_widgets
