use fyrox::{
    core::{color::Color, pool::Handle},
    gui::{
        border::BorderBuilder, brush::Brush, text::TextBuilder, widget::WidgetBuilder, Thickness,
        UiNode, UserInterface,
    },
};

// ANCHOR: create_border_with_button
fn create_border_with_button(ui: &mut UserInterface) -> Handle<UiNode> {
    BorderBuilder::new(
        WidgetBuilder::new().with_child(
            TextBuilder::new(WidgetBuilder::new())
                .with_text("I'm boxed in!")
                .build(&mut ui.build_ctx()),
        ),
    )
    //You can also use Thickness::uniform(1.0)
    .with_stroke_thickness(
        Thickness {
            left: 1.0,
            right: 1.0,
            top: 1.0,
            bottom: 1.0,
        }
        .into(),
    )
    .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_border_with_button

// ANCHOR: create_blue_border_with_red_background
fn create_blue_border_with_red_background(ui: &mut UserInterface) -> Handle<UiNode> {
    BorderBuilder::new(
        WidgetBuilder::new()
            .with_foreground(Brush::Solid(Color::opaque(0, 0, 200)).into())
            .with_background(Brush::Solid(Color::opaque(200, 0, 0)).into())
            .with_child(
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("I'm boxed in Blue and backed in Red!")
                    .build(&mut ui.build_ctx()),
            ),
    )
    .with_stroke_thickness(
        Thickness {
            left: 2.0,
            right: 2.0,
            top: 2.0,
            bottom: 2.0,
        }
        .into(),
    )
    .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_blue_border_with_red_background
