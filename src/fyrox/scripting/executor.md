# Executor 

Executor is a simple wrapper that drives your game plugins, it is intended to be used for production builds of your game.

## Usage

Executor is meant to be a part of your project's workspace, its typical look could something like this:

```rust,no_run,compile_fail
# extern crate fyrox;
use fyrox::engine::executor::Executor;
use your_game::Game;

fn main() {
    let mut executor = Executor::new();
    executor.add_plugin(Game::new());
    executor.run()
}
```

Executor has full access to the engine, and through it to the main application window. You can freely change desired
parts, `Executor` implements `Deref<Target = Engine> + DerefMut` traits, so you can use its instance as an "alias"
to engine instance. 

To add a plugin to the executor, just use `add_plugin` method, it accepts any entity that implements `Plugin` + 
`TypeUuidProvider` traits.

## Typical Use Cases

This section covers typical use cases for the `Executor`.

### Setting Window Title

You can set window title by accessing window instance and calling `set_title`:

```rust
# extern crate fyrox;
# use fyrox::engine::executor::Executor;
# let mut executor = Executor::new();
executor.get_window().set_title("My Game");
```