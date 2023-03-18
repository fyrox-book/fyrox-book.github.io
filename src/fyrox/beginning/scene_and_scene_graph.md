# Scene and Scene Graph

When you're playing a game, you often see various objects scattered around the screen, all of them are forming a
_scene_. A scene is just a set of a variety objects, as in many other game engines, Fyrox allows you to create multiple
scenes for multiple purposes, for example, one scene could be used for a menu, a bunch of others for game levels,
and another one for an ending screen. Scenes can also be used to create a source of data for other scenes, such scenes are called
_prefabs_. Scenes can also be rendered in a texture, which can be used in other scenes - this way you
can create interactive screens that show other places.

While playing games, you may have noticed that some objects behaves as if they were linked to other objects, for example,
a character in a role-playing game could carry a sword. While the character holds the sword, it is linked to his
arm. Such relations between the objects can be presented by a graph structure.

Simply speaking, a graph is a set of objects with hierarchical relationships between each object. Each object in the
graph is called a _node_. In the example with the sword and the character, the sword is a _child_ node of the character,
and the character is a _parent_ node of the sword (here we ignore the fact that in reality, character
models usually contain complex skeletons, with the sword actually being attached to one of the hands' bones, not to the character).

You can change the hierarchy of nodes in the editor using a simple drag'n'drop functionality in the `World Viewer` - drag a 
node onto some other node, and it will attach itself to it.

## Building Blocks or Scene Nodes

The engine offers various types of "building blocks" for your scene, each such block is called a _scene node_.

- [Base](../scene/base_node.md) - stores hierarchical information (a handle to the parent node and handles
  to children nodes), local and global transform, name, tag, lifetime, etc. It has self-describing name - it's used as a base node 
  for every other scene node via composition.
- [Mesh](../scene/mesh_node.md) - represents a 3D model. This one of the most commonly used nodes in almost every game.
  Meshes can be easily created either programmatically, or be made in some 3D modelling software, such as Blender, 
  and then loaded into the scene.
- [Light](../scene/light_node.md) - represents a light source. There are three types of light sources:
    - **Point** - emits light in every direction. A real-world example would be a light bulb.
    - **Spot** - emits light in a particular direction, with a cone-like shape. A real-world example would be a flashlight.
    - **Directional** - emits light in a particular direction, but does not have position. The closest real-world example would be the Sun.
- [Camera](../scene/camera_node.md) - allows you to see the world. You must have at least one camera in your scene to be able to see anything.
- [Sprite](../scene/sprite_node.md) - represents a quad that always faces towards a camera. It can have a texture and size and can also can be rotated around the "look" axis.
- [Particle system](../scene/particle_system_node.md) - allows you to create visual effects using a huge set of small particles. It
  can be used to create smoke, sparks, blood splatters, etc..
- [Terrain](../scene/terrain_node.md) - allows you to create complex landscapes with minimal effort.
- [Decal](../scene/decal_node.md) - paints on other nodes using a texture. It is used to simulate cracks in concrete walls,
  damaged parts of the road, blood splatters, bullet holes, etc.
- [Rigid Body](../physics/rigid_body.md) - a physical entity that is responsible for the dynamic of the rigid. There is a special 
variant for 2D - `RigidBody2D`.
- [Collider](../physics/collider.md) - a physical shape for a rigid body. It is responsible for contact manifold generation, 
without it, any rigid body will not participate in simulation correctly, so every rigid body must have at least
one collider. There is a special variant for 2D - `Collider2D`.
- [Joint](../physics/joint.md) - a physical entity that restricts motion between two rigid bodies. It has various amounts
of degrees of freedom depending on the type of the joint. There is a special variant for 2D - `Joint2D`.
- [Rectangle](../scene/rectangle.md) - a simple rectangle mesh that can have a texture and a color. It is a very simple version of 
a Mesh node, yet it uses very optimized renderer, that allows you to render dozens of rectangles simultaneously.
This node is intended for use in **2D games** only.
- [Sound](../sound/sound.md) - a sound source universal for 2D and 3D. Spatial blend factor allows you to select
a proportion between 2D and 3D.
- Listener - an audio receiver that captures the sound at a particular point in your scene and sends it to an audio
context for processing and outputting to an audio playback device.
- Animation Player - a container for multiple animations. It can play animations made in the 
[animation editor](../animation/anim_editor.md) and apply animation poses to respective scene nodes.
- Animation Blending State Machine - a state machine that mixes multiple animations from multiple states into one; each
state is backed by one or more animation playing or blending nodes. See its [respective chapter](../animation/absm_editor.md) 
for more info.

Every node can be created either in the editor (through `Create` on the main menu, or through `Add Child` after right-clicking on 
a game entitiy) or programmatically via their respective node builder (see [API docs](https://docs.rs/fyrox/latest/fyrox/scene/
index.html) for more info). These scene nodes allow you to build almost any kind of game. It is also possible to create your own 
types of nodes, but that is an advanced topic, which is covered in a [future chapter](../scene/custom_node.md).

## Local and Global Coordinates

A graph describes your scene in a very natural way, allowing you think in terms of relative and absolute coordinates
when working with _scene nodes_.

A scene node has two kinds of transform - a local and global. The local transform defines where the node is located
relative to its origin, its scale as a percentage, and its rotation around any arbitrary axis.
The global transform is almost the same, but it also includes the whole chain of transforms of the parent nodes. Going back to the example of the character and the sword, if the character moves, and by extension the sword, the global transform of the sword will reflect the changes made to the character position, yet its local transform will not, since that represents the sword's position's relative to the character's, which didn't change.

This mechanism is very simple, yet powerful. The full grace of it unfolds when you're working with 3D models with
skeletons. Each bone in a skeleton has its parent and a set of children, which allows you to rotate, translate, or scale them to
animate your entire character.
