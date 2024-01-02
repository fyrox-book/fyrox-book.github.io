# Mouse Input

Mouse input is usually used to control a camera rotation, to pick objects in game world, etc. Let's take a look at 
the most common use cases.

## Mouse Motion

The following example shows how to use raw mouse events to rotate an object. It could also be used to rotate a camera
in your game (with slight modifications).

```rust
{{#include ../code/snippets/src/input/mouse.rs:mouse}}
```

This example consists of two main parts - `on_os_event` and `on_update` methods. The first one is called when some 
event comes to the main window, and we need to check if this event is `DeviceEvent::MouseMotion`. After that, we're taking
relative offsets (`dx`, `dy`) and modifying the `pitch`, `yaw` variables accordingly. `on_update` method is called 
every frame and it is used to apply `pitch` and `yaw` values to the scene node the script is assigned to.

## Mouse Buttons

The following example shows how to handle events from mouse buttons.  

```rust
{{#include ../code/snippets/src/input/mouse.rs:clicker}}
```

At first, we're checking for `WindowEvent::MouseInput` and creating respective bindings to its internals (`button`, `state`)
and then all we need to do, is to check if the button was pressed and if so, which one.
