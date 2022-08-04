# Rectangle node

Rectangle is the simplest "2D" node, it can be used to create "2D" graphics. 2D is in quotes here because the node
is actually a 3D node, like everything else in the engine. Here is an example scene made with the rectangle nodes and 
an orthographic camera:

![2d scene](2d_scene.PNG)

As you can see it is a good basis for 2D games.

## How to create

Use the RectangleBuilder to create Rectangle nodes:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{algebra::Vector3, color::Color, pool::Handle},
#     engine::resource_manager::ResourceManager,
#     scene::{
#         base::BaseBuilder, dim2::rectangle::RectangleBuilder, graph::Graph, node::Node,
#         transform::TransformBuilder,
#     },
# };
fn create_rect(graph: &mut Graph, resource_manager: ResourceManager) -> Handle<Node> {
    RectangleBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                // Size of the rectangle is defined only by scale.
                .with_local_scale(Vector3::new(0.4, 0.2, 1.0))
                .build(),
        ),
    )
    .with_color(Color::RED)
    .with_texture(resource_manager.request_texture("path/to/your_texture.jpg"))
    .build(graph)
}
```

## Specifying image portion for rendering

By default, Rectangle node uses entire image for rendering, but for some applications it is not enough. For example,
you may want to use sprite sheets to animate your 2D entities. In this case you need to be able to use only portion
of an image. It is possible to do by using `set_uv_rect` method of the Rectangle node. Here's an example of setting
right-top quarter of an image to be used by a Rectangle node:

```rust,no_run
# extern crate fyrox;
# use fyrox::{core::math::Rect, scene::dim2::rectangle::Rectangle};
# 
fn set_2nd_quarter_image_portion(rectangle: &mut Rectangle) {
    rectangle.set_uv_rect(Rect::new(
        0.5, // Offset by 50% to the right
        0.0, // No need to offset to bottom.
        0.5, // Use half (50%) of width and height
        0.5,
    ));
}
```

Keep in mind that every part of uv rectangle is proportional. For example 0.5 means 50%, 1.5 = 150% and so on. If width
or height is exceeding 1.0 and the texture being used is set to Wrapping mode at respective axis, the image will tile
across axes.

## Performance

Rectangles use specialized renderer that is heavily optimized to render tons of rectangles at once, so you can use 
rectangles almost for everything in 2D games. 

## Limitations

Rectangle nodes does not support custom materials - it is a simplified version of a Mesh node that allows you draw a
rectangle with a texture and a color. Its main purpose is to be able to start making games as quick as possible without
diving too deep into details (shaders, render passes, etc.). You can still create a "rectangle" with custom material, use
Mesh node with single rectangle surface:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{
#         algebra::{Matrix4, Vector3},
#         parking_lot::Mutex,
#         pool::Handle,
#     },
#     material::Material,
#     scene::{
#         base::BaseBuilder,
#         graph::Graph,
#         mesh::{
#             surface::{SurfaceBuilder, SurfaceData},
#             MeshBuilder, RenderPath,
#         },
#         node::Node,
#         transform::TransformBuilder,
#     },
# };
# use std::sync::Arc;

fn create_rect_with_custom_material(
    graph: &mut Graph,
    material: Arc<Mutex<Material>>,
) -> Handle<Node> {
    MeshBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                .with_local_scale(Vector3::new(0.4, 0.2, 1.0))
                .build(),
        ),
    )
    .with_surfaces(vec![SurfaceBuilder::new(Arc::new(Mutex::new(
        SurfaceData::make_quad(&Matrix4::identity()),
    )))
    .with_material(material)
    .build()])
    .with_render_path(RenderPath::Forward)
    .build(graph)
}
```

This will effectively "mimic" the Rectangle node, but will allow you to use the full power of custom shaders. Keep in
mind that Mesh nodes will be rendered via Deferred Renderer, while Rectangle nodes rendered with specialized renderer,
that might result in some graphical artifacts.

Rectangle nodes has limited lighting support, it means that they still will be lit by standard scene lights, but it will
be a very simple diffuse lighting without any "physically correct" lighting. This is perfectly ok for 95% of 2D games,
if you want to add custom lighting then you should use custom shader.

Rectangle nodes works well with 2D physics nodes, check 2D physics section of the book for more info.