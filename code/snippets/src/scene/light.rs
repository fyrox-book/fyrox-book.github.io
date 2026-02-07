use fyrox::asset::manager::ResourceManager;
use fyrox::core::algebra::{UnitQuaternion, Vector3};
use fyrox::core::color::Color;
use fyrox::core::futures::executor::block_on;
use fyrox::gui::texture::TextureWrapMode;
use fyrox::resource::texture::Texture;
use fyrox::scene::light::directional::DirectionalLight;
use fyrox::scene::light::point::{PointLight, PointLightBuilder};
use fyrox::scene::light::spot::{SpotLight, SpotLightBuilder};
use fyrox::scene::light::BaseLight;
use fyrox::scene::skybox::{SkyBox, SkyBoxBuilder};
use fyrox::scene::transform::TransformBuilder;
use fyrox::scene::EnvironmentLightingSource;
use fyrox::{
    core::pool::Handle,
    graph::SceneGraphNode,
    scene::{
        base::BaseBuilder,
        light::{directional::DirectionalLightBuilder, BaseLightBuilder},
        node::Node,
        Scene,
    },
};

// ANCHOR: create_directional_light
fn create_directional_light(scene: &mut Scene) -> Handle<DirectionalLight> {
    DirectionalLightBuilder::new(BaseLightBuilder::new(BaseBuilder::new())).build(&mut scene.graph)
}
// ANCHOR_END: create_directional_light

// ANCHOR: create_oriented_directional_light
fn create_oriented_directional_light(scene: &mut Scene) -> Handle<DirectionalLight> {
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
fn create_point_light(scene: &mut Scene) -> Handle<PointLight> {
    PointLightBuilder::new(BaseLightBuilder::new(BaseBuilder::new()))
        .with_radius(5.0)
        .build(&mut scene.graph)
}
// ANCHOR_END: create_point_light

// ANCHOR: create_spot_light
fn create_spot_light(scene: &mut Scene) -> Handle<SpotLight> {
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
        .component_mut::<BaseLight>()
        .unwrap()
        .enable_scatter(false);
}
// ANCHOR_END: disable_light_scatter

// ANCHOR: use_rayleigh_scattering
fn use_rayleigh_scattering(scene: &mut Scene, light_handle: Handle<Node>) {
    scene.graph[light_handle]
        .component_mut::<BaseLight>()
        .unwrap()
        .set_scatter(Vector3::new(0.03, 0.035, 0.055));
}
// ANCHOR_END: use_rayleigh_scattering

// ANCHOR: switch_shadows
fn switch_shadows(scene: &mut Scene, light_handle: Handle<Node>, cast_shadows: bool) {
    scene.graph[light_handle]
        .component_mut::<BaseLight>()
        .unwrap()
        .set_cast_shadows(cast_shadows);
}
// ANCHOR_END: switch_shadows

// ANCHOR: set_ambient_lighting
fn set_ambient_lighting(scene: &mut Scene) {
    scene.rendering_options.ambient_lighting_color = Color::opaque(30, 30, 30);
    scene.rendering_options.environment_lighting_source = EnvironmentLightingSource::AmbientColor;
} // ANCHOR_END: set_ambient_lighting

// ANCHOR: set_scene_skybox
async fn create_skybox(resource_manager: ResourceManager) -> SkyBox {
    // Load skybox textures in parallel.
    let (front, back, left, right, top, bottom) = fyrox::core::futures::join!(
        resource_manager.request::<Texture>("path/to/front.jpg"),
        resource_manager.request::<Texture>("path/to/back.jpg"),
        resource_manager.request::<Texture>("path/to/left.jpg"),
        resource_manager.request::<Texture>("path/to/right.jpg"),
        resource_manager.request::<Texture>("path/to/up.jpg"),
        resource_manager.request::<Texture>("path/to/down.jpg")
    );

    // Unwrap everything.
    let skybox = SkyBoxBuilder {
        front: Some(front.unwrap()),
        back: Some(back.unwrap()),
        left: Some(left.unwrap()),
        right: Some(right.unwrap()),
        top: Some(top.unwrap()),
        bottom: Some(bottom.unwrap()),
    }
    .build()
    .unwrap();

    // Set S and T coordinate wrap mode, ClampToEdge will remove any possible seams on edges
    // of the skybox.
    let skybox_texture = skybox.cubemap().unwrap();
    let mut data = skybox_texture.data_ref();
    data.set_s_wrap_mode(TextureWrapMode::ClampToEdge);
    data.set_t_wrap_mode(TextureWrapMode::ClampToEdge);

    skybox
}

fn set_scene_skybox(scene: &mut Scene, resource_manager: ResourceManager) {
    scene.set_skybox(Some(block_on(create_skybox(resource_manager))));
}
// ANCHOR_END: set_scene_skybox
