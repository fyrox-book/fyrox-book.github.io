# Creating a Window

To create a window you should do something like this:

```rust
# use rg3d::{
#     core::pool::Handle,
#     gui::{window::{WindowBuilder, WindowTitle}, widget::WidgetBuilder, UiNode, UserInterface},
# };
fn create_window(ui: &mut UserInterface) {
    WindowBuilder::new(
        WidgetBuilder::new()
            .with_desired_position(Vector2::new(300.0, 0.0)
            .with_width(300.0),
    )
    .with_content(
        // This is where the gui is written
    )
    .with_title(WindowTitle::text("Window"))
    .can_close(true)
    .can_minimize(true)
    .open(true)
    .can_resize(false)
    .build(&mut ui.build_ctx());
}

Something to point out is the .with_desired_position. Its default 
position is the top-left of the screen. Also the .with_content should 
start with a GridBuilder if you plan to add more then one widget. The
rest is human readable and shouldn't need to be explained.

Window is editable object but can only be affected by UI Messages if
their corresponding variable has been set to true.

## TODO example of each ui message. 
I (DuckEater54) dont want to do it. ill probably get a lot wrong. Sorry
## FIXME missing lines, invalid syntax, missing imports
