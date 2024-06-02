use fyrox::{
    core::pool::Handle,
    gui::{numeric::NumericUpDownBuilder, widget::WidgetBuilder, BuildContext, UiNode},
};

// ANCHOR: create_numeric_widget
fn create_numeric_widget(ctx: &mut BuildContext) -> Handle<UiNode> {
    NumericUpDownBuilder::new(WidgetBuilder::new())
        .with_value(123.0f32)
        .build(ctx)
}
// ANCHOR_END: create_numeric_widget

// ANCHOR: create_numeric_widget_with_limits
fn create_numeric_widget_with_limits(ctx: &mut BuildContext) -> Handle<UiNode> {
    NumericUpDownBuilder::new(WidgetBuilder::new())
        .with_value(123.0f32)
        .with_min_value(42.0)
        .with_max_value(666.0)
        .build(ctx)
}
// ANCHOR_END: create_numeric_widget_with_limits

// ANCHOR: create_numeric_widget_with_step
fn create_numeric_widget_with_step(ctx: &mut BuildContext) -> Handle<UiNode> {
    NumericUpDownBuilder::new(WidgetBuilder::new())
        .with_value(125.0f32)
        .with_step(5.0)
        .build(ctx)
}
// ANCHOR_END: create_numeric_widget_with_step

// ANCHOR: create_numeric_widget_with_precision
fn create_numeric_widget_with_precision(ctx: &mut BuildContext) -> Handle<UiNode> {
    NumericUpDownBuilder::new(WidgetBuilder::new())
        .with_value(0.3333333f32)
        .with_precision(2)
        .build(ctx)
}
// ANCHOR_END: create_numeric_widget_with_precision
