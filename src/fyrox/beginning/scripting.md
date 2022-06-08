# Editor, Plugins and Scripts

Every Fyrox game is just a plugin for both the engine and the editor, such approach allows you to run your game in the 
editor and be able to edit game entities in it. Your game can define any number of scripts, which can be assigned 
to scene objects to run custom game logic on them. In this chapter you'll learn basics of plugins and scripting system,
also you'll learn how to run the editor.

## Project Generator

Fyrox plugins are static, this means that you must re-compile your game or editor if you change source code of your game,
such architecture requires some boilerplate code that is generic for every game. Fyrox offers special tiny tool - 
`fyrox-template` - it allows you to generate all boilerplate parts in a single command. Install it using the following 
command:

```shell
cargo install fyrox-template
```

Navigate to a folder where you want the project to be created and do the following command:

```shell
fyrox-template --name platformer
```

The tool accepts only one argument - project name (it could be extended in the future). Once you initialized your 
project, go to `game/src/lib.rs` - it is where your game logic is located, as you can see the `fyrox-template` generated
quite some code for you. There are tiny comments about which place is for what. For more info about each method,
please refer [to the docs](https://docs.rs/fyrox/0.26.0/fyrox/plugin/trait.Plugin.html).

After the project is generated, you should memorize two commands that will help you to run your game in different modes:

- `cargo run --package editor --release` - launches the editor with your game attached, the editor allows you to run your game
  inside it and edit game entities. It is intended to be used only for development.
- `cargo run --package executor --release` - creates and runs the production binary of your game that can be shipped (for
  example - to a store).

Navigate to your project's directory and run `cargo run --package editor --release`, after some time you should see the 
editor:

![editor](editor.png)

In the editor you can start making your game scene. **Important note:** your scene must have at least one camera,
otherwise you won't see anything.

## Script Structure

Typical script structure could be like this:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{
#         inspect::{Inspect, PropertyInfo},
#         uuid::{uuid, Uuid},
#         visitor::prelude::*,
#     },
#     event::Event,
#     gui::inspector::PropertyChanged,
#     scene::node::TypeUuidProvider,
#     script::{ScriptContext, ScriptTrait},
# };
# 
# struct Game;
# 
# impl Game {
#     fn type_uuid() -> Uuid {
#         todo!()
#     }
# }
# 
#[derive(Visit, Inspect, Debug, Clone, Default)]
struct MyScript;

impl TypeUuidProvider for MyScript {
    // Returns unique script id for serialization needs.
    fn type_uuid() -> Uuid {
        uuid!("c5671d19-9f1a-4286-8486-add4ebaadaec")
    }
}

impl ScriptTrait for MyScript {
    // Accepts events from Inspector in the editor and modifies self state accordingly.
    fn on_property_changed(&mut self, args: &PropertyChanged) -> bool {
        false
    }

    // Called once at initialization.
    fn on_init(&mut self, context: ScriptContext) {}

    // Called whenever there is an event from OS (mouse click, keypress, etc.)
    fn on_os_event(&mut self, event: &Event<()>, context: ScriptContext) {}

    // Called every frame at fixed rate of 60 FPS. Put entity game logic here.
    fn on_update(&mut self, context: ScriptContext) {}

    // Returns unique script ID for serialization needs.
    fn id(&self) -> Uuid {
        Self::type_uuid()
    }

    // Returns unique id of parent plugin.
    fn plugin_uuid(&self) -> Uuid {
        Game::type_uuid()
    }
}
```

Each script must implement following traits:

- `Visit` implements serialization/deserialization functionality, it is used by the editor to save your object to a scene file.
- `Inspect` implements read-only static reflection that provides introspection for your type - in other words, it allows the editor
  to "see" what's inside your structure.
- `Debug` - provides debugging functionality, it is mostly for the editor to let it print stuff into the console.
- `Clone` - makes your structure clone-able, since we can clone objects, we also want the script instance to be
  cloned.
- `Default` implementation is very important - the scripting system uses it to create your scripts in the default state.
  This is necessary to set some data to it and so on. If it's a special case, you can always implement your own `Default`'s
  implementation if it's necessary for your script.
- `TypeUuidProvider` is used to attach some unique id for your type, every script **must* have a unique ID, otherwise, the engine will
  not be able to save and load your scripts. To generate a new UUID, use [Online UUID Generator](https://www.uuidgenerator.net/) or
  any other tool that can generate UUIDs.

## Script Registration

Every script must be registered before use, otherwise the engine won't "see" your script and won't let you assign it
to an object. `Plugin` trait has `on_register` method exactly for script registration, to register a script you need 
to register it in the list of script constructors like so: 

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{
#         inspect::{Inspect, PropertyInfo},
#         uuid::Uuid,
#         visitor::prelude::*,
#     },
#     plugin::{Plugin, PluginRegistrationContext},
#     scene::node::TypeUuidProvider,
#     script::ScriptTrait,
# };
# 
# struct Game;
# 
# impl TypeUuidProvider for Game {
#     fn type_uuid() -> Uuid {
#         todo!()
#     }
# }
# 
# #[derive(Visit, Inspect, Default, Debug, Clone)]
# struct MyScript;
# 
# impl TypeUuidProvider for MyScript {
#     fn type_uuid() -> Uuid {
#         todo!()
#     }
# }
# 
# impl ScriptTrait for MyScript {
#     fn id(&self) -> Uuid {
#         todo!()
#     }
# 
#     fn plugin_uuid(&self) -> Uuid {
#         Game::type_uuid()
#     }
# }
# 
# impl Plugin for Game {
#     fn id(&self) -> Uuid {
#         todo!()
#     }
# 
fn on_register(&mut self, context: PluginRegistrationContext) {
    let script_constructors = &context.serialization_context.script_constructors;
    script_constructors.add::<Game, MyScript, _>("MyScript");
}
# }
```

## Script Attachment

To assign a script and see it in action, run the editor, select an object and find `Script` property, select your script
from the drop-down list. To see the script in action, click "Play/Stop" button.