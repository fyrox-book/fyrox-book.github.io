# Scripts

Script - is a container for game data and logic that can be assigned to a scene node. Fyrox uses Rust for scripting, 
so scripts are as fast as native code. 

## When to Use Scripts and When Not

Scripts are meant to be used to add data and some logic to scene nodes. That being said, you should not use scripts
to hold some global state of your game (use your game plugin for that). For example, use scripts for your game items, 
bots, player, level, etc. On the other hand **do not** use scripts for leader boards, game menus, progress information,
etc.

Also, scripts cannot be assigned to UI widgets due to intentional Game <-> UI decoupling reasons. All user interface 
components should be created and handled in the game plugin of your game.

## Script Structure

Typical script structure is something like this:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{uuid::{Uuid, uuid}, visitor::prelude::*, reflect::prelude::*, TypeUuidProvider},
#     event::Event, impl_component_provider,
#     scene::{graph::map::NodeHandleMap},
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
- `Debug` - provides debugging functionality, it is mostly for the editor to let it turn the structure and its fields 
into string.
- `Clone` - makes your structure clone-able, since we can clone objects, we also want the script instance to be 
cloned.
- `Default` implementation is very important - the scripting system uses it to create your scripts in the default state.
This is necessary to set some data to it and so on. If it's a special case, you can always implement your own `Default`'s
implementation if it's necessary for your script.
- `TypeUuidProvider` is used to attach some unique id for your type, every script **must** have a unique ID, otherwise, 
the engine will not be able to save and load your scripts. To generate a new UUID, use 
[Online UUID Generator](https://www.uuidgenerator.net/) or any other tool that can generate UUIDs.

## Script Template Generator

You can use `fyrox-template` tool to generate all required boilerplate code for a new script, it makes adding new scripts
much less tedious. To generate a new script use `script` command:

```shell
fyrox-template script --name MyScript
```

It will create a new file in `game/src` directory with `my_script.rs` name and fill with required code. Do not forget
to add the module with the new script to `lib.rs` like this: 

```rust,no_run,compile_fail
// Use your script name instead of `my_script` here.
pub mod my_script;
```

Comments in each generated method should help you to figure out which code should be placed where and what is the purpose
of every method.

> ⚠️ Keep in mind that every new script must be registered in `PluginConstructor::register`, otherwise you won't be able
> to assign the script in the editor to a node. See the next section for more info. 

## Script Registration

Every script must be registered before use, otherwise the engine won't "see" your script and won't let you assign it
to an object. `PluginConstructor` trait has `register` method exactly for script registration. To register a script
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
#             uuid::Uuid, TypeUuidProvider
#         },
#         impl_component_provider,
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
#         fn create_instance(&self, _scene_path: Option<&str>, _context: PluginContext) -> Box<dyn Plugin> {
#             todo!()
#         }
#     }
```

Every script type (`MyScript` in the code snippet above, you need to change it to your script type) must be registered using 
[ScriptConstructorsContainer::add](https://docs.rs/fyrox/latest/fyrox/script/constructor/struct.ScriptConstructorContainer.html#method.add) 
method, which accepts a script type as a generic argument and its name, that will be shown in the editor. The name can be 
arbitrary, it is used only in the editor. You can also change it at any time, it won't break existing scenes.

## Script Attachment

To assign a script and see it in action, run the editor, select an object and find `Script` property in the Inspector.
Select your script from the drop-down list. To see the script in action, click "Play/Stop" button. The editor will run
your game in separate process with the scene active in the editor.

The script can be attached to a scene node from code:

```rust, no_run
# extern crate fyrox;
# use fyrox::{
#     core::{reflect::prelude::*, uuid::Uuid, visitor::prelude::*, TypeUuidProvider},
#     impl_component_provider,
#     scene::node::{Node},
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
#     engine::{ScriptMessageDispatcher},
#     plugin::Plugin, asset::manager::ResourceManager,
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

## Execution order

Scripts have strictly defined execution order for their methods (the order if execution is linear and **do not** depend 
on actual tree structure of the graph where the script is located):

- `on_init` - called first for every script instance
- `on_start` - called after every `on_init` is called
- `on_update` - called zero or more times per one render frame. The engine stabilizes update rate of the logic, so if
your game runs at 15 FPS, the logic will still run at 60 FPS thus the `on_update` will be called 4 times per frame. The
method can also be not called at all, if the FPS is very high. For example, if your game runs at 240 FPS, then `on_update`
will be called once per 4 frames.
- `on_message` - called once per incoming message.
- `on_os_event` - called once per incoming OS event.
- `on_deinit` - called at the end of the update cycle once when the script (or parent node) is about to be deleted.

## Message passing

Script system of Fyrox supports message passing for scripts. Message passing is a mechanism that allows you to send some 
data (message) to a node, hierarchy of nodes or the entire graph. Each script can subscribe for a specific message type. 
It is an efficient way for decoupling scripts from each other. For instance, you may want to detect and respond to some 
event in your game. In this case when the event has happened, you send a message of a type and every "subscriber" will 
react to it. This way subscribers will not know anything about sender(s); they'll only use message data to do some actions.

A simple example where the message passing can be useful is when you need to react to some event in your game. Imagine,
that you have weapons in your game, and they can have a laser sight that flashes with a different color when some target
was hit. In very naive approach you can handle all laser sights where you handle all intersection for projectiles, but
this adds a very tight coupling between laser sight and projectiles. This is totally unnecessary coupling can be made
loose by using message passing. Instead of handling laser sights directly, all you need to do is to broadcast an
`ActorDamaged { actor: Handle<Node>, attacker: Handle<Node> }` message. Laser sight in its turn can subscribe for such
message and handle all incoming messages and compare `attacker` with owner of the laser sight and if the hit was made
by `attacker` flash with some different color. In code this would like so:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{pool::Handle, reflect::prelude::*, uuid::Uuid, visitor::prelude::*},
#     impl_component_provider,
#     scene::node::Node,
#     script::{ScriptContext, ScriptMessageContext, ScriptMessagePayload, ScriptTrait},
#     core::log::Log,
# };
# 
enum Message {
    Damage {
        actor: Handle<Node>,
        attacker: Handle<Node>,
    },
}

#[derive(Default, Clone, Reflect, Visit, Debug)]
struct Projectile;
# 
# impl_component_provider!(Projectile);

impl ScriptTrait for Projectile {
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        // Broadcast the message globally.
        ctx.message_sender.send_global(Message::Damage {
            actor: Default::default(),
            attacker: ctx.handle,
        });
    }
# 
#     fn id(&self) -> Uuid {
#         todo!()
#     }
}

#[derive(Default, Clone, Reflect, Visit, Debug)]
struct LaserSight;
# 
# impl_component_provider!(LaserSight);

impl ScriptTrait for LaserSight {
    fn on_start(&mut self, ctx: &mut ScriptContext) {
        // Subscript to messages.
        ctx.message_dispatcher.subscribe_to::<Message>(ctx.handle);
    }

    fn on_message(
        &mut self,
        message: &mut dyn ScriptMessagePayload,
        _ctx: &mut ScriptMessageContext,
    ) {
        // React to message.
        if let Some(Message::Damage { actor, attacker }) = message.downcast_ref::<Message>() {
            Log::info(format!("{actor} damaged {attacker}",))
        }
    }
# 
#     fn id(&self) -> Uuid {
#         todo!()
#     }
}
```

There are few key parts:

- You should explicitly subscribe script instance to a message type, otherwise messages of the type won't be delivered
to your script. This is done using the message dispatcher: `ctx.message_dispatcher.subscribe_to::<Message>(ctx.handle);`.
This should be done in `on_start` method, however it is possible to subscribe/unsubscribe at runime.
- You can react to messages only in special method `on_message` - here you just need to check for message type using
pattern matching and do something useful.

Try to use message passing in all cases, loose coupling significantly improves code quality and readability, however
in simple projects it can be ignored completely.