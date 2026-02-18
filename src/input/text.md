# Raw Text Input

Sometimes there's a need to catch all text input events. This can be done by listening to `WindowEvent::KeyboardInput`
like so:

```rust,no_run
{{#include ../code/snippets/src/input/text.rs:raw_text_input}}
```