use fyrox::core::algebra::Vector3;
use fyrox::core::pool::Handle;
use fyrox::scene::node::Node;
use fyrox::{
    scene::base::BaseBuilder,
    scene::pivot::PivotBuilder,
    scene::{camera::CameraBuilder, Scene},
};

fn build_node(scene: &mut Scene) {
    // ANCHOR: build_node
    let handle = PivotBuilder::new(BaseBuilder::new()).build(&mut scene.graph);
    // ANCHOR_END: build_node
}

fn build_complex_node(scene: &mut Scene) {
    // ANCHOR: build_complex_node
    let handle =
        PivotBuilder::new(
            BaseBuilder::new().with_children(&[
                CameraBuilder::new(BaseBuilder::new()).build(&mut scene.graph),
                PivotBuilder::new(BaseBuilder::new().with_children(&[
                    PivotBuilder::new(BaseBuilder::new()).build(&mut scene.graph),
                ]))
                .build(&mut scene.graph),
            ]),
        )
        .build(&mut scene.graph);
    // ANCHOR_END: build_complex_node
}

fn build_complex_node_flat(scene: &mut Scene) {
    // ANCHOR: build_complex_node_flat
    let camera = CameraBuilder::new(BaseBuilder::new()).build(&mut scene.graph);

    let child_base = PivotBuilder::new(BaseBuilder::new()).build(&mut scene.graph);

    let base =
        PivotBuilder::new(BaseBuilder::new().with_children(&[child_base])).build(&mut scene.graph);

    let handle = PivotBuilder::new(BaseBuilder::new().with_children(&[camera, base]))
        .build(&mut scene.graph);
    // ANCHOR_END: build_complex_node_flat
}

fn translate_node(scene: &mut Scene, node_handle: Handle<Node>) {
    // ANCHOR: translate_node
    scene.graph[node_handle]
        .local_transform_mut()
        .set_position(Vector3::new(1.0, 0.0, 2.0));
    // ANCHOR_END: translate_node
}

fn transform_node(scene: &mut Scene, node_handle: Handle<Node>) {
    // ANCHOR: transform_node
    scene.graph[node_handle]
        .local_transform_mut()
        .set_position(Vector3::new(1.0, 0.0, 2.0))
        .set_scale(Vector3::new(2.0, 2.0, 2.0))
        .set_rotation_offset(Vector3::new(1.0, 1.0, 0.0));
    // ANCHOR_END: transform_node
}
