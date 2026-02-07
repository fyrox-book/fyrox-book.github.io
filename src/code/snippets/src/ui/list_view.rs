use fyrox::core::pool::{Handle, HandlesVecExtension};
use fyrox::gui::list_view::{ListView, ListViewBuilder, ListViewMessage};
use fyrox::gui::message::UiMessage;
use fyrox::gui::text::TextBuilder;
use fyrox::gui::widget::WidgetBuilder;
use fyrox::gui::wrap_panel::WrapPanelBuilder;
use fyrox::gui::{BuildContext, UiNode, UserInterface};

// ANCHOR: create_list
fn create_list(ctx: &mut BuildContext) -> Handle<ListView> {
    ListViewBuilder::new(WidgetBuilder::new())
        .with_items(
            vec![
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Item0")
                    .build(ctx),
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Item1")
                    .build(ctx),
            ]
            .to_base(),
        )
        .build(ctx)
}
// ANCHOR_END: create_list

// ANCHOR: create_list_with_panel
fn create_list_with_panel(ctx: &mut BuildContext) -> Handle<ListView> {
    ListViewBuilder::new(WidgetBuilder::new())
        // Using WrapPanel instead of StackPanel:
        .with_items_panel(WrapPanelBuilder::new(WidgetBuilder::new()).build(ctx))
        .with_items(
            vec![
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Item0")
                    .build(ctx),
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Item1")
                    .build(ctx),
            ]
            .to_base(),
        )
        .build(ctx)
}
// ANCHOR_END: create_list_with_panel

// ANCHOR: change_selection
fn change_selection(my_list_view: Handle<UiNode>, ui: &UserInterface) {
    ui.send(my_list_view, ListViewMessage::Selection(vec![1]));
}
// ANCHOR_END: change_selection

// ANCHOR: do_something
fn do_something(my_list_view: Handle<UiNode>, message: &UiMessage) {
    if let Some(ListViewMessage::Selection(selection)) = message.data_from(my_list_view) {
        println!("New selection is: {:?}", selection);
    }
}
// ANCHOR_END: do_something

// ANCHOR: change_items
fn change_items(my_list_view: Handle<UiNode>, ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
    // Build new items first.
    let items = vec![
        TextBuilder::new(WidgetBuilder::new())
            .with_text("Item0")
            .build(ctx),
        TextBuilder::new(WidgetBuilder::new())
            .with_text("Item1")
            .build(ctx),
    ]
    .to_base();
    // Then send the message with their handles to the list view.
    ui.send(my_list_view, ListViewMessage::Items(items));
}
// ANCHOR_END: change_items

// ANCHOR: bring_item_into_view
fn bring_item_into_view(my_list_view: Handle<UiNode>, my_item: Handle<UiNode>, ui: &UserInterface) {
    ui.send(my_list_view, ListViewMessage::BringItemIntoView(my_item));
}
// ANCHOR_END: bring_item_into_view
