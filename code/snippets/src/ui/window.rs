use fyrox::{
    core::algebra::Vector2,
    gui::{
        text::TextBuilder,
        widget::WidgetBuilder,
        window::{WindowBuilder, WindowTitle},
        UserInterface,
    },
};

// ANCHOR: create_window
fn create_window(ui: &mut UserInterface) {
    WindowBuilder::new(
        WidgetBuilder::new()
            .with_desired_position(Vector2::new(300.0, 0.0))
            .with_width(300.0),
    )
    .with_content(
        TextBuilder::new(WidgetBuilder::new())
            .with_text("Example Window content.")
            .build(&mut ui.build_ctx()),
    )
    .with_title(WindowTitle::text("Window"))
    .can_close(true)
    .can_minimize(true)
    .open(true)
    .can_resize(false)
    .build(&mut ui.build_ctx());
}
// ANCHOR_END: create_window
