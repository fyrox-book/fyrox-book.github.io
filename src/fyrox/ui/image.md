# Image

Image widget is a rectangle with a texture, it is used draw custom bitmaps. The UI in the engine is vector-based, Image
widget is the only way to draw a bitmap. Usage of the Image is very simple:

## Usage

```rust
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     engine::resource_manager::ResourceManager,
#     gui::{image::ImageBuilder, widget::WidgetBuilder, BuildContext, UiNode},
#     utils::into_gui_texture,
# };

fn create_image(ctx: &mut BuildContext, resource_manager: ResourceManager) -> Handle<UiNode> {
    // You must explicitly set width and height of the image, otherwise it will collapse to a
    // point and you won't see anything.
    let width = 100.0;
    let height = 100.0;
    ImageBuilder::new(WidgetBuilder::new().with_width(width).with_height(height))        
        .with_texture(into_gui_texture(
            // Ask resource manager to load a texture.
            resource_manager.request_texture("path/to/your/texture.png"),
        ))
        .build(ctx)
}
```

There are one common pitfall when using Image widget - you must explicitly set width and height of the image if it is
not placed to some panel, that will stretch it automatically. In other words if you created an image with undefined
width and height, then putting it to some container like Grid' cell will stretch the image to fit cell bounds.

## Equal Size to Source

Sometimes you need your image to have equal size with the texture it uses, in this case you should fetch texture 
bounds first and then create an Image width these bounds:

```rust
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     engine::resource_manager::ResourceManager,
#     gui::{image::ImageBuilder, widget::WidgetBuilder, BuildContext, UiNode},
#     resource::texture::TextureKind,
#     utils::into_gui_texture,
# };

async fn create_image(
    ctx: &mut BuildContext<'_>,
    resource_manager: ResourceManager,
) -> Handle<UiNode> {
    // Ask resource manager to load the texture and wait while it loads using `.await`.
    if let Ok(texture) = resource_manager
        .request_texture("path/to/your/texture.png")
        .await
    {
        // A texture can be not only rectangular, so we must check that.
        let texture_kind = texture.data_ref().kind();
        if let TextureKind::Rectangle { width, height } = texture_kind {
            return ImageBuilder::new(
                WidgetBuilder::new()
                    .with_width(width as f32)
                    .with_height(height as f32),
            )
            .with_texture(into_gui_texture(texture))
            .build(ctx);
        }
    }

    // Image wasn't created.
    Handle::NONE
}
```

This function can be used as-is whenever you need to create an Image that have same size as the source file. It is
marked as `async` because resource loading (texture is a resource) happens in separate thread and to get actual texture
data we must wait it. If you don't want to use async, then use any executor to block current thread and execute the
promise immediately:

```rust
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     engine::resource_manager::ResourceManager,
#     gui::{BuildContext, UiNode},
# };
# 
# async fn create_image(
#     ctx: &mut BuildContext<'_>,
#     resource_manager: ResourceManager,
# ) -> Handle<UiNode> {
#     Handle::NONE
# }
fn create_image_sync(
    ctx: &mut BuildContext<'_>,
    resource_manager: ResourceManager,
) -> Handle<UiNode> {
    fyrox::core::futures::executor::block_on(create_image(ctx, resource_manager))
}
```

## Vertical Flip

In some rare cases you need to flip your source image before showing it, there is `.with_flip` option for that:

```rust
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     engine::resource_manager::ResourceManager,
#     gui::{image::ImageBuilder, widget::WidgetBuilder, BuildContext, UiNode},
#     utils::into_gui_texture,
# };

fn create_image(ctx: &mut BuildContext, resource_manager: ResourceManager) -> Handle<UiNode> {
    ImageBuilder::new(WidgetBuilder::new().with_width(100.0).with_height(100.0))
        .with_flip(true) // Flips an image vertically
        .with_texture(into_gui_texture(
            resource_manager.request_texture("path/to/your/texture.png"),
        ))
        .build(ctx)
}
```

There are few places where it can be helpful:

- You're using render target as a source texture for your Image instance, render targets are vertically flipped due
to mismatch of coordinates of UI and graphics API. The UI has origin at left top corner, the graphics API - bottom left.
- Your source image is vertically mirrored.