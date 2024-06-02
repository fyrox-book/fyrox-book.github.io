use fyrox::material::{Material, MaterialResource, MaterialResourceExtension};
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
    MaterialResource::new(Material::from_shader(
        shader,
        Some(resource_manager.clone()),
    ))
}
// ANCHOR_END: create_material
