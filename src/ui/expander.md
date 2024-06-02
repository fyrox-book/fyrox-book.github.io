# Expander

![expander](expander.gif)

Expander is a simple collapsible container that has a header and collapsible/expandable content zone. It is used to
create collapsible regions with headers.

## Examples

The following example creates a simple expander with a textual header and a stack panel widget with few
buttons a content:

```rust
{{#include ../code/snippets/src/ui/expander.rs:create_expander}}
```

## Customization

It is possible to completely change the arrow of the header of the expander. By default, the arrow consists
of `crate::check_box::CheckBox` widget. By changing the arrow, you can customize the look of the header.
For example, you can set the new check box with image check marks, which will use custom graphics:

```rust
{{#include ../code/snippets/src/ui/expander.rs:create_expander_with_image}}
```

## Messages

Use `ExpanderMessage::Expand` message to catch the moment when its state changes:

```rust
{{#include ../code/snippets/src/ui/expander.rs:message_handling}}
```

To switch expander state at runtime, send `ExpanderMessage::Expand` to your Expander widget instance with
`MessageDirection::ToWidget`.