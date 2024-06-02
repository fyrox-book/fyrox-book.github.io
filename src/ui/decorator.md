# Decorator

A visual element that changes its appearance by listening specific events. It can have "pressed", "hover", "selected" or 
normal appearance:

- `Pressed` - enables on mouse down message.
- `Selected` - whether decorator selected or not.
- `Hovered` - mouse is over the decorator.
- `Normal` - not selected, pressed or hovered.

This element is widely used to provide some generic visual behaviour for various widgets. For example, it used in 
buttons, tree items, dropdown list items, etc.; in other words - everywhere where a widget needs to give visual 
feedback the user.

## Example

```rust
{{#include ../code/snippets/src/ui/decorator.rs:create_decorator}}
```