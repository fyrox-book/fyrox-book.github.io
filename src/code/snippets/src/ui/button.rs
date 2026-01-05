use fyrox::plugin::error::GameResult;
use fyrox::{
    asset::manager::ResourceManager,
    core::pool::Handle,
    core::reflect::prelude::*,
    core::visitor::prelude::*,
    gui::image::ImageBuilder,
    gui::{
        button::ButtonBuilder, text::TextBuilder, widget::WidgetBuilder, HorizontalAlignment,
        UiNode, UserInterface, VerticalAlignment,
    },
    gui::{button::ButtonMessage, message::UiMessage},
    plugin::{Plugin, PluginContext},
    resource::texture::Texture,
};

// ANCHOR: create_button
fn create_button(ui: &mut UserInterface) -> Handle<UiNode> {
    ButtonBuilder::new(WidgetBuilder::new())
        .with_text("Click me!")
        .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_button

// ANCHOR: create_button_custom
fn create_button_custom(ui: &mut UserInterface) -> Handle<UiNode> {
    ButtonBuilder::new(WidgetBuilder::new().with_width(100.0).with_height(100.0))
        .with_content(
            TextBuilder::new(WidgetBuilder::new())
                .with_text("Click me!")
                .with_horizontal_text_alignment(HorizontalAlignment::Right)
                .with_vertical_text_alignment(VerticalAlignment::Center)
                .build(&mut ui.build_ctx()),
        )
        .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_button_custom

// ANCHOR: create_fancy_button
fn create_fancy_button(
    ui: &mut UserInterface,
    resource_manager: ResourceManager,
) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();

    ButtonBuilder::new(WidgetBuilder::new())
        .with_back(
            ImageBuilder::new(WidgetBuilder::new())
                .with_texture(resource_manager.request::<Texture>("path/to/your/texture"))
                .build(ctx),
        )
        .with_text("Click me!")
        .build(ctx)
}
// ANCHOR_END: create_fancy_button

// ANCHOR: button_click_handling
#[derive(Debug, Reflect, Visit)]
struct MyGame {
    button: Handle<UiNode>,
}

impl Plugin for MyGame {
    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        message: &UiMessage,
        ui_handle: Handle<UserInterface>,
    ) -> GameResult {
        if let Some(ButtonMessage::Click) = message.data_from(self.button) {
            //
            // Insert your code clicking handling code here.
            //
        }
        Ok(())
    }
}
// ANCHOR_END: button_click_handling

// ANCHOR: quit_button
#[derive(Visit, Reflect, Debug)]
struct Game {
    quit_button_handle: Handle<UiNode>,
}

fn create_quit_button(ui: &mut UserInterface) -> Handle<UiNode> {
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
            quit_button_handle: create_quit_button(ctx.user_interfaces.first_mut()),
        }
    }
}

impl Plugin for Game {
    fn on_ui_message(
        &mut self,
        context: &mut PluginContext,
        message: &UiMessage,
        ui_handle: Handle<UserInterface>,
    ) -> GameResult {
        if let Some(ButtonMessage::Click) = message.data_from(self.quit_button_handle) {
            context.loop_controller.exit();
        }
        Ok(())
    }
}
// ANCHOR_END: quit_button
