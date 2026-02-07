use fyrox::gui::message::MessageData;
use fyrox::plugin::error::GameResult;
use fyrox::{
    core::{
        color::{Color, Hsv},
        pool::Handle,
        reflect::prelude::*,
        type_traits::prelude::*,
        visitor::prelude::*,
        ComponentProvider, TypeUuidProvider,
    },
    gui::{
        border::BorderBuilder,
        brush::Brush,
        define_widget_deref,
        message::UiMessage,
        text::TextBuilder,
        widget::Widget,
        widget::{WidgetBuilder, WidgetMessage},
        BuildContext, Control, HorizontalAlignment, Thickness, UiNode, UserInterface,
        VerticalAlignment,
    },
    plugin::{Plugin, PluginContext},
};

// ANCHOR: widget_skeleton
#[derive(Clone, Debug, Reflect, Visit, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "e3b067e1-f3d8-4bac-a272-3c9edd960bf3")]
struct MyButtonExample {
    widget: Widget,
    border: Handle<UiNode>,
    text: Handle<UiNode>,
}

define_widget_deref!(MyButtonExample);

impl Control for MyButtonExample {
    fn handle_routed_message(&mut self, ui: &mut UserInterface, message: &mut UiMessage) {
        // Pass another message to the base widget first.
        self.widget.handle_routed_message(ui, message);
    }
}
// ANCHOR: widget_skeleton

// ANCHOR: my_button
#[derive(Debug, Clone, PartialEq)]
pub enum MyButtonMessage {
    // A message, that will be emitted when our button is clicked.
    Click,
}
impl MessageData for MyButtonMessage {}

#[derive(Clone, Debug, Reflect, Visit, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "e3b067e1-f3d8-4bac-a272-3c9edd960bf3")]
struct MyButton {
    widget: Widget,
    border: Handle<UiNode>,
    text: Handle<UiNode>,
}

define_widget_deref!(MyButton);

impl MyButton {
    fn set_colors(&self, ui: &UserInterface, text_color: Color, border_color: Color) {
        for (handle, color) in [(self.border, border_color), (self.text, text_color)] {
            ui.send(
                handle,
                WidgetMessage::Foreground(Brush::Solid(color).into()),
            );
        }

        // Make the fill brush of the border slightly dimmer than the input value.
        let mut border_color = Hsv::from(border_color);
        border_color.set_brightness(border_color.brightness() - 20.0);
        ui.send(
            self.border,
            WidgetMessage::Background(Brush::Solid(border_color.into()).into()),
        );
    }
}

impl Control for MyButton {
    fn handle_routed_message(&mut self, ui: &mut UserInterface, message: &mut UiMessage) {
        // Pass another message to the base widget first.
        self.widget.handle_routed_message(ui, message);

        // Then process it in our widget.
        if let Some(msg) = message.data::<WidgetMessage>() {
            if message.destination() == self.handle()
                || self.has_descendant(message.destination(), ui)
            {
                match msg {
                    WidgetMessage::MouseUp { .. } => {
                        // Send the message to outside world, saying that the button was clicked.
                        ui.post(self.handle(), MyButtonMessage::Click);
                        ui.release_mouse_capture();
                    }
                    WidgetMessage::MouseDown { .. } => {
                        ui.capture_mouse(message.destination());
                    }
                    WidgetMessage::MouseEnter => {
                        // Make both the border and text brighter when the mouse enter the bounds of our button.
                        self.set_colors(
                            ui,
                            Color::opaque(220, 220, 220),
                            Color::opaque(140, 140, 140),
                        );
                    }
                    WidgetMessage::MouseLeave => {
                        // Make both the border and text dimmer when the mouse leaves the bounds of our button.
                        self.set_colors(
                            ui,
                            Color::opaque(120, 120, 120),
                            Color::opaque(100, 100, 100),
                        );
                    }
                    _ => (),
                }
            }
        }
    }
}
// ANCHOR_END: my_button

// ANCHOR: my_button_builder
pub struct MyButtonBuilder {
    widget_builder: WidgetBuilder,
    // Some text of our button.
    text: String,
}

impl MyButtonBuilder {
    pub fn new(widget_builder: WidgetBuilder) -> Self {
        Self {
            widget_builder,
            text: Default::default(),
        }
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn build(self, ctx: &mut BuildContext) -> Handle<UiNode> {
        let text = TextBuilder::new(
            WidgetBuilder::new()
                .with_vertical_alignment(VerticalAlignment::Center)
                .with_horizontal_alignment(HorizontalAlignment::Center),
        )
        .with_text(self.text)
        .build(ctx);

        let border = BorderBuilder::new(WidgetBuilder::new().with_child(text))
            .with_stroke_thickness(Thickness::uniform(2.0).into())
            .build(ctx);

        let button = MyButton {
            widget: self.widget_builder.with_child(border).build(ctx),
            border,
            text,
        };

        ctx.add_node(UiNode::new(button))
    }
}
// ANCHOR_END: my_button_builder

// ANCHOR: my_button_builder_usage
fn my_button_builder_usage(ctx: &mut BuildContext) {
    MyButtonBuilder::new(WidgetBuilder::new().with_width(200.0).with_height(32.0))
        .with_text("Click Me!".to_string())
        .build(ctx);
}
// ANCHOR_END: my_button_builder_usage

// ANCHOR: reacting_to_click_messages
#[derive(Default, Visit, Reflect, Debug)]
struct MyPlugin {
    my_button: Handle<UiNode>,
}

impl Plugin for MyPlugin {
    fn on_ui_message(
        &mut self,
        context: &mut PluginContext,
        message: &UiMessage,
        ui_handle: Handle<UserInterface>,
    ) -> GameResult {
        if message.destination() == self.my_button {
            if let Some(MyButtonMessage::Click) = message.data() {
                // Do something.
            }
        }
        Ok(())
    }
}
// ANCHOR_END: reacting_to_click_messages
