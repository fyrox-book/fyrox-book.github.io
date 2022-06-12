# Prefabs

Prefab is a separate scene that can be instantiated to some other scene, while preserving links between properties
between instance and its parent prefab. Prefabs allow you to create a part of scene and create multiple instances of
it in some other scenes.

Let's quickly check what does that mean on practice. The engine has a prefab system which allows you to build 
hierarchical scenes which can include any number of other scenes as child scenes. Child scenes can have their own child
scenes and so on. This is very efficient decoupling mechanism that allows you to put pieces of the scene in separate 
scenes (prefabs) and modify them independently. The changes in child scenes will be automatically reflected to all parent
scenes. Here is the very simple example of why this is important: imagine you need to populate a town with 3D models of
cars. Each kind of car have its own 3D model and, for example, a collision body that won't allow the player to walk through
cars. How would you do this? The simplest (and dumbest) solution is to copy dozens of car models in the scene, and
you're done. Imagine that now you need to change something in your car, for example, add a trunk that can be opened.
What will you do? Of course, you should "iterate" over each car model and do the required changes, you simply don't have
any other option. This will eat huge amount of time and in general it is very non-productive.

This is where prefabs will save you hours of work. All you need to do is to create a car prefab and instantiate it
multiple times in your scene. When you'll need to change something in the car, you simply go to the prefab and change
it. After that every prefab instance will have your changes!

## How to create and use a prefab

All you need to do is to make a scene in the editor with all required objects and save it! After that you can use the
scene in other scenes and just do its instantiation, like with usual 3D models. You can either instantiate it from
editor by drag'n'drop a prefab to scene previewer, or do standard [model resource instantiation](../resources/model.md)

## Property inheritance

As was already mentioned in the intro section, instances inherit properties from their parent prefabs. For example, you
can change position of an object in prefab and every instance will reflect that change - the object's instances will
also move. This works until there's no manual change to a property in instance, if you do so, then your change is 
considered with higher priority. Such feature allows you to tweak instances, add some unique details to them, but keep
generic properties from parent prefabs.

## Hierarchical Prefabs

Prefabs can have other prefab instances inside, this means that you can, for example, create a room populated with
instances of other prefabs (bookshelves, chairs, tables, etc) and then use the room prefab to build a bigger scene.
The changes in base prefabs will be reflected in their instances, regardless how deep the hierarchy is.