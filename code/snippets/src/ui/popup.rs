use fyrox::core::pool::Handle;
use fyrox::gui::button::ButtonBuilder;
use fyrox::gui::popup::{Placement, Popup, PopupBuilder, PopupMessage};
use fyrox::gui::widget::WidgetBuilder;
use fyrox::gui::{BuildContext, UiNode, UserInterface};

// ANCHOR: create_popup_with_button
fn create_popup_with_button(ctx: &mut BuildContext) -> Handle<Popup> {
    PopupBuilder::new(WidgetBuilder::new())
        .with_content(
            ButtonBuilder::new(WidgetBuilder::new())
                .with_text("Click Me!")
                .build(ctx),
        )
        .build(ctx)
}
// ANCHOR_END: create_popup_with_button

// ANCHOR: create_popup_with_button_and_open_it
fn create_popup_with_button_and_open_it(ui: &mut UserInterface) -> Handle<Popup> {
    let popup = PopupBuilder::new(WidgetBuilder::new())
        .with_content(
            ButtonBuilder::new(WidgetBuilder::new())
                .with_text("Click Me!")
                .build(&mut ui.build_ctx()),
        )
        .build(&mut ui.build_ctx());
    // Open the popup explicitly.
    ui.send(popup, PopupMessage::Open);
    popup
}
// ANCHOR_END: create_popup_with_button_and_open_it

// ANCHOR: create_popup_with_button_and_placement_and_open_it
fn create_popup_with_button_and_placement_and_open_it(ui: &mut UserInterface) -> Handle<Popup> {
    let popup = PopupBuilder::new(WidgetBuilder::new())
        .with_content(
            ButtonBuilder::new(WidgetBuilder::new())
                .with_text("Click Me!")
                .build(&mut ui.build_ctx()),
        )
        // Set the placement. For simplicity it is just a cursor position with Handle::NONE as placement target.
        .with_placement(Placement::Cursor(Handle::NONE))
        .build(&mut ui.build_ctx());
    // Open the popup explicitly at the current placement.
    ui.send(popup, PopupMessage::Open);
    popup
}
// ANCHOR_END: create_popup_with_button_and_placement_and_open_it

// ANCHOR: create_popup_with_button_and_bottom_placement_and_open_it
fn create_popup_with_button_and_bottom_placement_and_open_it(
    dropdown_list: Handle<UiNode>,
    ui: &mut UserInterface,
) -> Handle<Popup> {
    let popup = PopupBuilder::new(WidgetBuilder::new())
        .with_content(
            ButtonBuilder::new(WidgetBuilder::new())
                .with_text("Click Me!")
                .build(&mut ui.build_ctx()),
        )
        // Set the placement to the dropdown list.
        .with_placement(Placement::LeftBottom(dropdown_list))
        .build(&mut ui.build_ctx());
    // Open the popup explicitly at the current placement.
    ui.send(popup, PopupMessage::Open);
    popup
}
// ANCHOR_END: create_popup_with_button_and_bottom_placement_and_open_it

// ANCHOR: create_popup_that_stays_open
fn create_popup_that_stays_open(ctx: &mut BuildContext) -> Handle<Popup> {
    PopupBuilder::new(WidgetBuilder::new())
        .with_content(
            ButtonBuilder::new(WidgetBuilder::new())
                .with_text("Click Me!")
                .build(ctx),
        )
        // This forces the popup to stay open when clicked outside of its bounds
        .stays_open(true)
        .build(ctx)
}
// ANCHOR_END: create_popup_that_stays_open
