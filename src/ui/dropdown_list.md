# Dropdown list 

![dropdown list](dropdown_list.gif)

Drop-down list is a control which shows the currently selected item and provides a dropdown 
list to select its current item. It is used to show a single selected item in a compact way.

## Example

A dropdown list with two text items with the last one selected, could be created like so:

```rust,no_run
{{#include ../code/snippets/src/ui/dropdown_list.rs:create_drop_down_list}}
```

Keep in mind, that items of a dropdown list could be any widget, but usually each item is wrapped
in some other widget that shows the current state of items (selected, hovered, clicked, etc.). One
of the most convenient ways of doing this is to use Decorator widget:

```rust,no_run
{{#include ../code/snippets/src/ui/dropdown_list.rs:create_drop_down_list_with_decorators}}
```

## Selection

Dropdown list supports two kinds of selection - `None` or `Some(index)`. To catch a moment when
selection changes, use the following code:

```rust,no_run
{{#include ../code/snippets/src/ui/dropdown_list.rs:selection}}
```

To change selection of a dropdown list, send `DropdownListMessage::SelectionChanged` message
to it.

## Items

To change current items of a dropdown list, create the items first and then send them to the
dropdown list using `DropdownListMessage::Items` message.

## Opening and Closing

A dropdown list could be opened and closed manually using `DropdownListMessage::Open` and
`DropdownListMessage::Close` messages.  