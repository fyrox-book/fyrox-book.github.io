# Stack Panel

Stack Panels are one of several methods to position multiple widgets in relation to each other. A Stack Panel Widget 
orders it's children widgets linerarly, aka in a stack of widgets, based on the order the widgets were added as children. 
So the first widget added will be at the top or left most position, while each additional widget will decend from top to 
bottom or continue from left most to right most. The below example code places 3 text widgets into a vertical stack:

```rust,no_run
# extern crate fyrox;
# use fyrox::gui::{
#     UiNode,
#     BuildContext,
#     widget::WidgetBuilder,
#     text::TextBuilder,
#     stack_panel::StackPanelBuilder,
# };

fn create_stack_panel(ctx: &mut BuildContext) -> fyrox::core::pool::Handle<UiNode> {

    StackPanelBuilder::new(
        WidgetBuilder::new()
            .with_child(
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Top")
                    .build(ctx)
            )
            .with_child(
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Middle")
                    .build(ctx)
            )
            .with_child(
                TextBuilder::new(WidgetBuilder::new())
                    .with_text("Bottom")
                    .build(ctx)
            )
    )
        .build(ctx)
    
}
```

As you can see from the example, creating a Stack Panel uses the standard method for creating widgets. Create a new 
StackPanelBuilder and provide it with a new WidgetBuilder. Adding widgets to the stack is done by adding childeren to 
the StackBuilder's WidgetBuilder.

## Stack Panel Orientation

As has been indicated, Stack Panels can be oriented to order it's children either Vertical, from top to bottom, or 
Horizontal, Left most to right most. This is done using the StackPanelBuilder's with_orientation function providing it 
with a gui::Orientation enum value. **By default** all Stack Panel's are Vertical.

```rust,no_run
# extern crate fyrox;
# use fyrox::gui::{
#     Orientation,
#     BuildContext,
#     widget::WidgetBuilder,
#     stack_panel::StackPanelBuilder,
# };

# fn build(ctx: &mut BuildContext) {
StackPanelBuilder::new(
    WidgetBuilder::new()
)
    .with_orientation(Orientation::Horizontal)
    .build(ctx);
# }
```
