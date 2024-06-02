# Path Finding

Fyrox has built-in [A* (A-star)](https://en.wikipedia.org/wiki/A*_search_algorithm) algorithm for pathfinding. It can be
used to find a path on arbitrary graph without cycles. It could be a simple grid where each point knows about its 
"neighbours", [navigational mesh](./navmesh.md), or some other graph.

## Examples

The simplest examples could be a search of path on uniform grid. This could be useful for games with open worlds, 
strategies, and any other types of games that uses uniform grid for pathfinding.

```rust,no_run
{{#include ../code/snippets/src/ai/pathfinding.rs:astar_on_uniform_grid}}
```

Keep in mind, that the returned path is always reversed (its first point corresponds to an `end` point). You need either
to reverse the path, or (which is much faster) just iterate in reverse over its points. 

## What to use

A* is very simple, yet powerful algorithm. However, it is not always suitable, because it searches only on graph vertices
and cannot build paths that are lying on a surface of arbitrary meshes. Simple path finding on a uniform grid is ok
for some games (strategies for instance), but in FPS games it will look awful. In this case you should use 
[navigational meshes](./navmesh.md) which can build path on a surface of arbitrary meshes.

## Performance

Current A* implementation is not ideal and may hurt performance if you need to calculate a lot of paths on large 
graphs. It will be optimized in the future (see [tracking issue](https://github.com/FyroxEngine/Fyrox/issues/442) for 
info).