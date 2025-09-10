# Reflection Probe

Reflection probe is an object that allows "capturing" a scene content in a cube texture, that
can later be used to render reflections and be used as a source of ambient lighting for a scene.

## Update Mode

Reflection probe can be updated either once or every frame. The default mode is [`UpdateMode::Once`].
If you need dynamic reflections, then use [`UpdateMode::EachFrame`] mode. However, it may lead
to performance issues.

## Performance
Reflection probe renders the scene six times which is quite slow. In most cases, it does not matter
because most of the probes can be updated just once (static probes). Such probes can have
increased resolution.

Dynamic probes are the heaviest and require careful performance tweaking. There should be a balance
between the resolution and the speed. Reflection probes does frustum culling, so some part of
the scene geometry can be excluded. This functionality can be tweaked by setting the far clipping
plane distance to lower values to prevent the probe to render distant objects.

## Interaction With Cameras

When rendering, the engine will automatically pick a reflection probe for a camera. It is done
by a simple point-box intersection test. This reflection probe will then be used for rendering
using the camera.

## Example

The following example creates a new reflection probe 20 units wide in all directions, centered
at (0.0, 10.0, 0.0) point with a rendering position offset by 10 units along X axis.

```rust,no_run
{{#include ../code/snippets/src/scene/probe.rs:create_probe}}
```
