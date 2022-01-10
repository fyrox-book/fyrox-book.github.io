# Mesh node

Mesh is a scane node that represents a 3D model. This one of the most commonly used nodes in almost every game.
Meshes could be easily created either programmatically, or be made in some 3D modelling software (like Blender)
and loaded in your scene.

## How to create

There are basically two ways, how to pick one depends your needs. In general, using a 3D modelling software is
the way to go, especially with tons and tons of free 3D models available online.

**Caveat:** The engine supports _only_ FBX file format for 3D models!

### Using a 3D modelling software

To create 3D model in you could use [Blender](https://www.blender.org/) and then export it to `FBX` file format.
To load your 3D model in the game, you should do few simple steps:

```rust
# extern crate Fyrox;

use fyrox::{
    core::{futures::executor::block_on, pool::Handle},
    engine::resource_manager::{ResourceManager},
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
        block_on(resource_manager.request_model(path))
            .unwrap();

    // Create an instance of the resource in the scene. 
    model_resource.instantiate_geometry(scene)
}
```

This code snippet intentionally omits proper `async/await` usage (instead it just blocks current thread until
model is loading) and error handling. In the real game you should carefully handle all errors and use `async/await`
properly.

### Creating a procedural mesh

A mesh instance could be created from code, such meshes called "procedural". They're suitable for cases when you
cannot create a mesh in 3D modelling software.

```rust
# extern crate Fyrox;

use fyrox::{
    core::{
        algebra::{Matrix4, Vector3},
        parking_lot::Mutex,
        pool::Handle,
        sstorage::ImmutableString,
    },
    engine::resource_manager::ResourceManager,
    material::{shader::SamplerFallback, Material, PropertyValue},
    scene::{
        base::BaseBuilder,
        mesh::{
            surface::{SurfaceBuilder, SurfaceData},
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
                value: Some(resource_manager.request_texture("some_texture.jpg")),
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
    .with_surfaces(vec![SurfaceBuilder::new(Arc::new(Mutex::new(
        // Our procedural mesh will have a form of squashed cube.
        // A mesh can have unlimited amount of surfaces.
        SurfaceData::make_cube(Matrix4::new_nonuniform_scaling(&Vector3::new(
            25.0, 0.25, 25.0,
        ))),
    )))
        .with_material(Arc::new(Mutex::new(material)))
        .build()])
    .build(&mut scene.graph)
}
```

As you can see, creating a mesh procedurally requires lots of manual work and not so easy.

## Animation

Meshes have full support of bone animation, it means that you can load pretty much any model in your game, and 
it will play animation with no problems. One difference that should be noted is that you should use `instantiate`
method instead of `instantiate_geometry` and then manually apply animation to the mesh in you game loop.

```rust
# extern crate Fyrox;
# use fyrox::resource::model::ModelInstance;
# use fyrox::{
#     core::{futures::executor::block_on, pool::Handle},
#     engine::resource_manager::{ResourceManager},
#     scene::{node::Node, Scene},
# };
# use std::path::Path;

fn load_model_to_scene(
    scene: &mut Scene,
    path: &Path,
    resource_manager: ResourceManager,
) -> ModelInstance {
    // Request model resource and block until it loading.
    let model_resource =
        block_on(resource_manager.request_model(path))
            .unwrap();

    // Create an instance of the resource in the scene.
    model_resource.instantiate(scene)
}

# fn animate(scene: &mut Scene, resource_manager: ResourceManager) {
// At initialization.
let ModelInstance { root, animations } = load_model_to_scene(
    scene,
    &Path::new("path/to/your/model.fbx"),
    resource_manager,
);

// .. Somewhere in the game loop ..

scene.animations[animations[0]]
    .get_pose()
    .apply(&mut scene.graph);
# }
```

### Retargetting

You don't have to store all possible animations inside a single 3D model file, instead you can store each animation
in separate file and retarget it to your mesh instance when needed. This also allows you to store non-animated
3D model in separate file, and all animations in their own files.