# Basics

This chapter explains what the engine offers for making multiplayer games.

## Protocols

Fyrox has basic support for simple network operations using TCP. This reliable network protocol with guaranteed
delivery, but it may suffer from large latency, thus it may not be suitable for some applications. In particular,
in high pace shooters TCP will result in quite noticeable lag. Such games should use mixed approach of UDP+TCP, where
UDP should be used for "insignificant" data and TCP to send important data. Insignificant data can include the data
that changes every frame (position, rotation, etc.). On the other hand, important data can include player connection
data. Everything related to TCP-based network entities is located in `fyrox::core::net` module.

## Client-Server Architecture

Multiplayer games are usually based on client-server architecture. As stated in the title, it consists of a server
and any number of clients. There's also a subdivision for the server side—it could be a dedicated server or a listen
server.

Dedicated server is an application that is responsible only for game logic and interaction with clients; it does 
not render anything on screen, does not accept any input from a player.

Listen server is a combination of a dedicated server and the client running in the same process (application). In most
cases, this is the most preferable option for typical players, since it allows them to start a server and play as 
a "client" and allow other players to join their server.

Typical structure is quite simple for TCP connections, and it is recommended if you've never ever made a multiplayer
game. The following example implements a listen server (where you have both the server and the client part running in
the same application), which is the easiest kind of server to implement. Its implementation could look like this:

```rust,no_run
{{#include ../code/snippets/src/net/client_server.rs:client_server}}
```

There's a lot of code, but it is very straightforward. There are two main entities here: `Server` and `Client`. 
The server holds a TCP listener, which internally handles handshake network operations that allows to establish 
a reliable connection between the listener and any number of clients. Each such connection is called stream.

The client holds a connection to the server (read: stream). The next interesting part is how the game update loop
is structured:

```rust,no_run
{{#include ../code/snippets/src/net/client_server.rs:update_loop}}
```

At first, the server tries to accept all incoming connections and only then tries to read all the incoming messages.
The client is also trying to read all the incoming messages. The server side will be `None` for other players (non-host).

The listen server could be quite confusing because it has both the server and the client at the same time. If you've
never implemented multiplayer games, it will be even more confusing. The main question that arises very often—how
to manage the server and client at the same time? The answer to this is very simple—the same as with dedicated 
servers: all interaction must be done via messages. The client must never access the server side directly and vice 
versa.

## Messages

Network interaction is usually based on some kind of messages. Message is a simple data storage that contains some
data to perform specific actions. There could be server and client messages, each serves different purpose. For example,
a server message is sent by a server and interpreted on a client side. It could include commands to load specific level, 
add other players, synchronize the state of objects, and so on. A client message is sent by a client to a server and 
interpreted on a server side. For example, it could include player input state. A typical set of messages could look like
this:

```rust,no_run
{{#include ../code/snippets/src/net/client_server.rs:messages}}
```

## Stable IDs

When it comes to game entities (scene nodes), it is essential to have unique IDs for them, to be able to synchronize their
states across multiple clients. The engine uses UUID for this purpose. Why can't we just use node handles for this? The
main reason is that it is unreliable, because scene node handles are basically just an index + generation id. The index
part could be different for different clients for the same scene node. Usually it happens when relatively heavy prefabs 
are instantiated in a separate async task (see the respective section below). If we assign a unique id for the scene node 
across all clients, and it will make client-server synchronization reliable, since we can use this id to find the entity.

There are a number of ways to assign unique ids for scene nodes. The easiest is to use `.with_id` method when building
a scene node:

```rust,no_run
{{#include ../code/snippets/src/net/mod.rs:create_node_with_id}}
```

What if we want to instantiate a prefab on all clients, how can we ensure that the ids of the entire hierarchy will
be the same? The easiest way to do this is to use a pair of built-in methods - `Model::generate_ids` and `.with_ids()`.
Typical structure could look like this. At first, define a new message type:

```rust,no_run
{{#include ../code/snippets/src/net/mod.rs:prefab_message}}
```

It contains all that is needed to instantiate a new prefab on the client side. The server side needs the following
function:

```rust,no_run
{{#include ../code/snippets/src/net/mod.rs:create_prefab_message}}
```

At first, it loads the desired prefab, then generates ids for all its sub-nodes and creates a new message with this
information. This message is then can be sent over the network to all clients. When a client receives such a message,
it could handle it like this:

```rust,no_run
{{#include ../code/snippets/src/net/mod.rs:on_prefab_message_received}}
```

The crucial part here is `.with_ids(..)` call, which forces the prefab's sub-nodes to have the provided set of ids.
This way all the nodes across all clients will have the same unique id per node.

## Dealing with Async

Leaving your game responsive while performing some computationally heavy work is crucial for smooth gameplay. Some of
the heaviest tasks in pretty much any game is loading an asset. The engine has a built-in task system to help avoid 
lags during this process.

```rust,no_run
{{#include ../code/snippets/src/net/mod.rs:on_prefab_message_received_async}}
```

## Synchronization

If your game has a lot of moving objects, you may want to synchronize their state across all the clients so their
state matches the server state.

**TODO**

## Example

There's one quite large multiplayer game built with Fyrox - [Fish Folly](https://github.com/mrDIMAS/FishFolly). It is
a "Fall Guys"-like platformer.