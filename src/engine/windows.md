# Window Management

This chapter of the book explains how to manage the main application window and its related parts.

> ⚠️ Main window exists only if there's a graphics context. That's why the following examples
> checks for a graphics context to be available. Graphics context could be missing if you're using 
> the engine in "headless" mode (could be useful for game servers) or on some platforms (such as Android)
> that support application suspension.

## Title

Setting a title is very easy to do:

```rust
{{#include ../code/snippets/src/window/mod.rs:title}}
```

## Cursor

This section contains the code for the most common use cases of the mouse cursor. 

### Show or Hide

You can show or hide the mouse cursor using the following code:

```rust
{{#include ../code/snippets/src/window/mod.rs:hide_cursor}}
```

### Lock Inside Window

It is possible to lock the mouse cursor in the window bounds. You can do it using the `set_cursor_grab`
method if the main window:

```rust
{{#include ../code/snippets/src/window/mod.rs:lock_cursor}}
```

## Fullscreen Mode

```rust
{{#include ../code/snippets/src/window/mod.rs:fullscreen}}
```