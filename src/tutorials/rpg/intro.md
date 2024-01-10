# Role-Playing Game Tutorial

This tutorial starts the series of tutorials about writing a role-playing game in Rust using Fyrox game engine. Strangely,
but Fyrox has a reputation of an engine for 3D shooters. In this series I'll try to prove that it is a general purpose
game engine.

## Source Code

Source code for the entire tutorial is [available here](https://github.com/fyrox-book/fyrox-book.github.io/tree/main/src/code/tutorials/rpg).

## Engine Version

> ⚠️ Keep in mind that this tutorial uses specific version of the engine. You need to set this to your root `Cargo.toml` as
> dependencies to ensure that it will compile:

```toml
[workspace.dependencies.fyrox]
git = "https://github.com/FyroxEngine/Fyrox"
rev = "47b79d4acaf11b755d279ccb36de0f2ffd08172f"
[workspace.dependencies.fyroxed_base]
git = "https://github.com/FyroxEngine/Fyrox"
rev = "47b79d4acaf11b755d279ccb36de0f2ffd08172f"
```

Also make sure to install `fyrox-template` directly from the engine repo using the following command:

```shell
cargo install fyrox-template --git=https://github.com/FyroxEngine/Fyrox --rev 47b79d4acaf11b755d279ccb36de0f2ffd08172f
```