use fyrox::asset::manager::ResourceManager;
use fyrox::core::pool::Handle;
use fyrox::resource::texture::Texture;
use fyrox::scene::base::BaseBuilder;
use fyrox::scene::decal::{Decal, DecalBuilder};
use fyrox::scene::Scene;

// ANCHOR: create_decal
fn create_decal(scene: &mut Scene, resource_manager: ResourceManager) -> Handle<Decal> {
    DecalBuilder::new(BaseBuilder::new())
        .with_diffuse_texture(resource_manager.request::<Texture>("path/to/your/decal.png"))
        .build(&mut scene.graph)
}
// ANCHOR_END: create_decal
