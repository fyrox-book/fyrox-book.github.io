use fyrox::gui::{
    button::ButtonBuilder,
    core::pool::Handle,
    grid::{Column, GridBuilder, Row},
    screen::ScreenBuilder,
    stack_panel::StackPanelBuilder,
    widget::WidgetBuilder,
    BuildContext, UiNode,
};

// ANCHOR: create_always_centered_game_menu
fn create_always_centered_game_menu(ctx: &mut BuildContext) -> Handle<UiNode> {
    // Screen widget will provide current screen size to its Grid widget as a layout constraint,
    // thus making it fit to the current screen bounds.
    ScreenBuilder::new(
        WidgetBuilder::new().with_child(
            GridBuilder::new(
                WidgetBuilder::new()
                    .with_width(300.0)
                    .with_height(400.0)
                    .with_child(
                        // Buttons will be stacked one on top of another.
                        StackPanelBuilder::new(
                            WidgetBuilder::new()
                                .on_row(1)
                                .on_column(1)
                                .with_child(
                                    ButtonBuilder::new(WidgetBuilder::new())
                                        .with_text("New Game")
                                        .build(ctx),
                                )
                                .with_child(
                                    ButtonBuilder::new(WidgetBuilder::new())
                                        .with_text("Exit")
                                        .build(ctx),
                                ),
                        )
                        .build(ctx),
                    ),
            )
            // Split the grid into 3 rows and 3 columns. The center cell contain the stack panel
            // instance, that basically stacks main menu buttons one on top of another. The center
            // cell will also be always centered in screen bounds.
            .add_row(Row::stretch())
            .add_row(Row::auto())
            .add_row(Row::stretch())
            .add_column(Column::stretch())
            .add_column(Column::auto())
            .add_column(Column::stretch())
            .build(ctx),
        ),
    )
    .build(ctx)
}
// ANCHOR_END: create_always_centered_game_menu
