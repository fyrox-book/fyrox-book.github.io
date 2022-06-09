# Assets

Pretty much every game depends on various assets such as 3D models, textures, sounds, etc. The engine has its own 
assets pipeline that is made to make your life easier. 

## Asset Types

The engine offers a fixed set of assets that should cover all your needs:

- [Models](../resources/model.md) - model is a set of objects, it can be a simple 3D model (like barrel, bush, weapon, 
etc.) or complex scene with lots of object and possibly other model instances. There are two main formats supported:
FBX - could be used to import 3D models, RGS - a scene that is made in Fyroxed. RGS models are special, they can be 
used as _hierarchical prefabs_.
- [Textures](../resources/texture.md) - texture is an image that is used to add graphical details to objects. The
engine supports various texture formats such as PNG, JPG, BMP, etc. There is also support for compressed textures in
DDS format.
- [Sound buffers](../resources/sound.md) - a data buffer for sound sources. The engine supports WAV and OGG formats. 
- [Curves](../resources/curve.md) - parametric curve. It is used to create complex functions for numeric parameters.
Curves can be made in `Curve Editor` (`Utils -> Curve Editor`)
- [Animation Machines](../resources/absm.md) - animation blending state machines (ABSM) - allows you to blend multiple 
animations into one to create complex animations. ABSM can be made in `Animation Editor` (`Utils -> Animation Editor`)

## Asset Management

Asset management is performed from `Asset Browser` window in the editor, you can select an asset, preview it and edit
its import options. Here's the asset browser with a texture selected:

![asset browser](assets.png)

The most interesting part here is import options section under previewer, it allows to set asset-specific import options
and apply it. Every asset has its own set of import options. Check respective asset page from above section to learn
what import options is for what.

## Asset Instantiation

Some asset types can be instantiated in scene, for now you can create direct instance only from models. This
is done by simple drag'n'drop - find a model you want to instantiate and drag it `Scene Preview`. The instance should
appear in the `Scene Preview` once you release left mouse button.

## Loading Assets

Usually there is no need to manually handle assets loading, since you have the editor that can help you with that - create
a scene with all required assets. However, sometimes you need to instantiate an asset dynamically - for example a
bot prefab. In this case you can use `ResourceManager` and respective set of methods (like `request_model`, 
`request_texture`, etc.). See respective asset page for more info.