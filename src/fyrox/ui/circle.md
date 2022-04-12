# Circle

This section will show how to create Custom widgets using `VectorImageBuilder` in order to create a circle

Increasing the number of segments the result will be more likely a circle.

In this example the circle is formed from 4 segments so it will appear like a rhombus. With 5 segments it will like a pentagon and so on. 

```rust, no_run

# extern crate fyrox;

# use fyrox::{
#    core::{algebra::Vector2, color::Color},
#    engine::{framework::prelude::*, Engine},
#    gui::{
#        brush::Brush,
#        message::UiMessage,
#        vector_image::{Primitive, VectorImageBuilder},
#        widget::WidgetBuilder,
#    },
# };
#
# struct Game {
#    // Empty for now
# }

# impl GameState for Game {
#    fn init(engine: &mut Engine) -> Self
#    where
#        Self: Sized,
#    {
        // Build context will be used in the next chapters.
        let ctx = &mut engine.user_interface.build_ctx();
        //
        // All widgets will be created here in the next chapters.
        //
        VectorImageBuilder::new(
            WidgetBuilder::new()
                .with_desired_position(Vector2::new(100f32, 100f32))
                .with_foreground(Brush::Solid(Color::BLUE))
                .with_height(200f32) // optional
                .with_width(200f32) // optional
                .with_opacity(Some(0.5)),
        )
        .with_primitives(vec![Primitive::Circle {
            center: Vector2::new(100.0, 100.0),
            radius: 100.0,
            segments: 4,
        }])
        .build(ctx);
#        Self {}
#    }

# }

# fn main() {
#    Framework::<Game>::new()
#        .unwrap()
#        .title("User Interface")
#        .run();
# }
```
