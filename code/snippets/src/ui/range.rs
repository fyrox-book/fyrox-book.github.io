use fyrox::{
    core::pool::Handle,
    gui::{
        message::UiMessage,
        range::{RangeEditor, RangeEditorBuilder, RangeEditorMessage},
        widget::WidgetBuilder,
        BuildContext, UiNode, UserInterface,
    },
};

// ANCHOR: create_range_editor
fn create_range_editor(ctx: &mut BuildContext) -> Handle<RangeEditor<u32>> {
    RangeEditorBuilder::new(WidgetBuilder::new())
        .with_value(0u32..100u32)
        .build(ctx)
}
// ANCHOR_END: create_range_editor

// ANCHOR: change_value
fn change_value(range_editor: Handle<UiNode>, ui: &UserInterface) {
    ui.send(range_editor, RangeEditorMessage::Value(5u32..20u32))
}
// ANCHOR_END: change_value

// ANCHOR: fetch_value
fn fetch_value(range_editor: Handle<UiNode>, message: &UiMessage) {
    if let Some(RangeEditorMessage::Value(range)) =
        message.data_from::<RangeEditorMessage<u32>>(range_editor)
    {
        println!("The new value is: {:?}", range)
    }
}
// ANCHOR_END: fetch_value
