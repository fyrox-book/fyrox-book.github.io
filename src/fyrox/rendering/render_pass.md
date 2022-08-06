# Render Pass

You can define your own render passes that extends the renderer, currently there are render passes only for scenes,
so no custom post-effects (this is planned to be improved in Fyrox 0.28). Render pass has full access to graphics 
framework (which is a thin wrapper around OpenGL) so it can utilize full power of it to implement various graphical
effects.

## Creating a render pass

Render pass is a complex thing, that requires relatively deep knowledge in computer graphics. It is intended to be used
by experienced graphics programmers. Here's the simplest render pass that renders unit quad without any textures.

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::{algebra::Matrix4, pool::Handle, sstorage::ImmutableString},
#     renderer::{
#         framework::{
#             error::FrameworkError,
#             framebuffer::DrawParameters,
#             geometry_buffer::{GeometryBuffer, GeometryBufferKind},
#             gpu_program::{GpuProgram, UniformLocation},
#         },
#         RenderPassStatistics, Renderer, SceneRenderPass, SceneRenderPassContext,
#     },
#     scene::{mesh::surface::SurfaceData, Scene},
# };
# use std::{cell::RefCell, rc::Rc};
# 
struct MyRenderPass {
    enabled: bool,
    shader: GpuProgram,
    target_scene: Handle<Scene>,
    quad: GeometryBuffer,
    world_view_proj: UniformLocation,
}

impl MyRenderPass {
    pub fn new(
        renderer: &mut Renderer,
        target_scene: Handle<Scene>,
    ) -> Result<Self, FrameworkError> {
        let vs = r#"
                layout(location = 0) in vec3 vertexPosition;
                
                uniform mat4 c;
                         
                void main()
                {
                    gl_Position = worldViewProjectionMatrix * vertexPosition;
                }
            "#;

        let fs = r#"                
                out vec4 FragColor;             
                
                void main()
                {
                    FragColor = vec4(1.0, 0.0, 0.0, 1.0);
                }
            "#;

        let shader = GpuProgram::from_source(&mut renderer.state, "MyShader", vs, fs)?;

        Ok(Self {
            enabled: true,
            world_view_proj: shader.uniform_location(
                &renderer.state,
                &ImmutableString::new("worldViewProjectionMatrix"),
            )?,
            target_scene,
            quad: GeometryBuffer::from_surface_data(
                &SurfaceData::make_quad(&Matrix4::identity()),
                GeometryBufferKind::StaticDraw,
                &mut renderer.state,
            ),
            shader,
        })
    }
}

impl SceneRenderPass for MyRenderPass {
    fn on_ldr_render(
        &mut self,
        ctx: SceneRenderPassContext,
    ) -> Result<RenderPassStatistics, FrameworkError> {
        let mut stats = RenderPassStatistics::default();

        // Make sure to render only to target scene.
        if self.enabled && ctx.scene_handle == self.target_scene {
            stats += ctx.framebuffer.draw(
                &self.quad,
                ctx.pipeline_state,
                ctx.viewport,
                &self.shader,
                &DrawParameters::default(),
                |mut program| {
                    program.set_matrix4(&self.world_view_proj, &Matrix4::identity());
                },
            );
        }

        Ok(stats)
    }
}
```

The code snippet shows how to create a shader, find its uniforms, and finally how to actually render something in 
target frame buffer.

## Registering a render pass

Every render pass must be registered in the renderer, otherwise it won't be used. You can register a render pass using
`add_render_pass` method of the `Renderer`:

```rust,no_run
# extern crate fyrox;
# use fyrox::renderer::{Renderer, SceneRenderPass};
# use std::{cell::RefCell, rc::Rc};
# 
# struct MyRenderPass;
# impl SceneRenderPass for MyRenderPass {}
# 
fn usage_example(renderer: &mut Renderer, render_pass: MyRenderPass) {
    let shared_pass = Rc::new(RefCell::new(render_pass));
    // You can share the pass across multiple places to be able to control it.
    renderer.add_render_pass(shared_pass);
}
```

Please notice that we've wrapped render pass in `Rc<RefCell<..>>`, this means that you can share it across multiple places
and modify its data from the code of your game.
