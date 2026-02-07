use fyrox::gui::scroll_panel::{ScrollPanel, ScrollPanelMessage};
use fyrox::gui::{
    button::ButtonBuilder,
    core::{algebra::Vector2, pool::Handle},
    grid::{Column, GridBuilder, Row},
    scroll_panel::ScrollPanelBuilder,
    widget::WidgetBuilder,
    BuildContext, UiNode, UserInterface,
};

// ANCHOR: create_scroll_panel
fn create_scroll_panel(ctx: &mut BuildContext) -> Handle<ScrollPanel> {
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
// ANCHOR_END: create_scroll_panel

// ANCHOR: set_scrolling_value
fn set_scrolling_value(
    scroll_panel: Handle<UiNode>,
    horizontal: f32,
    vertical: f32,
    ui: &UserInterface,
) {
    ui.send(
        scroll_panel,
        ScrollPanelMessage::HorizontalScroll(horizontal),
    );
    ui.send(scroll_panel, ScrollPanelMessage::VerticalScroll(vertical));
}
// ANCHOR_END: set_scrolling_value

// ANCHOR: bring_child_into_view
fn bring_child_into_view(scroll_panel: Handle<UiNode>, child: Handle<UiNode>, ui: &UserInterface) {
    ui.send(scroll_panel, ScrollPanelMessage::BringIntoView(child))
}
// ANCHOR_END: bring_child_into_view
