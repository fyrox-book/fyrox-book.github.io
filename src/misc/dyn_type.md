# Dynamic Types

Dynamic type (dyntype for short) is a user-defined data structure that supports additional features which makes it
available from the editor and serializable to the standard asset format. Dyntypes also supports handles to scene or
ui nodes, and the engine will maintain their correctness automatically. Typical definition of a dyntype could be the
following:

```rust,no_run
{{#include ../code/snippets/src/misc/dyn_type.rs:my_data}}
```

## Registration

Dyntypes must be registered in the engine to be serialiable, otherwise the engine will simply ignore them. This can 
be done like so:

```rust,no_run
{{#include ../code/snippets/src/misc/dyn_type.rs:registration}}
```

## Hot-reloading

Dyntypes are also correctly handled during native code hot reloading; the engine will serialize their content first
before the reload and then deserialize it back in the type instance after the reload is finished. This fact makes 
dyntypes safe in native code hot reloading.