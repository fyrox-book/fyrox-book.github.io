use fyrox::core::uuid;
use fyrox::gui::{
    tab_control::{TabControlBuilder, TabDefinition},
    text::TextBuilder,
    widget::WidgetBuilder,
    BuildContext, Thickness,
};

// ANCHOR: create_tab_control
fn create_tab_control(ctx: &mut BuildContext) {
    TabControlBuilder::new(WidgetBuilder::new())
        .with_tab(TabDefinition {
            uuid: uuid!("fefcd6a1-4872-4571-8562-77a32c33bcaf"),
            header: TextBuilder::new(WidgetBuilder::new())
                .with_text("First")
                .build(ctx)
                .to_base(),
            content: TextBuilder::new(WidgetBuilder::new())
                .with_text("First tab's contents!")
                .build(ctx)
                .to_base(),
            can_be_closed: true,
            user_data: None,
        })
        .with_tab(TabDefinition {
            uuid: uuid!("042bd473-afc6-438c-93f0-2a81b11bc02f"),
            header: TextBuilder::new(WidgetBuilder::new())
                .with_text("Second")
                .build(ctx)
                .to_base(),
            content: TextBuilder::new(WidgetBuilder::new())
                .with_text("Second tab's contents!")
                .build(ctx)
                .to_base(),
            can_be_closed: true,
            user_data: None,
        })
        .build(ctx);
}
// ANCHOR_END: create_tab_control

// ANCHOR: create_tab_control_with_header
fn create_tab_control_with_header(ctx: &mut BuildContext) {
    TabControlBuilder::new(WidgetBuilder::new()).with_tab(TabDefinition {
        uuid: uuid!("e866c6ea-de94-4471-ba4a-9920631189d6"),
        header: TextBuilder::new(WidgetBuilder::new().with_margin(Thickness::uniform(4.0)))
            .with_text("First")
            .build(ctx)
            .to_base(),
        content: TextBuilder::new(WidgetBuilder::new())
            .with_text("First tab's contents!")
            .build(ctx)
            .to_base(),
        can_be_closed: true,
        user_data: None,
    });
}
// ANCHOR_END: create_tab_control_with_header
