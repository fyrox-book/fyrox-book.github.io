use fyrox::asset::untyped::ResourceKind;
use fyrox::material::MaterialResource;
use fyrox::{
    asset::manager::ResourceManager,
    core::{algebra::Vector2, algebra::Vector3, pool::Handle, sstorage::ImmutableString},
    material::{shader::SamplerFallback, Material, PropertyValue},
    rand::{thread_rng, Rng},
    resource::texture::Texture,
    scene::{
        base::BaseBuilder,
        graph::Graph,
        node::Node,
        terrain::{Brush, BrushMode, BrushShape, Layer, TerrainBuilder},
    },
};

// ANCHOR: create_random_two_layer_terrain
fn setup_layer_material(
    material: &mut Material,
    resource_manager: ResourceManager,
    diffuse_texture: &str,
    normal_texture: &str,
) {
    material
        .set_property(
            &ImmutableString::new("diffuseTexture"),
            PropertyValue::Sampler {
                value: Some(resource_manager.request::<Texture>(diffuse_texture)),
                fallback: SamplerFallback::White,
            },
        )
        .unwrap();
    material
        .set_property(
            &ImmutableString::new("normalTexture"),
            PropertyValue::Sampler {
                value: Some(resource_manager.request::<Texture>(normal_texture)),
                fallback: SamplerFallback::Normal,
            },
        )
        .unwrap();
    material
        .set_property(
            &ImmutableString::new("texCoordScale"),
            PropertyValue::Vector2(Vector2::new(10.0, 10.0)),
        )
        .unwrap();
}

fn create_random_two_layer_terrain(
    graph: &mut Graph,
    resource_manager: &ResourceManager,
) -> Handle<Node> {
    let terrain = TerrainBuilder::new(BaseBuilder::new())
        .with_layers(vec![
            Layer {
                material: {
                    let mut material = Material::standard_terrain();
                    setup_layer_material(
                        &mut material,
                        resource_manager.clone(),
                        "examples/data/Grass_DiffuseColor.jpg",
                        "examples/data/Grass_NormalColor.jpg",
                    );
                    MaterialResource::new_ok(ResourceKind::Embedded, material)
                },
                ..Default::default()
            },
            Layer {
                material: {
                    let mut material = Material::standard_terrain();
                    setup_layer_material(
                        &mut material,
                        resource_manager.clone(),
                        "examples/data/Rock_DiffuseColor.jpg",
                        "examples/data/Rock_Normal.jpg",
                    );
                    MaterialResource::new_ok(ResourceKind::Embedded, material)
                },
                ..Default::default()
            },
        ])
        .build(graph);

    let terrain_ref = graph[terrain].as_terrain_mut();

    // Draw something on the terrain.
    for _ in 0..60 {
        let x = thread_rng().gen_range(4.0..60.00);
        let z = thread_rng().gen_range(4.0..60.00);
        let radius = thread_rng().gen_range(2.0..4.0);
        let height = thread_rng().gen_range(1.0..3.0);

        // Pull terrain.
        terrain_ref.draw(&Brush {
            center: Vector3::new(x, 0.0, z),
            shape: BrushShape::Circle { radius },
            mode: BrushMode::ModifyHeightMap { amount: height },
        });

        // Draw rock texture on top.
        terrain_ref.draw(&Brush {
            center: Vector3::new(x, 0.0, z),
            shape: BrushShape::Circle { radius },
            mode: BrushMode::DrawOnMask {
                layer: 1,
                alpha: 1.0,
            },
        });
    }

    terrain
}
// ANCHOR_END: create_random_two_layer_terrain
