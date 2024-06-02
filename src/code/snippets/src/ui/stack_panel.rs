use fyrox::gui::{
    stack_panel::StackPanelBuilder, text::TextBuilder, widget::WidgetBuilder, BuildContext,
    Orientation, UiNode,
};

// ANCHOR: create_stack_panel
fn create_stack_panel(ctx: &mut BuildContext) -> fyrox::core::pool::Handle<UiNode> {
    StackPanelBuilder::new(
        WidgetBuilder::new()
            .with_child(
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Top")
                    .build(ctx),
            )
            .with_child(
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Middle")
                    .build(ctx),
            )
            .with_child(
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Bottom")
                    .build(ctx),
            ),
    )
    .build(ctx)
}
// ANCHOR_END: create_stack_panel

// ANCHOR: create_horizontal_stack_panel
fn create_horizontal_stack_panel(ctx: &mut BuildContext) -> fyrox::core::pool::Handle<UiNode> {
    StackPanelBuilder::new(
        WidgetBuilder::new()
            .with_child(
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Left")
                    .build(ctx),
            )
            .with_child(
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Middle")
                    .build(ctx),
            )
            .with_child(
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Right")
                    .build(ctx),
            ),
    )
    .with_orientation(Orientation::Horizontal)
    .build(ctx)
}
// ANCHOR_END: create_horizontal_stack_panel
