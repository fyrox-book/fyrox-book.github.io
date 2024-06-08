# Sound

In Fyrox, sounds are nodes of type `Sound`, with all the consequent properties and workflows.

## How to create

There are two major ways to create sound sources: from the editor and from code.

### From Editor

A sound source could be created from `Create` menu (or from the same menu by right-clicking on a node in the world viewer):

![create](create.png)

After the source is created, you can select it and start editing its properties:

![sound](sound.png)

- `Buffer` - a sound buffer resource, that will be used as a source of samples. If it is empty, then no sound will be
played. Drag'n'drop a sound resource from the Asset Browser here to assign it to the source. 
- `Play Once` - a flag, that defines whether the engine should automatically delete the sound source node from the scene
when it is finished playing. Could be useful for one-shot sounds.
- `Gain` - a numeric value in `[0..1]` range, that defines total volume of the sound source. Keep in mind, that this 
value sets the volume in _linear_ scale, while physically-correct approach would be to use logarithmic scale. This 
will be fixed in future versions.
- `Panning` - a numeric value in `[-1..1]` range, that defines how loud audio channels will be. `-1` - all the sound
will be routed to the left channel, `1` - to the right channel. This option works only with 2D sounds (whose spatial
blend factor is `0.0`)
- `Status` - a switch with three possible states: `Stopped`, `Playing`, `Paused`. By default, every sound source is 
in stopped state, do not forget to switch it to the `Playing` state, otherwise you won't hear anything.
- `Looping` - a flag, that defines whether the sound source should be playing infinitely, or not. Looping sound source
will never switch their status to `Stopped`.
- `Pitch` - playback speed multiplier. By default, it is `1.0` which means default speed.  
- `Max Distance` - maximum distance, at which the sound source is affected by distance attenuation (for 3D sounds). 
By default, it set to max possible value. Lower values could be used to prevent sound source from be silent at certain
distance.
- `Rolloff Factor` - a numeric value, that defines how fast the volume of the sound source will decay with increasing 
distance to a listener.
- `Playback Time` - desired time from which the playback should start (in seconds).
- `Spatial Blend` - a numeric value, that defines blending factor between 2D and 3D sound, where `0.0` - the sound is 
fully 2D, `1.0` - the sound is fully 3D. By default, the value is `1.0`.
- `Audio Bus` - a name of an audio bus, that will be used to process the samples from the sound source. By default, it 
is set to `Primary`. It should match the name of some audio bus, that will be used in your scene. More info about 
audio processing could found [here](bus.md).

### From Code

Audio files are loaded using the resource manager:

```rust,no_run
{{#include ../code/snippets/src/scene/sound.rs:load_sound}}
```

Then, the node is built using the standard builder pattern:

```rust,no_run
{{#include ../code/snippets/src/scene/sound.rs:build_sound_node}}
```

There are a few notable things in the example above.

The first is that sounds don't play automatically; in order to do so, we need to invoke `.with_status(Status::Playing)`.

The second is that sound nodes are not dropped automatically after playback; dropping it can be performed in two ways. 
One way is to use the convenient builder API `.with_play_once(true)`; another is to use the graph APIs:

```rust,no_run
{{#include ../code/snippets/src/scene/sound.rs:sound_removal}}
```

If we want to play background music (or anyway a repeated sound), we just set the `looping` property when building the node:

```rust,no_run
{{#include ../code/snippets/src/scene/sound.rs:looping}}
```

In order to stream large audio files, instead of loading them entirely in memory, the simplest strategy is to create a 
corresponding `.options` file, with the following content:

```json
(
  stream: true
)
```

If the audio file is called, for example, `/path/to/background.ogg`, call this `/path/to/background.ogg.options`.

## 2D and 3D 

There's no strict separation between 2D and 3D sound sources. The same source could be switched from 2D to 3D (and vice
versa) at runtime, by just adjusting `Spatial Blend` property. Spatial blend factor is a numeric value, that defines 
blending factor between 2D and 3D sound, where `0.0` - the sound is fully 2D, `1.0` - the sound is fully 3D. By default, 
the value is `1.0` which makes it 3D. Intermediate values could be used to create "ambisonic" sound sources - when the 
source sounds like it is placed at some position in the world, but some part of it is just 2D and does not depend on 
positioning.

## Audio bus

It is possible to specify target audio bus to which the sound will output its audio samples. Audio bus is responsible
for various audio processing, such as filtering, reverb, etc. To specify output audio bus, just use the `set_audio_bus`
method and set the name of an audio bus.