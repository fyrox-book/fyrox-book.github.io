# Rigid body node

Rigid body node is the one of main physical entities in the engine. Rigid body nodes can be affected by gravity, 
external forces and other rigid bodies. Use rigid body node everywhere you need natural physical behaviour for
your objects.

## How to create

Use RigidBodyBuilder to create a rigid body instance:

```rust,no_run
{{#include ../code/snippets/src/scene/rigid_body.rs:create_cube_rigid_body}}
```

## Colliders

Rigid body must have at least one collider to participate in simulation properly, multiple colliders can be used to
create complex shapes from simple shapes, you can create concave objects this way. Every collider **must** be a direct
child node of a rigid body. In the editor it could look like this:

![colliders](./colliders.png)

Note that, `Box` node here is an instance of `Rigid Body 2D`, and it has `Collider 2D` as a child and some sprite. This 
structure (when a rigid body has a collider as a child) is mandatory for physics engine to work correctly! Collider
won't work (participate in physical simulation) without a rigid body and a rigid body won't work without a collider.
This applied to both 2D and 3D.

Keep in mind, that your graphical representation of an object (some node like `Mesh`, `Sprite`, etc.) must be attached
to a rigid body. Otherwise, the rigid body will move, but the graphical representation won't. You can also arrange
it other way around: a graphical node can have rigid body with a collider, but that requires the rigid body to be 
kinematic. This is used to create [hit boxes](./collider.md#using-colliders-for-hit-boxes), or any other things 
that should have physical representation, but move together with graphical node.

## Force and torque

You can apply forces and torque to any rigid body, but only dynamic bodies will be affected. There is two ways of
applying force to a rigid body: at center of mass or at particular point at the body:

```rust,no_run
{{#include ../code/snippets/src/scene/rigid_body.rs:apply_force_and_torque}}
```

## Kinematic rigid bodies

Sometimes you may want to have direct control over position/rotation of a rigid body and tell the physics engine to not
do simulation for the body. This can be achieved by making the rigid body _kinematic_:

```rust,no_run
{{#include ../code/snippets/src/scene/rigid_body.rs:create_kinematic_rigid_body}}
```

## Continuous collision detection

Fast-moving rigid bodies can "fly through" other objects (for example a bullet can completely ignore walls if it is 
moving too fast), this happens because of discrete calculation. This can be fixed by using continuous collision detection,
to enable it use either `.with_ccd_enabled(state)` of `RigidBodyBuilder` or `.set_ccd_enabled(state)` of `RigidBody`.

## Dominance

Dominance allows you to set a priority of forces applied to rigid bodies. It defines which rigid body can affect what rigid
body, for example you can set the highest dominance for actors and leave dominance of  everything else at zero, this way
actors will be able to push any other dynamic bodies, but dynamic bodies won't affect actors. This is useful when you don't
want your actors be pushed by surrounding objects (like if someone throws a box at an actor, it will stay still if it has
higher dominance)

## 2D rigid bodies

2D rigid bodies have no difference with 3D, except the simulation happens in oXY plane and Z coordinate is ignored.