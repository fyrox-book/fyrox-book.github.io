# Custom Resources

In Fyrox, you can create your own, custom resource type that can be embedded in the standard resource management pipeline.
It could be useful to access specific data using engine's resource manager. Custom resources has a few major advantages 
over manual resource management via direct files access:

1) Since Fyrox resource system is asynchronous, your resource can be loaded in separate worker thread which speeds up
loading (since it may run on a separate CPU core).
2) You can access your resources from the Asset Browser and assign their handles to scripts directly from the editor.
3) File access for resource management has an abstraction, that unifies the access over all supported platforms. This 
means that you don't need to use `fetch` API directly, if you're targeting WebAssembly platform, or use `AssetManager`
on Android.

To create a custom resource, you need to do three major steps:

1) Define your resource structure with all required traits implemented.
2) Add a custom resource loader, that will be used by the resource manager to load your custom resource.
3) Register the resource loader in the resource manager.

See the code snippet in the next section as a guide. 

## Example

Custom resource is just an ordinary struct with some data. It must implement `Debug`, `Reflect`, `Visit`, `ResourceData`
traits. Also, it must contain at least path to external file with the content. Here's the simplest custom resource, that
contains some string data.

```rust,no_run,edition2018
{{#include ../code/snippets/src/resource/custom.rs:custom_resource}}
```

Keep in mind, that you must provide **unique** UUID for every resource type that you're creating. Otherwise, using
existing id multiple times will cause incorrect serialization and type casting. The next step is to register the new 
resource in the resource manager. This can be done by adding the following code to the `register` method for
`impl PluginConstructor for GameConstructor`:

```rust,no_run
{{#include ../code/snippets/src/resource/custom.rs:custom_resource_registration}}
```

After doing so, any attempt to load a resource with `my_resource` extension will call the `load` method of your 
resource loader.

## Editor Support

There's one more step before your custom resource is fully usable - you need to register a property editor for it, so
any fields in your scripts that has `my_resource: Option<Resource<CustomResource>>` fields can be editable in the editor. 
Otherwise, you'll see an error message in the Inspector instead of resource selector field. To register a property editor,
add the following lines to `editor/src/main.rs` file, somewhere after the editor instance is created:

```rust,no_run
{{#include ../code/snippets/src/resource/custom.rs:editor_support}}
```

After this, the editor will create this property editor for `my_resource` field and will allow you to set its value by
drag'n'dropping an asset from the Asset Browser.