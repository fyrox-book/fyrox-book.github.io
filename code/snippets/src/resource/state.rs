use fyrox::asset::Resource;
use fyrox::core::{futures, uuid};
use fyrox::resource::texture::{
    CompressionOptions, Texture, TextureImportOptions, TextureMinificationFilter, TextureResource,
    TextureResourceExtension,
};

// ANCHOR: checked_access
fn checked_access(texture_resource: &Resource<Texture>) {
    let mut state = texture_resource.state();
    if let Some(texture) = state.data() {
        println!("Kind: {:?}", texture.kind());
    }
}
// ANCHOR_END: checked_access

// ANCHOR: unchecked_access
fn unchecked_access(texture_resource: &Resource<Texture>) {
    let texture = texture_resource.data_ref();
    println!("Kind: {:?}", texture.kind());
}
// ANCHOR_END: unchecked_access

// ANCHOR: await_resource
async fn await_resource(texture_resource: Resource<Texture>) {
    if let Ok(result) = texture_resource.await {
        // `data_ref` will never panic after the above check.
        let texture = result.data_ref();
        println!("Kind: {:?}", texture.kind());
    };
}
// ANCHOR_END: await_resource

// ANCHOR: block_and_wait
fn block_and_wait(texture_resource: Resource<Texture>) {
    // Block the current thread and wait until the resource is loaded.
    if let Ok(result) = futures::executor::block_on(texture_resource) {
        // `data_ref` will never panic after the above check.
        let texture = result.data_ref();
        println!("Kind: {:?}", texture.kind());
    };
}
// ANCHOR_END: block_and_wait

// ANCHOR: embedded_texture
fn embedded_resource() -> Option<Resource<Texture>> {
    let data = include_bytes!("texture.png");
    TextureResource::load_from_memory(
        uuid!("cb05986b-5483-4509-826c-8d90280b2eb8"),
        Default::default(),
        data,
        TextureImportOptions::default()
            .with_compression(CompressionOptions::NoCompression)
            .with_minification_filter(TextureMinificationFilter::Linear),
    )
    .ok()
}
// ANCHOR_END: embedded_texture
