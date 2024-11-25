# Materials

Material defines a set of values for a shader. Materials usually contains textures (diffuse, normal, height, emission and other maps), numerical values (floats, integers), vectors, booleans, matrices and arrays of each type, except
textures. Each parameter can be changed in runtime giving you the ability to create animated materials. However,
in practice, most materials are static, this means that once it's created, it won't be changed anymore.

Please keep in mind that the actual "rules" of drawing an entity are stored in the shader,
**material is only a storage** for specific uses of the shader.

Multiple materials can share the same shader, for example standard shader covers 95% of most common use cases,
and it is shared across multiple materials. The only difference are property values, for example you can draw
multiple cubes using the same shader, but with different textures.

Material itself can be shared across multiple places as well as the shader. This gives you the ability to render
multiple objects with the same material efficiently.

## Performance

It is very important re-use materials as much as possible, because the number of materials used per frame
significantly correlates with performance. The more unique materials you have per frame, the more work
the renderer and video driver need in order to render a frame and more time the frame will require for
rendering, thus lowering your FPS.

## Standard material

The engine offers a standard PBR material, PBR stands for "Physically-Based Rendering" which gives you the quality
of shading which is very close to materials in real world (to some extent of course).

The standard material can cover 95% of use cases, and it is suitable for almost any kind of game, except maybe
some cartoon-ish or stylized games.

The standard material has quite a lot of properties that can be used to fully utilize the power of PBR rendering:

- **diffuseColor** - an RGBA color that will be used as a base color for you object. **Caveat:** the opacity value
(alpha) will be used only with `Forward` render path! This means that you will need to switch render path on your
mesh ([see below](#transparency))
- **diffuseTexture** - a 2D texture containing the unlit "basic" colors of your object, this is the most commonly
used texture. For example, you can assign a brick wall texture to this property and your object will look like a brick
wall.
- **normalTexture** - a 2D texture containing per-pixel normal vectors.
- **metallicTexture** - a 2D texture containing per-pixel metallic factor, where 0 - dielectric, 1 - metal.
In simple words it defines whether your object reflects (1.0) the environment or not (0.0).
- **roughnessTexture** - a 2D texture containing per-pixel roughness factor, where 0 - completely flat, 1 -
very rough.
- **heightTexture** - a 2D texture containing per-pixel displacement value, it is used with parallax mapping to
crate an effect of volume on a flat surface.
- **emissionTexture** - a 2D texture containing per-pixel emission lighting. You could use this to create emissive
surfaces like small lamps on wall of sci-fi ship, or to create glowing eyes for your monsters that will scare
the player.
- **lightmapTexture** - a 2D texture containing per-pixel **static** lighting. It is used to apply precomputed
light to your 3D models, and the most common use case is to lit a static object using a static light. Precomputed
light is very cheap. The engine offers built-in lightmapper that can generate lightmaps for you.
- **aoTexture** - a 2D texture containing per-pixel shading values, allows you to "bake" shadows in for your 3D
object.
- **texCoordScale** - a 2D vector that allows you to scale texture coordinates used to sample the textures
mentioned above (except lightmaps, they're using separate texture coordinates)
- **layerIndex** - a natural number that is used for decals masking, a decal will only be applied to your mesh
if and only if the decal has matching index.
- **emissionStrength** - a 3D vector that allows you to set the strength of emission per-channel (R, G, B) for
your `emissionTexture`

## Transparency

The standard material offers very basic transparency support, to use it you have to explicitly switch render
path on your mesh object. It could be done in this way:

```rust,no_run
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle,
#     scene::{mesh::RenderPath, node::Node, Scene},
# };
#
# fn set_forward_render_path(scene: &mut Scene, mesh_handle: Handle<Node>) {
    scene.graph[mesh_handle]
        .as_mesh_mut()
        .set_render_path(RenderPath::Forward);
# }
```

After this, your mesh will be rendered using a specialized render pass called Forward which supports alpha-blending
and transparent objects. **Caveat:** Current forward renderer implementation does not support any kind of lighting,
if you need lighting, you will need to use custom shader for that!

## Material import

When you're loading a 3D model in the engine, the engine tries to convert the materials stored inside to standard
material. In most cases there is no way to create 100% matching material on the fly, instead the engine tries
to do its best to make sure the material will be imported as closely as possible to the original one. Various 3D modelling
tools use different material system, but all of them allow you to export your 3D model in one of the commonly
used formats (such as FBX).

### Blender

When using Blender, make sure you are using **Principled BSDF** material, it is the closest material that can be converted
to engine's standard material at almost 100% fidelity.

### 3Ds max

It highly depends on the version of the 3Ds max, but in general the default material should work fine.
