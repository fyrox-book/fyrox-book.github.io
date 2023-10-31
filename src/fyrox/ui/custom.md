# Custom Widget

It is possible to create your own widgets, that could solve a specific task that couldn't be solved easily with the
widgets that the engine provides.

Let's say, for instance, that we need to have a custom button with specific visual effects. It will have a border and
a text, and it will also react to mouse events:

![custom widget](custom_widget.gif)

A "skeleton" of such widget could be something like this (for now it does nothing):  

```rust ,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{pool::Handle, reflect::prelude::*, visitor::prelude::*},
#     gui::{
#         message::UiMessage, widget::Widget, widget::WidgetMessage, Control, UiNode,
#         UserInterface, define_widget_deref
#     },
# };
# use std::{
#     any::{Any, TypeId},
#     ops::{Deref, DerefMut},
# };
# 
#[derive(Clone, Debug, Reflect, Visit)]
struct MyButton {
    widget: Widget,
    border: Handle<UiNode>,
    text: Handle<UiNode>,
}

define_widget_deref!(MyButton);

impl Control for MyButton {
    fn query_component(&self, type_id: TypeId) -> Option<&dyn Any> {
        if type_id == TypeId::of::<Self>() {
            Some(self)
        } else {
            None
        }
    }

    fn handle_routed_message(&mut self, ui: &mut UserInterface, message: &mut UiMessage) {
        // Pass another message to the base widget first.
        self.widget.handle_routed_message(ui, message);
    }
}
```

Every widget in the engine must have an instance of `Widget` (`widget: Widget` field) type in them and implement the 
`Control` trait with two required methods. `query_component` is used for dynamic component fetching and could be used 
to support behavior mix-ins and support derived widgets, that based on some other widgets.

`handle_routed_message` is used to react to various messages, but only in the `child -> parent -> parent_of_parent -> ...`
chain. For example, if some of the child widget of our button will receive a message, it will also be passed to our button,
then to a parent of our button (if any), etc. This routing strategy is called "bubble" routing (it acts like a bubble of
air in the water; it always goes up). See [this section](basic_concepts/basic_concepts.md#message-routing-strategies) for
more info.

## Custom Logic

Now let's add some logic to the button, that will handle various mouse events. The full version of our button widget's 
logic will be like so:

```rust ,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{
#         color::{Color, Hsv},
#         pool::Handle,
#         reflect::prelude::*,
#         visitor::prelude::*,
#     },
#     gui::{
#         border::BorderBuilder,
#         brush::Brush,
#         define_constructor, define_widget_deref,
#         message::{MessageDirection, UiMessage},
#         text::TextBuilder,
#         widget::{Widget, WidgetBuilder, WidgetMessage},
#         BuildContext, Control, HorizontalAlignment, Thickness, UiNode, UserInterface,
#         VerticalAlignment,
#     },
# };
# use std::{
#     any::{Any, TypeId},
#     ops::{Deref, DerefMut},
# };
# 
#[derive(Debug, Clone, PartialEq)]
pub enum MyButtonMessage {
    // A message, that will be emitted when our button is clicked.
    Click,
}

impl MyButtonMessage {
    // A constructor for `Click` message.
    define_constructor!(
        MyButtonMessage:Click => fn click(), layout: false
    );
}

#[derive(Clone, Debug, Reflect, Visit)]
struct MyButton {
    widget: Widget,
    border: Handle<UiNode>,
    text: Handle<UiNode>,
}

define_widget_deref!(MyButton);

impl MyButton {
    fn set_colors(&self, ui: &UserInterface, text_color: Color, border_color: Color) {
        for (handle, color) in [(self.border, border_color), (self.text, text_color)] {
            ui.send_message(WidgetMessage::foreground(
                handle,
                MessageDirection::ToWidget,
                Brush::Solid(color),
            ));
        }

        // Make the fill brush of the border slightly dimmer than the input value.
        let mut border_color = Hsv::from(border_color);
        border_color.set_brightness(border_color.brightness() - 20.0);
        ui.send_message(WidgetMessage::background(
            self.border,
            MessageDirection::ToWidget,
            Brush::Solid(border_color.into()),
        ));
    }
}

impl Control for MyButton {
    fn query_component(&self, type_id: TypeId) -> Option<&dyn Any> {
        if type_id == TypeId::of::<Self>() {
            Some(self)
        } else {
            None
        }
    }

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
                        ui.send_message(MyButtonMessage::click(
                            self.handle(),
                            MessageDirection::FromWidget,
                        ));
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
```

As you can see, the most of the code was placed in `handle_routed_message`, we using it to respond for four messages:
`MouseDown`, `MouseUp`, `MouseEnter`, `MouseLeave`. The first two messages is used to handle clicks and send appropriate
event to the outside world if a click has happened. The last two events are used to alter the visual appearance of the 
button by changing the colors of both the border and the text. The source code above is very simple and straightforward,
despite the look of it.

## Builder

Did you notice, that we didn't assign anything to `border` and `text` handles in our button widget? It is because, we
haven't made a respective builder for our button. Builder is a separate structure, that collects all the info from
the outside world and "compiles" it into a finished widget. Usually, widgets contains a bunch of children widgets, which
in their turn could have their own children and so on. In our case, the button will have two child widgets: a border and
a text.

```rust ,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{color::Color, pool::Handle, reflect::prelude::*, visitor::prelude::*},
#     gui::{
#         border::BorderBuilder,
#         brush::Brush,
#         define_constructor, define_widget_deref,
#         message::{MessageDirection, UiMessage},
#         text::TextBuilder,
#         widget::{Widget, WidgetBuilder, WidgetMessage},
#         BuildContext, Control, HorizontalAlignment, UiNode, UserInterface, VerticalAlignment,
#     },
# };
# use std::{
#     any::{Any, TypeId},
#     ops::{Deref, DerefMut},
# };
# 
# #[derive(Clone, Debug, Reflect, Visit)]
# struct MyButton {
#     widget: Widget,
#     border: Handle<UiNode>,
#     text: Handle<UiNode>,
# }
# 
# define_widget_deref!(MyButton);
# 
# impl Control for MyButton {
#     fn query_component(&self, type_id: TypeId) -> Option<&dyn Any> {
#         unimplemented!()
#     }
# 
#     fn handle_routed_message(&mut self, ui: &mut UserInterface, message: &mut UiMessage) {
#         unimplemented!()
#     }
# }
#
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
            .with_stroke_thickness(Thickness::uniform(2.0))
            .build(ctx);

        let button = MyButton {
            widget: self.widget_builder.with_child(border).build(),
            border,
            text,
        };

        ctx.add_node(UiNode::new(button))
    }
}
```

This is how a button is created, at first we're creating a border widget instance with a text widget as a child of it.
Text widget uses the actual text string from our builder, and also it sets the desired alignment in parent border's 
bounds. Finally, we're initializing an instance of `MyButton` with the handles of the widget we've just made and as
the last step we're adding the widget to the user interface.

## Using the Builder

The widget could be created using the builder we've just made like so:

```rust ,no_run,compile_fail
MyButtonBuilder::new(
    WidgetBuilder::new()
        .with_width(200.0)
        .with_height(32.0)
)
.with_text("Click Me!".to_string())
.build(ctx);
```

## Source Code and Web Demo

Full source code for this chapter can be found [here](https://github.com/FyroxEngine/Fyrox-demo-projects/blob/main/ui/game/src/custom.rs)
and you can also run [web demo](https://fyrox.rs/assets/demo/ui/index.html) to see it in action.