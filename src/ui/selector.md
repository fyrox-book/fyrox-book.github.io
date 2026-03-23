# Selector

Selector is a simple container widget that allows selecting an item from a fixed set of items.
Selector widget shows the currently selected item at the center and two buttons on the sides
that allows selecting either the previous or the next item.

## Example

The following examples creates a new selector with three items and selects the middle one as
active. The items can be of any type, even mixed types are allowed.

```rust
{{#include ../code/snippets/src/ui/selector.rs:create}}
```

## Selection

The newly selected item index can be received from a selector by listening to [`SelectorMessage::Current`]
message. To select a new item from code, send the same message with the desired index:

```rust
{{#include ../code/snippets/src/ui/selector.rs:on_ui_message}}
```