# Scroll viewer

![scroll viewer](scroll_bar.gif)

Scroll viewer is a scrollable region with two scroll bars for each axis. It is used to wrap a content of unknown
size to ensure that all of it will be accessible in its parent widget bounds. For example, it could be used in a
[Window](window.md) widget to allow a content of the window to be accessible, even if the window is smaller than 
the content.

## Example

A scroll viewer widget could be created using `ScrollViewerBuilder`:

```rust
{{#include ../code/snippets/src/ui/scroll_viewer.rs:create_scroll_viewer}}
```

Keep in mind, that you can change the content of a scroll viewer at runtime using `ScrollViewerMessage::Content` message.

## Scrolling Speed and Controls

Scroll viewer can have an arbitrary scrolling speed for each axis. Scrolling is performed via mouse wheel and by default it
scrolls vertical axis, which can be changed by holding `Shift` key. Scrolling speed can be set during the build phase:

```rust
{{#include ../code/snippets/src/ui/scroll_viewer.rs:create_scroll_viewer_with_speed}}
```

Also, it could be set using `ScrollViewerMessage::HScrollSpeed` or `ScrollViewerMessage::VScrollSpeed` messages.

## Bringing a child into view

Calculates the scroll values to bring a desired child into view, it can be used for automatic navigation:

```rust
{{#include ../code/snippets/src/ui/scroll_viewer.rs:bring_child_into_view}}
```