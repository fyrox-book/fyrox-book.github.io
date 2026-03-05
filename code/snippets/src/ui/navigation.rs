use fyrox::gui::{
    button::ButtonBuilder, navigation::NavigationLayerBuilder, stack_panel::StackPanelBuilder,
    text::TextBuilder, widget::WidgetBuilder, BuildContext,
};

// ANCHOR: create_navigation_layer
fn create_navigation_layer(ctx: &mut BuildContext) {
    NavigationLayerBuilder::new(
        WidgetBuilder::new().with_child(
            StackPanelBuilder::new(
                WidgetBuilder::new()
                    .with_child(
                        // This widget won't participate in Tab key navigation.
                        TextBuilder::new(WidgetBuilder::new())
                            .with_text("Do something?")
                            .build(ctx),
                    )
                    // The keyboard focus for the following two buttons can be cycled using Tab/Shift+Tab.
                    .with_child(
                        ButtonBuilder::new(WidgetBuilder::new().with_tab_index(Some(0)))
                            .with_text("OK")
                            .build(ctx),
                    )
                    .with_child(
                        ButtonBuilder::new(WidgetBuilder::new().with_tab_index(Some(1)))
                            .with_text("Cancel")
                            .build(ctx),
                    ),
            )
            .build(ctx),
        ),
    )
    .build(ctx);
}
// ANCHOR_END: create_navigation_layer
