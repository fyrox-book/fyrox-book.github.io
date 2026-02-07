use fyrox::asset::manager::ResourceManager;
use fyrox::core::futures::executor::block_on;
use fyrox::graph::SceneGraph;
use fyrox::resource::model::{Model, ModelResourceExtension};
use fyrox::scene::camera::Camera;
use fyrox::scene::mesh::MeshBuilder;
use fyrox::scene::sprite::SpriteBuilder;
use fyrox::{
    core::{algebra::Vector3, pool::Handle},
    scene::{
        base::BaseBuilder, camera::CameraBuilder, node::Node, transform::TransformBuilder, Scene,
    },
};

// ANCHOR: create_camera
fn create_camera(scene: &mut Scene) -> Handle<Camera> {
    CameraBuilder::new(
        // Here we passing a base builder. Note that, since we can build Base node separately
        // we can pass any custom values to it while building.
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                .with_local_position(Vector3::new(2.0, 0.0, 3.0))
                .build(),
        ),
    )
    // Here we just setting desired Camera properties.
    .with_fov(60.0f32.to_radians())
    .build(&mut scene.graph)
}
// ANCHOR_END: create_camera

// ANCHOR: create_node
fn create_node(scene: &mut Scene) -> Handle<Camera> {
    CameraBuilder::new(
        BaseBuilder::new()
            // Add some children nodes.
            .with_child(
                // A staff...
                MeshBuilder::new(
                    BaseBuilder::new()
                        .with_name("MyFancyStaff")
                        .with_local_transform(
                            TransformBuilder::new()
                                .with_local_position(Vector3::new(0.5, 0.5, 1.0))
                                .build(),
                        ),
                )
                .build(&mut scene.graph),
            )
            .with_child(
                // and a spell.
                SpriteBuilder::new(
                    BaseBuilder::new()
                        .with_name("MyFancyFireball")
                        .with_local_transform(
                            TransformBuilder::new()
                                .with_local_position(Vector3::new(-0.5, 0.5, 1.0))
                                .build(),
                        ),
                )
                .build(&mut scene.graph),
            )
            .with_local_transform(
                TransformBuilder::new()
                    .with_local_position(Vector3::new(2.0, 0.0, 3.0))
                    .build(),
            ),
    )
    .with_fov(60.0f32.to_radians())
    .build(&mut scene.graph)
}
// ANCHOR_END: create_node

// ANCHOR: create_node_manually
fn create_node_manually(scene: &mut Scene) -> Handle<Node> {
    let node: Node = CameraBuilder::new(BaseBuilder::new()).build_node();

    // We must explicitly add the node to the graph.
    scene.graph.add_node(node)
}
// ANCHOR_END: create_node_manually

// ANCHOR: link_weapon_to_camera
fn link_weapon_to_camera(
    scene: &mut Scene,
    camera: Handle<Node>,
    resource_manager: ResourceManager,
) {
    let weapon = block_on(resource_manager.request::<Model>("path/to/weapon.fbx"))
        .unwrap()
        .instantiate(scene);

    // Link weapon to the camera.
    scene.graph.link_nodes(weapon, camera);
}
// ANCHOR_END: link_weapon_to_camera

// ANCHOR: remove_preserve_children
fn remove_preserve_children(scene: &mut Scene, node_to_remove: Handle<Node>) {
    for child in scene.graph[node_to_remove].children().to_vec() {
        scene.graph.unlink_node(child);
    }

    scene.graph.remove_node(node_to_remove);
}
// ANCHOR_END: remove_preserve_children
