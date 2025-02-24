# Synchronization

Pretty much every game has dozens of changing objects, and you need to synchronize their state across all the clients, 
so their state matches the server state. In the simplest case, all that you need is to collect information about node
positions and rotations and send them to the clients:

```rust,no_run
{{#include ../code/snippets/src/net/client_server.rs:simple_syncing}}
```

If the number of objects is low, it will work fine on most machines and network connections. However, this algorithm
is somewhat dumb, because it gathers a ton of useless information. For example, most of the scene objects remain 
static, and thus their "new" position and rotation can be ignored because it is not new at all—it's the same
as in the previous frame.

## Delta Compression

Delta compression is used to minimize the amount of data being sent from the server to clients and vice versa. It is
a simple, yet very efficient way of reducing the required network bandwidth and performance requirements of your game.

The simplest delta compression could be implemented like this:

```rust,no_run
{{#include ../code/snippets/src/net/client_server.rs:syncing_with_delta_compression}}
```

## Physics

Client-server model assumes that the server has the priority in calculations and its state has the top priority 
than clients. This applies to physics simulation as well, no need to run expensive physics simulation if its state
is overwritten by the server state anyway. That being said, the client must have physics simulation disabled. This can 
be done like this:

```rust,no_run
{{#include ../code/snippets/src/net/client_server.rs:disable_physics}}
```



