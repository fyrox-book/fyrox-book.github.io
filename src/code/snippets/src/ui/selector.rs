use fyrox::gui::{
    core::pool::{Handle, HandlesVecExtension},
    message::UiMessage,
    selector::{Selector, SelectorBuilder, SelectorMessage},
    text::TextBuilder,
    widget::WidgetBuilder,
    BuildContext, UserInterface,
};

// ANCHOR: create
fn create_selector(ctx: &mut BuildContext) -> Handle<Selector> {
    SelectorBuilder::new(WidgetBuilder::new())
        .with_items(
            vec![
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Item1")
                    .build(ctx),
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Item2")
                    .build(ctx),
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Item3")
                    .build(ctx),
            ]
            .to_base(),
        )
        .with_current_item(1)
        .build(ctx)
}
// ANCHOR_END: create

// ANCHOR: on_ui_message
fn on_ui_message(selector: Handle<Selector>, message: &UiMessage, ui: &UserInterface) {
    if let Some(SelectorMessage::Current(Some(index))) = message.data_from(selector) {
        println!("The new selection is {index}!");

        if *index != 0 {
            // The selection can be changed by sending the same message to the widget:
            ui.send(selector, SelectorMessage::Current(Some(0)));
        }
    }
}
// ANCHOR_END: on_ui_message