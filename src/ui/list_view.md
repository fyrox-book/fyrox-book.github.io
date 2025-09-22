# List view

![list view](list_view.gif)

List view is used to display lists with arbitrary items. It supports multiple selection and by
default, it stacks the items vertically (this can be changed by providing a custom panel for the
items, see the section below).

## Example

`ListView` can be created using `ListViewBuilder`:

```rust,no_run
{{#include ../code/snippets/src/ui/list_view.rs:create_list}}
```

Keep in mind, that the items of the list view can be pretty much any other widget. They also don't have to be the same
type, you can mix any type of widgets.

## Custom Items Panel

By default, list view creates inner `StackPanel` to arrange its items. It is enough for most cases, however,
in rare cases you might want to use something else. For example, you could use `WrapPanel` to create list view with
selectable "tiles":

```rust,no_run
{{#include ../code/snippets/src/ui/list_view.rs:create_list_with_panel}}
```

## Selection

List view supports any number of selected items (you can add items to the current selecting by holding Ctrl key),
you can change it at runtime by sending [`ListViewMessage::SelectionChanged`] message with
[`MessageDirection::ToWidget`] like so:

```rust,no_run
{{#include ../code/snippets/src/ui/list_view.rs:change_selection}}
```

It is also possible to not have selected item at all, to do this you need to send an empty vector as a selection.
To catch the moment when selection has changed (either by a user or by the `ListViewMessage::SelectionChanged`)
you need to listen to the same message but with opposite direction, like so:

```rust,no_run
{{#include ../code/snippets/src/ui/list_view.rs:do_something}}
```

## Adding/removing items

To change items of the list view you can use the variety of following messages: ListViewMessage::AddItem`, 
`ListViewMessage::RemoveItem`, `ListViewMessage::Items`. To decide which one to use, is very simple - if you 
adding/removing a few items, use `ListViewMessage::AddItem` and `ListViewMessage::RemoveItem`, otherwise use 
`ListViewMessage::Items`, which changes the items at once.

```rust,no_run
{{#include ../code/snippets/src/ui/list_view.rs:change_items}}
```

## Bringing a particular item into view

It is possible to bring a particular item into view, which is useful when you have hundreds or thousands of items and
you
want to bring only particular item into view. It could be done by sending a `ListViewMessage::BringItemIntoView`
message:

```rust,no_run
{{#include ../code/snippets/src/ui/list_view.rs:bring_item_into_view}}
```