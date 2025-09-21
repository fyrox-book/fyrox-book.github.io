# Keyboard Input

There are two major ways to check the keyboard state - the simple and the event-based ones.

## Simple way

The simplest way to check if a key was pressed/released or if it is still down is three methods of `InputState`:

- `is_key_down` - returns `true` if the specified key is pressed, `false` - otherwise.
- `is_key_pressed` - returns `true` if the specified key was pressed in the current frame, `false` - otherwise.
  This method will return `false` if the key is still pressed in the next frame. This is useful to check if a key
  was pressed and some action, but do not repeat the same action over and over until the key is released.
- `is_key_released` - returns `true` if the specified key was released in the current frame, `false` - otherwise.
  This method will return `false` if the key is still released in the next frame. This is useful to check if a
  key was released and some action, but do not repeat the same action over and over until the key is pressed.

Typical usage of these methods is the following.

```rust,no_run
{{#include ../code/snippets/src/input/keyboard_simple.rs:keyboard}}
``` 

## Event-based approach

The more advanced approach is to use keyboard events directly. This may seem harder to maintain than the simple
approach with functions like `is_key_down`, but in reality it is slightly more verbose. The main advantage is
that this approach allows you to carefully select which entity will receive an event.

Keyboard input events can be handled by listening to `WindowEvent::KeyboardInput`, for example you can check for A, D
keys and save their state in some variables in your script. These variables will tell the script that an entity, to
which the script was assigned, should move in a certain direction. This could be expressed like so:

```rust,no_run
{{#include ../code/snippets/src/input/keyboard.rs:keyboard}}
``` 

The main method here is `on_os_event`, which listens for keyboard events and modifies script variables accordingly.
These two variables are then used in the `on_update` method to move the entity, to which the script is assigned to.