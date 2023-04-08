# Editor, Plugins and Scripts

Every Fyrox game is just a plugin for both the engine and the editor, such approach allows you to run your game from the 
editor and to be able to edit the game entities in it. Your game can define any number of scripts, which can be assigned 
to scene objects to run custom game logic on them. In this chapter you'll learn the basics of the plugins and scripting system,
as well as how to run the editor.

## Project Generator

Fyrox plugins are static, this means that you must re-compile your game or editor if the source code of your game changes,
such architecture requires some boilerplate code for any game. Fyrox offers a special tiny tool - 
`fyrox-template` - that helps you generate all this boilerplate with a single command. Install it by running the following 
command:

```shell
cargo install fyrox-template
```

_Note for Linux:_ This installs it in `$user/.cargo/bin`. If you get errors about the `fyrox-template` command not found 
then you need to add this folder to your `$PATH` still.

Navigate to the folder where you want the project to be created and run the following command:

```shell
fyrox-template init --name my_game --style 3d
```

Note that unlike `cargo init`, this will create a new folder with the given name.

The tool accepts two arguments - a project name (`--name`) and a style (`--style`), which defines the contents of the default
scene. Once you initialize your project, go to `game/src/lib.rs` - this is where your game logic is located, as you can 
see, the `fyrox-template` generated quite a bit of code for you. There are comments explaining what each place is for. For 
more info about each method, please refer [to the docs](https://docs.rs/fyrox/latest/fyrox/plugin/trait.Plugin.html).

Once the project is generated, you should memorize the two commands that will help you to run your game in different modes:

- `cargo run --package editor --release` - launches the editor with your game attached. The editor allows you to run your game
  from it and edit its game entities. It is intended to be used only for development.
- `cargo run --package executor --release` - creates and runs the production binary of your game, which can be shipped (for
  example - to a store).

Navigate to your project's directory and run `cargo run --package editor --release`, after some time you should see the 
editor:

![editor](editor.png)

In the editor you can start building your game scene. **Important note:** your scene must have at least one camera,
otherwise you won't see a thing. Read the next chapter to learn how to use the editor.

## Using the Latest Engine Version

Due to the nature of the software development, some bugs will inevitably sneak into the major releases, due to this, 
you may want to use the latest engine version from the repository on GitHub, since it is the most likely to have bugs fixed
(you can also contribute by fixing any bugs you find or at least, by [filing an issue](https://github.com/FyroxEngine/Fyrox/issues)).

> ⚠️ Latest `fyrox-template` from the engine's repo has special sub-command - `upgrade` to quickly upgrade to desired engine
> version. To upgrade to the latest version ("nightly") you should execute `fyrox-template upgrade --version nightly` 
> command in your game's directory.

The first step you need to take is to install the latest `fyrox-template`, this can be done with a single `cargo` command:

```shell
cargo install fyrox-template --force --git https://github.com/FyroxEngine/Fyrox
```

This will ensure you're using the latest project/script template generator, which is important, since old versions
of the template generator will most likely generate outdated code, no longer be compatible with the engine.

To switch existing projects to the latest version of the engine, you need to specify paths pointing to the remote repository 
for the `fyrox` and `fyroxed_base` dependencies. You need to do this in the `game`, `executor`, and `editor` projects. First,
open `game/Cargo.toml` and change the `fyrox` dependency to the following:

```toml
[dependencies]
fyrox = { git = "https://github.com/FyroxEngine/Fyrox" }
```

Do the same for `executor/Cargo.toml`. The `editor` has two dependencies we need to change: `fyrox` and `fyroxed_base`.
Open the `editor/Cargo.toml` and set both dependencies to the following:

```toml
[dependencies]
fyrox = { git = "https://github.com/FyroxEngine/Fyrox" }
fyroxed_base = { git = "https://github.com/FyroxEngine/Fyrox" }
```

Now your game will use the latest engine and editor, but beware - new commits could bring some API breaks. You can avoid these by 
specifying a particular commit, just add `rev = "desired_commit_hash"` to every dependency like so:

```toml
[dependencies]
fyrox = { git = "https://github.com/FyroxEngine/Fyrox", rev = "0195666b30562c1961a9808be38b5e5715da43af" }
fyroxed_base = { git = "https://github.com/FyroxEngine/Fyrox", rev = "0195666b30562c1961a9808be38b5e5715da43af" }
```

To bring a local git repository of the engine to being up-to-date, just call `cargo update` at the root of the project's
workspace. This will pull the latest changes from the remote, unless there is no `rev` specified.

Learn more about dependency paths on the official `cargo` documentation, 
[here](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories).

## Adding Game Logic

Any object-specific game logic should be added using scripts. A script is a "container" for data and code, that will be
executed by the engine. Read the [Scripts](../scripting/script.md) chapter to learn how to create, edit, and use scripts in
your game.
