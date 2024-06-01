use fyrox::asset::untyped::ResourceKind;
use fyrox::core::math::Rect;
use fyrox::material::{Material, MaterialResource};
use fyrox::scene::dim2::rectangle::Rectangle;
use fyrox::{
    asset::manager::ResourceManager,
    core::{algebra::Vector3, color::Color, pool::Handle},
    resource::texture::Texture,
    scene::{
        base::BaseBuilder, dim2::rectangle::RectangleBuilder, graph::Graph, node::Node,
        transform::TransformBuilder,
    },
};

// ANCHOR: create_rect
fn create_rect(graph: &mut Graph, resource_manager: ResourceManager) -> Handle<Node> {
    let mut material = Material::standard_2d();
    material
        .set_texture(
            &"diffuseTexture".into(),
            Some(resource_manager.request::<Texture>("path/to/your_texture.jpg")),
        )
        .unwrap();

    // Material resources can be shared across multiple rectangles (via simple `clone`).
    // This significantly improves performance if you have multiple rectangles with the
    // same material.
    let material_resource = MaterialResource::new_ok(ResourceKind::Embedded, material);

    RectangleBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                // Size of the rectangle is defined only by scale.
                .with_local_scale(Vector3::new(0.4, 0.2, 1.0))
                .build(),
        ),
    )
    .with_color(Color::RED)
    .with_material(material_resource)
    .build(graph)
}
// ANCHOR_END: create_rect

// ANCHOR: set_2nd_quarter_image_portion
fn set_2nd_quarter_image_portion(rectangle: &mut Rectangle) {
    rectangle.set_uv_rect(Rect::new(
        0.5, // Offset by 50% to the right
        0.0, // No need to offset to bottom.
        0.5, // Use half (50%) of width and height
        0.5,
    ));
}
// ANCHOR_END: set_2nd_quarter_image_portion
