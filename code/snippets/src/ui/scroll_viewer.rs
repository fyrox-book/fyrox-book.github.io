use fyrox::gui::message::MessageDirection;
use fyrox::gui::scroll_viewer::ScrollViewerMessage;
use fyrox::gui::{
    button::ButtonBuilder, core::pool::Handle, scroll_viewer::ScrollViewerBuilder,
    stack_panel::StackPanelBuilder, text::TextBuilder, widget::WidgetBuilder, BuildContext, UiNode,
    UserInterface,
};

// ANCHOR: create_scroll_viewer
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
// ANCHOR_END: create_scroll_viewer

// ANCHOR: create_scroll_viewer_with_speed
fn create_scroll_viewer_with_speed(ctx: &mut BuildContext) -> Handle<UiNode> {
    ScrollViewerBuilder::new(WidgetBuilder::new())
        // Set vertical scrolling speed twice as fast as default scrolling speed.
        .with_v_scroll_speed(60.0)
        // Set horizontal scrolling speed slightly lower than the default value (30.0).
        .with_h_scroll_speed(20.0)
        .build(ctx)
}
// ANCHOR_END: create_scroll_viewer_with_speed

// ANCHOR: bring_child_into_view
fn bring_child_into_view(scroll_viewer: Handle<UiNode>, child: Handle<UiNode>, ui: &UserInterface) {
    ui.send_message(ScrollViewerMessage::bring_into_view(
        scroll_viewer,
        MessageDirection::ToWidget,
        child,
    ))
}
// ANCHOR_END: bring_child_into_view
