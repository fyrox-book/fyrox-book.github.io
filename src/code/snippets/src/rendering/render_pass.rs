use fyrox::{
    core::{algebra::Matrix4, pool::Handle},
    renderer::{
        framework::{
            buffer::BufferUsage,
            error::FrameworkError,
            geometry_buffer::GeometryBuffer,
            gl::{program::GlProgram, server::GlGraphicsServer},
            gpu_program::{GpuProgram, UniformLocation},
            DrawParameters, ElementRange, GeometryBufferExt,
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
    shader: GlProgram,
    target_scene: Handle<Scene>,
    quad: Box<dyn GeometryBuffer>,
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

        let server = renderer
            .server
            .as_any()
            .downcast_ref::<GlGraphicsServer>()
            .unwrap();
        let shader = GlProgram::from_source(server, "MyShader", vs, fs)?;

        Ok(Self {
            enabled: true,
            world_view_proj: shader.uniform_location(&"worldViewProjectionMatrix".into())?,
            target_scene,
            quad: <(dyn GeometryBuffer + 'static) as GeometryBufferExt>::from_surface_data(
                &SurfaceData::make_quad(&Matrix4::identity()),
                BufferUsage::StaticDraw,
                renderer.server.as_ref(),
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
            let resource_binding_groups = &[];
            stats += ctx.framebuffer.draw(
                self.quad.as_ref(),
                ctx.viewport,
                &self.shader,
                &DrawParameters::default(),
                resource_binding_groups,
                ElementRange::Full,
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
