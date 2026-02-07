use fyrox::gui::progress_bar::ProgressBar;
use fyrox::gui::{
    core::pool::Handle,
    progress_bar::{ProgressBarBuilder, ProgressBarMessage},
    widget::WidgetBuilder,
    BuildContext, UiNode, UserInterface,
};

// ANCHOR: create_progress_bar
fn create_progress_bar(ctx: &mut BuildContext) -> Handle<ProgressBar> {
    ProgressBarBuilder::new(WidgetBuilder::new())
        // Keep mind, that the progress is "normalized", which means that it is defined on
        // [0..1] range, where 0 - no progress at all, 1 - maximum progress.
        .with_progress(0.25)
        .build(ctx)
}
// ANCHOR_END: create_progress_bar

// ANCHOR: change_progress
fn change_progress(progress_bar: Handle<UiNode>, ui: &UserInterface) {
    ui.send(progress_bar, ProgressBarMessage::Progress(0.33));
}
// ANCHOR_END: change_progress
