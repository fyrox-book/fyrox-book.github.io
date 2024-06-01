# Light node

The engine offers complex lighting system with various types of light sources. 

## Light types

There are three main types of light sources: directional, point, and spotlights.

### Directional light

Directional light does not have a position, its rays are always parallel, and it has a particular direction in space.
An example of directional light in real-life could be our Sun. Even if it is a point light, it is so far away from
the Earth, so we can assume that its rays are always parallel. Directional light sources are suitable for outdoor 
scenes.

A directional light source could be created like this:

```rust,no_run
{{#include ../code/snippets/src/scene/light.rs:create_directional_light}}
```

By default, the light source will be oriented to lit "the ground". In other words its direction will be faced towards
`(0.0, -1.0, 0.0)` vector. You can rotate it as you want by setting local transform of it while building. Something
like this:

```rust,no_run
{{#include ../code/snippets/src/scene/light.rs:create_oriented_directional_light}}
```

### Point light

Point light is a light source that emits lights in all directions, it has a position, but does not have an orientation.
An example of a point light source: light bulb. 

```rust,no_run
{{#include ../code/snippets/src/scene/light.rs:create_point_light}}
```

### Spotlight

Spotlight is a light source that emits lights in cone shape, it has a position and orientation. An example of 
a spotlight source: flashlight.

```rust,no_run
{{#include ../code/snippets/src/scene/light.rs:create_spot_light}}
```

## Light scattering

![scattering](scattering.png)

Spot and point lights support light scattering effect. Imagine you're walking with a flashlight in a foggy weather,
the fog will scatter the light from your flashlight making it, so you'll see the "light volume". Light scattering is
**enabled by default**, so you don't have to do anything to enable it. However, in some cases you might want to disable 
it, you can do this either while building a light source or change light scattering options on existing light source.
Here is the small example of how to do that.

```rust,no_run
{{#include ../code/snippets/src/scene/light.rs:disable_light_scatter}}
```

You could also change the amount of scattering per each color channel, using this you could imitate the 
[Rayleigh scattering](https://en.wikipedia.org/wiki/Rayleigh_scattering):

```rust,no_run
{{#include ../code/snippets/src/scene/light.rs:use_rayleigh_scattering}}
```

## Shadows

By default, light sources cast shadows. You can change this by using `set_cast_shadows` method of a light source. You
should carefully manage shadows: shadows giving the most significant performance impact, you should keep the amount of 
light sources that can cast shadows at lowest possible amount to keep performance at good levels. You can also turn 
on/off shadows when you need:

```rust,no_run
{{#include ../code/snippets/src/scene/light.rs:switch_shadows}}
```

Not every light should cast shadows, for example a small light that a player can see only in a distance can have
shadows disabled. You should set the appropriate values depending on your scene, just remember: the fewer the shadows
the better the performance. The most expensive shadows are from point lights, the less, from spotlights and directional
lights. 

## Performance

Lights are not cheap, every light source has some performance impact. As a general rule, try to keep the amount
of light sources at reasonable levels and especially try to avoid creating tons of light sources in a small area.
Keep in mind that the less area the light needs to "cover", the higher the performance. This means that you can have
tons of small light sources for free.