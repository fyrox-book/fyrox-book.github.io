# 2D Platformer Tutorial (WORK IN PROGRESS)

**THIS IS UNFINISHED TUTORIAL, IGNORE IT**

In this tutorial we'll make a 2D platformer using new plugin and scripting system that was become available in Fyrox 0.25.

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

What do we need to use to make player move? We'll be using physical rigid body that is already added in the scene, we just need to
somehow be able to pass it to the script. Let's start by adding required field in the `Player` struct:

```rust,no_run
#[derive(Visit, Inspect, Debug, Clone)]
struct Player {
    body: Handle<Node>,
}

impl Default for Player {
    fn default() -> Self {
        Self { body: Handle::NONE }
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
        Player::BODY => {
            self.body = value.cast_clone().unwrap();
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