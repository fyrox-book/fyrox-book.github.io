# Model resources

## Supported formats

rg3d supports these file formats for 3D models:

- FBX
- RGS - native scenes format produced by rusty-editor

## Tips for Blender

Blender's FBX exporter has exporting scale properties usually set to 100%, this may lead to incorrect scale
of your model in the engine. It will have `(100.0, 100.0, 100.0)` scale which is very huge. To fix that, set
the scale in the exporter to `0.01`.