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
# extern crate fyrox;
# use fyrox::{
#     asset::{
#         event::ResourceEventBroadcaster,
#         loader::{BoxedLoaderFuture, ResourceLoader},
#         untyped::UntypedResource,
#         ResourceData,
#     },
#     core::{
#         io::{self},
#         reflect::prelude::*,
#         uuid::{uuid, Uuid},
#         visitor::prelude::*,
#         TypeUuidProvider,
#     },
# };
# use std::{
#     any::Any,
#     borrow::Cow,
#     path::{Path, PathBuf},
# };
# 
#[derive(Debug, Visit, Reflect)]
struct CustomResource {
    // You resource must store the path.
    path: PathBuf,
    some_data: String,
}

impl TypeUuidProvider for CustomResource {
    // Every resource must provide a unique identifier, that is used for dynamic type
    // casting, serialization, etc.
    fn type_uuid() -> Uuid {
        uuid!("15551157-651b-4f1d-a5fb-6874fbfe8637")
    }
}

impl ResourceData for CustomResource {
    fn path(&self) -> Cow<Path> {
        Cow::Borrowed(&self.path)
    }

    fn set_path(&mut self, path: PathBuf) {
        self.path = path;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }
}

struct CustomResourceLoader;

impl ResourceLoader for CustomResourceLoader {
    fn extensions(&self) -> &[&str] {
        // An array of extensitions, supported by this loader. There could be any number of extensions
        // since sometimes multiple extensions map to a single resource (for instance, jpg, png, bmp, are
        // all images).
        &["my_resource"]
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn load(
        &self,
        resource: UntypedResource,
        event_broadcaster: ResourceEventBroadcaster,
        reload: bool,
    ) -> BoxedLoaderFuture {
        Box::pin(async move {
            let path = resource.path();
            match io::load_file(&path).await {
                Ok(content) => {
                    let my_resource = CustomResource {
                        path,
                        some_data: String::from_utf8(content).unwrap(),
                    };

                    resource.commit_ok(my_resource);

                    // Notify potential subscribers that the resource was loader.
                    event_broadcaster.broadcast_loaded_or_reloaded(resource, reload);
                }
                Err(err) => {
                    resource.commit_error(path, err);
                }
            }
        })
    }
}
```

Keep in mind, that you must provide **unique** UUID for every resource type that you're creating. Otherwise, using
existing id multiple times will cause incorrect serialization and type casting. The next step is to register the new 
resource in the resource manager. This can be done by: `resource_manager.state().loaders.set::<CustomResourceLoader>()`.
After doing so, any attempt to load a resource with `my_resource` extension will call the `load` method of your 
resource loader. See [custom_loader](https://github.com/FyroxEngine/Fyrox/blob/master/examples/custom_loader.rs) for 
runnable example.