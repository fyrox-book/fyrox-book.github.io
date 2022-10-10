# Introduction to Fyrox

Fyrox is a feature-rich, general purpose game engine that is suitable for any kind of games. It is capable to power
games with small- or medium-sized worlds, large-sized world most likely will require some manual work. 

Games made with the engine are capable to run on desktop platforms (PC, Mac, Linux) and Web (WebAssembly). Mobile is
planned for future releases.

## What can the engine do?

You can create pretty much any kind of game or interactive applications. Here's some examples of what the engine can 
do:

![Station Iapetus](game_example1.jpg)
![Fish Folly](game_example2.jpg)
![2D Platformer](game_example3.jpg)

## How the engine work?

The engine consists of two parts that you'll be actively using: the framework and the editor. The framework is a 
foundation of the engine, it manages rendering, sound, scripts, plugins, etc. While the editor contains lots of tools 
that can be used to create game worlds, manage assets, edit game objects, scripts and more.

![Fish Folly](editor.png)

## Programming languages

Everything of your game can be written entirely in Rust, utilizing its safety guarantees as well as speed. However, it
is possible to use any scripting language you want, but that's have no built-in support and you need to implement this
manually.