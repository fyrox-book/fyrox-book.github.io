# Custom Scene Node

Sometimes there is a need to have custom scene nodes, it is possible to do, but it requires quite a lot of boilerplate
code.

```rust,no_run
# extern crate fyrox;
use fyrox::{
    core::{
        reflect::prelude::*,
        math::aabb::AxisAlignedBoundingBox,
        pool::Handle,
        uuid::{uuid, Uuid},
        variable::InheritError,
        visitor::prelude::*,
    },
    engine::resource_manager::ResourceManager,
    scene::{
        base::Base,
        node::{Node, NodeTrait},
    },
};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Reflect, Visit, Debug)]
pub struct CustomNode {
    base: Base,
}

impl Deref for CustomNode {
    type Target = Base;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for CustomNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl NodeTrait for CustomNode {
    fyrox::impl_query_component!();

    fn local_bounding_box(&self) -> AxisAlignedBoundingBox {
        self.base.local_bounding_box()
    }

    fn world_bounding_box(&self) -> AxisAlignedBoundingBox {
        self.base.world_bounding_box()
    }

    fn id(&self) -> Uuid {
        // Provide unique id for serialization needs. It must be unique, use https://www.uuidgenerator.net/
        // to generate one.
        uuid!("f592e7f7-5e34-4043-9226-407c7457bb48")
    }
}
```

Once the node is defined, you can create is as usual and put in the graph:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     scene::{camera::Camera, graph::Graph, node::Node},
# };
# type CustomNode = Camera;
# 
fn add_custom_node(graph: &mut Graph) -> Handle<Node> {
    graph.add_node(Node::new(CustomNode::default()))
}
```

## Limitations

Scene nodes have no access to outer context, this means that you cannot reference any data that is located outside 
graph easily. You still can define a global variable that **will** be accessible, but it is considered as a hack and
should be avoided. If you want to add custom logic to scene nodes, then you should use scripts instead. Custom nodes
are intended for very specific use cases, such as adding "data sources" for renderer, etc.

## Editor support

Current `NodeTrait` implementation clearly states that you cannot edit properties of your objects from the editor. 
This is a bug and will be fixed in future versions.
