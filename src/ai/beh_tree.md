# Behavior Trees

Naive approach for implementing logic of NPCs using a bunch of `if`s and flags often leads to very convoluted
code. Behavior trees aims to solve this issue by creating a tree where each node represents an action and
a condition to select the next execution node.

## Overview

The behavior tree consists of at least one node, where each node can do something useful and define execution branch. A
node can contain either one of built-in behavior, or a user-defined behavior.

Built-in nodes defined by `BehaviorNode`:

- `BehaviorNode::Root` - entry point of the tree, can contain only one child node.
- `BehaviorNode::Composite` - composite behavior node that contains multiple children nodes. The actual
  behavior of this node is defined by its kind, which can be one of the following:
    - `CompositeNodeKind::Sequence` - node will execute children nodes consecutively until `Status::Failure` is returned
      from any descendant node. In other words `Sequence` implements **AND** logical function.
    - `CompositeNodeKind::Selector` - node will execute children until `Status::Success` is returned from any descendant
      node. In other worlds `Selector` implements **OR** logical function.
- `BehaviorNode::Inverter` - A node, that inverts its child state (`Status::Failure` becomes `Status::Success` and
  vice versa, `Status::Running` remains unchanged)
- `BehaviorNode::Leaf` - a node with user-defined logic, contains an instance of a type that implements `Behavior`
  trait.

## `Behavior` trait

Each node implements the `Behavior` trait, which defines the actual logic.

```rust
pub trait Behavior<'a>: BaseBehavior {
    /// A context in which the behavior will be performed.
    type Context;

    /// A function that will be called each frame depending on
    /// the current execution path of the behavior tree it belongs
    /// to.
    fn tick(&mut self, context: &mut Self::Context) -> Result<Status, GameError>;
}
```

The `Context` is typically `PluginContext`, but can be any type that may contain additional information. The logic of
the behavior is defined by the contents of `tick` method. This method accepts a context and returns execution result.
On success, it returns one of the `Status` enumeration variants or `GameError` on failure (
see [error handling chapter](../scripting/error.md) for more info). `Status` enumeration has the following variants:

- `Status::Success` - an action was successful.
- `Status::Failure` - failed to perform an action.
- `Status::Running` - need another iteration to perform an action.

## Example

The following example shows a simple behavior tree for a "bot" that can walk, open doors, step through doorways and
close the door after itself.

```rust ,no_run
{{#include ../code/snippets/src/ai/beh_tree.rs:beh_tree}}
```