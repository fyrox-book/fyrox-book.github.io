# Keyboard Input 

Keyboard input events can be handled by listening to `WindowEvent::KeyboardInput`, for example you can check for A, D
keys and save their state in some variables in your script. These variables will tell the script that an entity, to
which the script was assigned, should move in a certain direction. This could be expressed like so:

```rust 
{{#include ../code/snippets/src/input/keyboard.rs:keyboard}}
``` 

The main method here is `on_os_event`, which listens for keyboard events and modifies script variables accordingly.
These two variables are then used in the `on_update` method to move the entity, to which the script is assigned to.