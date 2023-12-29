# Asset Management

This chapter covers asset management in the engine. Asset management is performed by `Asset Browser` in the editor 
and by `ResourceManager` from API.

## General Info

Assets loading is asynchronous, it is possible to load multiple assets in parallel or load until a specific asset is 
loaded.

## Best Practices

It is strongly advised to specify all resources used by your game entities inside your scripts, instead of requesting 
resources directly from the resource manager on demand. This approach solves two common issues:

1) It allows you to set resources directly from the editor by a simple drag'n'drop from the Asset Browser.
2) The engine will be able to wait until all resources used by a scene are fully loaded. This is especially important,
because this way can guarantee, that scene loading will be "seamless" and if the scene was loaded, it means that all
resources used by it are loaded too. 

This can be achieved by adding a respective field in your script. For example, you may a have a weapon script that 
shoots some projectiles. In this case all you need to add a `projectile: Option<ModelResource>` field in your script,
assign it to some prefab in the editor and then [instantiate](model.md#instantiation) it from code when shooting. Storing
resource handle directly in your script helps the engine to gather all resources used by parent scene and preload them 
too while loading the scene itself. Such approach prevent lags when doing actual shooting, which is especially important 
if you're targeting a WebAssembly platform. On WebAssembly all the files accessed over network API which could work with 
unstable connection. In any case, even on PC it helps a lot.

Requesting resources on demand could be useful in limited situations:

1) You're loading a new game level - in this case it is perfectly fine to request the resource manually.
2) You're doing some background work (level streaming for instance).

## Asset Browser

Asset browser allows you to preview your assets and edit their import properties. It looks something like this (keep
in mind that the screenshot could be outdated).

![Asset Browser](asset_browser.png)

There are three main areas in it:

1) Left directory tree - shows all _directories_ starting from project root. It does _not_ show any files, this is 
for what the center section is.
2) Center asset previewer - shows all assets from selected directory. The path at the top of the section shows asset
path.
3) Right asset import options inspector - it shows import properties of selected asset.

Typical workflow could look like this:

1) Select desired directory from the left tree
2) Select desired asset in the center previewer
3) Edit import properties of selected asset and click "Apply" button to save import options and re-load the asset with
new options.

Alternatively, you can just type in the name of some resource you're looking for in the search bar at the top of the 
Asset Browser.

Check next chapters to learn how to manage specific asset types and what their import does what.

## API Docs

Please read API docs [here](https://docs.rs/fyrox/latest/fyrox/engine/resource_manager/index.html)