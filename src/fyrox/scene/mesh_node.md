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

> ⚠️ The engine supports _only_ FBX file format for 3D models!

### Using a 3D modelling software

To create a 3D model, you could use [Blender](https://www.blender.org/) and then export it to `FBX` file format.
To load your 3D model in the game, you should do few simple steps (loading a 3D model does not differ from a prefab 
instantiation):

```rust,no_run
# extern crate fyrox;

use fyrox::{
    core::{futures::executor::block_on, pool::Handle},
    asset::manager::{ResourceManager}, resource::model::{Model, ModelResourceExtension},
    scene::{node::Node, Scene},
};
use std::path::Path;

fn load_model_to_scene(
    scene: &mut Scene,
    path: &Path,
    resource_manager: ResourceManager,
) -> Handle<Node> {
    // Request model resource and block until it loading. 
    let model_resource =
        block_on(resource_manager.request::<Model, _>(path))
            .unwrap();

    // Create an instance of the resource in the scene. 
    model_resource.instantiate(scene)
}
```

This code snippet intentionally omits proper `async/await` usage (instead it just blocks current thread until
model is loading) and error handling. In the real game you should carefully handle all errors and use `async/await`
properly.

### Creating a procedural mesh

A mesh instance could be created from code, such meshes are called "procedural". They're suitable for cases when you
cannot create a mesh in 3D modelling software.

```rust,no_run
# extern crate fyrox;

use fyrox::{
    core::{
        algebra::{Matrix4, Vector3},
        parking_lot::Mutex,
        pool::Handle,
        sstorage::ImmutableString,
    },
    asset::manager::ResourceManager, resource::model::{Model, ModelResourceExtension},
    resource::texture::Texture,
    material::{shader::SamplerFallback, Material, PropertyValue, SharedMaterial},
    scene::{
        base::BaseBuilder,
        mesh::{
            surface::{SurfaceBuilder, SurfaceData, SurfaceSharedData},
            MeshBuilder,
        },
        node::Node,
        transform::TransformBuilder,
        Scene,
    },
};
use std::sync::Arc;

fn create_procedural_mesh(
    scene: &mut Scene,
    resource_manager: ResourceManager,
) -> Handle<Node> {
    let mut material = Material::standard();

    // Material is completely optional, but here we'll demonstrate that it is possible to
    // create procedural meshes with any material you want.
    material
        .set_property(
            &ImmutableString::new("diffuseTexture"),
            PropertyValue::Sampler {
                value: Some(resource_manager.request::<Texture, _>("some_texture.jpg")),
                fallback: SamplerFallback::White,
            },
        )
        .unwrap();

    // Notice the MeshBuilder.
    MeshBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                .with_local_position(Vector3::new(0.0, -0.25, 0.0))
                .build(),
        ),
    )
    .with_surfaces(vec![SurfaceBuilder::new(SurfaceSharedData::new(
        // Our procedural mesh will have a form of squashed cube.
        // A mesh can have unlimited amount of surfaces.
        SurfaceData::make_cube(Matrix4::new_nonuniform_scaling(&Vector3::new(
            25.0, 0.25, 25.0,
        ))),
    ))
        .with_material(SharedMaterial::new(material))
        .build()])
    .build(&mut scene.graph)
}
```

As you can see, creating a mesh procedurally requires lots of manual work and not so easy.

## Animation

Mesh node supports bone-based animation (skinning). See [Animation chapter](./../animation/animation.md) for more info.
