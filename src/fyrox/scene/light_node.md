# Light node

The engine offers complex lighting system with various types of light sources. 

## Light types

There are three main types of light sources: directional, point, and spot lights.

### Directional light

Directional light does not have a position, its rays are always parallel,and it has a particular direction in space.
An example of directional light in real-life could be our Sun. Even if it is a point light, it is so far away from
the Earth, so we can assume that its rays are always parallel. Directional light sources are suitable for outdoor 
scenes.

A directional light source could be created something like this:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     scene::{
#         base::BaseBuilder,
#         light::{directional::DirectionalLightBuilder, BaseLightBuilder},
#         node::Node,
#         Scene,
#     },
# };

fn create_directional_light(scene: &mut Scene) -> Handle<Node> {
    DirectionalLightBuilder::new(BaseLightBuilder::new(BaseBuilder::new()))
        .build(&mut scene.graph)
}
```

By default, the light source will be oriented to lit "the ground". In other words its direction will be faced towards
`(0.0, -1.0, 0.0)` vector. You can rotate it as you want by setting local transform of it while building. Something
like this:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{
#         algebra::{UnitQuaternion, Vector3},
#         pool::Handle,
#     },
#     scene::{
#         base::BaseBuilder,
#         light::{directional::DirectionalLightBuilder, BaseLightBuilder},
#         node::Node,
#         transform::TransformBuilder,
#         Scene,
#     },
# };

fn create_directional_light(scene: &mut Scene) -> Handle<Node> {
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
```

### Point light

Point light is a light source that emits lights in all directions, it has a position, but does not have an orientation.
An example of a point light source: light bulb. 

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     scene::{
#         base::BaseBuilder,
#         light::{point::PointLightBuilder, BaseLightBuilder},
#         node::Node,
#         Scene,
#     },
# };

fn create_point_light(scene: &mut Scene) -> Handle<Node> {
    PointLightBuilder::new(BaseLightBuilder::new(BaseBuilder::new()))
        .with_radius(5.0)
        .build(&mut scene.graph)
}
```

### Spotlight

Spot light is a light source that emits lights in cone shape, it has a position and orientation. An example of 
a spot light source: flashlight.

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     scene::{
#         base::BaseBuilder,
#         light::{spot::SpotLightBuilder, BaseLightBuilder},
#         node::Node,
#         Scene,
#     },
# };

fn create_spot_light(scene: &mut Scene) -> Handle<Node> {
    SpotLightBuilder::new(BaseLightBuilder::new(BaseBuilder::new()))
        .with_distance(5.0)
        .with_hotspot_cone_angle(50.0f32.to_radians())
        .with_falloff_angle_delta(10.0f32.to_radians())
        .build(&mut scene.graph)
}
```

## Light scattering

Spot and point lights supports light scattering effect. Imagine you're walking with a flashlight in a foggy weather,
the fog will scatter the light from your flashlight making it, so you'll see the "light volume". Light scattering is
**enabled by default**, so you don't have to do anything to enable it. However, in some cases you might want to disable 
it, you can do this either while building a light source or change light scattering options on existing light source.
Here is the small example how to do that.

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     scene::{node::Node, light::BaseLight, Scene},
# };

fn disable_light_scatter(scene: &mut Scene, light_handle: Handle<Node>) {
    scene.graph[light_handle]
        .query_component_mut::<BaseLight>()
        .unwrap()
        .enable_scatter(false);
}
```

You could also change the amount of scattering per each color channel, using this you could imitate the 
[Rayleigh scattering](https://en.wikipedia.org/wiki/Rayleigh_scattering):

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{algebra::Vector3, pool::Handle},
#     scene::{node::Node, light::BaseLight, Scene},
# };

fn use_rayleigh_scattering(scene: &mut Scene, light_handle: Handle<Node>) {
    scene.graph[light_handle]
        .query_component_mut::<BaseLight>()
        .unwrap()
        .set_scatter(Vector3::new(0.03, 0.035, 0.055))
}
```

## Shadows

Spot and point lights are both supports shadows, however directional light still lacks shadows. There is a
[tracking issue](https://github.com/FyroxEngine/Fyrox/issues/220) for that.

## Performance

Lights are not very cheap, every light source has some performance impact. As a general rule, try to keep amount
of light sources at reasonable levels and especially try to avoid creating tons of light sources in a small area.
Keep in mind that the less area the light need to "cover", the higher the performance. This means that you can have
tons of small light sources almost free.

Shadows giving the most significant performance impact, you should keep amount of light sources that can cast
shadows at lowest possible amount to keep performance at good levels. You can also turn on/off shadows when you 
need:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     scene::{node::Node, light::BaseLight, Scene},
# };

fn switch_shadows(scene: &mut Scene, light_handle: Handle<Node>, cast_shadows: bool) {
    scene.graph[light_handle]
        .query_component_mut::<BaseLight>()
        .unwrap()
        .set_cast_shadows(cast_shadows)
}
```

Not every light should cast shadows, for example a small light that a player can see only in a distance can have
shadows disabled. You should set the appropriate values depending on your scene, just remember - the fewer shadows
the more performance you'll get. The most expensive shadows are from point lights, the less - from spot lights.