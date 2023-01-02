# Animation Blending State Machine (ABSM) Editor

While it is possible to create and manage animation blending and state manually from code, it quickly becomes too 
annoying and hardly manageable. To help you create and manage blending machines in easy way, the engine offers 
a ABSM Editor tool.

![absm editor](./absm.png)

The editor has four main parts (windows):

1. `Toolbar` 
2. `Parameters` - allows you to edit various variables that are responsible for transitions, weight parameters for 
blending, etc.
3. `State Graph` - allows you to create, delete, edit states and transition between them.
4. `State Viewer` - allows you to edit pose source for a state. Pose source can be represented either by a single 
node that plays an animation, or a series of play animation nodes connected to blending nodes (which can be connected
to other blending nodes, etc.)