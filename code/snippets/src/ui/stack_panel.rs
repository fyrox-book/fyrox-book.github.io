use fyrox::core::pool::Handle;
use fyrox::gui::stack_panel::StackPanel;
use fyrox::gui::{
    stack_panel::StackPanelBuilder, text::TextBuilder, widget::WidgetBuilder, BuildContext,
    Orientation,
};

// ANCHOR: create_stack_panel
fn create_stack_panel(ctx: &mut BuildContext) -> Handle<StackPanel> {
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
fn create_horizontal_stack_panel(ctx: &mut BuildContext) -> Handle<StackPanel> {
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
