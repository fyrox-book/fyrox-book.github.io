# Popup

Popup is used to display other widgets in floating panel that could lock input in its own bounds.

## How to create

A simple popup with a button could be created using the following code:

```rust,no_run
{{#include ../code/snippets/src/ui/popup.rs:create_popup_with_button}}
```

Keep in mind that the popup is closed by default. You need to open it explicitly by sending a `PopupMessage::Open` to
it, otherwise you won't see it:

```rust,no_run
{{#include ../code/snippets/src/ui/popup.rs:create_popup_with_button_and_open_it}}
```

## Placement

Since popups are usually used to show useful context-specific information (like context menus, drop-down lists, etc.),
they're usually open above some other widget with specific alignment (right, left, center, etc.).

```rust,no_run
{{#include ../code/snippets/src/ui/popup.rs:create_popup_with_button_and_placement_and_open_it}}
```

The example uses `Placement::Cursor` with `Handle::NONE` placement target for simplicity reasons, however in
the real-world usages this handle must be a handle of some widget that is located under the popup. It is very
important to specify it correctly, otherwise you will lose the built-in ability to fetch the actual placement target.
For example, imagine that you're building your own custom `DropdownList` widget and the popup
is used to display content of the list. In this case, you could specify the placement target like this:

```rust,no_run
{{#include ../code/snippets/src/ui/popup.rs:create_popup_with_button_and_bottom_placement_and_open_it}}
```

In this case, the popup will open at the left bottom corner of the dropdown list automatically. Placement target is also
useful to build context menus, especially for lists with multiple items. Each item in the list usually has the same
context menu, and this is an ideal use case for popups, since the single context menu can be shared across multiple list
items. To find which item cause the context menu to open, catch [`PopupMessage::Placement`] and extract the node
handle - this will be your actual item.

## Opening mode

By default, when you click outside your popup, it will automatically close. It is pretty common behavior in the UI,
you can see it almost every time you use context menus in various apps. There are cases when this behavior is undesired
and it can be turned off:

```rust,no_run
{{#include ../code/snippets/src/ui/popup.rs:create_popup_that_stays_open}}
```

## Smart placement

Popup widget can automatically adjust its position to always remain on screen, which is useful for tooltips, dropdown
lists, etc. To enable this option, use `PopupBuilder::with_smart_placement` with `true` as the first argument.