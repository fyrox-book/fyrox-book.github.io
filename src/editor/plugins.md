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

All interaction with scene nodes should be performed using interaction modes. Interaction mode is tiny abstraction layer,
that re-routes input from the scene previewer to the modes. We'll create our own interaction mode that will allow
us to move points of the line. Typical interaction mode looks like this:

```rust
{{#include ../code/snippets/src/editor/plugins.rs:interaction_mode}}
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

(TODO)

## Commands

As was mentioned previously, any modification to scene node's content (including scripts) must be done using commands.
There are two ways of using commands: use reflection-based command, or use custom command. Reflection-based commands
usually used when you need to set a new value to some property. On the other hand, custom commands could perform complex
actions, that cannot be done using reflection-based command. In our case we'll be using both command types.  

(TODO)

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