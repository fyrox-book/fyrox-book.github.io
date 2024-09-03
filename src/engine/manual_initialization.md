# Manual Engine Initialization

It is possible to initialize the engine manually and have custom game logic loop. It could be done something like this:

```rust,no_run
{{#include ../code/snippets/src/game/mod.rs:custom_game_loop}}
```

Keep in mind, that this code does **NOT** have plugins nor editor support. It is just barebones engine without anything
attached to it. If you still need the editor and plugins, but don't like the built-in game loop or initialization routines,
read the next section.

## Custom Executor

The recommended way of using the engine is to generate a project, that has an editor and a bunch of platform-dependent 
executor crates. If the built-in executor do not have features you need, or have some issues you know how to fix, you
can create custom executor. All you need to do is to copy [the built-in one](https://github.com/FyroxEngine/Fyrox/blob/master/fyrox-impl/src/engine/executor.rs),
add it your module tree and modify as you like. Then you need to replace all usages of built-in one with your custom one
and that's pretty much it - now you have full control in engine initialization and game loop.