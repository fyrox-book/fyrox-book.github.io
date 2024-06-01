# Camera node

Camera is a special scene node that allows you to "look" at your scene from any point and with any orientation.
Currently, the engine supports only _perspective_ cameras, which could be represented as a frustum volume. Everything
that "intersects" with the frustum will be rendered. 

![Frustum](./frustum.svg)

## How to create

An instance of camera node could be created using `CameraBuilder`: 

```rust,no_run
{{#include ../code/snippets/src/scene/camera.rs:create_camera}}
```

Orientation and position should be set in `BaseBuilder` as usual.

## Projection modes

Projection mode defines how your scene will look like after rendering, there are two projection modes available.

### Perspective

Perspective projection makes distant objects smaller and parallel lines converging when using it, it is the most 
common projection type for 3D games. By default, each camera uses perspective projection. It's defined by three 
parameters that describes frustum volume:

- Field of view angle
- Near clipping plane location
- Far clipping plane location

Here is a simple example of how to create a camera with perspective projection:

```rust,no_run
{{#include ../code/snippets/src/scene/camera.rs:create_perspective_camera}}
```

### Orthographic

Orthographic projection prevents parallel lines from converging, it does not affect object size with distance.
If you're making 2D games or isometric 3D games, this is the projection mode you're looking for. Orthographic
projection defined by three parameters:

- Vertical Size
- Near Clipping Plane
- Far Clipping Plane

Vertical size defines how large the "box" will be in vertical axis, horizontal size is derived from vertical
size by multiplying vertical size with aspect ratio.

Here is a simple example of how to create a camera with orthographic projection:

```rust,no_run

```

## Performance

Each camera forces engine to re-render scene one more time, which can be very resource-intensive (both CPU and GPU)
operation. 

To reduce GPU load, try to keep the Far Clipping Plane at lowest possible values. For example, if you're making a game
with closed environment (lots of corridors, small rooms, etc.) set the Far clipping Plane to max possible distance that 
can be "seen" in your game - if the largest thing is a corridor, then set the Far clipping Plane to slightly exceed the 
length. This will force the engine to clip everything that is out of bounds and do not draw such objects.

## Skybox

Outdoor scenes usually have distant objects that can't be reached, these can be mountains, sky, distant forest, etc.
such objects can be pre-rendered and then applied to a huge cube around camera, it will always be rendered first and will
be the background of your scene. To create a Skybox and set it to a camera, you can use the following code:

```rust,no_run,edition2018
{{#include ../code/snippets/src/scene/camera.rs:create_camera_with_skybox}}
```

## Color grading look-up tables

Color grading Look-Up Tables (LUT) allows you to transform color space of your frame. Probably everyone saw the
famous "mexican" movie effect when everything becomes yellow-ish when action takes place in Mexico, this is done
via color grading LUT effect. When used wisely, it can significantly improve perception of your scene.

Here is the same scene having no color correction along with another case that has "mexico" color correction:

| Scene                                                 | Look-up-table                     |
|-------------------------------------------------------|-----------------------------------|
| ![No Color Correction](./no_color_correction.PNG)     | ![Neutral LUT](./lut_neutral.jpg) |
| ![With Color Correction](./with_color_correction.PNG) | ![Neutral LUT](./lut_mexico.jpg)  |

To use color grading LUT you could do something like this:

```rust,no_run
{{#include ../code/snippets/src/scene/camera.rs:create_camera_with_lut}}
```

## Picking 

In some games you may need to do mouse picking of objects in your scene. To do that, at first you need to somehow convert
a point on the screen to ray in the world. `Camera` has `make_ray` method exactly for that purpose:

```rust,no_run
{{#include ../code/snippets/src/scene/camera.rs:make_picking_ray}}
```

The ray then can be used to [perform a ray cast over physics entities](../physics/ray.md). This is the simplest way
of camera picking, and you should prefer it most of the time.

### Advanced picking

**Important**: The following picking method is for advanced engine users only, if you don't know the math you should not
use it.

If you know the math and don't want to create physical entities, you can use this ray to perform manual 
ray intersection check:

```rust,no_run
{{#include ../code/snippets/src/scene/camera.rs:precise_ray_test}}
```

`precise_ray_test` is what you need, it performs precise intersection check with geometry of a mesh node. It returns a
tuple of the closest distance and the closest intersection point. 

## Exposure and HDR

(WIP)