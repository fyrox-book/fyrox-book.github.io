use fyrox::{
    core::{algebra::Matrix4, pool::Handle, sstorage::ImmutableString},
    renderer::{
        framework::{
            error::FrameworkError,
            framebuffer::DrawParameters,
            geometry_buffer::{ElementRange, GeometryBuffer, GeometryBufferKind},
            gpu_program::{GpuProgram, UniformLocation},
        },
        RenderPassStatistics, Renderer, SceneRenderPass, SceneRenderPassContext,
    },
    scene::{mesh::surface::SurfaceData, Scene},
};
use std::any::{Any, TypeId};
use std::{cell::RefCell, rc::Rc};

// ANCHOR: render_pass
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
        let vs = r"
                layout(location = 0) in vec3 vertexPosition;
                
                uniform mat4 c;
                         
                void main()
                {
                    gl_Position = worldViewProjectionMatrix * vertexPosition;
                }
            ";

        let fs = r"                
                out vec4 FragColor;             
                
                void main()
                {
                    FragColor = vec4(1.0, 0.0, 0.0, 1.0);
                }
            ";

        let shader = GpuProgram::from_source(&renderer.state, "MyShader", vs, fs)?;

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
                & renderer.state,
            )?,
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
                ElementRange::Full,
                |mut program| {
                    program.set_matrix4(&self.world_view_proj, &Matrix4::identity());
                },
            )?;
        }

        Ok(stats)
    }

    fn source_type_id(&self) -> TypeId {
        ().type_id()
    }
}
// ANCHOR_END: render_pass

// ANCHOR: usage_example
fn usage_example(renderer: &mut Renderer, render_pass: MyRenderPass) {
    let shared_pass = Rc::new(RefCell::new(render_pass));
    // You can share the pass across multiple places to be able to control it.
    renderer.add_render_pass(shared_pass);
}
// ANCHOR_END: usage_example
