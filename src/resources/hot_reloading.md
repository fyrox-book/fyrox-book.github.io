# Asset Hot Reloading

Fyrox supports asset hot reloading for most of the supported asset types. Hot reloading is a very useful feature that
allows you to reload assets from disk when they're changing. For example, you can change a texture, save it and the
engine will automatically reload it and the changes will reflect in the game (and the editor). This section of the book
explains how asset hot reloading works for specific asset types and what to expect from it.

## Textures

Content of textures will be automatically reloaded when their source files are changed. Textures loading is usually quite
fast and even large number of changed textures shouldn't cause significant lags.

## Sound

Content of sound buffers will be automatically reloaded when their source files are changed. There might be a "pop" sound
when a buffer is reloaded, this happens because of a sudden change of amplitude of the signal. Reloading of sound buffers
could be quite slow for large sounds (such a music), since usually sound buffers are encoded with some algorithm and this
data needs to be decoded when reloading.

## Models

Model resource (which is prefab also) supports hot reloading as well, but with some small limitations.

If a node in FBX or GLTF model changes its name, then its instance in the running game won't receive the changes from
the source file. This happens, because the engine uses object name to search for the "ancestor" from which it then takes
the data. If you swap names between two or more objects, their properties will be swapped in the game also. This issue
does not exist if you're changing names in native engine prefabs.

Hierarchy changes in a source file will be reflected in all instances, however it could not work correctly if you're changing
hierarchy in FBX or GLTF model if there are duplicated names. This issue does not exist if you're changing names in native
engine prefabs.

Objects deleted in models will be also deleted in the running game, which could result in crash if you're expecting the
objects to be always alive.

Any change in a leaf prefab in a chain of hierarchical prefabs will cause an update pass of its first ancestor. In other
words, if you have a level with a room prefab, and this room prefab has chair prefab instances in it then any change in the
chair prefab source file will be applied to the chair prefab itself, then its instances in the room prefab. See
[property inheritance](../scene/inheritance.md) chapter for more info.
