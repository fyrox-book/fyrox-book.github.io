# Image

![image](image.png)

Image widget is a rectangle with a texture, it is used draw custom bitmaps. The UI in the engine is vector-based, Image
widget is the only way to draw a bitmap. Usage of the Image is very simple:

## Usage

```rust,no_run
{{#include ../code/snippets/src/ui/image.rs:create_image}}
```

There are one common pitfall when using Image widget - you must explicitly set width and height of the image if it is
not placed to some panel, that will stretch it automatically. In other words if you created an image with undefined
width and height, then putting it to some container like Grid' cell will stretch the image to fit cell bounds.

## Equal Size to Source

Sometimes you need your image to have equal size with the texture it uses, in this case you should fetch texture 
bounds first and then create an Image width these bounds:

```rust,no_run,edition2018
{{#include ../code/snippets/src/ui/image.rs:create_image_equal_in_size_to_source}}
```

This function can be used as-is whenever you need to create an Image that have same size as the source file. It is
marked as `async` because resource loading (texture is a resource) happens in separate thread and to get actual texture
data we must wait it. If you don't want to use async, then use any executor to block current thread and execute the
promise immediately:

```rust,no_run,edition2018
{{#include ../code/snippets/src/ui/image.rs:create_image_sync}}
```

## Vertical Flip

In some rare cases you need to flip your source image before showing it, there is `.with_flip` option for that:

```rust,no_run
{{#include ../code/snippets/src/ui/image.rs:create_flipped_image}}
```

There are few places where it can be helpful:

- You're using render target as a source texture for your Image instance, render targets are vertically flipped due
to mismatch of coordinates of UI and graphics API. The UI has origin at left top corner, the graphics API - bottom left.
- Your source image is vertically mirrored.