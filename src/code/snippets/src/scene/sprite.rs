use fyrox::asset::manager::ResourceManager;
use fyrox::asset::untyped::ResourceKind;
use fyrox::core::uuid;
use fyrox::material::{Material, MaterialResource};
use fyrox::resource::texture::Texture;
use fyrox::scene::sprite::Sprite;
use fyrox::{
    core::{color::Color, pool::Handle},
    scene::{base::BaseBuilder, sprite::SpriteBuilder, Scene},
};

// ANCHOR: create_sprite
fn create_sprite(scene: &mut Scene) -> Handle<Sprite> {
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
) -> Handle<Sprite> {
    let mut material = Material::standard_sprite();
    material.bind(
        "diffuseTexture",
        Some(resource_manager.request::<Texture>("path/to/your_texture.jpg")),
    );

    // Material resources can be shared across multiple sprites (via simple `clone`).
    // This significantly improves performance if you have multiple rectangles with the
    // same material.
    let material_resource = MaterialResource::new_ok(
        uuid!("4799c329-5aa2-468d-a079-ea2170be1c74"),
        ResourceKind::Embedded,
        material,
    );

    SpriteBuilder::new(BaseBuilder::new())
        .with_material(material_resource)
        .build(&mut scene.graph)
}
// ANCHOR_END: create_sprite_with_texture
