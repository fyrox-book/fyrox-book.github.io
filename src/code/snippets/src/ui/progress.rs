use fyrox::gui::{
    core::pool::Handle,
    message::MessageDirection,
    progress_bar::{ProgressBarBuilder, ProgressBarMessage},
    widget::WidgetBuilder,
    BuildContext, UiNode, UserInterface,
};

// ANCHOR: create_progress_bar
fn create_progress_bar(ctx: &mut BuildContext) -> Handle<UiNode> {
    ProgressBarBuilder::new(WidgetBuilder::new())
        // Keep mind, that the progress is "normalized", which means that it is defined on
        // [0..1] range, where 0 - no progress at all, 1 - maximum progress.
        .with_progress(0.25)
        .build(ctx)
}
// ANCHOR_END: create_progress_bar

// ANCHOR: change_progress
fn change_progress(progress_bar: Handle<UiNode>, ui: &UserInterface) {
    ui.send_message(ProgressBarMessage::progress(
        progress_bar,
        MessageDirection::ToWidget,
        0.33,
    ));
}
// ANCHOR_END: change_progress
