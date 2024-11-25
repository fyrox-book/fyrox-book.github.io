# Events

Resource manager is able to notify its subscribers about specific events of resources. There are four kinds of resource
events:

1) `Loaded` - occurs when a resource was fully loaded without any errors. 
2) `Reloaded` - occurs when a resource was already fully loaded, but was reloaded by an explicit request.
3) `Added` - occurs when a resource was just added to the resource manager. This event is fired right after a resource
was requested from the manager.
4) `Removed` - occurs when a resource was removed from the resource manager. This event is fired when the resource 
manager removes and unloads an unused resource. 

To subscribe for resource events use the event broadcaster:

```rust
{{#include ../code/snippets/src/resource/events.rs:subscribe_to_events}}
```