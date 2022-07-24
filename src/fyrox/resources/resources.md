# Asset Management

This chapter covers asset management in the engine. Asset management is performed by `Asset Browser` in the editor 
and by `ResourceManager` from API.

## General Info

Assets loading is asynchronous, it is possible to load multiple assets in parallel or load until a specific asset is 
loaded.

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

Check next chapters to learn how to manage specific asset types and what their import does what.

## API Docs

Please read API docs [here](https://docs.rs/fyrox/latest/fyrox/engine/resource_manager/index.html)