# Getting started

Despite the look of it, the engine is quite friendly to newcomers, especially with some good guides. This section
of the book will guide you the basics of the engine.

To begin using the engine, just add following lines to your `Cargo.toml`:

```toml
[dependencies]
rg3d = { git = "https://github.com/rg3dengine/rg3d" }
```

This will update your build files fairly often. If you dont want game breaking changes to occur because of engine updates or want to not compile daily,
change `rg3d = { git = "https://github.com/rg3dengine/rg3d" }` to `rg3d = { git = "https://github.com/rg3dengine/rg3d", rev = "" }` with the commit
hash in the quotes.

Check the next chapter to create a simple application.
