# Sound Buffer

Sound sources uses dedicated resource type to store the actual waveform they play. Sound buffer could be 
loaded from a few supported formats: WAV, OGG.

## How to Load

Sound buffers could be loaded using standard resource manager methods:

```rust
{{#include ../code/snippets/src/resource/sound.rs:load_sound_buffer}}
```

## Streaming

In order to stream large audio files, instead of loading them entirely in memory, the simplest strategy is to create a
corresponding `.options` file near the source file, with the following content:

```json
(
  stream: true
)
```

Keep in mind, that sound buffers that uses streaming **cannot** be shared across multiple sound sources.
Streaming should only be used in unique, large sound source, such as game music.