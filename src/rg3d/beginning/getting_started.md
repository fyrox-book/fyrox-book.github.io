Despite the look of it, the engine is quite friendly to newcomers, especially with some good guides. This section
of the book will guide you the basics of the engine.

## Framework

The engine offers special framework to start making games as quickly as possible. It cares about engine initialization,
handles window events, calls every required methods and so on. In other words it helps you to get started as quickly
as possible with a need to put dozens lines of code to just create a window with a game loop.

The simplest app could be created with this code:

```rust
use rg3d::{
    engine::Engine,
    engine::framework::prelude::*,
};

struct Game { }

impl GameState for Game {
    fn init(_engine: &mut Engine) -> Self where Self: Sized {
        Self { }
    }
}

fn main() {
    Framework::<Game>::new()
        .unwrap()
        .title("Simple")
        .run();
}
```

The "work-horse" here is the `GameState` trait, it offers some optional method that could be used depending on 
your needs. 

- `fn init(engine: &mut Engine) -> Self where Self: Sized` - should create the instance of your game. It accepts the engine instance as the first argument
which gives you full access to the engine during the initialization.
- `fn on_tick(&mut self, engine: &mut Engine, dt: f32, control_flow: &mut ControlFlow)` - the game loop, it will
be called at fixed 60 FPS rate allowing you to run your game logic. The method gives you full access to the engine
so you're able to work it freely. The `dt` argument returns the amount of seconds that passed from the previous
call. The last argument (`control_flow`) allowing you to change the execution flow, for example setting it to 
`ControlFlow::Exit` will force the game to quit.
- `fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage)` - the main function that listens events 
from the user interface and gives you the ability to react. You should handle your UI in here.
- `fn on_device_event(&mut self, engine: &mut Engine, device_id: DeviceId, event: DeviceEvent)` - the function that
allows you to handle input from physical devices (mouse, keyboard, gamepads, etc.).
- `fn on_window_event(&mut self, engine: &mut Engine, event: WindowEvent)` - the function that allows you to do 
something when the window of the game receives an event from operating system. The variety of events is large, and
everything depends on what you need. For example this method could be useful handle keyboard events.
- `fn on_exit(&mut self, engine: &mut Engine)` - the function that will be called right before your application 
about to shut down allowing you to do some clean up or some other actions.

As you can see it is very concise and simple, every method serves a particular purpose. 

## Custom game loop (WIP)

The framework might be limiting for some cases, in such situations the engine could be initialized manually. It 
requires some decent amount of code and could error-prone for beginners. You should use custom game loop only if 
you're experienced game developer!

## Scenes and graphs

When you're playing a game, you often see various objects scattered around on screen, all of them forming a
_scene_. Scene is just a set of various objects, as in many other game engines, rg3d allows you to create multiple
scenes for various purposes. For example, one scene could be used for menu, a bunch could be used for game levels,
and one for ending screen. Scenes also could be used to create a source of data for other scenes, such scenes called
_prefabs_. A scene could also be rendered in a texture, and the texture can be used in other scene - this way you
can create interactive screens that showing some other places. 

While playing games, you could've noticed that some objects behaves like they're linked to other objects, for example
your character in a role-playing game could carry a sword. While the character holds the sword, it is linked to his
arm. Such relations between the objects could be presented by a graph structure. 

Simply speaking, graph is a set of objects with hierarchical relationships between each object. Each object in the
graph is called _node_. In the example with the sword and the character, the sword is a _child_ node of a character,
which in its turn is a _parent_ node of a character. (Here we intentionally omit the fact that usually character
model contains complex skeleton with multiple bones and the sword is actually attached to one of hand's bones.)

### Building blocks or scene nodes

The engine offers various types of "building blocks" for your scene, each such block is called _scene node_. 

- **Base** - a node that stores hierarchical information (a handle to the parent node and a set of handles
to children nodes), local and global transform, name, tag, lifetime, etc. It has self-describing name - it 
is used as a base node for every other scene node (via composition).
- **Mesh** - a node that represents a 3D model. This one of the most commonly used nodes in almost every game.
Meshes could be easily created either programmatically, or be made in some 3D modelling software (like Blender)
and loaded in your scene.
- **Light** - a node that represents a light source. There are three types of light sources:
  - **Directional** - a light source that does not have position, only direction. The closest real-world example
    is our Sun.
  - **Point** - a light source that emits light in every direction. Real-world example: light bulb.
  - **Spot** - a light source that emits light in a particular direction with a cone-like shape. Real-world example:
    flashlight.
- **Camera** - a node that allows you to see the world. You must have at least one camera in your scene to be 
able to see anything.
- **Sprite** - a node that represents a quad that always faced towards a camera. It can have a texture, size, it
also can be rotated around the "look" axis.
- **Particle system** - a node that allows you to build visual effects using a huge set of small particles, it
can be used to create smoke, sparks, blood splatters, etc. effects.
- **Terrain** - a node that allows you to create complex landscapes with minimal effort. 
- **Decal** - a node that paints on other nodes using a texture. It is used to simulate cracks in concrete walls, 
damaged parts of the road, blood splatters, bullet holes, etc.

These scene nodes allow you to build almost any kind of game. 

### Local and global coordinates

Graph describes your scene in a very natural way, allowing you think in terms of relative and absolute coordinates
when working with _scene nodes_.

Scene node has two kinds of transform - local and global. Local transform defines where the node is located 
(translation) relative to origin, how much it is scaled (in percent) and rotated (around any arbitrary axis).
Global transform is almost the same, but it also includes the whole chain of transforms of parent nodes. In the
previous example with the character, the sword has its own local transform which tells how much it should be 
moved from origin to be exactly on a hand of the character. But global transform of the swords includes transform 
of the entire character. So if you move the character, the local transform of the sword will remain the same, but
global transform will include the transform of the character.

This mechanism is very simple, yet powerful. The full grace of it unfolds when you're working with 3D models with
skeleton, each bone in the skeleton has its parent and a set of children. You can rotate/translate/scale bones to
animate your character.

## Data management and generational arenas (pools)

The engine uses generation arenas (pools in engine's terminology) for efficient data management. Pool is a
vector with entries that can be either vacant or occupied. Each entry, no matter occupied it or vacant, also
stores a special number called _generation_. The generation number is used to understand whether an entry has
been changed over time or not. When an entry is reused, its generation number is increased leaving all previously
created handle leading to the entry invalid. This is a very simple and efficient algorithm for tracking the 
"lifetime" of the objects.

To access the data in entries, the engine uses _handles_. The handle is a pair of index of an entry and a
_generation_ number. When you put an object in the pool, it gives you the handle that "leads" to the object.
At this moment the generation of the handle matches the generation of the corresponding entry so the handle
is valid. It will remain valid util you "free" the object, which will make the entry vacant again.

Since the pool is just a contiguous memory block, it is much more CPU cache-friendly. This means that in most
cases the data portions will be loaded in CPU caches, making the access to the data blazing fast.

Almost every entity in the engine "lives" in its own pool, this make it easy to create such data structures
like graphs, where a node refers to other nodes. In this case scene nodes stores just handles (which is just 
8 bytes of memory) to other nodes.