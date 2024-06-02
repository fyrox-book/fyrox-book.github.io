use fyrox::resource::texture::TextureKind;
use fyrox::{
    asset::manager::ResourceManager,
    core::pool::Handle,
    gui::{image::ImageBuilder, widget::WidgetBuilder, BuildContext, UiNode},
    resource::texture::Texture,
};

// ANCHOR: create_image
fn create_image(ctx: &mut BuildContext, resource_manager: ResourceManager) -> Handle<UiNode> {
    // You must explicitly set width and height of the image, otherwise it will collapse to a
    // point and you won't see anything.
    let width = 100.0;
    let height = 100.0;
    ImageBuilder::new(WidgetBuilder::new().with_width(width).with_height(height))
        .with_texture(
            // Ask resource manager to load a texture.
            resource_manager
                .request::<Texture>("path/to/your/texture.png")
                .into(),
        )
        .build(ctx)
}
// ANCHOR_END: create_image

// ANCHOR: create_image_equal_in_size_to_source
async fn create_image_equal_in_size_to_source(
    ctx: &mut BuildContext<'_>,
    resource_manager: ResourceManager,
) -> Handle<UiNode> {
    // Ask resource manager to load the texture and wait while it loads using `.await`.
    if let Ok(texture) = resource_manager
        .request::<Texture>("path/to/your/texture.png")
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
            .with_texture(texture.into())
            .build(ctx);
        }
    }

    // Image wasn't created.
    Handle::NONE
}
// ANCHOR_END: create_image_equal_in_size_to_source

// ANCHOR: create_image_sync
fn create_image_sync(
    ctx: &mut BuildContext<'_>,
    resource_manager: ResourceManager,
) -> Handle<UiNode> {
    fyrox::core::futures::executor::block_on(create_image_equal_in_size_to_source(
        ctx,
        resource_manager,
    ))
}
// ANCHOR_END: create_image_sync

// ANCHOR: create_flipped_image
fn create_flipped_image(
    ctx: &mut BuildContext,
    resource_manager: ResourceManager,
) -> Handle<UiNode> {
    ImageBuilder::new(WidgetBuilder::new().with_width(100.0).with_height(100.0))
        .with_flip(true) // Flips an image vertically
        .with_texture(
            resource_manager
                .request::<Texture>("path/to/your/texture.png")
                .into(),
        )
        .build(ctx)
}
// ANCHOR_END: create_flipped_image
