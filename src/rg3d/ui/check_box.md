# Check box

To create a checkbox you should do something like this:

```rust
# extern crate rg3d;
# use rg3d::{
#     core::pool::Handle,
#     gui::{check_box::CheckBoxBuilder, widget::WidgetBuilder, UiNode, UserInterface},
# };
fn create_checkbox(ui: &mut UserInterface) -> Handle<UiNode> {
    CheckBoxBuilder::new(WidgetBuilder::new())
        .checked(false)
        .build(&mut ui.build_ctx())
}
```

## Message handling

Checkboxes are not static widget and have muliple states.
To handle a message from a checkbox, you need to read a 
`CheckBoxMessage::checked` variable. To do so, you can do
something like this:

```rust
use rg3d::{
    core::pool::Handle,
    engine::{framework::GameState, Engine},
    gui::{
        check_box::CheckBoxBuilder,
        message::{CheckboxMessage, UiMessage, UiMessageData},
        widget::WidgetBuilder,
        UiNode,
    },
};

struct Game {
    checkbox: Handle<UiNode>,
}

impl GameState for Game {
      // ...
    fn init(engine: &mut Engine) -> Self
        where
            Self: Sized,
    {
        Self {
            checkbox: CheckBoxBuilder::new(WidgetBuilder::new())
                .checked(false)
                .build(&mut ui.build_ctx()),
        }
    }

    fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {
        if let UiMessageData::CheckBox(CheckBoxMessage::Check) = message.data() {
            if message.destination() == self.checkbox {
                //
                // Insert your code clicking handling code here.
                //
            }
        }
    }
}
```
