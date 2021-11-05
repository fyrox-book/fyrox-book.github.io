# Scene 

Scene is a container for game entities. Currently, scenes in the engine manage following entities:

1) Graph
2) Animations
3) Physics (rigid bodies, colliders, joints)
4) Sound 

Scene allows you to create isolated "world" which won't interact with other scenes, it is very useful for many
more or less complex games.

## How to create

A scene could be created either in rusty-editor or programmatically. You can also combine both approaches, where
you build all "static" content in the editor and adding rest of the entities (bots, interactive objects, etc.)
manually.

### Using rusty-editor

There is a [separate chapter](../../rusty-editor/introduction.md) in the book that should help you to create a scene.

After a scene is created, you can load it as any other 3D model using the resource manager:

```rust
# extern crate rg3d;
# use rg3d::{
#     core::{futures::executor::block_on, pool::Handle},
#     engine::resource_manager::{MaterialSearchOptions, ResourceManager},
#     scene::{node::Node, Scene},
# };
# use std::path::Path;

fn load_scene(resource_manager: ResourceManager) -> Scene {
    // Create parent scene.
    let mut scene = Scene::new();

    // Request child scene and block until it loading.
    let scene_resource = block_on(
        resource_manager
            .request_model("path/to/your/scene.rgs", MaterialSearchOptions::RecursiveUp),
    )
        .unwrap();

    // Create an instance of the scene in the parent scene.
    let child_scene = scene_resource.instantiate_geometry(&mut scene);

    scene
}
```

Please note that here we're creating an empty scene and only then instantiating another scene into it. Why is this
needed? 

**Short answer:** child scene is considered as prefab, and it is "instantiated" in the parent scene. Considering 
it as prefab allows you modifying your scene separately and serialization/deserialization will be able to correctly
apply any changes in the scene.

**Long answer:** the engine has a prefab system which allows you to build hierarchical scenes which can include any
number of other scenes as child scenes. Child scenes can have their own child scenes and so on. This is very 
efficient decoupling mechanism that allows you to put pieces of the scene in separate scenes (prefabs) and modify
them independently. The changes in child scenes will be automatically reflected to all parent scenes. Here is the
very simple example of why this is important: imagine you need to populate a town with 3D models of cars. Each
kind of car have its own 3D model and, for example, a collision body that won't allow the player to walk through
cars. How would you do this? The simplest (and dumbest) solution is to copy dozens of car models in the scene, and
you're done. Imagine that now you need to change something in your car, for example, add a trunk that can be opened.
What will you do? Of course, you should "iterate" over each car model and do the required changes, you simply don't have
any other option. This will eat huge amount of time and in general it is very non-productive.  

This is where prefabs will save you hours of work. All you need to do is to create a car prefab and instantiate it
multiple times in your scene. When you'll need to change something in the car, you simply go to the prefab and change
it. After that every prefab instance will have your changes!

### Create scene manually

A scene could also be created manually:

```rust
# extern crate rg3d;
# use rg3d::{core::pool::Handle, engine::Engine, scene::Scene};

fn create_scene(engine: &mut Engine) -> Handle<Scene> {
    let mut scene = Scene::new();

    // Use node builders, create sounds, add physics, etc. here to fill the scene.

    engine.scenes.add(scene)
}
```

## Where all my scenes located?

All scenes "lives" in the engine, the engine has single ownership over your scene after you've added it in the engine.
You can borrow a scene at any time using its handle and do some changes.

## Building scene asynchronously 

You can create your scene in separate thread and then pass it to main thread to insert it in the engine. Why this 
is needed? Remember the last time you've played a relatively large game, you've probably noticed that it have 
loading screens and loading screen has some fancy interactive stuff with progress bar. Loading screen is fully 
responsive while the game doing hard job loading the world for you. Got it already? Asynchronous scene loading is
needed to create/load large scenes with tons of resources without blocking main thread, thus leaving the game 
fully responsive. 

There is comprehensive example of asynchronous scene loading, it can be found 
[here](https://github.com/rg3dengine/rg3d/blob/master/examples/async.rs)  

## Managing multiple scenes

Usually you should have only one scene active (unless you're making something very special), you should use 
`.enabled` flag of a scene to turn it off or on. Deactivated scenes won't be rendered, the physics won't be
updated, the sound will stop, and so on. In other words the scene will be frozen. This is useful for situations
when you often need to switch between scenes, leaving other scene in frozen state. One of the examples where this
can be useful is menus. In most games when you're entering the menu, game world is paused. 