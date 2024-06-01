use fyrox::core::algebra::{UnitQuaternion, Vector3};
use fyrox::scene::light::point::PointLightBuilder;
use fyrox::scene::light::spot::SpotLightBuilder;
use fyrox::scene::light::BaseLight;
use fyrox::scene::transform::TransformBuilder;
use fyrox::{
    core::pool::Handle,
    scene::{
        base::BaseBuilder,
        light::{directional::DirectionalLightBuilder, BaseLightBuilder},
        node::Node,
        Scene,
    },
};

// ANCHOR: create_directional_light
fn create_directional_light(scene: &mut Scene) -> Handle<Node> {
    DirectionalLightBuilder::new(BaseLightBuilder::new(BaseBuilder::new())).build(&mut scene.graph)
}
// ANCHOR_END: create_directional_light

// ANCHOR: create_oriented_directional_light
fn create_oriented_directional_light(scene: &mut Scene) -> Handle<Node> {
    DirectionalLightBuilder::new(BaseLightBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                .with_local_rotation(UnitQuaternion::from_axis_angle(
                    &Vector3::x_axis(),
                    -45.0f32.to_radians(),
                ))
                .build(),
        ),
    ))
    .build(&mut scene.graph)
}
// ANCHOR_END: create_oriented_directional_light

// ANCHOR: create_point_light
fn create_point_light(scene: &mut Scene) -> Handle<Node> {
    PointLightBuilder::new(BaseLightBuilder::new(BaseBuilder::new()))
        .with_radius(5.0)
        .build(&mut scene.graph)
}
// ANCHOR_END: create_point_light

// ANCHOR: create_spot_light
fn create_spot_light(scene: &mut Scene) -> Handle<Node> {
    SpotLightBuilder::new(BaseLightBuilder::new(BaseBuilder::new()))
        .with_distance(5.0)
        .with_hotspot_cone_angle(50.0f32.to_radians())
        .with_falloff_angle_delta(10.0f32.to_radians())
        .build(&mut scene.graph)
}
// ANCHOR_END: create_spot_light

// ANCHOR: disable_light_scatter
fn disable_light_scatter(scene: &mut Scene, light_handle: Handle<Node>) {
    scene.graph[light_handle]
        .query_component_mut::<BaseLight>()
        .unwrap()
        .enable_scatter(false);
}
// ANCHOR_END: disable_light_scatter

// ANCHOR: use_rayleigh_scattering
fn use_rayleigh_scattering(scene: &mut Scene, light_handle: Handle<Node>) {
    scene.graph[light_handle]
        .query_component_mut::<BaseLight>()
        .unwrap()
        .set_scatter(Vector3::new(0.03, 0.035, 0.055));
}
// ANCHOR_END: use_rayleigh_scattering

// ANCHOR: switch_shadows
fn switch_shadows(scene: &mut Scene, light_handle: Handle<Node>, cast_shadows: bool) {
    scene.graph[light_handle]
        .query_component_mut::<BaseLight>()
        .unwrap()
        .set_cast_shadows(cast_shadows);
}
// ANCHOR_END: switch_shadows
