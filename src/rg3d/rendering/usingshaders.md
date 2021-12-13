# Using shaders

## Using the standard shader

## Using an external GLSL file

First we will create a `.glsl` file in the `data/shader` directory of our project.
We will use the example in the previous chapter for this file. *See below*

```glsl
void main(
    // A set of properties, there could be any amount of properties.
    properties: [
        (
            // Each property must have a name. This name must match with respective
            // uniforms! That's is the whole point of having properties.
            name: "diffuseTexture",
            // Value has limited set of possible variants.
            value: Sampler(default: None, fallback: White)
        )
    ],
    // A set of render passes (see next section for more info)
    passes: [
        (
            // Name must match with the name of either standard render pass (see below) or
            // one of your passes.
            name: "Forward",
            // A set of parameters that regulate renderer pipeline state.
            // This is mandatory field of each render pass.
            draw_parameters: DrawParameters(
                // A face to cull. Either Front or Back.
                cull_face: Some(Back),
                // Color mask. Defines which colors should be written to render target.
                color_write: ColorMask(
                    red: true,
                    green: true,
                    blue: true,
                    alpha: true,
                ),
                // Whether to modify depth buffer or not.
                depth_write: true,
                // Whether to use stencil test or not.
                stencil_test: None,
                // Whether to perform depth test when drawing.
                depth_test: true,
                // Blending options.
                blend: Some(BlendFunc(
                    sfactor: SrcAlpha,
                    dfactor: OneMinusSrcAlpha,
                )),
                // Stencil options.
                stencil_op: StencilOp(
                    fail: Keep,
                    zfail: Keep,
                    zpass: Keep,
                    write_mask: 0xFFFF_FFFF,
                ),
            ),
            // Vertex shader code.
            vertex_shader:
                r#"
                layout(location = 0) in vec3 vertexPosition;
                layout(location = 1) in vec2 vertexTexCoord;
                uniform mat4 rg3d_worldViewProjection;
                out vec2 texCoord;
                void main()
                {
                    texCoord = vertexTexCoord;
                    gl_Position = rg3d_worldViewProjection * vertexPosition;
                }
                "#;
            // Pixel shader code.
            pixel_shader:
                r#"
                // Note that the name of this uniform match the name of the property up above.
                uniform sampler2D diffuseTexture;
                out vec4 FragColor;
                in vec2 texCoord;
                void main()
                {
                    FragColor = diffuseColor * texture(diffuseTexture, texCoord);
                }
                "#;
        )
    ],
)
```

To use this file in your scene, call the `Shader::from_str` function. *Example below*

```rust
# extern crate rg3d;
use rg3d::{
    material::shader::Shader,
};
fn use_shader(resource_manager: ResourceManager) {
    let shader = Shader::from_str("data/shader/shader.glsl").await.unwrap();
    shader
}
```

### Using a shader in a material

Using shaders in a material is arguable a lot easier than using it in a scene.
To do so, here is a template for you to use.

```rust
# use rg3d::{
#     engine::resource_manager::ResourceManager,
#     material::{Material, PropertyValue},
#     core::algebra::Vector3
# };

async fn create_grass_material(resource_manager: ResourceManager) -> Material {
    let shader = resource_manager.request_shader("my_grass_shader.glsl").await.unwrap();
     
    // Here we assume that the material really has the properties defined below.
    let mut material = Material::from_shader(shader, Some(resource_manager));

    material.set_property(
        "windDirection",
        PropertyValue::Vector3(Vector3::new(1.0, 0.0, 0.5))
        )
        .unwrap();

    material
}
```
