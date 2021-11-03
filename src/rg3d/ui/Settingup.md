# Setting up

There is no framework for rg3d-ui itself. This makes it a much more involved part of the engine.

To start off we will use an already available program (This will not do anything)

```rust
use rg3d::engine::resource_manager::MaterialSearchOptions;
use rg3d::engine::Engine;
use rg3d::gui::UiNode;
use rg3d::utils::log::{Log, MessageKind};
use rg3d::{
    animation::Animation,
    core::{
        algebra::{UnitQuaternion, Vector2, Vector3},
        color::Color,
        pool::Handle,
    },
    engine::resource_manager::ResourceManager,
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    gui::{
        border::BorderBuilder,
        button::ButtonBuilder,
        decorator::DecoratorBuilder,
        dropdown_list::DropdownListBuilder,
        grid::{Column, GridBuilder, Row},
        message::{
            ButtonMessage, DropdownListMessage, MessageDirection, ScrollBarMessage, TextMessage,
            UiMessageData,
        },
        scroll_bar::ScrollBarBuilder,
        stack_panel::StackPanelBuilder,
        text::TextBuilder,
        widget::WidgetBuilder,
        window::{WindowBuilder, WindowTitle},
        HorizontalAlignment, Orientation, Thickness, VerticalAlignment,
    },
    monitor::VideoMode,
    scene::{node::Node, Scene},
    utils::translate_event,
    window::Fullscreen,

        
