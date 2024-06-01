# Sprite

Sprite is just a quad mesh that is always facing camera. It has size, color, rotation around "look" axis and a texture.
Sprites are useful mostly for projectiles, like glowing plasma, and for things that should always face a camera.

> ⚠️ It should be noted that **sprites are not meant to be used for 2D games**, they're only for 3D. 
> Use [Rectangle node](./rectangle.md) if you need 2D sprites.

## How to create

A sprite instance could be created using `SpriteBuilder`:

```rust,no_run
{{#include ../code/snippets/src/scene/sprite.rs:create_sprite}}
```

A sprite with a texture could be created by using `.with_material` method of the builder:

```rust,no_run
{{#include ../code/snippets/src/scene/sprite.rs:create_sprite_with_texture}}
```

Please note, that this code create a material per each sprite. This could be very unoptimal if you're using tons of 
sprites at once, share the same material resource across multiple sprites if you can. Otherwise, each sprite will be
rendered in a separate draw call and the overall performance will be very low.

## Animation

See [Sprite Animation](../animation/spritesheet/spritesheet.md) chapter for more info.

## General rules

Sprites **must not** be used to create any visual effects that involve many particles. You should use 
[particle systems](particle_system_node.md) for that. Why so? Particles systems are very well optimized for managing
huge amounts of particles at the same time, but sprites are not. Each sprite is quite heavy to be used as a particle in 
particle systems, it has a lot of "useless" info that will eat a lot of memory.