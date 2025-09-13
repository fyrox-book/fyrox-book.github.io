# Installation and Project Creation

Fyrox is a compiled game engine, this means that your game needs to be compiled to native code before it can be run.
This fact requires a specific project structure which is generated when you're making a new project. This project
consists of a few Rust crates (game, editor, executors for each platform). Every Fyrox game is just a plugin for both
the engine and the editor crates. This approach allows the game to run from the editor and enables editing of the game
entities from within it, or just run the game without the editor (since not every game needs an editor). This chapter
will cover how to install the engine with its platform-specific dependencies, how to use the plugins and scripting
system, and how to run the editor.

## Platform-specific Dependencies

Before starting to use the engine, make sure all required platform-specific development dependencies are installed. If
using Windows or macOS, no additional dependencies are required other than
the [latest Rust installed](https://rustup.rs) with the appropriate toolchain for your platform.

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

[Download the project manager](https://fyrox.rs/download.html) for your OS from the website or install it using
`cargo install fyrox-project-manager` and run it.
Then click the `+Create` button, select the path where you want the project to be located and click `Create`. Select the
new project in the list and click `Edit` button to run the editor.

You project needs to be compiled from scratch before it can be run, it may take some time, usually it takes up to 10
minutes on a CPU with 4 cores (8 core CPU will compile the engine in just 5 minutes or so). Next runs of the editor will
only compile your game, which usually takes a few seconds.

## Adding Game Logic

Any object-specific game logic should be added using scripts. A script is a "container" for data and code, that will be
executed by the engine. Read the [Scripts](../scripting/script.md) chapter to learn how to create, edit, and use scripts
in your game.
