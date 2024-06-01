# Custom Scene Node

Sometimes there is a need to have custom scene nodes, it is possible to do, but it requires quite a lot of boilerplate
code.

```rust,no_run
{{#include ../code/snippets/src/scene/custom.rs:custom_node}}
```

Once the node is defined, you can create is as usual and put in the graph:

```rust,no_run
{{#include ../code/snippets/src/scene/custom.rs:add_custom_node}}
```

## Limitations

Scene nodes have no access to outer context, this means that you cannot reference any data that is located outside 
graph easily. You still can define a global variable that **will** be accessible, but it is considered as a hack and
should be avoided. If you want to add custom logic to scene nodes, then you should use scripts instead. Custom nodes
are intended for very specific use cases, such as adding "data sources" for renderer, etc.

## Editor support

For now, you cannot create custom nodes from the editor. This will be available in future versions of the engine.