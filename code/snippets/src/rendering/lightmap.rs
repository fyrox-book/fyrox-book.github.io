use fyrox::asset::manager::ResourceManager;
use fyrox::{
    asset::ResourceData,
    core::algebra::{Matrix4, Vector3},
    scene::{
        base::BaseBuilder,
        light::{point::PointLightBuilder, BaseLightBuilder},
        mesh::{
            surface::SurfaceSharedData,
            surface::{SurfaceBuilder, SurfaceData},
            MeshBuilder,
        },
        transform::TransformBuilder,
        Scene,
    },
    utils::lightmap::{Lightmap, LightmapInputData},
};
use std::path::Path;

// ANCHOR: generate_lightmap
fn generate_lightmap() {
    // Create a test scene first.
    let mut scene = Scene::new();

    let data = SurfaceData::make_cone(
        16,
        1.0,
        1.0,
        &Matrix4::new_nonuniform_scaling(&Vector3::new(1.0, 1.1, 1.0)),
    );

    MeshBuilder::new(BaseBuilder::new())
        .with_surfaces(vec![
            SurfaceBuilder::new(SurfaceSharedData::new(data)).build()
        ])
        .build(&mut scene.graph);

    PointLightBuilder::new(BaseLightBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                .with_local_position(Vector3::new(0.0, 2.0, 0.0))
                .build(),
        ),
    ))
    .with_radius(4.0)
    .build(&mut scene.graph);

    // Prepare the data for generation using the scene.
    let data =
        LightmapInputData::from_scene(&scene, |_, _| true, Default::default(), Default::default())
            .unwrap();

    // Generate the lightmap.
    let lightmap = Lightmap::new(data, 64, 0.005, Default::default(), Default::default()).unwrap();

    // Save each texture to disk.
    let mut counter = 0;
    for entry_set in lightmap.map.values() {
        for entry in entry_set {
            let mut data = entry.texture.as_ref().unwrap().data_ref();
            data.save(Path::new(&format!("{}.png", counter))).unwrap();
            counter += 1;
        }
    }
}
// ANCHOR_END: generate_lightmap

// ANCHOR: change_light_map
fn change_light_map(scene: &mut Scene, resource_manager: ResourceManager) {
    let light_map = fyrox::core::futures::executor::block_on(Lightmap::load(
        "a/path/to/lightmap.lmp",
        resource_manager,
    ))
    .unwrap();

    scene.graph.set_lightmap(light_map).unwrap();
}
// ANCHOR_END: change_light_map
