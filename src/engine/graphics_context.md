# Graphics Con**t**ext

Graphics context stores the main application window and the renderer. Graphics context could not exist at all, this is 
so called "headless" mode which could be useful for dedicated servers.

## Creation

Graphics context is created and destroyed automatically when the engine receives `Resumed` and `Suspended`
events. Usually, `Suspended` event is sent only on platforms that supports suspension, such as Android.
Suspension happens when you switch to another application on your smartphone. Other supported platforms
do not support suspension, so this event is not sent on them.

Keep in mind, that when the engine is just initialized there's no graphics context at all, so you
can't access it (for example, to change the window title). Instead, you have to use 
`Plugin::on_graphics_context_initialized` method to do this, or check if the graphics context is alive in
the game loop and do required actions.

## Interaction with Plugins

There's very clear interaction between plugins and graphics context. There are two plugin methods that 
will be called when the graphics context is either created or destroyed - `on_graphics_context_initialized` 
and `on_graphics_context_destroyed`:

```rust,no_run
{{#include ../code/snippets/src/engine/graphics_context.rs:graphics_context}}
```

You can also do a checked borrow of the graphics context at any place in a plugin. For example, the following
code tries to fetch current graphics context, and if it succeeds, then it prints current FPS in the window 
title:

```rust,no_run
{{#include ../code/snippets/src/engine/graphics_context.rs:update}}
```

## Window

See the next chapter to learn more how to interact with the main application window.