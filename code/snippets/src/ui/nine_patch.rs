use fyrox::gui::texture::TextureResource;
use fyrox::gui::{
    core::{math::Rect, pool::Handle},
    nine_patch::NinePatch,
    nine_patch::NinePatchBuilder,
    widget::WidgetBuilder,
    UserInterface,
};

// ANCHOR: create_nine_patch
fn create_nine_patch(texture: TextureResource, ui: &mut UserInterface) -> Handle<NinePatch> {
    NinePatchBuilder::new(WidgetBuilder::new())
        // Specify margins for each side in pixels.
        .with_left_margin(12)
        .with_right_margin(12)
        .with_top_margin(12)
        .with_bottom_margin(12)
        .with_texture(texture)
        .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_nine_patch

// ANCHOR: create_nine_patch_with_region
fn create_nine_patch_with_region(
    texture: TextureResource,
    ui: &mut UserInterface,
) -> Handle<NinePatch> {
    NinePatchBuilder::new(WidgetBuilder::new())
        // Specify margins for each side in pixels.
        .with_left_margin(12)
        .with_right_margin(12)
        .with_top_margin(12)
        .with_bottom_margin(12)
        .with_texture(texture)
        // Additionally, you can also specify a region in a texture to use. It is useful if you
        // have a texture atlas where most of the UI elements are packed.
        .with_texture_region(Rect::new(200, 200, 400, 400))
        .build(&mut ui.build_ctx())
}
// ANCHOR_END: create_nine_patch_with_region
