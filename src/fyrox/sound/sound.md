# Sound

In Fyrox, sounds are nodes of type Sound, with all the consequent properties and workflows.

Audio files are loaded using the resource manager:

```rust,no_run
# extern crate fyrox;
# use fyrox::{engine::Engine, scene::Scene};
# fn build_node(engine: Engine, scene: &mut Scene) {
let sound = engine
    .resource_manager
    .request_sound_buffer("/path/to/resource.ogg");
# }
```

Then, the node is built using the standard builder pattern:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     engine::Engine,
#     scene::{
#         base::BaseBuilder,
#         sound::{SoundBuilder, Status},
#         Scene,
#     },
# };
# fn build_node(engine: Engine, scene: &mut Scene) {
# let sound = engine
#     .resource_manager
#     .request_sound_buffer("/path/to/resource.ogg");
#
let sound_handle = SoundBuilder::new(BaseBuilder::new())
    .with_buffer(Some(sound))
    .with_status(Status::Playing)
    .with_play_once(true)
    .build(&mut scene.graph);
# }
```

There are a few notable things in the example above.

The first is that sounds don't play automatically; in order to do so, we need to invoke `.with_status(Status::Playing)`.

The second is that sound nodes are not dropped automatically after playback; dropping it can be performed in two ways. One way is to use the convenient builder API `.with_play_once(true)`; another is to use the graph APIs:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     engine::Engine,
#     scene::{
#         base::BaseBuilder,
#         sound::{SoundBuilder, Status},
#         Scene,
#     },
# };
# fn build_node(engine: Engine, scene: &mut Scene) {
let sound_handle = SoundBuilder::new(BaseBuilder::new()).build(&mut scene.graph);

let sound = scene.graph[sound_handle].as_sound();

if sound.status() == Status::Stopped {
    scene.remove_node(sound_handle);
}
# }
```

If we want to play background music (or anyway a repeated sound), we just set the `looping` property when building the node:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     engine::Engine,
#     scene::{base::BaseBuilder, sound::SoundBuilder, Scene},
# };
# fn build_node(engine: Engine, scene: &mut Scene) {
SoundBuilder::new(BaseBuilder::new())
    .with_looping(true)
    // etc.
    .build(&mut scene.graph);
# }
```

In order to stream large audio files, instead of loading them entirely in memory, the simplest strategy is to create a corresponding `.options` file, with the following content:

```ron
(
  stream: true
)
```

If the audio file is called, for example, `/path/to/background.ogg`, call this `/path/to/background.ogg.options`.