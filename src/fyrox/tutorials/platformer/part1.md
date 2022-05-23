# 2D Platformer Tutorial (WORK IN PROGRESS)

**THIS IS UNFINISHED TUTORIAL, IGNORE IT**

## Table of Contents


## Introduction

In this tutorial we'll make a 2D platformer using new plugin and scripting system that was become available in Fyrox 0.25.

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
about each method, please refer [to the docs](https://docs.rs/fyrox/0.25.0/fyrox/plugin/trait.Plugin.html).

## Editor

For now we don't even need to write a single line of code, we can create a scene entirely in the editor. To save your time,
I prepared game-ready scene which we'll be using for the rest of the tutorial. To learn how to use the editor, please refer to
the `FyroxEd` section of the book.

Download the scene from [here](scene.zip) and unpack it to `platformer/data`, run the editor and use `File -> Load Scene` to 
load the scene. You should see something like this (do not forget to switch to Orthographic camera mode in scene previewer).

![editor with scene](editor_with_scene.png)

Run the game using `Play/Stop` button at the top of the scene previewer. Now we can start writing code, at first let's make our
life easier and force the editor to load the scene for us on startup. Go to `editor/src/main.rs` and replace this:

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

Now if you run the editor, it will automatically load the requested scene, this saves some extra clicks and only a few seconds,
but if you multiply that to a number of restarts, this will give you decent time save.

## Scripts - Player

Our scene has pretty much everything we need to start adding scripts, we'll start from `Player` script and make our character 
move.

Navigate to `game/src/lib.rs` and at the end of the file add the following code snippet:

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
        todo!()
    }

    // Called once at initialization.
    fn on_init(&mut self, context: ScriptContext) {
        todo!()
    }

    // Called everytime when there is an event from OS (mouse click, key press, etc.)
    fn on_os_event(&mut self, event: &Event<()>, context: ScriptContext) {
        todo!()
    }

    // Called every frame at fixed rate of 60 FPS.
    fn on_update(&mut self, context: ScriptContext) {
        todo!()
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

This is a typical "skeleton" of any script, for now it is filled with lots of `todo`s, we'll fill it with actual code right now.
But before we continue, we must tell the engine that our script exist - we must register it. Remember that `on_register` method
in `Plugin` trait implementation? It is exactly for script registration, replace its implementation with following code snippet:

```rust,no_run
fn on_register(&mut self, context: PluginRegistrationContext) {
    let script_constructors = &context.serialization_context.script_constructors;
    script_constructors.add::<Game, Player, _>("Player");
}
```

Now the engine know about our script and will be able to use it. You cannot use it in current state yet, we need to take care about
all `todo`s first, otherwise the game will panic. If you're eager to see what will happen in the editor, just remove all `todo`s and
run the editor.

What do we need to use to make player move? We'll be using physical rigid body that is already added in the scene, but we also need to
animate sprite (which is static for now) - we just need to somehow be able to pass it to the script. Let's start by adding required 
field in the `Player` struct:

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
future versions of the engine will do this automatically. We're interested in `on_property_changed` method, fill it the following
code snippet:

```rust,no_run
if let FieldKind::Object(ref value) = args.value {
    match args.name.as_ref() {
        Player::SPRITE => {
            self.sprite = value.cast_clone().unwrap();
            true
        }
        _ => false,
    }
} else {
    false
}
```

At first, the code checks value kind, if it is a simple object, then we're checking the name of the property and if it is our `body`,
setting the value. The method expects `true` or `false` as a return value, what does each mean? `true` means that the property was 
handled and `false` - otherwise. If the editor sees `false` it prints warning message informing you that something is missing.

Finally we can add our script to the player instance, run the editor, find the player and change its `Script` property to `Player`:

![script_selection](script_selection.png)

To assign the correct handle to the sprite, hold `Alt` and start dragging sprite node from the world viewer to the respective field
in the player script.

Now we can start adding basic movement for the player. Go to `Player` structure and add following fields:

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

The movement is working, but the player does not change orientation, if we'll go to the left - it looks of (despite lack of animation),
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