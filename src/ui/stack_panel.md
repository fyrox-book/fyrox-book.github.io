# Stack Panel

Stack Panels are one of several methods to position multiple widgets in relation to each other. A Stack Panel Widget 
orders its children widgets linearly, aka in a stack of widgets, based on the order the widgets were added as children. 
So the first widget added will be at the top or left most position, while each additional widget will descend from top to 
bottom or continue from left most to right most. The below example code places 3 text widgets into a vertical stack:

```rust,no_run
{{#include ../code/snippets/src/ui/stack_panel.rs:create_stack_panel}}
```

As you can see from the example, creating a Stack Panel uses the standard method for creating widgets. Create a new 
StackPanelBuilder and provide it with a new WidgetBuilder. Adding widgets to the stack is done by adding children to 
the StackBuilder's WidgetBuilder.

## Stack Panel Orientation

As has been indicated, Stack Panels can be oriented to order its children either Vertical, from top to bottom, or 
Horizontal, Left most to right most. This is done using the StackPanelBuilder's with_orientation function providing it 
with a gui::Orientation enum value. **By default,** all StackPanel's are Vertical.

```rust,no_run
{{#include ../code/snippets/src/ui/stack_panel.rs:create_horizontal_stack_panel}}
```
