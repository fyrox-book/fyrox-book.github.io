use fyrox::asset::manager::ResourceManager;
use fyrox::asset::untyped::ResourceKind;
use fyrox::core::algebra::Vector3;
use fyrox::core::color::Color;
use fyrox::core::color_gradient::{ColorGradient, GradientPoint};
use fyrox::material::{Material, MaterialResource};
use fyrox::resource::texture::Texture;
use fyrox::scene::base::BaseBuilder;
use fyrox::scene::graph::Graph;
use fyrox::scene::particle_system::{
    emitter::base::BaseEmitterBuilder, emitter::sphere::SphereEmitterBuilder, ParticleSystemBuilder,
};
use fyrox::scene::transform::TransformBuilder;

// ANCHOR: create_smoke
fn create_smoke(graph: &mut Graph, resource_manager: &mut ResourceManager, pos: Vector3<f32>) {
    let mut material = Material::standard_particle_system();
    material.bind(
        "diffuseTexture",
        Some(resource_manager.request::<Texture>("data/particles/smoke_04.tga")),
    );
    let material_resource = MaterialResource::new_ok(ResourceKind::Embedded, material);

    ParticleSystemBuilder::new(
        BaseBuilder::new()
            .with_lifetime(5.0)
            .with_local_transform(TransformBuilder::new().with_local_position(pos).build()),
    )
    .with_acceleration(Vector3::new(0.0, 0.0, 0.0))
    .with_color_over_lifetime_gradient({
        let mut gradient = ColorGradient::new();
        gradient.add_point(GradientPoint::new(0.00, Color::from_rgba(150, 150, 150, 0)));
        gradient.add_point(GradientPoint::new(
            0.05,
            Color::from_rgba(150, 150, 150, 220),
        ));
        gradient.add_point(GradientPoint::new(
            0.85,
            Color::from_rgba(255, 255, 255, 180),
        ));
        gradient.add_point(GradientPoint::new(1.00, Color::from_rgba(255, 255, 255, 0)));
        gradient
    })
    .with_emitters(vec![SphereEmitterBuilder::new(
        BaseEmitterBuilder::new()
            .with_max_particles(100)
            .with_spawn_rate(50)
            .with_x_velocity_range(-0.01..0.01)
            .with_y_velocity_range(0.02..0.03)
            .with_z_velocity_range(-0.01..0.01),
    )
    .with_radius(0.01)
    .build()])
    .with_material(material_resource)
    .build(graph);
}
// ANCHOR_END: create_smoke
