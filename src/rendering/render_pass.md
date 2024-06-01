# Render Pass

You can define your own render passes that extends the renderer, currently there are render passes only for scenes,
so no custom post-effects (this is planned to be improved in Fyrox 0.28). Render pass has full access to graphics 
framework (which is a thin wrapper around OpenGL) so it can utilize full power of it to implement various graphical
effects.

## Creating a render pass

Render pass is a complex thing, that requires relatively deep knowledge in computer graphics. It is intended to be used
by experienced graphics programmers. Here's the simplest render pass that renders unit quad without any textures.

```rust,no_run
{{#include ../code/snippets/src/rendering/render_pass.rs:render_pass}}
```

The code snippet shows how to create a shader, find its uniforms, and finally how to actually render something in 
target frame buffer.

## Registering a render pass

Every render pass must be registered in the renderer, otherwise it won't be used. You can register a render pass using
`add_render_pass` method of the `Renderer`:

```rust,no_run
{{#include ../code/snippets/src/rendering/render_pass.rs:usage_example}}
```

Please notice that we've wrapped render pass in `Rc<RefCell<..>>`, this means that you can share it across multiple places
and modify its data from the code of your game.
