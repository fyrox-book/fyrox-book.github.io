# Joint

Joint is a configurable link between two rigid bodies, it restricts relative motion of two bodies. Fyrox provides a 
fixed set of joints that are suitable for various applications.

- Fixed Joint - hard link between two bodies, it is the same is if two rigid bodies were "welded" to each other with 
a metal rod.
- Revolute Joint - restricts all translational movement and any rotations around Y and Z axes, but leaves rotation
around local X axis free. An example of the joint from real world is a door hinge, it allows the door to rotate around 
single axis, but not move.
- Prismatic Joint - restricts all rotations, movement is allowed along single axis (local X of the joint). An example
of the joint from real world could be a slider that supports drawers on a table.
- Ball Joint - restricts all movement, but leaves rotations unrestricted. An example of a ball joint from real world 
could be human shoulder.

2D joints does not have revolute joints, because it degenerates into ball joint.

## Bodies Binding

When the joint is created and all bodies are set to it, it uses self global transform and bodies global transforms to
calculate local frames for bodies. This process is called _binding_, it happens once when the joint is created, but
can be initiated by moving the joint to some other position by changing local transform of the joint.

## How to create

To create a joint from code use `JointBuilder`:

```rust,no_run
{{#include ../code/snippets/src/scene/joint.rs:create_joint}}
```

Once the joint is created, it will bind given bodies, using the process describe in the above section.

To create a joint from editor, use  `MainMenu -> Create -> Physics -> Joint`, select the new joint and find `Body1` and
`Body2` properties. Assign the fields by holding `Alt` key and drag'n'drop a rigid body to a field. Move the joint to 
correct position to ensure the binding will happen as intended.

## Limits 

You can restrict motion on primary joint axis (rotational and translational) by setting a limit to desired axis. 

- Ball Joint have three angular limits, one per rotation around an axis. The angle range is given in radians.
- Prismatic Joint have only one limit it is maximum linear distance between two bodies along primary joint axis.
- Revolute Joint have a single angular limit around primary axis. The angle range is given in radians.
- Fixed Joint does not have any limit setting, because it locks all degrees of freedom.

## Usage

Joints can be used to create many game entities, such as doors, chains and rag dolls. The most interesting here is 
rag doll. It is used to create realistic behaviour for humans and creatures in games. In general, it is a set of 
rigid bodies, colliders and joints. Where each joint configured to match joints of a creature, for example ball joint
could be used for shoulders, revolute joints for knees and elbows.