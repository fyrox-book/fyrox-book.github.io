# Editor, Plugins and Scripts

Every Fyrox game is just a plugin for both the engine and the editor, such approach allows you to run your game from the 
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
fyrox-template init --name my_game --style 3d
```

The tool accepts two arguments - a project name (`--name`) and a style (`--style`) which defines the contents of default
scene. Once you initialized your project, go to `game/src/lib.rs` - it is where your game logic is located, as you can 
see the `fyrox-template` generated quite some code for you. There are tiny comments about which place is for what. For 
more info about each method, please refer [to the docs](https://docs.rs/fyrox/latest/fyrox/plugin/trait.Plugin.html).

After the project is generated, you should memorize two commands that will help you to run your game in different modes:

- `cargo run --package editor --release` - launches the editor with your game attached, the editor allows you to run your game
  from it and edit game entities. It is intended to be used only for development.
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
#     core::{inspect::prelude::*, uuid::{Uuid, uuid}, visitor::prelude::*},
#     engine::resource_manager::ResourceManager,
#     event::Event,
#     gui::inspector::PropertyChanged,
#     handle_object_property_changed, impl_component_provider,
#     scene::{graph::map::NodeHandleMap, node::TypeUuidProvider},
#     script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
# };
# struct GameConstructor;
# impl GameConstructor {
#     fn type_uuid() -> Uuid { todo!() } 
# }
#[derive(Visit, Inspect, Default, Debug, Clone)]
struct MyScript {
    // Add fields here.
}

impl_component_provider!(MyScript);

impl TypeUuidProvider for MyScript {
    fn type_uuid() -> Uuid {
        uuid!("bf0f9804-56cb-4a2e-beba-93d75371a568")
    }
}

impl ScriptTrait for MyScript {
    fn on_property_changed(&mut self, args: &PropertyChanged) -> bool {
        // Use `handle_object_property_changed!(self, args, Self::FOO => foo)` here to handle
        // changing of object properties.
        false
    }

    fn on_init(&mut self, context: ScriptContext) {
        // Put initialization logic here.
    }

    fn on_deinit(&mut self, context: ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: ScriptContext) {
        // Respond to OS events here.
    }

    fn on_update(&mut self, context: ScriptContext) {
        // Put object logic here.
    }

    fn remap_handles(&mut self, old_new_mapping: &NodeHandleMap) {
        // Remap handles to other scene nodes here.
    }

    fn restore_resources(&mut self, resource_manager: ResourceManager) {
        // Restore resource handles here.
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }

    fn plugin_uuid(&self) -> Uuid {
        GameConstructor::type_uuid()
    }
}
```

Each script must implement following traits:

- `Visit` implements serialization/deserialization functionality, it is used by the editor to save your object to a scene file.
- `Inspect` implements read-only static reflection that provides introspection for your type - in other words, it allows the editor
  to "see" what's inside your structures.
- `Debug` - provides debugging functionality, it is mostly for the editor to let it print stuff into the console.
- `Clone` - makes your structure clone-able, since we can clone objects, we also want the script instance to be
  cloned.
- `Default` implementation is very important - the scripting system uses it to create your scripts in the default state.
  This is necessary to set some data to it and so on. If it's a special case, you can always implement your own `Default`'s
  implementation if it's necessary for your script.
- `TypeUuidProvider` is used to attach some unique id for your type, every script **must* have a unique ID, otherwise, the engine will
  not be able to save and load your scripts. To generate a new UUID, use [Online UUID Generator](https://www.uuidgenerator.net/) or
  any other tool that can generate UUIDs.

## Script Template Generator

You can use `fyrox-template` tool to generate all required boilerplate code for a new script, it makes adding new scripts
much less frustrating. To generate a new script use `script` command:

```shell
fyrox-template script --name MyScript
```

It will create a new file in `game/src` directory with `my_script.rs` name and fill with required code. Comments should
help you to figure out which code should be placed where.

## Script Registration

Every script must be registered before use, otherwise the engine won't "see" your script and won't let you assign it
to an object. `PluginConstructor` trait has `register` method exactly for script registration, to register a script 
you need to register it in the list of script constructors like so: 

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#         scene::Scene,
#         plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
#         core::{
#             visitor::prelude::*,
#             inspect::prelude::*,
#             pool::Handle,
#             uuid::Uuid
#         },
#         impl_component_provider,
#         scene::node::TypeUuidProvider,
#         script::ScriptTrait,
#     };
# 
#     #[derive(Inspect, Visit, Default, Copy, Clone, Debug)]
#     struct MyScript;
# 
#     impl TypeUuidProvider for MyScript {
#         fn type_uuid() -> Uuid {
#             todo!()
#         }
#     }
# 
#     impl_component_provider!(MyScript);
# 
#     impl ScriptTrait for MyScript {
#         fn id(&self) -> Uuid {
#             todo!()
#         }
# 
#         fn plugin_uuid(&self) -> Uuid {
#             todo!()
#         }
#     }
# 
#     struct Constructor;
# 
#     impl PluginConstructor for Constructor {
fn register(&self, context: PluginRegistrationContext) {
    context.serialization_context.script_constructors.add::<MyScript>("My Script");
}
# 
#         fn create_instance(&self, _override_scene: Handle<Scene>, _context: PluginContext) -> Box<dyn Plugin> {
#             todo!()
#         }
#     }
```

## Script Attachment

To assign a script and see it in action, run the editor, select an object and find `Script` property in the Inspector.
Select your script from the drop-down list. To see the script in action, click "Play/Stop" button. The editor will run
your game in separate process with the scene active in the editor.

Read the next chapter to learn how to use the editor.