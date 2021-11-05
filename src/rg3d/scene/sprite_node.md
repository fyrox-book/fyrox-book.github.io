# Sprite

Sprite is just a quad mesh that is always facing camera. It has size, color, rotation around "look" axis and a texture.
Sprites are useful mostly for projectiles, like glowing plasma, and for things that should always face a camera.

**Important:** It should be noted that **sprites are not meant to be used for 2D games**, they're only for 3D. 
There is a separate 2D scenes with their own nodes, which are very well optimized for 2D games.

## How to create

A sprite instance could be created using `SpriteBuilder`:

```rust
# extern crate rg3d;
# use rg3d::{
#     core::{color::Color, pool::Handle},
#     scene::{base::BaseBuilder, node::Node, sprite::SpriteBuilder, Scene},
# };

fn create_sprite(scene: &mut Scene) -> Handle<Node> {
    SpriteBuilder::new(BaseBuilder::new())
        .with_size(2.0)
        .with_rotation(45.0f32.to_radians())
        .with_color(Color::RED)
        .build(&mut scene.graph)
}
```

A sprite with texture could be created by using `.with_texture` method of the builder:

```rust
use rg3d::{
    core::pool::Handle,
    engine::resource_manager::ResourceManager,
    scene::{base::BaseBuilder, node::Node, sprite::SpriteBuilder, Scene},
};

fn create_sprite(scene: &mut Scene, resource_manager: ResourceManager) -> Handle<Node> {
    SpriteBuilder::new(BaseBuilder::new())
        .with_texture(resource_manager.request_texture("path/to/your/texture.png", None))
        .build(&mut scene.graph)
}
```

## General rules

Sprites **must not** be used to create any visual effects, that involve many particles. You should use particle
systems for that. Why so? Particles systems are very well optimized for managing huge amounts of particles at the
same time, but sprites are not. Each sprite is very heavy to be used as a particle in particle systems, it has
a lot of "useless" info that bloats its size up to 600 bytes. 

Current the renderer will render each sprite in a separate draw call, which is very inefficient. So you should 
avoid creating lots of sprites.

## Limitations

Sprites are not support any sort of lighting, if you need lighted sprites, you need to create your own render
pass and use `Mesh` node with custom shader that will orient all faces towards camera and will do lighting 
calculations. 