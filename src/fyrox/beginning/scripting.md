# Editor, Plugins and Scripts

Every Fyrox game is just a plugin for both the engine and the editor, such approach allows you to run your game from the 
editor and be able to edit game entities in it. Your game can define any number of scripts, which can be assigned 
to scene objects to run custom game logic on them. In this chapter you'll learn basics of plugins and scripting system,
also you'll learn how to run the editor.

## Project Generator

Fyrox plugins are static, this means that you must re-compile your game or editor if you change source code of your game,
such architecture requires some boilerplate code that is generic for every game. Fyrox offers special tiny tool - 
`fyrox-template` - it helps you to generate all boilerplate parts in a single command. Install it using the following 
command:

```shell
cargo install fyrox-template
```

For Linux, you may need to specify installation directory explicitly, because `cargo` puts binaries into `/usr/.cargo/bin`
which may not be in `PATH`. You can either register the previous path in `PATH` environment variable, or directly
specify the location that is already in path:

```shell
cargo install fyrox-template --root /usr/bin
```

Navigate to a folder where you want the project to be created and do the following command:

```shell
fyrox-template init --name my_game --style 3d
```

The tool accepts two arguments - a project name (`--name`) and a style (`--style`) which defines the contents of default
scene. Once you initialized your project, go to `game/src/lib.rs` - it is where your game logic is located, as you can 
see the `fyrox-template` generated quite some code for you. There are tiny comments about which place is for what. For 
more info about each method, please refer [to the docs](https://docs.rs/fyrox/latest/fyrox/plugin/trait.Plugin.html).

After the project is generated, you should memorize two commands that will help you to run your game in different modes:

- `cargo run --package editor --release` - launches the editor with your game attached, the editor allows you to run your game
  from it and edit game entities. It is intended to be used only for development.
- `cargo run --package executor --release` - creates and runs the production binary of your game that can be shipped (for
  example - to a store).

Navigate to your project's directory and run `cargo run --package editor --release`, after some time you should see the 
editor:

![editor](editor.png)

In the editor you can start making your game scene. **Important note:** your scene must have at least one camera,
otherwise you won't see anything. Read the next chapter to learn how to use the editor.

## Adding Game Logic

Any object-specific game logic should be added using scripts. Script is a "container" for data and code, that will be
executed by the engine. Read [Scripts](../scripting/script.md) chapter to learn how to create, edit, and use scripts in
your game