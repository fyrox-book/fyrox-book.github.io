# Scene

Scene is a container for game entities. Scene allows you to create an isolated "world" which won't 
interact with other scenes, it is very useful for many more or less complex games. 

What game scenes cannot handle is user interface entities, they're handled by a separate scene class, 
so-called UI scene. UI scenes are discussed in the [respective chapter](../ui/ui.md). UI can still 
be incorporated in the game scene, see [this chapter](../ui/rendering.md) for more info.

## How to create

A scene could be created either in the editor or programmatically. You can also combine both approaches, where
you build all "static" content in the editor and adding rest of the entities (bots, interactive objects, etc.)
manually by instantiating the respective prefabs at runtime.

### Using FyroxEd

There is a [separate chapter](../beginning/editor_overview.md) in the book that should help you to create a
scene. After a scene is created, you can load it using async scene loader:

```rust,no_run
{{#include ../code/snippets/src/scene/mod.rs:load_scene}}
```

The code is quite straightforward. At first, we're using async scene loader to create a scene loading request.
This request will be processed in a separate thread, leaving your game fully responsible while the scene is loading.
Next, when the scene is fully loaded and added to the engine, `on_scene_loaded` method is called. Usually there's 
only one active scene, so we're removing the previous one and setting the new one as active.

There are two additional methods:

1) `on_scene_begin_loading` - is called when a scene is just began to load. Keep in mind that async scene loader
could load multiple scenes at once, and this method is guaranteed to be called exactly before the scene is started
to load.
2) `on_scene_loading_failed` - is called when a scene is failed to load. This method could be useful if you're using
non-verified scenes (i.e. from game mods) and want to react somehow when the scene is failed to load.

### Create scene manually

A scene could also be created manually, the following code creates a cube and a camera to visualize the cube:

```rust,no_run
{{#include ../code/snippets/src/scene/mod.rs:create_scene}}
```

See the respective node builders [docs](../scene/graph.md#using-node-builders) to populate the scene.

## Where are all my scenes located?

All scenes "live" in the engine, the engine has ownership over your scene after you've added it to the engine.
You can borrow a scene at any time using its handle and do some changes to it and its nodes:

```rust,no_run
{{#include ../code/snippets/src/scene/mod.rs:scene_borrowing}}
```

## Building a scene asynchronously

You can create your scene in a separate thread and then pass it to the main thread to insert it in the engine. Why this
is needed? Remember the last time you've played a relatively large game, you've probably noticed that it has 
loading screens and loading screen has some fancy interactive stuff with progress bar. Loading screen is fully
responsive while the game is doing a hard job loading the world for you. Got it already? Asynchronous scene loading is
needed to create/load large scenes with tons of resources without blocking the main thread, thus leaving the game
fully responsive.

## Managing multiple scenes

Usually you should have only one scene active (unless you're making something very special), you should use
`.enabled` flag of a scene to turn it off or on. Deactivated scenes won't be rendered, the physics won't be
updated, the sound will stop, and so on. In other words, the scene will be frozen. This is useful for situations
when you often need to switch between scenes, leaving other scenes in frozen state. One of the examples where this
can be useful is menus. In most games when you're entering the menu, game world is paused.

If multiple scenes are enabled, they'll be rendered in the same order as they were created. Multiple active 
scenes have very limited use.


