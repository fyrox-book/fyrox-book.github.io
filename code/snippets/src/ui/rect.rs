use fyrox::gui::{
    core::{math::Rect, pool::Handle},
    message::{MessageDirection, UiMessage},
    rect::{RectEditor, RectEditorBuilder, RectEditorMessage},
    widget::WidgetBuilder,
    BuildContext, UiNode, UserInterface,
};

// ANCHOR: create_rect_editor
fn create_rect_editor(ctx: &mut BuildContext) -> Handle<RectEditor<u32>> {
    RectEditorBuilder::new(WidgetBuilder::new())
        .with_value(Rect::new(0, 0, 10, 20))
        .build(ctx)
}
// ANCHOR_END: create_rect_editor

// ANCHOR: change_value
fn change_value(rect_editor: Handle<UiNode>, ui: &UserInterface) {
    ui.send(
        rect_editor,
        RectEditorMessage::Value(Rect::new(20, 20, 60, 80)),
    );
}
// ANCHOR_END: change_value

// ANCHOR: fetch_value
fn fetch_value(rect_editor: Handle<UiNode>, message: &UiMessage) {
    if let Some(RectEditorMessage::Value(value)) = message.data::<RectEditorMessage<u32>>() {
        if message.destination() == rect_editor
            && message.direction() == MessageDirection::FromWidget
        {
            println!("The new value is: {:?}", value)
        }
    }
}
// ANCHOR_END: fetch_value