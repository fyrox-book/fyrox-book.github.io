use fyrox::asset::manager::ResourceManager;
use fyrox::asset::untyped::ResourceKind;
use fyrox::material::{Material, MaterialResource};
use fyrox::resource::texture::Texture;
use fyrox::{
    core::{color::Color, pool::Handle},
    scene::{base::BaseBuilder, node::Node, sprite::SpriteBuilder, Scene},
};

// ANCHOR: create_sprite
fn create_sprite(scene: &mut Scene) -> Handle<Node> {
    SpriteBuilder::new(BaseBuilder::new())
        .with_size(2.0)
        .with_rotation(45.0f32.to_radians())
        .with_color(Color::RED)
        .build(&mut scene.graph)
}
// ANCHOR_END: create_sprite

// ANCHOR: create_sprite_with_texture
fn create_sprite_with_texture(
    scene: &mut Scene,
    resource_manager: ResourceManager,
) -> Handle<Node> {
    let mut material = Material::standard_sprite();
    material
        .set_texture(
            &"diffuseTexture".into(),
            Some(resource_manager.request::<Texture>("path/to/your_texture.jpg")),
        )
        .unwrap();

    // Material resources can be shared across multiple sprites (via simple `clone`).
    // This significantly improves performance if you have multiple rectangles with the
    // same material.
    let material_resource = MaterialResource::new_ok(ResourceKind::Embedded, material);

    SpriteBuilder::new(BaseBuilder::new())
        .with_material(material_resource)
        .build(&mut scene.graph)
}
// ANCHOR_END: create_sprite_with_texture
