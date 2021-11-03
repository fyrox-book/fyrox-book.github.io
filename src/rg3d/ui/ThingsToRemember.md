# Things to remember

## Conveniences

- While a single interface can be written on a single line,
this is not recommended as it becomes harder to read and check
for errors
- Swapping between multiple lines and single line in your program
won't affect anything. *An example:*
```rust
ButtonBuilder(WidgetBuilder::new())
    //! An example of a single line. This is so the image builder 
    //! is easier to read laterally. Single line is not used very 
    //! often but it is an option.
    .with_back(ImageBuilder::new(WidgetBuilder::new()).with_texture(into_gui_texture(resource_manager.request_texture("your_texture"))).build(ctx))
    //! An example of multiple lines. Widget-specific data is usually 
    //! used with multiple lines.
    .with_text("My Button")
    .build(ctx); 
```

These are by no means a must-do thing, but they are common.

## Things to Remember

- Every widget has to have a WidgetBuilder::new() in it
- Every widget has its own information that can be used to 
customise it
- Every Widget needs to have open brackets for the WidgetBuilder
and subsequent children to be stored in. Widget-specific 
information needs to be stored on the outside of the brackets.
- Every widget needs build(*variable*),
- GridBuilder::new starts off at row(0)
- Only the most recent parent GridBuilder can affect your 
widgets placement
- UiMessageData doesn't change much between different Ui events.
To find information on their differences, looking deep in the docs
is necessary.

## Further resources and information

To find information on widget options, click [here](https://docs.rs/rg3d).

To learn more about WidgetBuilder option click [here](https://docs.rs/rg3d-ui/0.14.0/rg3d_ui/widget/struct.WidgetBuilder.html)
