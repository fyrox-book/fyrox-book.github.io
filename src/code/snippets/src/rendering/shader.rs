use fyrox::engine::{GraphicsContext, InitializedGraphicsContext};
use fyrox::graphics_gl::server::GlGraphicsServer;
use fyrox::material::{Material, MaterialResource, MaterialResourceExtension};
use fyrox::plugin::PluginContext;
use fyrox::{
    asset::manager::ResourceManager,
    material::shader::{Shader, ShaderResource},
};

// ANCHOR: load_shader
fn load_shader(resource_manager: &ResourceManager) -> ShaderResource {
    resource_manager.request::<Shader>("path/to/my/cool.shader")
}
// ANCHOR_END: load_shader

// ANCHOR: create_material
fn create_material(resource_manager: &ResourceManager) -> MaterialResource {
    let shader = resource_manager.request::<Shader>("path/to/my/cool.shader");
    MaterialResource::new(Material::from_shader(shader))
}
// ANCHOR_END: create_material

// ANCHOR: use_gl_compute_shader
fn use_gl_compute_shader(ctx: &PluginContext) {
    let GraphicsContext::Initialized(initialized_context) = ctx.graphics_context else {
        return;
    };
    let Some(gl_server) = initialized_context
        .renderer
        .server
        .as_any()
        .downcast_ref::<GlGraphicsServer>()
    else {
        return;
    };

    use glow::HasContext;
    unsafe {
        // Create a compute shader program and use it.
        // ...
        gl_server.gl.dispatch_compute(3, 3, 3);
    }
}
// ANCHOR_END: use_gl_compute_shader
