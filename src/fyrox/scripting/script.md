# Scripts

Script - is a container for game data and logic that can be assigned to a scene node. Fyrox uses Rust for scripting, 
so scripts are as fast as native code. 

## Script Structure

Typical script structure could be like this:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{uuid::{Uuid, uuid}, visitor::prelude::*, reflect::prelude::*},
#     event::Event, impl_component_provider,
#     scene::{graph::map::NodeHandleMap, node::TypeUuidProvider},
#     script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
# };
#[derive(Visit, Reflect, Default, Debug, Clone)]
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
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
    }
    
    fn on_start(&mut self, context: &mut ScriptContext) {
        // Put start logic - it is called when every other script is already initialized.
    }

    fn on_deinit(&mut self, context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        // Respond to OS events here.
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        // Put object logic here.
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
```

Each script must implement following traits:

- `Visit` implements serialization/deserialization functionality, it is used by the editor to save your object to a 
scene file.
- `Reflect` implements compile-time reflection that provides a way to iterate over script fields, set their values, 
find fields by their paths, etc.
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
#             reflect::prelude::*,
#             pool::Handle,
#             uuid::Uuid
#         },
#         impl_component_provider,
#         scene::node::TypeUuidProvider,
#         script::ScriptTrait,
#     };
# 
#     #[derive(Reflect, Visit, Default, Copy, Clone, Debug)]
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

The script can be attached to a scene node from code:

```rust, no_run
# extern crate fyrox;
# use fyrox::{
#     core::{reflect::prelude::*, uuid::Uuid, visitor::prelude::*},
#     impl_component_provider,
#     scene::node::{Node, TypeUuidProvider},
#     script::{Script, ScriptTrait},
# };
# 
# #[derive(Reflect, Visit, Default, Copy, Clone, Debug)]
# struct MyScript;
# 
# impl TypeUuidProvider for MyScript {
#     fn type_uuid() -> Uuid {
#         todo!()
#     }
# }
# 
# impl_component_provider!(MyScript);
# 
# impl ScriptTrait for MyScript {
#     fn id(&self) -> Uuid {
#         todo!()
#     }
# }
# 
fn set_script<T: ScriptTrait>(node: &mut Node, script: T) {
    node.set_script(Some(Script::new(script)))
}
```

Initialization as well as update of newly assigned script will happen on next update tick of the engine.

## Script Context

Script context provides access to the environment that can be used to modify engine and game state from scripts. Typical
content of the context is something like this:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     engine::{resource_manager::ResourceManager, ScriptMessageDispatcher},
#     plugin::Plugin,
#     scene::{node::Node, Scene},
#     script::ScriptMessageSender
# };
pub struct ScriptContext<'a, 'b, 'c> {
    pub dt: f32,
    pub elapsed_time: f32,
    pub plugins: &'a mut [Box<dyn Plugin>],
    pub handle: Handle<Node>,
    pub scene: &'b mut Scene,
    pub resource_manager: &'a ResourceManager,
    pub message_sender: &'c ScriptMessageSender,
    pub message_dispatcher: &'c mut ScriptMessageDispatcher,
}
```

- `dt` - amount of time passed since last frame. The value of the variable is implementation-defined, usually it is
something like 1/60 (0.016) of a second.
- `elapsed_time` - amount of time that passed since start of your game (in seconds).
- `plugins` - a mutable reference to all registered plugins, it allows you to access some "global" game data that does 
not belong to any object. For example, a plugin could store key mapping used for player controls, you can access it 
using `plugins` field and find desired plugin. In case of a single plugin, you just need to cast the reference to a 
particular type using `context.plugins[0].cast::<MyPlugin>().unwrap()` call.
- `handle` - a handle of the node to which the script is assigned to (parent node). You can borrow the node using
`context.scene.graph[handle]` call. Typecasting can be used to obtain a reference to a particular node type.
- `scene` - a reference to parent scene of the script, it provides you full access to scene content, allowing you to
add/modify/remove scene nodes.
- `resource_manager` - a reference to resource manager, you can use it to load and instantiate assets. 
- `message_sender` - a message sender. Every message sent via this sender will be then passed to every 
`ScriptTrait::on_message` method of every script.
- `message_dispatcher` - a message dispatcher. If you need to receive messages of a particular type, you must subscribe 
to a type explicitly.