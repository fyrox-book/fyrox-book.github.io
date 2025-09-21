# Input 

This chapter explains how the input handling in the engine works. The input system based on various events, that comes
to the window from the OS. It could be mouse events (such as mouse motion, button clicks), keyboard events, touchpad
events, etc. 

There are two major points for event handling: `Plugin::on_os_event` and `Script::on_os_event`, the first one is used
to react to OS events on plugin scale and the latter - on script scale. Here's a list (not full) of the most common
events that could be used in your game (some rare events are omitted):

```rust
{{#include ../code/snippets/src/input/mod.rs:events_example}}
```

As you can see, to do an action in response to an event, all you need to do is to write some code in a desired branch.
You can also put the handler code into a method of your plugin/script and call it instead. 

## Immediate input state fetching

You may be used to much simpler approach of immediate input state fetching, such as `keyboard.is_key_pressed(..)` or
`mouse.position()` - Fyrox has this functionality as well. It is up to you which one to pick, but in general, the 
event-based approach is much more predictable, consumes less CPU resources, and it leads to less convoluted code.
Read the next few chapters to learn about both approaches and find the one that suits you the most.