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
            header: TextBuilder::new(WidgetBuilder::new())
                .with_text("First")
                .build(ctx),
            content: TextBuilder::new(WidgetBuilder::new())
                .with_text("First tab's contents!")
                .build(ctx),
            can_be_closed: true,
            user_data: None,
        })
        .with_tab(TabDefinition {
            header: TextBuilder::new(WidgetBuilder::new())
                .with_text("Second")
                .build(ctx),
            content: TextBuilder::new(WidgetBuilder::new())
                .with_text("Second tab's contents!")
                .build(ctx),
            can_be_closed: true,
            user_data: None,
        })
        .build(ctx);
}
// ANCHOR_END: create_tab_control

// ANCHOR: create_tab_control_with_header
fn create_tab_control_with_header(ctx: &mut BuildContext) {
    TabControlBuilder::new(WidgetBuilder::new()).with_tab(TabDefinition {
        header: TextBuilder::new(WidgetBuilder::new().with_margin(Thickness::uniform(4.0)))
            .with_text("First")
            .build(ctx),
        content: TextBuilder::new(WidgetBuilder::new())
            .with_text("First tab's contents!")
            .build(ctx),
        can_be_closed: true,
        user_data: None,
    });
}
// ANCHOR_END: create_tab_control_with_header
