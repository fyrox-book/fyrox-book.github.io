use fyrox::core::pool::HandlesVecExtension;
use fyrox::gui::decorator::Decorator;
use fyrox::gui::dropdown_list::DropdownList;
use fyrox::{
    core::pool::Handle,
    gui::{
        border::BorderBuilder,
        decorator::DecoratorBuilder,
        dropdown_list::{DropdownListBuilder, DropdownListMessage},
        message::UiMessage,
        text::TextBuilder,
        widget::WidgetBuilder,
        BuildContext, UiNode,
    },
};

// ANCHOR: create_drop_down_list
fn create_drop_down_list(ctx: &mut BuildContext) -> Handle<DropdownList> {
    DropdownListBuilder::new(WidgetBuilder::new())
        .with_items(
            vec![
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Item 0")
                    .build(ctx),
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Item 1")
                    .build(ctx),
            ]
            .to_base(),
        )
        .with_selected(1)
        .build(ctx)
}
// ANCHOR_END: create_drop_down_list

// ANCHOR: create_drop_down_list_with_decorators
fn make_item(text: &str, ctx: &mut BuildContext) -> Handle<Decorator> {
    DecoratorBuilder::new(BorderBuilder::new(
        WidgetBuilder::new().with_child(
            TextBuilder::new(WidgetBuilder::new())
                .with_text(text)
                .build(ctx),
        ),
    ))
    .build(ctx)
}

fn create_drop_down_list_with_decorators(ctx: &mut BuildContext) -> Handle<DropdownList> {
    DropdownListBuilder::new(WidgetBuilder::new())
        .with_items(vec![make_item("Item 0", ctx), make_item("Item 1", ctx)].to_base())
        .with_selected(1)
        .build(ctx)
}
// ANCHOR_END: create_drop_down_list_with_decorators

// ANCHOR: selection
struct Foo {
    dropdown_list: Handle<UiNode>,
}

impl Foo {
    fn on_ui_message(&mut self, message: &UiMessage) {
        if let Some(DropdownListMessage::Selection(new_selection)) =
            message.data_from(self.dropdown_list)
        {
            // Do something.
            dbg!(new_selection);
        }
    }
}
// ANCHOR_END: selection
