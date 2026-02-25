use fyrox::{
    gui::messagebox::MessageBox,
    gui::UserInterface,
    gui::{
        core::pool::Handle,
        messagebox::{MessageBoxBuilder, MessageBoxButtons},
        widget::WidgetBuilder,
        window::WindowBuilder,
        BuildContext, UiNode,
    },
    gui::{
        message::UiMessage,
        messagebox::{MessageBoxMessage, MessageBoxResult},
    },
};

// ANCHOR: create_message_box
fn create_message_box(ctx: &mut BuildContext) -> Handle<MessageBox> {
    MessageBoxBuilder::new(WindowBuilder::new(WidgetBuilder::new()))
        .with_buttons(MessageBoxButtons::YesNo)
        .with_text("Do you want to save your changes?")
        .build(ctx)
}
// ANCHOR_END: create_message_box

// ANCHOR: on_ui_message
fn on_ui_message(my_message_box: Handle<UiNode>, message: &UiMessage) {
    if message.destination() == my_message_box {
        if let Some(MessageBoxMessage::Close(result)) = message.data() {
            match result {
                MessageBoxResult::No => {
                    println!("No");
                }
                MessageBoxResult::Yes => {
                    println!("Yes");
                }
                _ => (),
            }
        }
    }
}
// ANCHOR_END: on_ui_message

// ANCHOR: open_message_box
fn open_message_box(my_message_box: Handle<UiNode>, ui: &UserInterface) {
    ui.send(
        my_message_box,
        MessageBoxMessage::Open {
            title: Some("This is the new title".to_string()),
            text: Some("This is the new text".to_string()),
        },
    )
}
// ANCHOR_END: open_message_box
