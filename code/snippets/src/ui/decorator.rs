use fyrox::gui::decorator::Decorator;
use fyrox::gui::{
    border::BorderBuilder,
    brush::Brush,
    core::{color::Color, pool::Handle},
    decorator::DecoratorBuilder,
    widget::WidgetBuilder,
    BuildContext,
};

// ANCHOR: create_decorator
fn create_decorator(ctx: &mut BuildContext) -> Handle<Decorator> {
    DecoratorBuilder::new(BorderBuilder::new(WidgetBuilder::new()))
        .with_hover_brush(Brush::Solid(Color::opaque(0, 255, 0)).into())
        .build(ctx)
}
// ANCHOR_END: create_decorator
