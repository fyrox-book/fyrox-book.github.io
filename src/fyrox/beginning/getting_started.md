# Getting started

Despite the look of it, the engine is quite friendly to newcomers, especially with some good guides. This section
of the book will guide you through the basics of the engine. Fyrox 0.25+ changed game development paradigm - you
should use the editor (FyroxEd) to make your game, like in many other game engines.

## Engine vs Framework

There are two distinct concepts in game development - engine and framework, but they're often "mixes" and people gets
confused what is framework and what is engine. Fyrox is full-featured game **engine**, what does that mean and why it
is not a framework? Key features that allow you to understand that you're looking at game engine are following:

- Editor - true game engine provides integrated game development environment, that allows you to run your game from
it, tweak parameters of your game entities, etc.
- Assets pipeline - engine has pretty rigid assets processing pipeline. 
- Standard programming patterns - engine "forces" you to use it in almost single way, there is pretty much no space 
for maneuver.

## Obsolete "Fyrox as Framework"

Before 0.25, Fyrox was just a framework (yet it still had an editor), you can still use it in old way (do manual
initialization, do not use editor, etc.), but it is considered obsolete and eventually "framework mode" won't be 
supported. The book has few chapters that are marked `Obsolete`, this means that they're not advised to be used.

Read next chapter to learn how to use the engine in a "modern" way. 

## API Documentation

The book is primarily focused on game development with Fyrox, not on its API. You can find API docs 
[here](https://docs.rs/fyrox/latest/fyrox/). 

