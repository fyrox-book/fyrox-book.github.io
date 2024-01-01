# Physics 

The engine have full-featured physics engine under the hood (Rapier), it helps you to simulate physics in your games.
There is first-class support for both 2D and 3D physics. There are three main physics entities in the engine:

- Rigid Body - responsible for rigid body dynamics simulation, must have at least one collider to be able to interact with
other rigid bodies in the world.
- Collider - responsible for collision detection.
- Joint - responsible for motion restriction between two rigid bodies.

All these entities are ordinary scene nodes, so they can be arranged into any hierarchy in the scene. However there some
rules that have to be followed to make physics simulation work as intended:

- Rigid body node must have at least one _direct child_ Collider node, otherwise rigid body won't interact with other
rigid bodies in the world.
- Joint node must have two _direct child_ rigid bodies, otherwise joint will have no effect.

## Differences between 3D and 2D

There is a very few differences between 3D and 2D physics, the most obvious is that 2D physics does simulation only in 
oXY plane (the plane of the screen). 2D physics has less collider shapes available since some 3D shapes degenerate in
2D, for example cylinder 3D shape in 2D is just a rectangle. There is also lesser amount of joints available in 2D, 
there is no revolute joint for example. Unlike 3D physics entities, 2D physics entities exist in the separate
`scene::dim2` module.