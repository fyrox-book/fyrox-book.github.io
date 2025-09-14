# Game Logic

A game based on Fyrox is a plugin to the engine and the editor. The plugin defines global application logic and can provide
a set of scripts, that can be used to assign custom logic to scene nodes. Every script can be attached to only one 
plugin. All of these entities "live" in the engine, which is initialized from one of platform-specific executors. It
is some sort of OS-dependent entry point of your game. Read the [respective chapter](executor.md) for more info.

Fyrox uses scripts to create custom game logic, scripts can be written only in Rust which ensures that your game will
be crash-free, fast and easy to refactor.

The overall structure of plugins and scripts could be described in this diagram:

![structure](structure.svg)

The next chapters will cover all parts and will help you to learn how to use plugins and scripts correctly.