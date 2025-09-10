# Lighting

This chapter explains how the lighting works in the engine and which methods of lighting it uses. Fyrox uses
industry-standard physically based rendering pipeline by default.

## Ambient lighting

The engine uses PBR pipeline which supports two major ways of applying ambient lighting.

### Environment lighting (via IBL)

Physically correct ambient image-based lighting (IBL). It uses an environment map as a source of lighting for the
entire scene. By default, every scene has an environment map with bright-blue sky, and this bright lighting is
used to light the scene. To change it, use the following code snippet:

```rust,no_run
{{#include ../code/snippets/src/scene/light.rs:set_scene_skybox}}
```

This, however, may still produce weird lighting inside indoor scenes - all objects will be equally lit from
the respective side of the skybox. The engine has a special mechanism to solve this issue called reflection
probes. See the [respective chapter for more info](probe.md).

### Single-color lighting

Simplest possible lighting, but not physically correct. Every scene has default ambient lighting setting, it is
defined by a single RGB color. By default, every scene has some pre-defined ambient lighting, it is bright enough,
so you can see your objects. In some cases, you may need to adjust it or even make it black (for horror games for
instance), this can be achieved by a few lines of code:

```rust,no_run
{{#include ../code/snippets/src/scene/light.rs:set_ambient_lighting}}
```

Please keep in mind that ambient lighting does not mean global illumination, it is a different lighting technique
which is not available in the engine yet.

## Light Sources

There are number of built-in light sources:

1) Directional Light
2) Point Light
3) Spot Light

