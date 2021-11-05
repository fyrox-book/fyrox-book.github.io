# Base node

Base node is a scene node that stores hierarchical information (a handle to the parent node and a set of handles
to children nodes), local and global transform, name, tag, lifetime, etc. It has self-describing name - it
is used as a base node for every other scene node (via composition).

It has no graphical information, so it is invisible all the time, but it is useful as a "container" for children
nodes.

## How to create

Use the `BaseBuilder` to create an instance of the Base node:

```rust
# extern crate rg3d;
# use rg3d::scene::{base::BaseBuilder, Scene};
# fn build_node(scene: &mut Scene) {
let handle = BaseBuilder::new().build(&mut scene.graph);
# }
```

## Building a complex hierarchy

To build a complex hierarchy of some nodes, use `.with_children()` method of the `BaseBuilder`, it allows you
to build a hierarchy of any complexity:

```rust
# extern crate rg3d;
# use rg3d::scene::{base::BaseBuilder, camera::CameraBuilder, Scene};
#
# fn build_node(scene: &mut Scene) {
let handle = BaseBuilder::new()
    .with_children(&[
        CameraBuilder::new(BaseBuilder::new()).build(&mut scene.graph),
        BaseBuilder::new()
            .with_children(&[BaseBuilder::new().build(&mut scene.graph)])
            .build(&mut scene.graph),
    ])
    .build(&mut scene.graph);
# }
```

Note that when we're building a `Camera` instance, we're passing a new instance of `BaseBuilder` to it, this
instance can also be used to set some properties and a set of children nodes.

The "fluent syntax" is not mandatory to use, the above code snipped could be rewritten like this:

```rust
# extern crate rg3d;
# use rg3d::scene::{base::BaseBuilder, camera::CameraBuilder, Scene};
# 
# fn build_node(scene: &mut Scene) {
let camera = CameraBuilder::new(BaseBuilder::new()).build(&mut scene.graph);

let child_base = BaseBuilder::new().build(&mut scene.graph);

let base = BaseBuilder::new()
    .with_children(&[child_base])
    .build(&mut scene.graph);

let handle = BaseBuilder::new()
    .with_children(&[camera, base])
    .build(&mut scene.graph);
# }
```

However, it looks less informative, because it loses the hierarchical view and it is harder to tell the relations
between objects.

## Transform

Base node has a local transform that allows you to translate/scale/rotate/etc. your node as you want to. For example,
to move a node at specific location you could use this:

```rust
# extern crate rg3d;
# use rg3d::{
#    core::{algebra::Vector3, pool::Handle},
#    scene::{node::Node, Scene},
# };
#
# fn translate_node(scene: &mut Scene, node_handle: Handle<Node>) {
scene.graph[node_handle]
    .local_transform_mut()
    .set_position(Vector3::new(1.0, 0.0, 2.0));
# }
```

You could also chain multiple `set_x` calls, like so:

```rust
# extern crate rg3d;
# use rg3d::{
#    core::{algebra::Vector3, pool::Handle},
#    scene::{node::Node, Scene},
# };
#
# fn transform_node(scene: &mut Scene, node_handle: Handle<Node>) {
scene.graph[node_handle]
    .local_transform_mut()
    .set_position(Vector3::new(1.0, 0.0, 2.0))
    .set_scale(Vector3::new(2.0, 2.0, 2.0))
    .set_rotation_offset(Vector3::new(1.0, 1.0, 0.0));
# }
```