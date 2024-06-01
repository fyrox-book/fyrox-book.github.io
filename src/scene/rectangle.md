# Rectangle node

Rectangle is the simplest "2D" node, it can be used to create "2D" graphics. 2D is in quotes here because the node
is actually a 3D node, like everything else in the engine. Here is an example scene made with the rectangle nodes and 
an orthographic camera:

![2d scene](2d_scene.PNG)

As you can see it is a good basis for 2D games.

## How to create

Use the RectangleBuilder to create Rectangle nodes:

```rust,no_run
{{#include ../code/snippets/src/scene/rectangle.rs:create_rect}}
```

## Specifying image portion for rendering

By default, Rectangle node uses entire image for rendering, but for some applications it is not enough. For example,
you may want to use sprite sheets to animate your 2D entities. In this case you need to be able to use only portion
of an image. It is possible to do by using `set_uv_rect` method of the Rectangle node. Here's an example of setting
right-top quarter of an image to be used by a Rectangle node:

```rust,no_run
{{#include ../code/snippets/src/scene/rectangle.rs:set_2nd_quarter_image_portion}}
```

Keep in mind that every part of uv rectangle is proportional. For example 0.5 means 50%, 1.5 = 150% and so on. If width
or height is exceeding 1.0 and the texture being used is set to Wrapping mode at respective axis, the image will tile
across axes.

## Animation

See [Sprite Animation](../animation/spritesheet/spritesheet.md) chapter for more info.

## Performance

Rectangles use specialized renderer that is heavily optimized to render tons of rectangles at once, so you can use 
rectangles almost for everything in 2D games.