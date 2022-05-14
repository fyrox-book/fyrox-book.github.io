# Border

The Border widget provides a stylized, static border around it's child widget. Below is an example of creating a 1 pixel thick border around a button widget:

```rust
# extern crate fyrox;
use fyrox::gui::{
    UserInterface,
    widget::WidgetBuilder, 
    border::BorderBuilder, 
    Thickness, 
    text::TextBuilder,
};

fn create_border_with_button(ui: &mut UserInterface) {
    BorderBuilder::new(
        WidgetBuilder::new()
            .with_child(
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("I'm boxed in!")
                    .build(&mut ui.build_ctx())
            )
    )
    //You can also use Thickness::uniform(1.0)
    .with_stroke_thickness(Thickness {left: 1.0, right: 1.0, top: 1.0, bottom: 1.0})
    .build(&mut ui.build_ctx());
}
```

As with other UI elements, we create the border using the BorderBuilder helper struct. The widget that should have a border around it is added as a child of the base WidgetBuilder, and the border thickness can be set by providing a Thickness struct to the BorderBuilder's *with_stroke_thickness* function. This means you can set different thicknesses for each edge of the border.

You can style the border by creating a Brush and setting the border's base WidgetBuilder's foreground or background. The foreground will set the style of the boarder itself, while setting the background will color the whole area within the border. Below is an example of a blue border and a red background with white text inside.

```rust
use fyrox::gui::{
    brush::Brush,
    core::color::Color,
};

BorderBuilder::new(
    WidgetBuilder::new()
        .with_foreground(Brush::Solid(Color::opaque(0, 0, 200)))
        .with_background(Brush::Solid(Color::opaque(200, 0, 0)))
        .with_child(
            TextBuilder::new(WidgetBuilder::new())
                .with_text("I'm boxed in Blue and backed in Red!")
                .build(&mut ui.build_ctx())
        )
)
.with_stroke_thickness(Thickness {left: 2.0, right: 2.0, top: 2.0, bottom: 2.0})
.build(&mut ui.build_ctx());
```
