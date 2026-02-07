use fyrox::gui::wrap_panel::WrapPanel;
use fyrox::{
    core::pool::Handle,
    gui::{widget::WidgetBuilder, wrap_panel::WrapPanelBuilder, BuildContext, Orientation},
};

// ANCHOR: create_wrap_panel
fn create_wrap_panel(ctx: &mut BuildContext) -> Handle<WrapPanel> {
    WrapPanelBuilder::new(WidgetBuilder::new())
        .with_orientation(Orientation::Horizontal)
        .build(ctx)
}
// ANCHOR_END: create_wrap_panel
