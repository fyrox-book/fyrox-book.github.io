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

There is a [separate chapter](../../fyrox/beginning/editor_overview.md) in the book that should help you to create a 
scene. After a scene is created, you can load it as any other 3D model (or prefab) using the resource manager:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{futures::executor::block_on, pool::Handle},
#     asset::manager::{ResourceManager}, resource::model::{Model, ModelResourceExtension},
#     scene::{node::Node, Scene},
# };
# use std::path::Path;

fn load_scene(resource_manager: ResourceManager) -> Scene {
    // Create parent scene.
    let mut scene = Scene::new();

    // Request child scene and block until it loading.
    let scene_resource = block_on(
        resource_manager
            .request::<Model, _>("path/to/your/scene.rgs"),
    )
        .unwrap();

    // Create an instance of the scene in the parent scene.
    let child_scene = scene_resource.instantiate(&mut scene);

    scene
}
```

Please note that here we're creating an empty scene and only then instantiating another scene into it. Why is this
needed? Child scene is considered as [prefab](./prefab.md), and it is "instantiated" in the parent scene. Considering 
it as prefab allows you modifying your scene separately and serialization/deserialization will be able to correctly
apply any changes in the scene.

### Create scene manually

A scene could also be created manually:

```rust,no_run
# extern crate fyrox;
# use fyrox::{core::pool::Handle, engine::Engine, scene::Scene};

fn create_scene(engine: &mut Engine) -> Handle<Scene> {
    let mut scene = Scene::new();

    // Use node builders, create sounds, add physics, etc. here to fill the scene.

    engine.scenes.add(scene)
}
```

See respective node builders [docs](../scene/graph.md#using-node-builders) to populate the scene.

## Where all my scenes located?

All scenes "lives" in the engine, the engine has ownership over your scene after you've added it in the engine.
You can borrow a scene at any time using its handle and do some changes:

```rust
# extern crate fyrox;
# use crate::{
#     core::pool::Handle,
#     event_loop::ControlFlow,
#     plugin::{Plugin, PluginContext},
#     scene::Scene,
# };
# 
struct Game {
    scene: Handle<Scene>,
}

impl Plugin for Game {
    fn update(&mut self, context: &mut PluginContext, control_flow: &mut ControlFlow) {
        // Borrow a scene using its handle. `try_get` performs immutable borrow, to mutably borrow the scene
        // use `try_get_mut`.
        if let Some(scene) = context.scenes.try_get(self.scene) {
            // Do something.
            println!("{:?}", scene.graph.performance_statistics);
        }
    }
}
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
# extern crate fyrox;
# use fyrox::scene::Scene;
# use fyrox::core::color::Color;
# let mut scene = Scene::default();
# 
scene.ambient_lighting_color = Color::opaque(30, 30, 30); 
```

Please keep in mind that ambient lighting does not mean global illumination, it is a different lighting technique
which is not available in the engine yet.