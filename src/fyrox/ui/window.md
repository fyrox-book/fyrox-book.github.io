# Creating a Window

To create a window you should do something like this:

```rust
# extern crate fyrox;
# use fyrox::{
#     core::{pool::Handle, algebra::Vector2},
#     gui::{window::{WindowBuilder, WindowTitle}, widget::WidgetBuilder, UiNode, UserInterface},
# };
fn create_window(ui: &mut UserInterface) {
    WindowBuilder::new(
        WidgetBuilder::new()
            .with_desired_position(Vector2::new(300.0, 0.0))
            .with_width(300.0),
    )
    .with_content(
#       Handle::NONE
    )
    .with_title(WindowTitle::text("Window"))
    .can_close(true)
    .can_minimize(true)
    .open(true)
    .can_resize(false)
    .build(&mut ui.build_ctx());
}
```

Something to point out is the .with_desired_position. Its default 
position is the top-left of the screen. Also the .with_content should 
start with a GridBuilder if you plan to add more then one widget. The
rest is human readable and shouldn't need to be explained.

Window is editable object but can only be affected by UI Messages if
their corresponding variable has been set to true.
