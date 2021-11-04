## Simple button with text

To create a simple button with text you should do something like this:

```rust
# extern crate rg3d;
# use rg3d::{
#     core::pool::Handle,
#     gui::{button::ButtonBuilder, widget::WidgetBuilder, UiNode, UserInterface},
# };
fn create_button(ui: &mut UserInterface) -> Handle<UiNode> {
    ButtonBuilder::new(WidgetBuilder::new())
        .with_text("Click me!")
        .build(&mut ui.build_ctx())
}
```

## A button with image

More fancy-looking button with an image as a background could be created using this code snippet:

```rust
# extern crate rg3d;
# use rg3d::{
#     core::pool::Handle,
#     engine::resource_manager::ResourceManager,
#     gui::{
#         button::ButtonBuilder, image::ImageBuilder, widget::WidgetBuilder, UiNode,
#         UserInterface,
#     },
#     utils::into_gui_texture,
# };
fn create_fancy_button(ui: &mut UserInterface, resource_manager: ResourceManager) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();

    ButtonBuilder::new(WidgetBuilder::new())
        .with_back(
            ImageBuilder::new(WidgetBuilder::new())
                .with_texture(into_gui_texture(
                    resource_manager.request_texture("path/to/your/texture", None),
                ))
                .build(ctx),
        )
        .with_text("Click me!")
        .build(ctx)
}
```

## Message handling

When clicked, a button sends a `ButtonMessage::Click` message, you can catch it in your code and do something
useful:

```rust
# extern crate rg3d;
# use rg3d::{
#     core::pool::Handle,
#     engine::{framework::GameState, Engine},
#     gui::{
#         button::ButtonBuilder,
#         message::{ButtonMessage, UiMessage, UiMessageData},
#         widget::WidgetBuilder,
#         UiNode,
#     },
# };
# 
# struct Game {
#     button: Handle<UiNode>,
# }
# 
impl GameState for Game {
      // ...
#     fn init(engine: &mut Engine) -> Self
#         where
#             Self: Sized,
#     {
#         Self {
#             button: ButtonBuilder::new(WidgetBuilder::new())
#                 .with_text("Click me!")
#                 .build(&mut engine.user_interface.build_ctx()),
#         }
#     }
# 
    fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {
        if let UiMessageData::Button(ButtonMessage::Click) = message.data() {
            if message.destination() == self.button {
                //
                // Insert your code clicking handling code here.
                //
            }
        }
    }
}
```

