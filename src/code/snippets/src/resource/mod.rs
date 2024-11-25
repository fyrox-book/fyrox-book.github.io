pub mod custom;
pub mod events;
pub mod model;
pub mod sound;
pub mod state;

use fyrox::{
    asset::manager::ResourceManager,
    core::pool::Handle,
    resource::model::{Model, ModelResourceExtension},
    scene::{node::Node, Scene},
};
use std::path::Path;

// ANCHOR: instantiate_model
async fn instantiate_model(
    path: &Path,
    resource_manager: ResourceManager,
    scene: &mut Scene,
) -> Handle<Node> {
    // Load the model first. Alternatively, you can store the resource handle somewhere and use it for instantiation.
    let model = resource_manager.request::<Model>(path).await.unwrap();

    model.instantiate(scene)
}
// ANCHOR_END: instantiate_model
