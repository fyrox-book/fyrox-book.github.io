# Executor 

Executor is a simple wrapper that drives your game plugins, it is intended to be used for production builds of your game.
The editor runs the executor in a separate process when you're entering the play mode. Basically, there is no significant 
difference between running the game from the editor or running it as a separate application. The main difference is that
the editor passes `scene_path` parameter for the executor when entering the play mode.

## Motivation

Why there's a need for such a thing like executor? The first reason was already highlighted - game process isolation
during the game development. The next major reason is the fact that your game can be run on a number of different platforms.
If you [check the list of supported platforms](../introduction/requirements.md#supported-platforms), you'll understand
why there's such a need. Each platform has its own way of how the native code is processed. For example, on WebAssembly
the game code is compiled into an WASM module which is then loaded by a web browser and executed. This platform requires
the native code to be compiled into a "dynamic library", while, for instance, the PC builds can be compiled directly 
into an executable file. Android, for instance, requires the game to be compiled into a shared library (`.so`) and then 
loaded by a bunch of "glue" code and executed. 

## Usage

Executor is meant to be a part of your project's workspace, its typical look could something like this:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{pool::Handle, uuid::Uuid},
#     engine::executor::Executor,
#     plugin::{Plugin, PluginConstructor, PluginContext},
#     scene::{Scene},
# };
# struct GameConstructor;
# impl PluginConstructor for GameConstructor {
#     fn create_instance(
#         &self,
#         _scene_path: Option<&str>,
#         _context: PluginContext,
#     ) -> Box<dyn Plugin> {
#         todo!()
#     }
# }
fn main() {
    let mut executor = Executor::new();
    // Register your game constructor here.
    executor.add_plugin_constructor(GameConstructor);
    executor.run()
}
```

The executor has full access to the engine, and through it to the main application window. You can freely change desired
parts, `Executor` implements `Deref<Target = Engine> + DerefMut` traits, so you can use its instance as an "alias"
to engine instance. 

To add a plugin to the executor, just use `add_plugin_constructor` method, it accepts any entity that implements
`PluginConstructor` traits.

## Typical Use Cases

This section covers typical use cases for the `Executor`.

### Setting Window Title

You can set window title when creating executor instance:

```rust,no_run
{{#include ../code/snippets/src/engine/executor.rs:executor_window_title}}
```