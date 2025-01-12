use fyrox::{
    core::{algebra::Matrix4, pool::Handle},
    renderer::{
        framework::{
            buffer::{Buffer, BufferKind, BufferUsage},
            error::FrameworkError,
            framebuffer::{BufferDataUsage, BufferLocation, ResourceBindGroup, ResourceBinding},
            geometry_buffer::GeometryBuffer,
            gpu_program::{GpuProgram, UniformLocation},
            uniform::StaticUniformBuffer,
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
    shader: Box<dyn GpuProgram>,
    target_scene: Handle<Scene>,
    quad: Box<dyn GeometryBuffer>,
    world_view_proj_location: UniformLocation,
    world_view_proj: Box<dyn Buffer>,
}

impl MyRenderPass {
    pub fn new(
        renderer: &mut Renderer,
        target_scene: Handle<Scene>,
    ) -> Result<Self, FrameworkError> {
        let vs = r"
                layout(location = 0) in vec3 vertexPosition;
                
                uniform mat4 worldViewProjectionMatrix;
                         
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

        let shader = renderer.server.create_program("MyShader", vs, fs)?;
        let world_view_proj =
            renderer
                .server
                .create_buffer(256, BufferKind::Uniform, BufferUsage::DynamicDraw)?;

        world_view_proj.write_data_of_type(
            &StaticUniformBuffer::<256>::new()
                .with(&Matrix4::identity())
                .finish(),
        )?;

        Ok(Self {
            enabled: true,
            world_view_proj_location: shader
                .uniform_location(&"worldViewProjectionMatrix".into())?,
            world_view_proj,
            target_scene,
            quad: <dyn GeometryBuffer as GeometryBufferExt>::from_surface_data(
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
            let resources = ResourceBindGroup {
                bindings: &[ResourceBinding::Buffer {
                    buffer: self.world_view_proj.as_ref(),
                    binding: BufferLocation::Explicit {
                        binding: self.world_view_proj_location.id.0 as usize,
                    },
                    data_usage: BufferDataUsage::default(),
                }],
            };
            stats += ctx.framebuffer.draw(
                self.quad.as_ref(),
                ctx.viewport,
                self.shader.as_ref(),
                &DrawParameters::default(),
                &[resources],
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
