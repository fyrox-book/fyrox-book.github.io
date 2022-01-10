# Border

To create a simple thick border you should do something like this:

```rust
# extern crate rg3d;
# use fyrox::{
#     core::pool::Handle,
#     gui::{button::ButtonBuilder, widget::WidgetBuilder, border::BorderBuilder, UiNode, Thickness, UserInterface},
# };
fn create_border_with_button(ui: &mut UserInterface) -> Handle<UiNode> {
    BorderBuilder::new(WidgetBuilder::new()
            .with_child(
                ButtonBuilder::new(WidgetBuilder::new())
                .with_text("Click me")
                .build(&mut ui.build_ctx()),
            )
    )
    .with_stroke_thickness(Thickness {left: 1.0, right: 1.0, top: 1.0, bottom: 1.0})
    .build(&mut ui.build_ctx())
}
```

BorderBuilder is a completely static item that only helps to
beautify the widgets around it, it doesnt have many usages.

// FIXME
