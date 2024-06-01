# Scene

Scene is a container for game entities. Currently, scenes in the engine manage following entities:

1) Graph
2) Animations
3) Physics (rigid bodies, colliders, joints)
4) Sound

Scene allows you to create isolated "world" which won't interact with other scenes, it is very useful for many
more or less complex games.

## How to create

A scene could be created either in FyroxEd or programmatically. You can also combine both approaches, where
you build all "static" content in the editor and adding rest of the entities (bots, interactive objects, etc.)
manually by instantiating respective prefabs at runtime.

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

1) `on_scene_begin_loading` - is called when a scene is just began to load. Keep in mind, that async scene loader
could load multiple scenes at once and this method is guaranteed to be called exactly before the scene is started
to load.
2) `on_scene_loading_failed` - is called when a scene is failed to load. This method could be useful if you're using
non-verified scenes (i.e. from game mods) and want to react somehow when the scene is failed to load.

### Create scene manually

A scene could also be created manually:

```rust,no_run
{{#include ../code/snippets/src/scene/mod.rs:create_scene}}
```

See respective node builders [docs](../scene/graph.md#using-node-builders) to populate the scene.

## Where all my scenes located?

All scenes "lives" in the engine, the engine has ownership over your scene after you've added it in the engine.
You can borrow a scene at any time using its handle and do some changes:

```rust,no_run
{{#include ../code/snippets/src/scene/mod.rs:scene_borrowing}}
```

## Building scene asynchronously

You can create your scene in separate thread and then pass it to main thread to insert it in the engine. Why this
is needed? Remember the last time you've played a relatively large game, you've probably noticed that it have
loading screens and loading screen has some fancy interactive stuff with progress bar. Loading screen is fully
responsive while the game doing hard job loading the world for you. Got it already? Asynchronous scene loading is
needed to create/load large scenes with tons of resources without blocking main thread, thus leaving the game
fully responsive.

## Managing multiple scenes

Usually you should have only one scene active (unless you're making something very special), you should use
`.enabled` flag of a scene to turn it off or on. Deactivated scenes won't be rendered, the physics won't be
updated, the sound will stop, and so on. In other words the scene will be frozen. This is useful for situations
when you often need to switch between scenes, leaving other scene in frozen state. One of the examples where this
can be useful is menus. In most games when you're entering the menu, game world is paused.

## Ambient lighting

Every scene has default ambient lighting, it is defined by a single RGB color. By default, every scene has
some pre-defined ambient lighting, it is bright enough, so you can see your objects. In some cases you may
need to adjust it or even make it black (for horror games for instance), this can be achieved by a single
line of code:

```rust,no_run
{{#include ../code/snippets/src/scene/mod.rs:set_ambient_lighting}}
```

Please keep in mind that ambient lighting does not mean global illumination, it is a different lighting technique
which is not available in the engine yet.
