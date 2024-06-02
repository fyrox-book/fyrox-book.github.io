# Wrap panel

![wrap panel](wrap_panel.gif)

Wrap panel is used to stack children widgets either in vertical or horizontal direction with overflow - every widget
that does not have enough space on current line, will automatically be placed on the next line.

## How to create

Use `WrapPanelBuilder` to create new wrap panel instance:

```rust,no_run
{{#include ../code/snippets/src/ui/wrap_panel.rs:create_wrap_panel}}
```

## Orientation

Wrap panel can stack your widgets either in vertical or horizontal direction. Use `.with_orientation` while building 
the panel to switch orientation to desired.

## Use cases

One of many use case examples could be picture gallery, or asset browser in the Fyroxed:

![wrap panel](wrap_panel.png)