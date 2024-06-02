use fyrox::gui::{
    grid::{GridBuilder, GridDimension},
    text::TextBuilder,
    widget::WidgetBuilder,
    BuildContext, UiNode,
};

// ANCHOR: create_text_grid
fn create_text_grid(ctx: &mut BuildContext) -> fyrox::core::pool::Handle<UiNode> {
    GridBuilder::new(
        WidgetBuilder::new()
            .with_child(
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("top left ")
                    .build(ctx),
            )
            .with_child(
                TextBuilder::new(WidgetBuilder::new().on_column(1))
                    .with_text(" top right")
                    .build(ctx),
            )
            .with_child(
                TextBuilder::new(WidgetBuilder::new().on_row(1))
                    .with_text("bottom left ")
                    .build(ctx),
            )
            .with_child(
                TextBuilder::new(WidgetBuilder::new().on_row(1).on_column(1))
                    .with_text(" bottom right")
                    .build(ctx),
            ),
    )
    .add_row(GridDimension::auto())
    .add_row(GridDimension::auto())
    .add_column(GridDimension::auto())
    .add_column(GridDimension::auto())
    .build(ctx)
}
// ANCHOR_END: create_text_grid
