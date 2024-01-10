# First-person Shooter Tutorial

This tutorial series will guide your through a process of creation a simple 3D shooter, that will have basic 
character controller, weapons, projectiles, bots, animation, and simple AI.

Keep in mind, that every tutorial part expects that you've read every previous part. It is needed to not explain
all required actions over and over again.

Source code for the entire tutorial is [available here](https://github.com/fyrox-book/fyrox-book.github.io/tree/main/src/code/tutorials/fps).

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

In other words, it uses specific commit from the engine repository. It is required, because the engine is still in 
pre 1.0 version and can contain small-to-big breaking changes here and there.