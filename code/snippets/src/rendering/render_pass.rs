use fyrox::renderer::Renderer;
use fyrox::{
    core::{pool::Handle, sstorage::ImmutableString},
    graphics::{
        buffer::BufferUsage, error::FrameworkError, geometry_buffer::GpuGeometryBuffer,
        server::GraphicsServer,
    },
    renderer::{
        cache::shader::{binding, property, PropertyGroup, RenderMaterial, RenderPassContainer},
        framework::GeometryBufferExt,
        RenderPassStatistics, SceneRenderPass, SceneRenderPassContext,
    },
    scene::{mesh::surface::SurfaceData, Scene},
};
use std::{any::TypeId, cell::RefCell, rc::Rc};

struct MyPlugin;

// ANCHOR: render_pass
const SHADER: &'static str = r##"
(
    name: "Overlay",
    resources: [
        (
            name: "properties",
            kind: PropertyGroup([
                (name: "worldViewProjectionMatrix", kind: Matrix4()),
            ]),
            binding: 0
        ),
    ],
    passes: [
        (
            name: "Primary",

            draw_parameters: DrawParameters(
                cull_face: None,
                color_write: ColorMask(
                    red: true,
                    green: true,
                    blue: true,
                    alpha: true,
                ),
                depth_write: false,
                stencil_test: None,
                depth_test: None,
                blend: Some(BlendParameters(
                    func: BlendFunc(
                        sfactor: SrcAlpha,
                        dfactor: OneMinusSrcAlpha,
                        alpha_sfactor: SrcAlpha,
                        alpha_dfactor: OneMinusSrcAlpha,
                    ),
                    equation: BlendEquation(
                        rgb: Add,
                        alpha: Add
                    )
                )),
                stencil_op: StencilOp(
                    fail: Keep,
                    zfail: Keep,
                    zpass: Keep,
                    write_mask: 0xFFFF_FFFF,
                ),
                scissor_box: None
            ),

            vertex_shader:
                r#"
                    layout(location = 0) in vec3 vertexPosition;

                    void main()
                    {
                        gl_Position = properties.worldViewProjectionMatrix * vec4(vertexPosition, 1.0);
                    }
                "#,

            fragment_shader:
                r#"
                    out vec4 FragColor;

                    void main()
                    {
                        FragColor = vec4(1.0, 0.0, 0.0, 1.0);
                    }
                "#,
        )
    ]
)
"##;

pub struct MyRenderPass {
    quad: GpuGeometryBuffer,
    shader: RenderPassContainer,
    pub scene_handle: Handle<Scene>,
}

impl MyRenderPass {
    pub fn new(server: &dyn GraphicsServer) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            quad: GpuGeometryBuffer::from_surface_data(
                "Quad",
                &SurfaceData::make_unit_xy_quad(),
                BufferUsage::StaticDraw,
                server,
            )
            .unwrap(),
            shader: RenderPassContainer::from_str(server, SHADER).unwrap(),
            scene_handle: Default::default(),
        }))
    }
}

impl SceneRenderPass for MyRenderPass {
    fn on_hdr_render(
        &mut self,
        ctx: SceneRenderPassContext,
    ) -> Result<RenderPassStatistics, FrameworkError> {
        let mut stats = RenderPassStatistics::default();
        if ctx.scene_handle != self.scene_handle {
            return Ok(stats);
        }

        let view_projection = ctx.observer.position.view_projection_matrix;

        let properties =
            PropertyGroup::from([property("worldViewProjectionMatrix", &view_projection)]);
        let material = RenderMaterial::from([binding("properties", &properties)]);

        stats += self.shader.run_pass(
            1,
            &ImmutableString::new("Primary"),
            ctx.framebuffer,
            &self.quad,
            ctx.observer.viewport,
            &material,
            ctx.uniform_buffer_cache,
            Default::default(),
            None,
        )?;

        Ok(stats)
    }

    fn source_type_id(&self) -> TypeId {
        TypeId::of::<MyPlugin>()
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
