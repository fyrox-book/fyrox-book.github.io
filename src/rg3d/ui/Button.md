## Simple button with text

To create a simple button with text you should do something like this:

```rust
let button = ButtonBuilder::new(WidgetBuilder::new())
        .with_text("Click me!")
        .build(ctx);
```

## A button with image

More fancy-looking button could be created using this code snippet:

```rust
let button = ButtonBuilder::new(WidgetBuilder::new())
    .with_back(
        ImageBuilder::new(WidgetBuilder::new())
            .with_texture(into_gui_texture(
                resource_manager.request_texture("path/to/your/texture"),
            ))
            .build(ctx),
    )
    .with_text("Click me!")
    .build(ctx);
```

## Message handling

When clicked, a button sends a `ButtonMessage::Click` message, you can catch it in your code and do something
useful:

```rust
fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {    
    if let UiMessageData::Button(ButtonMessage::Click) = message.data() {
        if message.destination() == self.button {
            // 
            // Insert your code clicking handling code here.
            //
        }
    }
}
```

