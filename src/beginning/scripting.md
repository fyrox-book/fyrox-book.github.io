# Editor, Plugins and Scripts

Every Fyrox game is just a plugin for both the engine and the editor, such approach allows the game to run from the 
editor and be able to edit the game entities in it. A game can define any number of scripts, which can be assigned 
to scene objects to run custom game logic on them. This chapter will cover how to install the engine with its platform-
specific dependencies, how to use the plugins and scripting system, how to run the editor.


## Platform-specific Dependencies

Before starting to use the engine, make sure all required platform-specific development dependencies are installed. If 
using Windows or macOS, no additional dependencies are required other than the latest Rust installed with appropriate 
toolchain for your platform.

### Linux

On Linux, Fyrox needs the following libraries for development: `libxcb-shape0`, `libxcb-xfixes0`, `libxcb1`, 
`libxkbcommon`, `libasound2` and the `build-essential` package group.

For Debian based distros like Ubuntu, they can be installed like below:

```shell
sudo apt install libxcb-shape0-dev libxcb-xfixes0-dev libxcb1-dev libxkbcommon-dev libasound2-dev build-essential
```

For NixOS, you can use a `shell.nix` like below:

```nix
{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell rec {
  nativeBuildInputs = with pkgs.buildPackages; [
    pkg-config
    xorg.libxcb
    alsa-lib
    wayland
    libxkbcommon
    libGL
  ];

  shellHook = with pkgs.lib; ''
    export LD_LIBRARY_PATH=${makeLibraryPath nativeBuildInputs}:/run/opengl-driver/lib:$LD_LIBRARY_PATH
  '';
}
```

## Quick Start

Run the following commands to start using the editor as quickly as possible.

```sh
cargo install fyrox-template
fyrox-template init --name fyrox_test --style 2d
cd fyrox_test
cargo run --package editor --release
```

## Project Generator

Fyrox plugins are written in Rust, this means that if the source code of the game changes one must recompile. 
This architecture requires some boilerplate code. Fyrox offers a special tiny command line tool - `fyrox-template`. It 
helps generate all this boilerplate with a single command. Install it by running the following command:

```shell
cargo install fyrox-template
```

_Note for Linux:_ This installs it in `$user/.cargo/bin`. If receiving errors about the `fyrox-template` command not  
being found, add this hidden cargo bin folder to the operating systems `$PATH` environment variable.

Now, navigate to the desired project folder and run the following command:

```shell
fyrox-template init --name my_game --style 3d
```

Note that unlike `cargo init`, this will create a new folder with the given name.

The tool accepts two arguments - a project name (`--name`) and a style (`--style`), which defines the contents of the default
scene. After initializing the project, go to `game/src/lib.rs` - this is where the game logic is located, as you can 
see, the `fyrox-template` generated quite a bit of code for you. There are comments explaining what each place is for. For 
more info about each method, please refer [to the docs](https://docs.rs/fyrox/latest/fyrox/plugin/trait.Plugin.html).

Once the project is generated, memorize the two commands that will help run your game in different modes:

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

### Automatic

> ⚠️ `fyrox-template` has special sub-command - `upgrade` to quickly upgrade to desired engine version. To upgrade to 
> the latest version (`nightly`) you should execute `fyrox-template upgrade --version nightly` command in your game's 
> directory.

There are three main variants for `--version` switch:

- `nightly` - uses latest nightly version of the engine from GitHub directly. This is the preferable version if you want
to use the latest changes and bug fixes as they release.
- `latest` - uses latest stable version of the engine. This option also supports `--local` key, that sets the path to
the engine to `../Fyrox/fyrox` and the editor to `../Fyrox/editor`. Obviously, such path requires the engine to be located
in the parent directory of your project. This option could be useful if you want to use custom version of the engine 
(for example, if you're developing a patch for the engine).
- `major.minor.patch` - uses specific stable version from crates.io (`0.30.0` for example).

### Manual

Engine version can also be updated manually. The first step to take is to install the latest `fyrox-template`, this can be done
with a single `cargo` command:

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
