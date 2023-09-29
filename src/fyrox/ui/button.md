# Button

## Simple button with text

To create a simple button with text you should do something like this:

```rust,no_run
# extern crate fyrox;
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

How to create a button using custom dimensions (100x100) and custom text alignment (Vertical centered and Horizontal 
right aligned):

```rust,no_run
# extern crate fyrox;
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

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle, resource::texture::Texture,
#     asset::manager::ResourceManager,
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
                    resource_manager.request::<Texture, _>("path/to/your/texture"),
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

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     event_loop::ControlFlow,
#     gui::{button::ButtonMessage, message::UiMessage, UiNode},
#     plugin::{PluginContext, Plugin},
# };
# 
struct Game {
    button: Handle<UiNode>,
}

impl Plugin for Game {
    fn on_ui_message(
        &mut self,
        context: &mut PluginContext,
        message: &UiMessage,
        control_flow: &mut ControlFlow,
    ) {
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

This example shows how to create a button that will close your game.

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     event_loop::ControlFlow,
#     gui::{
#         button::{ButtonBuilder, ButtonMessage},
#         message::UiMessage,
#         text::TextBuilder,
#         widget::WidgetBuilder,
#         UiNode, UserInterface,
#     },
#     plugin::PluginContext,
# };
# 
struct Game {
    quit_button_handle: Handle<UiNode>,
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

impl Game {
    fn new(ctx: PluginContext) -> Self {
        Self {
            quit_button_handle: create_button(ctx.user_interface),
        }
    }

    fn on_ui_message(
        &mut self,
        context: &mut PluginContext,
        message: &UiMessage,
        control_flow: &mut ControlFlow,
    ) {
        if let Some(ButtonMessage::Click) = message.data() {
            if message.destination() == self.quit_button_handle {
                *control_flow = ControlFlow::Exit;
            }
        }
    }
}
```

