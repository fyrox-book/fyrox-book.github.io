# Scroll panel 

Scroll panel widget does the same as [Scroll Viewer](scroll_viewer.md) widget, but it does not have any additional
widgets and does not have any graphics. It is a panel widget that provides basic scrolling functionality and 
[Scroll Viewer](scroll_viewer.md) is built on top of it. Strictly speaking, scroll panel widget is used to arrange its 
children widgets, so they can be offset by a certain amount of units from top-left corner. It is used to provide basic 
scrolling functionality.

## Examples

```rust
{{#include ../code/snippets/src/ui/scroll_panel.rs:create_scroll_panel}}
```

## Scrolling

Scrolling value for both axes can be set via `ScrollPanelMessage::VerticalScroll` and `ScrollPanelMessage::HorizontalScroll`:

```rust
{{#include ../code/snippets/src/ui/scroll_panel.rs:set_scrolling_value}}
```

## Bringing child into view

Calculates the scroll values to bring a desired child into view, it can be used for automatic navigation:

```rust
{{#include ../code/snippets/src/ui/scroll_panel.rs:bring_child_into_view}}
```