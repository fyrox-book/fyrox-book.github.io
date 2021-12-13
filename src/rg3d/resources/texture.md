# Textures (WIP)

 Texture is an image that used to fill faces to add details to them.

 In most cases textures are just 2D images, however there are some exclusions to that -
 for example cube maps, that may be used for environment mapping. rg3d supports 1D, 2D,
 3D and Cube textures.

 ## Supported formats

 To load images and decode them, rg3d uses image and ddsfile crates. Here is the list of
 supported formats: png, tga, bmp, dds, jpg, gif, tiff, dds.

 ## Compressed textures

 rg3d supports most commonly used formats of compressed textures: DXT1, DXT3, DXT5.

 ## Render target

 Texture can be used as render target to render scene in it. To do this you should use
 new_render_target method and pass its result to scene's render target property. Renderer
 will automatically provide you info about metrics of texture, but it won't give you
 access to pixels of render target.