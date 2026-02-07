use fyrox::scene::probe::ReflectionProbe;
use fyrox::{
    core::{algebra::Vector3, pool::Handle},
    scene::{
        base::BaseBuilder,
        graph::Graph,
        probe::{ReflectionProbeBuilder, UpdateMode},
        transform::TransformBuilder,
    },
};

// ANCHOR: create_probe
fn create_probe(graph: &mut Graph) -> Handle<ReflectionProbe> {
    ReflectionProbeBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                // The center of the probe's bounding box is located 10 units above the ground.
                .with_local_position(Vector3::new(0.0, 10.0, 0.0))
                // The size of the probe's bounding box is 20 units.
                .with_local_scale(Vector3::repeat(20.0))
                .build(),
        ),
    )
    // Set resolution of the probe.
    .with_resolution(256)
    // The probe will capture the scene once it is created.
    .with_update_mode(UpdateMode::Once)
    // Set the capture point slightly off-center. The probe will capture the scene at
    // (10.0, 10.0, 0.0) point.
    .with_rendering_local_position(Vector3::new(10.0, 0.0, 0.0))
    .build(graph)
}
// ANCHOR_END: create_probe
