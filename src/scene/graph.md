# Graph

Graph is a set of objects with hierarchical relationships between each object. It is one of the most important 
entities in the engine. Graph takes care of your scene objects and does all the hard work for you.

## How to create

You don't need to create a graph manually, every scene has its own instance of the graph. It can be accessed pretty
easily: `scene_ref.graph`

## Adding nodes

There are two ways of adding nodes to the graph, either using _node builders_ or manually by calling `graph.add_node`.

### Using node builders

Every node in the engine has its respective builder which can be used to create an instance of the node. Using
builders is a preferable way to create scene nodes. There are following node builders:

1) `BaseBuilder` - creates an instance of base node. See [Base node](./base_node.md) for more info.
2) `PivotBuilder` - creates an instance of pivot node. See [Base node](./base_node.md) for more info.
3) `CameraBuilder` - creates an instance of camera node. See [Camera node](./camera_node.md) for more info.
4) `MeshBuilder` - creates an instance of mesh node. See [Mesh node](./mesh_node.md) for more info.
5) `LightBuilder` - creates an instance of light node. See [Light node](./light_node.md) for more info.
6) `SpriteBuilder` - creates an instance of sprite node. See [Sprite node](./sprite_node.md) for more info.
7) `ParticleSystemBuilder` - creates an instance of particle system node. 
See [Particle system node](./particle_system_node.md) for more info.
8) `TerrainBuilder` - creates an instance of terrain node. See [Terrain node](./terrain_node.md) for more info.
9) `DecalBuilder` - creates an instance of decal node. See [Decal node](./decal_node.md) for more info.
10) `RigidBody` - creates an instance of rigid body node. See [Rigid body](../physics/rigid_body.md) for more info.
11) `Collider` - creates an instance of collider node. See [Collider](../physics/collider.md) for more info.
12) `Joint` - creates an instance of joint node. See [Joint](../physics/joint.md) for more info.
13) `Rectangle` - creates an instance of 2D rectangle node. See [Rectangle](./rectangle.md) for more info.

Every builder, other than `BaseBuilder`, accepts `BaseBuilder` as a parameter in `.new(..)` method. Why so?
Because every node (other than Base) is "derived" from Base via composition and the derived
builder must know how to build Base node. While it may sound confusing, it is actually very useful and clear.
Consider this example:

```rust,no_run
{{#include ../code/snippets/src/scene/graph.rs:create_camera}}
```

As you can see, we're creating an instance of BaseBuilder and fill it with desired properties as well as filling
the CameraBuilder's instance properties. This is a very flexible mechanism, allowing you to build complex hierarchies
in a declarative manner:

```rust,no_run
{{#include ../code/snippets/src/scene/graph.rs:create_node}}
```

This code snippet creates a camera for first-person role-playing game's player, it will have a staff in "right-hand"
and a spell in the left hand. Of course all of this is very simplified, but should give you the main idea. Note
that staff and fireball will be children nodes of camera, and when setting their transform, we're actually setting
**local** transform which means that the transform will be relative to camera's. The staff and the spell will move
together with the camera.

### Adding a node manually

For some rare cases you may also want to delay adding a node to the graph, specifically for that purpose, every node 
builder has `.build_node` method which creates an instance of `Node`  but does not add it to the graph.

```rust,no_run
{{#include ../code/snippets/src/scene/graph.rs:create_node_manually}}
```

## How to modify the hierarchy

For many cases you can't use builders to create complex hierarchy, the simplest example of such situation when 
you're creating an instance of some 3D model. If you want the instance to be a child object of some other object,
you should attach it explicitly by using `graph.link_nodes(..)`:

```rust,no_run
{{#include ../code/snippets/src/scene/graph.rs:link_weapon_to_camera}}
```

Here we've loaded a weapon 3D model, instantiated it on scene and attached to _existing_ camera. 

## How to remove nodes

A node could be removed by simply calling `graph.remove_node(handle)`, this method removes the node from the 
graph **with all of its children nodes**. Sometimes this is unwanted behaviour, and you want to preserve children
nodes while deleting parent node. To do that, you need to explicitly detach children nodes of the node you're about
to delete:

```rust,no_run
{{#include ../code/snippets/src/scene/graph.rs:remove_preserve_children}}
```

After calling this function, every child node of `node_to_remove` will be detached from it and the `node_to_remove`
will be deleted. `remove_node` has some limitations: it cannot be used to extract "sub-graph" from the graph, it
just drops nodes immediately. 