# 2D Platformer Tutorial

## Table of Contents

- [Introduction](#introduction)
- [Project](#project)
- [Using the Editor](#using-the-editor)
- [Scripts - Player](#scripts---player)
- [Animation](#animation)
- [Final Steps](#final-steps)
- [Standalone Game](#standalone-game)
- [Conclusion](#conclusion)

## Introduction

In this tutorial we'll make a 2D platformer using new plugin and scripting system that has become available in Fyrox 0.25 and 
improved in Fyrox 0.26. Here's what you'll get after finishing the tutorial:

<iframe width="560" height="315" src="https://youtube.com/embed/EcvtwEkBxNU" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

You can find source code of the tutorial [here](https://github.com/FyroxEngine/Fyrox-tutorials/tree/main/platformer), you can 
test it yourself by cloning the repository and `cargo run --package editor --release` in `platformer` directory.

## Project

Let's start by making a new project using special tiny tool - `fyrox-template` - it allows you to generate all boilerplate
parts in a single call. Install it using following command:

```shell
cargo install fyrox-template
```

Navigate to a folder where you want the project to be created and do the following command:

```shell
fyrox-template --name platformer
```

The tool accepts only one argument - project name (it could be extended in the future). After the project is generated, you 
should memorize two commands:

- `cargo run --package editor --release` - launches the editor with your game attached, the editor allows you to run your game
inside it and edit game entities. It is intended to be used only for development.
- `cargo run --package executor --release` - creates and runs production binary of your game that can be shipped (for
example - to a store).

Navigate to the `platformer` directory and run `cargo run --package editor --release`, after some time you should see the editor:

![editor](editor.png)

Great! Now we can actually start making our game. Go to `game/src/lib.rs` - it is where your game logic located, as you can see 
the `fyrox-template` generate quite some code for you. There are tiny comments about which place is for what. For more info
about each method, please refer [to the docs](https://docs.rs/fyrox/0.26.0/fyrox/plugin/trait.Plugin.html).

## Using the Editor

For now we don't even need to write a single line of code, we can create a scene entirely in the editor. This section will guide
you through the process of scene creation, as a final result we'll get something similar to this:

![editor with scene](editor_with_scene.png)

At first, we need some assets, I prepared all required (and some more) in a separate zip archive, so you don't need to search 
assets all over the internet. Download assets from [here](assets.zip) and unpack them in a `data` folder in the root folder of
your project. 

Let's start by creating a new scene. Run the editor and go to `File -> New Scene`. Since we're making 2D game, switch the editor 
camera mode to `Orthographic` at the right top corner of scene preview window. Now we need to populate the scene with some objects,
we'll start by adding a simple ground block. Right click on `__ROOT__` of the scene in `World Viewer` and select 
`Add Child -> Physics2D -> Rigid Body`. This will create a rigid body for the ground block, select the rigid body and 
set `Body Type` to `Static` in `Inspector`, by doing this we're telling the physics engine that our ground block should not move
and be rock-solid. Every rigid body requires a collider, otherwise the physics engine will not know how to handle collisions, right 
click on the rigid body in `Inspector` and click `Add Child -> Physics2D -> Collider`. We've just added a new collider to the rigid
body, by default it has `Cuboid` shape with `1.0` meter in height and width. Finally we need to add some graphics to the rigid body,
right click on the rigid body and click `Add Child -> 2D -> Rectangle`. This adds a simple 2D sprite, select it and set a texture
to it by drag'n'dropping it from asset browser on the white field of `Texture` field in the `Inspector`. For my scene I'm gonna
be using three sprites 

- `data/tiles/13.png` - left ground block
- `data/tiles/14.png` - center ground block
- `data/tiles/15.png` - right ground block

You can use any other textures and build your own level as you like. After doing all these steps you should get something like this:

![editor_step1](editor_step1.png)

Clone the block by selecting its rigid body and pressing `Ctrl+C` followed by `Ctrl+V`, navigate to sprite in the copy and change its
texture to either left or right end of the block. Use `Move Tool` to move the block somewhere you like (you can also using grid snapping
by going to `File -> Setting` and setting `Snap To Grid` for `Move Interaction Mode`). Do this one more time for opposite end and you
should get something like this:

![editor_step2](editor_step2.png)

Repeat these steps if you like, to add more platforms. You can also add some background objects, by creating a new sprite 
(right click `__ROOT__` and click `Add Child -> 2D -> Rectangle`) and assigning a texture to it:

![editor_step3](editor_step3.png)

As a last step of world editing, let's add some dynamic objects, like boxes. Pick some random ground block, select its rigid body and 
clone it. Switch body type of the copy to `Dynamic`. Now change it's sprite texture to a box (drag'n'drop `data/objects/Crate.png` to
`Texture` field) and clone the box few times, you should get something like this:

![editor_step4](editor_step4.png)

Now for the player. As always, let's start by creating a new rigid body, add a 2D collider to it and set its shape to capsule with following
parameters - `Begin = 0.0, 0.0` and `End = 0.0, 0.3`. Add a 2D sprite (rectangle) to the rigid body and set its texture to a texture from
`data/characters/adventurer/Individual Sprites`. We also need a camera, otherwise we won't see anything. Add it as a child to player's 
rigid body. By default our camera will have no background, there'll be a black "void", this is not great and let's fix that. Select the camera
and set `Skybox` property to `Some`. Now go to asset browser and find `data/background/BG.png`, drag'n'drop it to the `Front` field of the 
`Skybox` property. Don't forget to adjust far plane distance to something like `20.0`, otherwise you'll see just a portion of background image.
If everything is done correctly, you should get something like this:

![editor_step5](editor_step5.png)

Save your scene to `data/scene.rgs` - go to `File -> Save Scene`, select `data` folder in the tree and set file name to `scene.rgs`. Now we can
run the game using `Play/Stop` button at the top of the scene previewer. You should see pretty much the same as in the scene preview, except
service graphics, such as rigid body shapes, node bounds and so on. Now we can start writing scripts, but at first let's make our life easier
and force the editor to load the scene for us on startup. Go to `editor/src/main.rs` and replace this:

```rust,no_run
Some(StartupData {
    working_directory: Default::default(),
    // Set this to `"path/to/your/scene.rgs".into()` to force the editor to load the scene on startup.
    scene: Default::default(),
})
```

with this

```rust,no_run
Some(StartupData {
    working_directory: Default::default(),
    scene: "data/scene.rgs".into(),
})
```

Now if you re-run the editor, it will automatically load the requested scene, this saves some extra clicks and only a few seconds,
but if you multiply that to a number of restarts, this will give you decent time save.

And a last preparation step, let's import all entities at the beginning, so you don't need to find them manually, add the following code 
at the beginning of the `game/src/lib.rs`:

```rust,no_run
use fyrox::{
    core::{
        algebra::{Vector2, Vector3},
        futures::executor::block_on,
        inspect::{Inspect, PropertyInfo},
        pool::Handle,
        uuid::{uuid, Uuid},
        visitor::prelude::*,
    },
    engine::resource_manager::ResourceManager,
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    gui::inspector::{CollectionChanged, FieldKind, PropertyChanged},
    handle_collection_property_changed, handle_object_property_changed,
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    resource::texture::Texture,
    scene::{
        camera::Camera,
        dim2::{rectangle::Rectangle, rigidbody::RigidBody},
        node::{Node, TypeUuidProvider},
        Scene, SceneLoader,
    },
    script::{ScriptContext, ScriptTrait},
};
```

## Scripts - Player

Our scene has pretty much everything we need to start adding scripts, we'll start from `Player` script and make our character 
move. Navigate to `game/src/lib.rs` and at the end of the file add the following code snippet:

```rust,no_run
#[derive(Visit, Inspect, Debug, Clone)]
struct Player {}

impl Default for Player {
    fn default() -> Self {
        Self {}
    }
}

impl TypeUuidProvider for Player {
    // Returns unique script id for serialization needs.
    fn type_uuid() -> Uuid {
        uuid!("c5671d19-9f1a-4286-8486-add4ebaadaec")
    }
}

impl ScriptTrait for Player {
    // Accepts events from Inspector in the editor and modifies self state accordingly.
    fn on_property_changed(&mut self, args: &PropertyChanged) -> bool {
        
    }

    // Called once at initialization.
    fn on_init(&mut self, context: ScriptContext) {
        
    }

    // Called everytime when there is an event from OS (mouse click, key press, etc.)
    fn on_os_event(&mut self, event: &Event<()>, context: ScriptContext) {
        
    }

    // Called every frame at fixed rate of 60 FPS.
    fn on_update(&mut self, context: ScriptContext) {
        
    }

    // Returns unique script id for serialization needs.
    fn id(&self) -> Uuid {
        Self::type_uuid()
    }

    // Returns unique id of parent plugin.
    fn plugin_uuid(&self) -> Uuid {
        Game::type_uuid()
    }
}
```

This is a typical "skeleton" of any script, for now its methods are pretty much empty, we'll fill it with actual code very soon.
Let's go over most important parts. The snippet starts from `Player` structure definition which has `#[derive(Visit, Inspect, Debug, Clone)]`
attributes:

- `Visit` implements serialization/deserialization functionality, it is used by the editor to save your object to scene file.
- `Inspect` implements read-only static reflection that provides introspection for your type - in other words it allows the editor
to "see" what's inside your structure.
- `Debug` - provides debugging functionality, it is mostly for editor to let it print stuff into console.
- `Clone` - makes your structure clone-able, why do we need this? We can clone objects and we also want the script instance to be 
copied.

Next goes `Default` implementation, it is very important - scripting system uses it to create your scripts in default state and then 
it is able to set some data to it and so on.

`TypeUuidProvider` is used to attach some unique id for your type, every script **must* have unique id, otherwise the engine will 
not be able to save and load your scripts. To generate a new uuid, use [Online UUID Generator](https://www.uuidgenerator.net/) or
any other tool that is able to generate UUIDs.

Finally we implement `ScriptTrait` for the `Player`. It has a bunch of methods, their names speaks for themselves. Learn more about
every method in [documentation](https://docs.rs/fyrox/0.26.0/fyrox/script/trait.ScriptTrait.html)

Before we can use the script in the editor, we must tell the engine that our script exist - we must register it. Remember that 
`on_register` method in `Plugin` trait implementation? It is exactly for script registration, replace its implementation with following 
code snippet:

```rust,no_run
fn on_register(&mut self, context: PluginRegistrationContext) {
    let script_constructors = &context.serialization_context.script_constructors;
    script_constructors.add::<Game, Player, _>("Player");
}
```

Now the engine know about our script and will be able to use it. It is pretty much useless in the current state, but we can already
assign it to the player. Select player's rigid body node and find `Script` in the `Inspector`, select `Player` from respective 
drop-down list and that's pretty much it - now the script is assigned:

![script_selection](script_selection.png)

Let's learn how to edit script properties from the editor. In the next section, we'll be adding key frame animation for you character,
it is perfect opportunity to learn how the engine and the editor operate with user-defined properties in scripts. To animate the player
we need to get its sprite first. Let's start by adding required field in the `Player` structure:

```rust,no_run
#[derive(Visit, Inspect, Debug, Clone)]
struct Player {
    sprite: Handle<Node>,
}

impl Default for Player {
    fn default() -> Self {
        Self { sprite: Handle::NONE }
    }
}
```

After adding this, the editor will be able to see the field and give you an ability to edit it in the Inspector, but for now any
changes done in Inspector will not be applied to the script instance. We need to take care about this, it is a bit of manual work,
future versions of the engine will most likely do this automatically. We're interested in `on_property_changed` method, fill it 
the following code snippet:

```rust,no_run
handle_object_property_changed!(self, args, Self::SPRITE => sprite)
```

This single line of code applies changes to `sprite` field if in the editor it's "view" was edited. The macro hides most of boilerplate
code from you, when macro is expanded the result code would look like:

```rust,no_run
if let FieldKind::Object(ref value) = args.value {
    match args.name.as_ref() {
        Self::SPRITE => {
            self.sprite = value.cast_clone().unwrap();
            true
        }
        _ => false,
    }
} else {
    false
}
```

It is good to know what it is doing, before use macros. At first, the code checks value kind, if it is a simple object, then we're 
checking the name of the property and if it is our `sprite`, setting the value. The method expects `true` or `false` as a return value,
what does each mean? `true` means that the property was handled and `false` - opposite. If the editor sees `false` it prints warning
message informing you that property handler is missing.

To assign the correct handle of the sprite to the respective field in script properties, hold `Alt` and start dragging sprite node from
the world viewer to the respective field in the player script. Release the mouse button and if everything is ok, the field should "say"
something different than "Unassigned".

Alright, at this point we know how to work with script properties, now we can start adding basic movement for the player. Go to `Player` 
structure and add following fields:

```rust,no_run
move_left: bool,
move_right: bool,
jump: bool,
```

Don't forget to initialize the fields in `Default` trait implementation:

```rust,no_run
impl Default for Player {
    fn default() -> Self {
        Self {
            body: Handle::NONE,
            move_left: false,
            move_right: false,
            jump: false,
        }
    }
}
```

These fields will store state of keyboard keys responsible for player movement. Now for `on_os_event`, add following code in there:

```rust,no_run
if let Event::WindowEvent { event, .. } = event {
    if let WindowEvent::KeyboardInput { input, .. } = event {
        if let Some(keycode) = input.virtual_keycode {
            let is_pressed = input.state == ElementState::Pressed;

            match keycode {
                VirtualKeyCode::A => self.move_left = is_pressed,
                VirtualKeyCode::D => self.move_right = is_pressed,
                VirtualKeyCode::Space => self.jump = is_pressed,
                _ => (),
            }
        }
    }
}
```

The code responds to OS events and modifies internal movement flags accordingly. Now we need to use the flags somehow, it's time for
`on_update`. The method is called each frame and allows you to put game logic there:

```rust,no_run
// Called every frame at fixed rate of 60 FPS.
fn on_update(&mut self, context: ScriptContext) {
    // The script can be assigned to any scene node, but we assert that it will work only with
    // 2d rigid body nodes.
    if let Some(rigid_body) = context.node.cast_mut::<RigidBody>() {
        let x_speed = if self.move_left {
            3.0
        } else if self.move_right {
            -3.0
        } else {
            0.0
        };

        if self.jump {
            rigid_body.set_lin_vel(Vector2::new(x_speed, 4.0))
        } else {
            rigid_body.set_lin_vel(Vector2::new(x_speed, rigid_body.lin_vel().y))
        };
    }
}
```

Finally, some interesting code. At first we checking if the node to which the script is assigned is actually a 2d rigid body, next
we're checking movement flags and forming horizontal speed and applying velocity to the body. Velocity is applied in two ways: if 
the jump button was pressed - apply horizontal velocity and some vertical velocity for jumping. If jump button wasn't pressed - 
just change horizontal velocity - this will allow the player to free fall.

Run the editor and enter play mode, press `[A][D][Space]` buttons to check if everything works correctly - the player should move 
horizontally and be able to jump. You can jump to the boxes on the right and push them off the ledge. 

The movement is working, but the player does not change orientation, if we'll go to the left - it looks ok (despite lack of animation),
but if we'll move to the right - it looks like player moves backwards. Let's fix that by changing horizontal scaling of the player's 
sprite. Add following code at the end of the `if let ...` block of the code above:

```rust,no_run
// It is always a good practice to check whether the handles are valid, at this point we don't know
// for sure what's the value of the `sprite` field. It can be unassigned and the following code won't
// execute. A simple `context.scene.graph[self.sprite]` would just panicked in this case.
if let Some(sprite) = context.scene.graph.try_get_mut(self.sprite) {
    // We want to change player orientation only if he's moving.
    if x_speed != 0.0 {
        let local_transform = sprite.local_transform_mut();

        let current_scale = **local_transform.scale();

        local_transform.set_scale(Vector3::new(
            // Just change X scaling to mirror player's sprite.
            current_scale.x.copysign(-x_speed),
            current_scale.y,
            current_scale.z,
        ));
    }
}
```

The comments should clarify what's going on here, but in short we're changing horizontal scaling of player's sprite if the player is
moving. The line `current_scale.x.copysign(-x_speed)` could be confusing, what it does? It replaces sign of current horizontal scaling
using the opposite sign of `x_speed`. 

Now if you run the game, the player will "look" in correct direction depending on velocity vector.

## Animation

Since we're making 2d game, we'll be using simple animations based on continuous change of key frames. In other words we'll be changing
texture on player body sprite. The engine does not provide such functionality yet, simply because it was focused primarily on 3D games
for quite long period of time. It is easy to make such animation "system" ourselves.

Put this code snippets somewhere at the end of `lib.rs` of the `game` project and we'll start learning what it's doing:

```rust
#[derive(Default, Inspect, Visit, Debug, Clone)]
pub struct KeyFrameTexture {
    texture: Option<Texture>,
}

impl KeyFrameTexture {
    fn on_property_changed(&mut self, args: &PropertyChanged) -> bool {
        handle_object_property_changed!(self, args, Self::TEXTURE => texture)
    }

    fn restore_resources(&mut self, resource_manager: ResourceManager) {
        // It is very important to restore texture handle after loading, otherwise the handle will
        // remain in "shallow" state when it just has path to data, but not the actual resource handle.
        resource_manager
            .state()
            .containers_mut()
            .textures
            .try_restore_optional_resource(&mut self.texture);
    }
}

#[derive(Inspect, Visit, Debug, Clone)]
pub struct Animation {
    name: String,
    keyframes: Vec<KeyFrameTexture>,
    current_frame: u32,
    speed: f32,

    // We don't want this field to be visible from the editor, because this is internal parameter.
    #[inspect(skip)]
    t: f32,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            name: "Unnamed".to_string(),
            keyframes: vec![],
            current_frame: 0,
            speed: 10.0,
            t: 0.0,
        }
    }
}

impl Animation {
    // Once again, we must implement support for property editing, it is a bit tedious
    // but must be done once.
    fn on_property_changed(&mut self, args: &PropertyChanged) -> bool {
        handle_object_property_changed!(self, args,
            Self::CURRENT_FRAME => current_frame,
            Self::NAME => name,
            Self::SPEED => speed
        ) || handle_collection_property_changed!(self, args, Self::KEYFRAMES => keyframes)
    }

    pub fn current_frame(&self) -> Option<&KeyFrameTexture> {
        self.keyframes.get(self.current_frame as usize)
    }

    fn restore_resources(&mut self, resource_manager: ResourceManager) {
        for key_frame in self.keyframes.iter_mut() {
            key_frame.restore_resources(resource_manager.clone());
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.t += self.speed * dt;

        if self.t >= 1.0 {
            self.t = 0.0;

            // Increase frame index and make sure it will be clamped in available bounds.
            self.current_frame = (self.current_frame + 1) % self.keyframes.len() as u32;
        }
    }
}
```

The code snippets is quite big, but this is pretty much everything we need for simple key frame animation.
We start by defining `KeyFrameTexture` structure - it is a simple new-type-ish structure that holds single
field - optional texture handle. 

Next goes the implementation of the structure, in the `on_property_changed` we're handing editor's messages
and syncing state of key frame. The next method is more interesting - `restore_resources` restores texture
handle on deserialization. Sounds complicated, why do we need to do anything after deserialization? Well,
the answer is simple - we don't store texture data in the texture, instead we just saving path to external
resource and requesting the resource manager to load the texture.

Finally we're at `Animation` structure, nothing unusual there, it just stores a list of key frames, index 
of current key frame, speed, and some service fields. The implementation of it is very straightforward too,
the most interesting method is `update`. Inside it we're updating internal `t` parameter which holds "fraction"
of next frame's index, when it reaches `1.0` we're increasing index of current frame and wrapping it so it
never exceed maximum amount of key frames - instead it will start counting from 0.

It's time to start using new animation system, just add the following fields to the `Player`: 

```rust,no_run
animations: Vec<Animation>,
current_animation: u32,
```

Don't forget to initialize the field in `impl Default for Player`:

```rust,no_run
animations: Default::default(),
current_animation: 0,
```

Player will use multiple animations in future tutorials, but for now it will use only two - idle and run.
Now we need to somehow switch animations. Go to `on_update` in `Player` and add following lines after 
`x_speed` declaration:

```rust,no_run
if x_speed != 0.0 {
    self.current_animation = 0;
} else {
    self.current_animation = 1;
}
```

Here we assume that run animation will be at index `0` and the idle animation at index `1`. We also need to 
apply texture from current animation to the player's sprite, add following lines in the end of `on_update`

```rust,no_run
if let Some(current_animation) = self.animations.get_mut(self.current_animation as usize) {
    current_animation.update(context.dt);

    if let Some(sprite) = context
        .scene
        .graph
        .try_get_mut(self.sprite)
        .and_then(|n| n.cast_mut::<Rectangle>())
    {
        // Set new frame to the sprite.
        sprite.set_texture(
            current_animation
                .current_frame()
                .and_then(|k| k.texture.clone()),
        )
    }
}
```

The code is pretty straightforward - we start by trying to get a reference to current animation by its index,
if we're succeeded, we update it. At the next step we're getting sprite and assigning a current frame of 
current animation. Experienced game developers could immediately ask - why not to use sprite sheets and get
better performance and stuff. Well, the main purpose of this tutorial is to teach how to use the engine to 
achieve some goal, such as _making a game_. You can always optimize your game later, when you'll have something
working and playable.

## Final Steps

Three more steps before we can run the game, we need to call `restore_resources` for each animation. To do that,
the script trait has `on_restore_resources` method, add the following code to `impl ScriptTrait for Player`

```rust,no_run
fn restore_resources(&mut self, resource_manager: ResourceManager) {
    for animation in self.animations.iter_mut() {
        animation.restore_resources(resource_manager.clone());
    }
}
```

As a second step, replace contents of the `editor/src/main.rs` with the following code snippet:

```rust,no_run
//! Editor with your game connected to it as a plugin.
use fyrox::gui::inspector::editors::collection::VecCollectionPropertyEditorDefinition;
use fyrox::{
    event_loop::EventLoop,
    gui::inspector::editors::inspectable::InspectablePropertyEditorDefinition,
};
use fyroxed_base::{Editor, StartupData};
use platformer::{Animation, Game, KeyFrameTexture};

fn main() {
    let event_loop = EventLoop::new();
    let mut editor = Editor::new(
        &event_loop,
        Some(StartupData {
            working_directory: Default::default(),
            scene: "data/scene.rgs".into(),
        }),
    );
    editor.add_game_plugin(Game::new());

    // Register property editors here.
    let property_editors = &editor.inspector.property_editors;
    property_editors.insert(InspectablePropertyEditorDefinition::<KeyFrameTexture>::new());
    property_editors.insert(InspectablePropertyEditorDefinition::<Animation>::new());
    property_editors.insert(VecCollectionPropertyEditorDefinition::<KeyFrameTexture>::new());
    property_editors.insert(VecCollectionPropertyEditorDefinition::<Animation>::new());

    editor.run(event_loop)
}
```

The most interesting code in here is this:

```rust,no_run
// Register property editors here.
let property_editors = &editor.inspector.property_editors;
property_editors.insert(InspectablePropertyEditorDefinition::<KeyFrameTexture>::new());
property_editors.insert(InspectablePropertyEditorDefinition::<Animation>::new());
property_editors.insert(VecCollectionPropertyEditorDefinition::<KeyFrameTexture>::new());
property_editors.insert(VecCollectionPropertyEditorDefinition::<Animation>::new());
```

Here we're registering _property editors_ for our game types. This very important step, it tells the editor how to 
visualize your data. In most cases you'll be using those two generic types - `InspectablePropertyEditorDefinition` 
and `VecCollectionPropertyEditorDefinition`. Which is responsible for what? 

- `InspectablePropertyEditorDefinition` - it is responsible for showing properties of any object that implements 
`Inspect` trait. All of your game entities must implement such trait.
- `VecCollectionPropertyEditorDefinition` - it is responsible for showing `Vec<T: Inspect>` collection, every
collection item **must** implement `Inspect` trait. This is a bit tedious, especially in case of simple collections
like `Vec<f32>`, but that's a limitation of current implementation. It can be mitigated by using a new-type technique,
which was shown earlier.

This is yet another place for manual work, but it must be done, editor cannot use "magic" to understand which widget 
to use to visualize your data, there's no magic here.

And a final step is to change how the script properties are handled:

```rust,no_run
fn on_property_changed(&mut self, args: &PropertyChanged) -> bool {
    handle_object_property_changed!(self, args, Self::SPRITE => sprite)
        // The following line handles collection modification.
        || handle_collection_property_changed!(self, args, Self::ANIMATIONS => animations)
}
```

Now we need to go to the editor again and add the animations to the `Player`, select player's rigid body and 
find `Script` section in the `Inspector`. Add two animations there like so:

![editor_step6](editor_step6.png)

## Standalone Game

As a final step we'll add proper support for standalone mode ("production builds"), let's replace `on_standalone_init`
with the following code:

```rust,no_run
fn on_standalone_init(&mut self, context: PluginContext) {
    let mut scene = block_on(
        block_on(SceneLoader::from_file(
            "data/scene.rgs",
            context.serialization_context.clone(),
        ))
        .unwrap()
        .finish(context.resource_manager.clone()),
    );

    self.set_scene(context.scenes.add(scene), context);
}
```

The code just loads our scene and sets it as current, we need to do this manually when the game runs in standalone mode
(without editor), because the editor does some work for us when we running the game inside it.

## Conclusion

In this tutorial we've learned basics of the new scripting system of the engine that was added in Fyrox 0.25. The game 
we've built is very simple, but it is just a beginning. It is easy to add more scripts for enemies, weapons, collectable
items and so on. 