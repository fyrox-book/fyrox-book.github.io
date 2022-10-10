# Basic concepts

Let's briefly get over some basic concepts of the engine, there's not much, but all of them are crucial to understand
design decisions made in the engine.

## Classic OOP

The engine uses somewhat classic OOP with composition over inheritance - complex objects in the engine can be constructed
using simpler objects.

## Scenes

In Fyrox, you break down your game in a set of reusable scenes. Pretty much anything can be a scene: a player, a weapon,
a bot, level parts, etc. Scenes can be nested one into another, this helps you to break down complex scenes into reusable
parts. Scene in Fyrox is also plays a role of prefab, there's pretty much no difference between them.

## Nodes and Scene Graph

A scene is made of one or more nodes (every scene must have at least one root node, to which everything else is attached).
Scene node contains specific set of properties as well as _one_ optional script instance which is responsible for custom
game logic. 

Typical structure of a scene node could be represented by the following example. The base object for every scene node is 
a `Base` node, it contains a transform, a list of children, etc. A more complex node, that _extends_ functionality of the `Base` 
node stores an instance of `Base` inside of them. For example, a `Mesh` node is a `Base` node _plus_ some specific info 
(a list of surfaces, material, etc.). The "hierarchy" depth is unlimited - a `Light` node in the engine is an enumeration 
of three possible types of light source. `Directional`, `Point`, and `Spot` light sources both use `BaseLight` node,
which in its turn contains `Base` node inside. Graphically it can be represented like so:

```text
`Point`
|__ Point Light Properties (radius, etc.)
|__`BaseLight`
   |__ Base Light Properties (color, etc.)
   |__`Base`
      |__ Base Node Properties (transform, children nodes, etc.)
```

As you can see, this forms the nice tree (graph) that shows what the object contains. This is very natural way of describing
scene nodes, it gives you the full power of building an object of any complexity.

## Plugins

Plugin is a container for "global" game data and logic, its main usage is to provide scripts with some data and to 
manage global game state.

## Scripts

Script - is a separate piece of data and logic, that can be attached to scene nodes. This is primary (but not single)
way of adding custom game logic.

