# Installation and Project Creation

Fyrox is a compiled game engine, this means that your game needs to be compiled to native code before it can be run. Every 
Fyrox game is just a plugin for both the engine and the editor.  This approach allows the game to run from the 
editor and enables editing of the game entities from within it. A game can define any number of scripts, which can be assigned 
to scene objects to run custom game logic on them. This chapter will cover how to install the engine with its 
platform-specific dependencies, how to use the plugins and scripting system, and how to run the editor.

## Platform-specific Dependencies

Before starting to use the engine, make sure all required platform-specific development dependencies are installed. If 
using Windows or macOS, no additional dependencies are required other than the [latest Rust installed](https://rustup.rs)
with the appropriate toolchain for your platform.

### Linux

On Linux, Fyrox needs the following libraries for development: `libxcb-shape0`, `libxcb-xfixes0`, `libxcb1`, 
`libxkbcommon`, `libasound2`, `libegl-mesa0` and the `build-essential` package group.

For Debian based distros like Ubuntu, they can be installed like below:

```shell
sudo apt install libxcb-shape0-dev libxcb-xfixes0-dev libxcb1-dev libxkbcommon-dev libasound2-dev libegl-mesa0 build-essential
```

For NixOS, add a file named `flake.nix` to the root of your repository with the following contents, add it to the git 
index (e.g., with `git add flake.nix`), and then run `nix develop` to open a shell with all the required dependencies.

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    ...
  }:
  let
    overlays = [
      (import rust-overlay)
    ];

    systems = [
      "x86_64-linux"
      "aarch64-linux"
    ];

    forAllSystems = f:
      nixpkgs.lib.genAttrs systems
      (system: f { pkgs = import nixpkgs { inherit system overlays; }; });
  in
  {
    devShells = forAllSystems ({ pkgs }: with pkgs; {
      default = mkShell rec {
        buildInputs = [
          rust-bin.stable.latest.default

          pkg-config
          xorg.libxcb
          alsa-lib
          wayland
          libxkbcommon
          libGL
        ];
        LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
      };
    });
  };
}
```

## Project Manager

![project manager](https://fyrox.rs/assets/0.36/project_manager.png)

Project manager is a part of the engine that allows you to manage multiple projects made with the engine all at once.
It allows you to create a new project or import an existing one, run the project or edit it in the editor, 
upgrade the project to a selected version of the engine, and many more options. 

[Download the project manager](https://fyrox.rs/download.html) for your OS and run it. Then click the `+Create`
button, select the path where you want the project to be located and click `Create`. Select the new project
in the list and click `Edit` button to run the editor.

## Quick Start Using Console Commands

Run the following commands to start using the editor as quickly as possible.

```sh
cargo install fyrox-template
fyrox-template init --name fyrox_test --style 2d
cd fyrox_test
cargo run --package editor --release
```

## Project Generator

> ⚠️ This section is mostly for console users and those who like building their software from source code.
> If you prefer to use Project Manager you can, it does the same as `fyrox-template` but with the benefits of a GUI.

Fyrox plugins are written in Rust, this means that if the source code of the game changes you will need to recompile. 
The architecture requires some boilerplate code. Fyrox offers a special tiny command line tool - `fyrox-template`. This 
helps generate all the boilerplate code with a single command. Install it by running the following command:

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
scene. After initializing the project, go to `game/src/lib.rs` - this is where the game logic is located, as you will 
see, the `fyrox-template` generates quite a bit of code for you. The code is decorated with comments explaining what each place is for. For 
more info about each method, please refer [to the docs](https://docs.rs/fyrox/latest/fyrox/plugin/trait.Plugin.html).

Once the project is generated, two commands can be used to run your game in different modes:

- `cargo run --package editor --release` - launches the editor with your game attached. The editor allows you to run your game
  from it and edit its game entities. This is intended to be used only for development purposes.
- `cargo run --package executor --release` - creates and runs the production binary of your game. This command generates executables which can be shipped (for example - to a store).

Navigate to your project's directory and run `cargo run --package editor --release`, after a short while, you should see the 
editor:

![editor](editor.png)

In the editor you can start building your game scene. **Important note:** your scene must have at least one camera,
otherwise you won't see a thing. Read the next chapter to learn how to use the editor.

## Using the Latest Engine Version

Due to the nature of software development, bugs will inevitably sneak into the major releases, due to this, 
it is recommended to always use the latest engine version from the repository on GitHub. This is most likely to have bugs fixed
(you can also contribute by fixing any bugs you find or at least, by [filing an issue](https://github.com/FyroxEngine/Fyrox/issues)).

### Automatic

> ⚠️ `fyrox-template` has a special sub-command - `upgrade` to quickly upgrade to a desired engine version. To upgrade to 
> the latest version (`nightly`) you should execute `fyrox-template upgrade --version nightly` command in your game's 
> directory.

There are three main variants for `--version` switch:

- `nightly` - uses latest nightly version of the engine from GitHub directly. This is the preferable version if you want
to use the latest changes and bug fixes as they are released.
- `latest` - uses latest stable version of the engine. This option also supports `--local` key, that sets the path to
the engine to `../Fyrox/fyrox` and the editor to `../Fyrox/editor`. Obviously, this path requires the engine to be located
in the parent directory of your project. This option could be useful if you want to use a custom version of the engine 
(for example, if you're developing a patch for the engine).
- `major.minor.patch` - uses a specific stable version from crates.io (`0.30.0` for example).

### Manual

The Engine version can also be updated manually. The first step is to install the latest `fyrox-template`, this can be done
with a single `cargo` command:

```shell
cargo install fyrox-template --force --git https://github.com/FyroxEngine/Fyrox
```

This will ensure you're using the latest project/script template generator, which is very important; old versions
of the template generator will most likely generate outdated code, which is no longer compatible with the engine.

To switch existing projects to the latest version of the engine, you will need to specify paths pointing to the remote repository 
for the `fyrox` and `fyroxed_base` dependencies. All you need to do is to change paths to these dependencies in the 
root `Cargo.toml`:

```toml
[workspace.dependencies.fyrox]
version = { git = "https://github.com/FyroxEngine/Fyrox" }
default-features = false
[workspace.dependencies.fyroxed_base]
version = { git = "https://github.com/FyroxEngine/Fyrox" }
```

Now your game will use the latest engine and editor, but beware - new commits may surface some API mis-matches. You can avoid 
these by specifying a particular commit, just add `rev = "desired_commit_hash"` to every dependency like so:

```toml
[dependencies]
[workspace.dependencies.fyrox]
version = { git = "https://github.com/FyroxEngine/Fyrox", rev = "0195666b30562c1961a9808be38b5e5715da43af" }
default-features = false
[workspace.dependencies.fyroxed_base]
version = { git = "https://github.com/FyroxEngine/Fyrox", rev = "0195666b30562c1961a9808be38b5e5715da43af" }
```

To bring a local git repository of the engine to the latest version, just call `cargo update` at the root of the project's
workspace. This will pull the latest changes from the remote, unless there is no `rev` specified.

Learn more about dependency paths on the official `cargo` documentation, [here](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories).

## Adding Game Logic

Any object-specific game logic should be added using scripts. A script is a "container" for data and code, that will be
executed by the engine. Read the [Scripts](../scripting/script.md) chapter to learn how to create, edit, and use scripts in
your game.
