# Basic concepts

Let's briefly get over some basic concepts of the engine, there's not much, but all of them are crucial to understand
design decisions made in the engine.

## Classic OOP

The engine uses somewhat classic OOP with composition over inheritance - complex objects in the engine can be constructed
using simpler objects. It is also possible to use ECS approach, but you need to manually synchronize ECS entities with 
the engine entities. This approach is recommended for games with a high number of entities. There's no built-in ECS
framework, but there are plenty of them in the Rust ecosystem.

## Scenes

In Fyrox, you break down your game in a set of reusable scenes. Pretty much anything can be a scene: a player, a weapon,
a bot, level parts, etc. Scenes can be nested one into another, this helps you to break down complex scenes into reusable
parts. A Scene in Fyrox also plays the role of a prefab, there's pretty much no difference between them.

## Nodes and Scene Graph

A scene is made of one or more nodes. Every scene must have at least one root node, to which everything else is attached.
A scene node contains specific set of properties as well as a number of scripts which are responsible for custom
game logic. 

The typical structure of a scene node can be represented by the following example. The base object for every scene node is 
a `Base` node, it contains a transform, a list of children, etc. A more complex node, that _extends_ functionality of the `Base` 
node stores an instance of `Base` inside it, i.e. composition. For example, a `Mesh` node is a `Base` node _plus_ some specific info 
(a list of surfaces, material, etc.). The "hierarchy" depth is unlimited e.g. a `Light` node in the engine is an enumeration 
of three possible types of light source: `Directional`, `Point`, and `Spot`. All three of these light sources all contain a `BaseLight` node,
which in turn contains a `Base` node. Graphically, it can be represented like so:

```text
`Point`
|__ Point Light Properties (radius, etc.)
|__`BaseLight`
   |__ Base Light Properties (color, etc.)
   |__`Base`
      |__ Base Node Properties (transform, children nodes, etc.)
```

As you can see, this forms the nice tree (graph) that shows what the object contains. This is a very natural way of describing
scene nodes, it gives you the full power of building an object of any complexity.

## Plugins

Plugin is a container for "global" game data and logic, its main usage is to provide scripts with some data and to 
manage global game state. Your game can have multiple plugins, but usually only one acts as an "entry point".

## Scripts

Script - is a separate piece of data and logic, that can be attached to scene nodes. This is the primary (but not only)
way of adding custom game logic.

