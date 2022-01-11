# Scene and scene graph

When you're playing a game, you often see various objects scattered around on screen, all of them forming a
_scene_. Scene is just a set of various objects, as in many other game engines, fyrox allows you to create multiple
scenes for various purposes. For example, one scene could be used for menu, a bunch could be used for game levels,
and one for ending screen. Scenes also could be used to create a source of data for other scenes, such scenes called
_prefabs_. A scene could also be rendered in a texture, and the texture can be used in other scene - this way you
can create interactive screens that showing some other places.

While playing games, you could've noticed that some objects behaves like they're linked to other objects, for example
your character in a role-playing game could carry a sword. While the character holds the sword, it is linked to his
arm. Such relations between the objects could be presented by a graph structure.

Simply speaking, graph is a set of objects with hierarchical relationships between each object. Each object in the
graph is called _node_. In the example with the sword and the character, the sword is a _child_ node of a character,
which in its turn is a _parent_ node of a character. (Here we intentionally omit the fact that usually character
model contains complex skeleton with multiple bones and the sword is actually attached to one of hand's bones.)

## Building blocks or scene nodes

The engine offers various types of "building blocks" for your scene, each such block is called _scene node_.

- **Base** - a node that stores hierarchical information (a handle to the parent node and a set of handles
  to children nodes), local and global transform, name, tag, lifetime, etc. It has self-describing name - it
  is used as a base node for every other scene node (via composition).
- **Mesh** - a node that represents a 3D model. This one of the most commonly used nodes in almost every game.
  Meshes could be easily created either programmatically, or be made in some 3D modelling software (like Blender)
  and loaded in your scene.
- **Light** - a node that represents a light source. There are three types of light sources:
    - **Directional** - a light source that does not have position, only direction. The closest real-world example
      is our Sun.
    - **Point** - a light source that emits light in every direction. Real-world example: light bulb.
    - **Spot** - a light source that emits light in a particular direction with a cone-like shape. Real-world example:
      flashlight.
- **Camera** - a node that allows you to see the world. You must have at least one camera in your scene to be
  able to see anything.
- **Sprite** - a node that represents a quad that always faced towards a camera. It can have a texture, size, it
  also can be rotated around the "look" axis.
- **Particle system** - a node that allows you to build visual effects using a huge set of small particles, it
  can be used to create smoke, sparks, blood splatters, etc. effects.
- **Terrain** - a node that allows you to create complex landscapes with minimal effort.
- **Decal** - a node that paints on other nodes using a texture. It is used to simulate cracks in concrete walls,
  damaged parts of the road, blood splatters, bullet holes, etc.

These scene nodes allow you to build almost any kind of game.

## Local and global coordinates

Graph describes your scene in a very natural way, allowing you think in terms of relative and absolute coordinates
when working with _scene nodes_.

Scene node has two kinds of transform - local and global. Local transform defines where the node is located
(translation) relative to origin, how much it is scaled (in percent) and rotated (around any arbitrary axis).
Global transform is almost the same, but it also includes the whole chain of transforms of parent nodes. In the
previous example with the character, the sword has its own local transform which tells how much it should be
moved from origin to be exactly on a hand of the character. But global transform of the swords includes transform
of the entire character. So if you move the character, the local transform of the sword will remain the same, but
global transform will include the transform of the character.

This mechanism is very simple, yet powerful. The full grace of it unfolds when you're working with 3D models with
skeleton, each bone in the skeleton has its parent and a set of children. You can rotate/translate/scale bones to
animate your character.
