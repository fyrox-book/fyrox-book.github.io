# Textures

Texture is an image that used to fill faces to add details to them.

In most cases textures are just 2D images, however there are some exclusions to that - for example cube maps,
that may be used for environment mapping. Fyrox supports 1D, 2D, 3D and Cube textures.

## Supported formats

To load images and decode them, Fyrox uses image and ddsfile crates. Here is the list of supported formats: png,
tga, bmp, dds, jpg, gif, tiff, dds.

### Compressed textures

Fyrox supports most commonly used formats of compressed textures: DXT1, DXT3, DXT5. Such textures can be loaded
only from `DDS` files. 

## Import options

It is possible to define custom import options. Using import options you could set desired compression quality,
filtering, wrapping, etc. Import options should be defined in a separate file with the same name as the source
texture, but with additional extension `options`. For example, you have a `foo.jpg` texture, a file with import
options should be called `foo.jpg.options`. It's content may look something like this:

```text
(
    minification_filter: Linear,
    magnification_filter: Linear,
    s_wrap_mode: Repeat,
    t_wrap_mode: ClampToEdge,
    anisotropy: 8.0,
    compression: NoCompression,    
)
```

Even if it is possible to modify it by hand, it strongly advised to use the editor to edit import options, because
it reduces chances to mess up something.

## Render target

Texture can be used as render target to render scene in it. To do this you should use new_render_target method and
pass its result to scene's render target property. Renderer will automatically provide you info about metrics of
texture, but it won't give you access to pixels of render target.