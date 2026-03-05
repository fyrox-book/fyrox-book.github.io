# Navigation

A widget, that handles keyboard navigation on its descendant widgets using Tab key. It should
be used as a root widget for a hierarchy, that should support Tab key navigation:

```rust
{{#include ../code/snippets/src/ui/navigation.rs:create_navigation_layer}}
 ```

This example shows how to create a simple confirmation dialog, that allows a user to use Tab key
to cycle from one button to another. A focused button then can be "clicked" using Enter key.