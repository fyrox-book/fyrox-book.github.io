# Button

## Simple button with text

To create a simple button with text you should do something like this:

```rust
# extern crate Fyrox;
# use fyrox::{
#     core::pool::Handle,
#     gui::{button::ButtonBuilder, widget::WidgetBuilder, UiNode, UserInterface},
# };
fn create_button(ui: &mut UserInterface) -> Handle<UiNode> {
    ButtonBuilder::new(WidgetBuilder::new())
        .with_text("Click me!")
        .build(&mut ui.build_ctx())
}
```

How to create a button using custom dimensions (100x100) and custom text alignment (Vertical centered and Horizontal right aligned):

```rust
# extern crate Fyrox;
# use fyrox::{
#     core::pool::Handle,
#     gui::{button::ButtonBuilder, widget::WidgetBuilder, UiNode, UserInterface, HorizontalAlignment, VerticalAlignment, text::TextBuilder},
# };
fn create_button(ui: &mut UserInterface) -> Handle<UiNode> {
    ButtonBuilder::new(
        WidgetBuilder::new()
            .with_width(100.0)
            .with_height(100.0),
    )
    .with_content(
        TextBuilder::new(WidgetBuilder::new())
            .with_text("Click me!")
            .with_horizontal_text_alignment(HorizontalAlignment::Right)
            .with_vertical_text_alignment(VerticalAlignment::Center)
            .build(&mut ui.build_ctx()),
    )
    .build(&mut ui.build_ctx())
}
```

## A button with image

More fancy-looking button with an image as a background could be created using this code snippet:

```rust
# extern crate Fyrox;
# use fyrox::{
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
                    resource_manager.request_texture("path/to/your/texture"),
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
# extern crate Fyrox;
# use fyrox::{
#     core::pool::Handle,
#     engine::{framework::GameState, Engine},
#     gui::{
#         button::{ButtonMessage, ButtonBuilder},
#         message::{UiMessage},
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
        if let Some(ButtonMessage::Click) = message.data() {
            if message.destination() == self.button {
                //
                // Insert your code clicking handling code here.
                //
            }
        }
    }
}
```

## Using a button to exit the game

Add a flag to your Game struct like `exit: bool` and set it in button handler to `true`, then check it in `on_tick` and set `*control_flow = ControlFlow::Exit` if the flag is raised

```rust
# extern crate Fyrox;
#
# use fyrox::{
#    core::pool::Handle,
#    engine::{framework::{GameState, Framework}, Engine},
#    event_loop::ControlFlow,
#    gui::{
#        button::{ButtonBuilder, ButtonMessage},
#        message::UiMessage,
#        widget::WidgetBuilder,
#        UiNode, UserInterface, text::TextBuilder,
#    },
# };

# struct Game {
#    quit_button_handle: Handle<UiNode>,
    exit: bool,
# }

impl GameState for Game {
    fn init(engine: &mut Engine) -> Self
    where
        Self: Sized,
    {
        let quit_button_handle = create_button(&mut engine.user_interface);
        Self {
            quit_button_handle,
            exit: false,
        }
    }

    fn on_tick(&mut self, engine: &mut Engine, dt: f32, control_flow: &mut ControlFlow) {
        if self.exit {
            *control_flow = ControlFlow::Exit;
        }
    }

    fn on_ui_message(&mut self, _engine: &mut Engine, message: UiMessage) {
        if let Some(ButtonMessage::Click) = message.data() {
            if message.destination() == self.quit_button_handle {
                self.exit = true;
            }
        }
    }
}
fn create_button(ui: &mut UserInterface) -> Handle<UiNode> {
    ButtonBuilder::new(WidgetBuilder::new())
        .with_content(
            TextBuilder::new(WidgetBuilder::new())
                .with_text("Quit")
                .build(&mut ui.build_ctx()),
        )
        .build(&mut ui.build_ctx())
}

```

