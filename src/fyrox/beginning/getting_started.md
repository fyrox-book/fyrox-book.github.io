# Getting started

Despite the look of it, the engine is quite friendly to newcomers, especially with some good guides. This section
of the book will guide you the basics of the engine.

## Installation

Since the engine is distributed as a library, it has to be added to `[dependecies]` section of `Cargo.toml` of 
your project to start using it, there is no pre-built executables or library files. If you don't know what is
`Cargo.toml` or `cargo`, please [read this](https://doc.rust-lang.org/cargo/) before continue. 

### Using stable version

To begin using the engine, just add following lines to your `Cargo.toml`:

```toml
[dependencies]
fyrox = "^0.25"
```

This will force Cargo to use latest **stable** version of the engine. 

### Using latest unstable version

Sometimes you want to use the latest features that are not yet released in a stable version, then you can use
the engine directly from its repository, simply add the following line to your `Cargo.toml`:

```toml
[dependencies]
fyrox = { git = "https://github.com/FyroxEngine/Fyrox" } 
```

This will update your build files fairly often. If you don't want game breaking changes to occur because of engine updates or want to not compile daily,
change `fyrox = { git = "https://github.com/FyroxEngine/Fyrox" }` to `fyrox = { git = "https://github.com/FyroxEngine/Fyrox", rev = "" }` with the commit
hash in the quotes.

## Editor installation

The engine offers an editor, to install a standalone version it, use `cargo install`:

```shell
cargo install fyroxed
```

Standalone version does not allow you to run your game inside it, but only allows you to edit scenes. 

After that you can run the editor with a single command:

```shell
fyroxed
```

Check the next chapter to create a simple application.
