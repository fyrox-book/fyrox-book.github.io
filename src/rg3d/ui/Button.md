# Adding a button

In this section we will add a button to the window and have it 
write to the console.

For now we will update the interface struct and event loop.

```rust
struct Interface {
    Button: Handle<UiNode>,
}
```

And

```rust
fn main() {
/// Snip ///
                while let Some(ui_message) = engine.user_interface.poll_message() {
                    match ui_message.data() {
                        UiMessageData::Button(ButtonMessage::Click) => {
                            // Once we received Click event from Reset button, we have to reset angle and scale
                            // of model. To do that we borrow each UI element in engine and set its value directly.
                            // This is not ideal because there is tight coupling between UI code and model values,
                            // but still good enough for example.
                            if ui_message.destination() == interface.Button {
                                println!("It works!");
                            }
                        }
/// Snip ///
}
```

What we have done is asked the UiMessages that get sent to the event loop if a button was pressed.
If a button was pressed, it asks what the name of the button that was pressed is.

Now we will create the button. To do so we will need to create a grid and a button.

```rust
fn UI(engine: &mut Engine) -> Interface {
    /// Snip ///
   let Button;
   WindowBuilder::new(
        WidgetBuilder::new()
            .with_desired_position(Vector2::new(window_width - 300.0, 0.0))
            .with_width(300.0),
    )
    .with_content(
        GridBuilder::new(
            WidgetBuilder::new()
                .with_child({
                    Button = ButtonBuilder::new(
                       WidgetBuilder::new()
                           .on_row(1) //the second row
                           .on_column(1)
                           .width(100)
                           .with_vertical_alignment(VerticalAlignment::Center),
                    )
                    .with_text("Test")
                    .build(ctx);
                    Button
                }),
        )
        .add_row(Row::strict(30.0))
        .add_row(Row::stretch())
        .add_row(Row::strict(30.0))
        .add_column(Column::strict(30.0))
        .add_column(Column::strict(240.0)
        .add_column(Column::strict(30.0))
        .build(ctx),
    )
    .with_title(WindowTitle::text("Graphics Options"))
    .can_close(false)
    .build(ctx);
}
```

What we have done is created 2 rows with a stretchable row in between and three columns that take all the space in the window.
We also wrote so that the button would be on the 2nd row and the 2nd column. Also, add the with content between the title and 
the WindowBuilder.
