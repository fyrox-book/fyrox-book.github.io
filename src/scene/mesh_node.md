# Mesh node

Mesh is a scene node that represents a 3D model. This one of the most commonly used nodes in almost every game.
Meshes could be easily created either programmatically or be made in some 3D modelling software (like Blender)
and loaded in your scene.

## Surfaces

Surface is a set of triangles that uses the same [material](../rendering/materials.md). Mesh node could contain zero of 
more surfaces; each surface contains a set of vertices and indices that binds vertices with triangles. Mesh nodes split 
into surfaces to be rendered effectively by modern GPUs.

## How to create

There are basically two ways, how to pick one depends on your needs. In general, using a 3D modelling software is
the way to go, especially with tons and tons of free 3D models available online.

> ⚠️ The engine supports _only_ FBX and GLTF file format for 3D models! 
> To use GLTF, specify `gltf` feature of the engine in your root Cargo.toml

### Using a 3D modelling software

To create a 3D model, you could use [Blender](https://www.blender.org/) and then export it to `FBX` file format.
To load your 3D model in the game, you should do few simple steps (loading a 3D model does not differ from a prefab 
instantiation):

```rust,no_run
{{#include ../code/snippets/src/scene/mesh.rs:load_model_to_scene}}
```

This code snippet intentionally omits proper `async/await` usage (instead it just blocks current thread until
model is loading) and error handling. In the real game you should carefully handle all errors and use `async/await`
properly.

### Creating a procedural mesh

A mesh instance could be created from code, such meshes are called "procedural". They're suitable for cases when you
cannot create a mesh in 3D modelling software.

```rust,no_run
{{#include ../code/snippets/src/scene/mesh.rs:create_procedural_mesh}}
```

As you can see, creating a mesh procedurally requires lots of manual work and not so easy.

## Animation

Mesh node supports bone-based animation (skinning) and blend shapes. See [Animation chapter](./../animation/animation.md) 
for more info.

## Data Buffers

It is possible to access vertex buffer and index buffer of a mesh to either read or write some data there. 
For example, the following code extracts world-space positions of every vertex of an animated mesh:

```rust ,no_run
{{#include ../code/snippets/src/scene/mesh.rs:extract_world_space_vertices}}
```
