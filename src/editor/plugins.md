# Editor Plugins

**WARNING: This article is not finished**

It is possible to extend editor functionality by custom plugins. This chapter will explain how to create one and how
editor plugins interact with the editor itself.

## Basic Concepts

There are few basic concepts that must be known before start writing an editor plugin.

1) MVC - the editor uses classic [MVC](https://en.wikipedia.org/wiki/Model%E2%80%93view%E2%80%93controller)
(model-view-controller) pattern. This means that the editor always "renders" the actual state of your data model and
its UI is used only to show the data - it does not store anything. Any user change forces the editor to sync the UI
with the new data.
2) Commands - the editor usually operates on scenes (there could be multiple opened scenes, but only one active) and any
modification of their content **must** be done via _commands_. [Command](https://en.wikipedia.org/wiki/Command_pattern) 
is a standard pattern that encapsulates an action. Command pattern is used for undo/redo functionality.
3) Preview Mode - sometimes there's a need to preview results in the scene itself, for example if you're making an 
animation editor plugin of some sort. Any changes to scene nodes done in the preview mode will be discarded after leaving
this mode.

Typical update iteration of the editor looks like this: execute scheduled commands, sync the UI with the new state
of the entities, sleep until new commands. If the preview mode is active, the editor will be always active (see 
respective section below for more info).

## Plugin

As an example, we'll create a plugin that will edit a script of a scene node. The script itself will contain a list
of points which forms a line in 3D space. Our plugin will allow to edit position of these points in 3D space using 
movement gizmo, like you move scene nodes. Despite the fact that it is possible to edit the points using Inspector,
is much more comfortable to edit them and see where they're directly in the scene previewer. A good tool is the one that 
saves time. Our script looks like this:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:my_script}}
```

All editor plugins must implement `EditorPlugin` trait, all methods of which are optional. For our purposes we'll use 
only a few of them - `on_message`, `on_update`, `on_sync_to_model`. See the API docs for `EditorPlugin` for more info
about other methods. Typical plugin definition could look like this:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:plugin_definition}}
{{#include ../code/snippets/src/editor/plugins.rs:plugin_impl_1}} 
{{#include ../code/snippets/src/editor/plugins.rs:plugin_impl_2}} 
```

Every plugin must be registered in the editor, it could be done from `editor` crate of your project. Simply add the 
following code after editor's initialization:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:plugin_registration}}
```

Our plugin will work with scene nodes that has particular script type, and we need to know a handle of object that is 
suitable for editing via our plugin, this is where `on_message` could be useful:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:selection_1}}
{{#include ../code/snippets/src/editor/plugins.rs:selection_2}}
{{#include ../code/snippets/src/editor/plugins.rs:selection_3}}
```

It is quite verbose, but in general it is very straightforward. We're fetching the active scene first, then checking
selection type of to be graph selection (there are a number of selection types), then checking that the scene is
game scene (there's also `UiScene`). All that is left to do is to iterate over selected scene nodes and check if
one of them has our script. Once node selection is done, we can write our own interaction mode to  

## Interaction Modes and Visualization

We need a way to show the points of the line in the scene previewer. The editor uses standard scene nodes for this, and
they all live under a "secret" root node (it is hidden in World Viewer, that's why you can't see it there). The good
approach for visualization is just a custom structure with a few methods:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:line_points_gizmo}}
```

`sync_to_model` method can be called on every frame in `update` method of the interaction mode (see below) - it tracks 
the number of scene nodes representing points of the line and if there's mismatch, it recreates the entire set. 
`remove_points` should be used when the gizmo is about to be deleted (usually together with the interaction mode).

All interaction with scene nodes should be performed using interaction modes. Interaction mode is a tiny abstraction layer,
that re-routes input from the scene previewer to the modes. We'll create our own interaction mode that will allow
us to move points of the line. Every interaction mode must implement `InteractionMode` 
[trait](https://docs.rs/fyroxed_base/latest/fyroxed_base/interaction/trait.InteractionMode.html). Unfortunately, the
editor's still mostly undocumented, due to its unstable API. There are quite a lot of methods in this trait:

- `on_left_mouse_button_down` - called when left mouse button was pressed in the scene viewer.
- `on_left_mouse_button_up` - called when left mouse button was released in the scene viewer.
- `on_mouse_move` - called when mouse cursor moves in the scene viewer.
- `update` - called every frame (only for active mode, inactive modes does are not updated).
- `activate` - called when an interaction mode became active.
- `deactivate` - called when an interaction mode became inactive (i.e. when you're switched to another mode).
- `on_key_down` - called when a key was pressed.
- `on_key_up` - called when a key was released.
- `handle_ui_message` - called when the editor receives a UI message
- `on_drop` - called on every interaction mode before the current scene is destroyed.
- `on_hot_key_pressed` - called when a hotkey was pressed. Could be used to switch sub-modes of interaction mode.
For example, tile map editor has single interaction mode, but the mode itself has draw/erase/pick/etc. sub modes which
could be switched using `Ctrl`/`Alt`/etc. hotkeys.
- `on_hot_key_released` - called when a hotkey was released.
- `make_button` - used to create a button, that will be placed.
- `uuid` - must return type UUID of the mode.

Every method has its particular use case, but we'll use only a handful of them. Let's create a new interaction mode:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:interaction_mode_definition}}
```

To create an interaction mode all that is needed is to add the following lines in `on_message`, right after 
`self.node_handle = *node_handle;`:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:interaction_mode_create}}
```

The mode must be deleted when we deselect something else, it could be done on `Message::SelectionChanged`:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:gizmo_destroy}}
```

Now onto the `InteractionMode` trait implementation, let's start by adding implementation for `make_button` method:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:make_button}}
```

There's nothing special about it - it uses built-in function, that creates a button with an image and a tooltip. You
could use any UI widget here that sends `ButtonMessage::Click` messages on interaction. Now onto the `on_left_mouse_button_down`
method:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:on_left_mouse_button_down}}
```

It is responsible for two things: it handles picking of scene nodes at the cursor position, and it is also changes 
currently selected point. Additionally, it creates dragging context if one of the axes of the movement gizmo was clicked
and there's some point selected.

When there's something to drag, we must use new mouse position to determine new location for points in 3D space. There's
`on_mouse_move` for that:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:on_mouse_move}}
```

The dragging could be finished simply by releasing the left mouse button:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:on_left_mouse_button_up}}
```

This is where the action must be "confirmed" - we're creating a new command and sending it for execution in the 
command stack of the current scene. The command used in this method could be defined like so:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:command}}
```

See the next section for more info about commands and how they interact with the editor.

The next step is to update the gizmo on each frame:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:update}}
```

## Commands

As was mentioned previously, any modification to scene node's content (including scripts) must be done using commands.
Commands encapsulates an "atomic" action, this could be simple property or collection modification or something complex,
that involves heavy calculations and so on. The editor has a command stack that executes incoming commands and saves them
for potential undo. The stack has a top command, when new command is added to the stack, it removes all command prior the
top and makes the new command the top one. Every removed command is finalized (see below).

There are two ways of using commands: use reflection-based command, or use custom command. Reflection-based commands
usually used when you need to set a new value to some property. On the other hand, custom commands could perform complex
actions, that cannot be done using reflection-based command. The previous section contains an example of custom command,
they're quite verbose and require decent amount of boilerplate code. 

### Custom Commands

Custom commands is the best way to get better understanding of command system and how it works. This section explains
how to create custom commands and how they're executed. Each command must implement `Command` trait which looks like
this:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:command_trait}}
```

This chapter already showed an example of a custom command:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:command}}
```

The main idea is very simple, `execute` must do the required change and `revert` must undo it. There's one special 
method that has very limited use, but it cannot be avoided. `finalize` is used to return reserved resources back to 
where they were obtained from. Typically, it is pool handles that can be reserved for further use. If they won't be
returned, pool will have empty unused entries forever. 

### Reflection-based Commands

There are three main types of reflection-based commands that can be used to manipulate scene objects:

#### `SetPropertyCommand` 

Sets a new value for a property at the given path. This command cannot change the size of collections (add or remove 
items), the next two commands are exactly for this (see next subsections). This is how you could use this command to
change position of a point at index 1: 

```rust
{{#include ../code/snippets/src/editor/plugins.rs:set_point_1}}
```

The first argument is a path to variable, it could be any "depth" and support enum variants, indices, etc: 
`foo.bar.baz@Some.collection[123].stuff`. Enum variants are marked by `@` sign. The second argument is a new value for
the property. It could be any object that implements `Reflect` trait, in our case it is `Vector3<f32>`. The last argument
is entity getter function. Its purpose is to provide a reference to an object in which the reflection system will search
for the property with the given name.

#### `AddCollectionItemCommand`

Adds a new collection item command at the given path. The collection could be anything that implements `ReflectList` 
trait (`Vec`, `ArrayVec`, custom types) or `ReflectHashMap` trait (`HashMap`, `FxHashMap`, custom types). Typical usage
is something like this:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:add_collection_element}}
```

The meaning of each argument is the same as in `SetPropertyCommand` command. 

#### `RemoveCollectionItemCommand` 

Removes an item from a collection by the given index. The collection could be anything that implements `ReflectList` 
trait (`Vec`, `ArrayVec`, custom types) or `ReflectHashMap` trait (`HashMap`, `FxHashMap`, custom types). In case of 
hash maps, the index cannot be used reliably, because hash maps do not have an ability to be randomly indexed. To remove
the exact element at the index, you must ensure that `hash_map.iter().nth(index)` corresponds to the item and only then 
use this index in the command. Typical usage is something like this:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:remove_collection_element}}
```

The first argument in this command a name of the collection property, the second - item index, and the third is the
entity getter. See `SetPropertyCommand` for more info.

## Contextual Panels

In some cases you may want to have a panel, that opens when you select a node with the script. This panel could contain
any UI elements. For educational purposes, we'll create a contextual panel that will create a line using two points
and a number of segments.

(TODO)

## Preview Mode

Preview mode allows you to see objects in dynamic directly in the scene preview window. It is a special mode of the 
editor, where it updates and renders every frame and power-saving mode is disabled. It could be useful to preview
various animations.

(TODO)