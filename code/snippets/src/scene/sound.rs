use fyrox::graph::SceneGraph;
use fyrox::scene::sound::Sound;
use fyrox::{
    asset::manager::ResourceManager,
    core::pool::Handle,
    scene::node::Node,
    scene::sound::SoundBufferResource,
    scene::{
        base::BaseBuilder,
        sound::{SoundBuffer, SoundBuilder, Status},
        Scene,
    },
};
use std::path::Path;

// ANCHOR: load_sound
pub fn load_sound(path: &Path, resource_manager: &ResourceManager) -> SoundBufferResource {
    resource_manager.request::<SoundBuffer>(path)
}
// ANCHOR_END: load_sound

// ANCHOR: build_sound_node
fn build_sound_node(resource_manager: &ResourceManager, scene: &mut Scene) -> Handle<Sound> {
    let sound = resource_manager.request::<SoundBuffer>("/path/to/resource.ogg");

    SoundBuilder::new(BaseBuilder::new())
        .with_buffer(Some(sound))
        .with_status(Status::Playing)
        .with_play_once(true)
        .build(&mut scene.graph)
}
// ANCHOR_END: build_sound_node

// ANCHOR: sound_removal
fn update_sound(sound_handle: Handle<Node>, scene: &mut Scene) {
    let sound = scene.graph[sound_handle].as_sound();

    if sound.status() == Status::Stopped {
        scene.graph.remove_node(sound_handle);
    }
}
// ANCHOR_END: sound_removal

// ANCHOR: looping
fn build_looping_sound(scene: &mut Scene) {
    SoundBuilder::new(BaseBuilder::new())
        .with_looping(true)
        // ...
        .build(&mut scene.graph);
}
// ANCHOR_END: looping
