pub mod border;
pub mod button;
pub mod canvas;
pub mod check_box;
pub mod custom;
pub mod decorator;
mod docking_manager;
pub mod dropdown_list;
pub mod expander;
pub mod font;
pub mod grid;
pub mod image;
pub mod list_view;
pub mod load;
pub mod numeric;
pub mod popup;
pub mod progress;
pub mod rendering;
pub mod screen;
pub mod scroll_bar;
pub mod scroll_panel;
pub mod scroll_viewer;
pub mod stack_panel;
pub mod tab;
pub mod text;
pub mod text_box;
pub mod vector_image;
pub mod window;
pub mod wrap_panel;

use fyrox::asset::manager::ResourceManager;
use fyrox::core::reflect::Reflect;
use fyrox::gui::button::{Button, ButtonBuilder};
use fyrox::gui::grid::{Column, GridBuilder, Row};
use fyrox::gui::image::ImageBuilder;
use fyrox::gui::text::TextBuilder;
use fyrox::gui::widget::WidgetBuilder;
use fyrox::gui::UserInterface;
use fyrox::plugin::error::GameResult;
use fyrox::resource::texture::Texture;
use fyrox::{
    core::pool::Handle,
    core::reflect::prelude::*,
    core::visitor::prelude::*,
    gui::{button::ButtonMessage, message::UiMessage, UiNode},
    plugin::{Plugin, PluginContext},
};

// ANCHOR: message_passing
#[derive(Visit, Clone, Reflect, Debug)]
struct MyPlugin {
    button: Handle<UiNode>,
}

impl Plugin for MyPlugin {
    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        message: &UiMessage,
        ui_handle: Handle<UserInterface>,
    ) -> GameResult {
        if let Some(ButtonMessage::Click) = message.data_from(self.button) {
            println!("The button was clicked!");
        }
        Ok(())
    }
}
// ANCHOR_END: message_passing

// ANCHOR: create_fancy_button
fn create_fancy_button(
    ui: &mut UserInterface,
    resource_manager: ResourceManager,
) -> Handle<Button> {
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

// ANCHOR: create_fancy_button_with_text
fn create_fancy_button_with_text(
    ui: &mut UserInterface,
    resource_manager: ResourceManager,
) -> Handle<Button> {
    let ctx = &mut ui.build_ctx();

    ButtonBuilder::new(WidgetBuilder::new())
        .with_content(
            GridBuilder::new(
                WidgetBuilder::new()
                    .with_child(
                        ImageBuilder::new(WidgetBuilder::new().on_column(0))
                            .with_texture(resource_manager.request::<Texture>("your_icon"))
                            .build(ctx),
                    )
                    .with_child(
                        TextBuilder::new(WidgetBuilder::new().on_column(1))
                            .with_text("My Button")
                            .build(ctx),
                    ),
            )
            .add_row(Row::stretch())
            .add_column(Column::auto())
            .add_column(Column::stretch())
            .build(ctx),
        )
        .build(ctx)
}
// ANCHOR_END: create_fancy_button_with_text

// ANCHOR: create_fancy_button_with_shortcut
fn create_fancy_button_with_shortcut(
    ui: &mut UserInterface,
    resource_manager: ResourceManager,
) -> Handle<Button> {
    let ctx = &mut ui.build_ctx();
    let image;
    ButtonBuilder::new(WidgetBuilder::new())
        .with_back({
            image = ImageBuilder::new(WidgetBuilder::new())
                .with_texture(resource_manager.request::<Texture>("path/to/your/texture"))
                .build(ctx);
            image
        })
        .with_text("Click me!")
        .build(ctx)
}
// ANCHOR_END: create_fancy_button_with_shortcut
