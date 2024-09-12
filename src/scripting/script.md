# Scripts

Script - is a container for game data and logic that can be assigned to a scene node. Fyrox uses Rust for scripting, 
so scripts are as fast as native code. Every scene node can have any number of scripts assigned.

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
{{#include ../code/snippets/src/scripting/example.rs:example_script}}
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
- `ComponentProvider` - gives access to inner fields of the script marked with `#[component(include)]` attribute.

`#[visit(optional)]` attribute is used to suppress serialization errors when some fields are missing or changed. 

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
{{#include ../code/snippets/src/scripting/example.rs:register}}
```

Every script type (`MyScript` in the code snippet above, you need to change it to your script type) must be registered using 
[ScriptConstructorsContainer::add](https://docs.rs/fyrox/latest/fyrox/script/constructor/struct.ScriptConstructorContainer.html#method.add) 
method, which accepts a script type as a generic argument and its name, that will be shown in the editor. The name can be 
arbitrary, it is used only in the editor. You can also change it at any time, it won't break existing scenes.

## Script Attachment

To assign a script and see it in action, run the editor, select an object and find `Scripts` property in the Inspector.
Click on a small `+` button and select your script from the drop-down list on the newly added entry. To see the script 
in action, click "Play/Stop" button. The editor will run your game in separate process with the scene active in the editor.

The script can be attached to a scene node from code:

```rust, no_run
{{#include ../code/snippets/src/scripting/example.rs:add_my_script}}
```

Initialization as well as update of newly assigned script will happen on next update tick of the engine.

## Script Context

Script context provides access to the environment that can be used to modify engine and game state from scripts. Typical
content of the context is something like this:

```rust,no_run
{{#include ../code/snippets/src/scripting/context.rs:context}}
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
- `scene_handle` - a handle of a scene the script instance belongs to.
- `resource_manager` - a reference to resource manager, you can use it to load and instantiate assets. 
- `message_sender` - a message sender. Every message sent via this sender will be then passed to every 
`ScriptTrait::on_message` method of every script.
- `message_dispatcher` - a message dispatcher. If you need to receive messages of a particular type, you must subscribe
to a type explicitly.
- `task_pool` - task pool for asynchronous task management.
- `graphics_context` - Current graphics context of the engine.
- `user_interfaces` - a reference to user interface container of the engine. The engine guarantees that there's
at least one user interface exists. Use `context.user_interfaces.first()/first_mut()` to get a reference to it.
- `script_index` - index of the script. Never save this index, it is only valid while this context exists!

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

If a scene node has multiple scripts assigned, then they will be processed as described above in the same order as they
assigned to the scene node.

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
{{#include ../code/snippets/src/scripting/mod.rs:message_passing}}
```

There are few key parts:

- You should explicitly subscribe script instance to a message type, otherwise messages of the type won't be delivered
to your script. This is done using the message dispatcher: `ctx.message_dispatcher.subscribe_to::<Message>(ctx.handle);`.
This should be done in `on_start` method, however it is possible to subscribe/unsubscribe at runime.
- You can react to messages only in special method `on_message` - here you just need to check for message type using
pattern matching and do something useful.

Try to use message passing in all cases, loose coupling significantly improves code quality and readability, however
in simple projects it can be ignored completely.

## Accessing Other Script's Data

Every script "lives" on some scene node, so to access a script data from some other script you need to know
a handle of a scene node with that script first. You can do this like so:

```rust,no_run
{{#include ../code/snippets/src/scripting/mod.rs:access_other_1}}
{{#include ../code/snippets/src/scripting/mod.rs:access_other_2}}
```

In this example we have the two script types: `MyScript` and `MyOtherScript`. Now imagine that we have two scene
nodes, where the first one contains `MyScript` and the second one `MyOtherScript`. `MyScript` knows about
the second node by storing a handle of in `second_node` field. `MyScript` waits until `MyOtherScript` will count
its internal counter to `60.0` and then prints a message into the log. This code does immutable borrowing and 
does not allow you to modify other script's data. If you a mutable access, then use `try_get_script_of_mut`
method (or `try_get_script_mut` for the alternative code).

`second_node` field of the `MyScript` is usually assigned in the editor, but you can also find the node in
your scene by using the following code:

```rust,no_run
{{#include ../code/snippets/src/scripting/mod.rs:find_node}}
```

This code searches for a node with `SomeName` and assigns its handle to the `second_node` variable in the script
for later use.

## Accessing Plugins From Scripts

Sometimes there's a need to access plugin data from scripts, there may be various reasons for that, for example 
you may need to register a bot in the list of bots. This list could then be used for AI to search targets without 
searching in the entire scene graph at every frame.

Accessing plugins from scripts is very easy, all you need to do is to call `get/get_mut` method from `ctx.plugins`:

```rust,no_run
{{#include ../code/snippets/src/scripting/mod.rs:access_plugin}}
```

In this example the Bot script registers itself in a global list of bots on start, and unregisters on destruction.
`update` is then used to search for targets in that list. 

In multiplayer games, plugin could store server/client instances and scripts could easily access them to send messages
across the network for other players. In general, you could use plugins as an arbitrary, global data storage for your
scripts.