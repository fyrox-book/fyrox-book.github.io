# Property Editors

The editor uses [Inspector](../ui/inspector.md) widget to show the contents of your scripts and when you're using 
custom structures inside your scripts the editor needs to know how to show them in the UI. Inspector widget has a 
special mechanism for this called _property editors_. Basically, it defines a pair `TypeId -> Widget` - a type has
an associated widget that is responsible for showing the content of the type and (optionally) edit it. If there's no
widget associated with a type, the editor will print an error message near this field, basically telling you that 
you need to fix this.

## Adding Property Editors

The engine has property editors for pretty much every case, all you need to do is to associate your type with one of
them. The following sections covers the most common use cases, each of them should be added to `editor/src/main.rs` 
file, after editor's initialization.

### Structures

This is the most common case when you need to associate your type with a property editor, and in this case the property
editor will be `InspectablePropertyEditorDefinition`:

```rust,no_run
# extern crate fyrox;
use fyrox::{
    core::reflect::prelude::*,
    gui::inspector::editors::{
        inspectable::InspectablePropertyEditorDefinition, PropertyEditorDefinitionContainer,
    },
};
# 
# struct Editor {
#     inspector: Inspector,
# }
# 
# struct Inspector {
#     property_editors: PropertyEditorDefinitionContainer,
# }

#[derive(Reflect, Debug)]
struct MyStruct {
    foo: u32,
    bar: String,
}

# fn add_property_editor(editor: &Editor) {
editor
    .inspector
    .property_editors
    .insert(InspectablePropertyEditorDefinition::<MyStruct>::new());
# }
```

Keep in mind, that your structure must implement `Reflect` trait, otherwise you'll get a compilation error.

### Enumerations

Enumerations are a bit trickier to support, than simple structures, because they require a bit more traits to be 
implemented for your enumeration. At first, make sure that your `editor` project has the following dependencies:

```toml
#[dependencies]
strum = "0.25.0"
strum_macros = "0.25.0"
```

These two crates responsible for enum to string (and vice versa) conversions which will be very useful for us. The 
following example shows a typical usage:

```rust,no_run
use fyrox::{
    core::reflect::prelude::*,
    gui::inspector::editors::{
        enumeration::EnumPropertyEditorDefinition, PropertyEditorDefinitionContainer,
    },
};
use strum_macros::{AsRefStr, EnumString, EnumVariantNames};
# 
# struct Editor {
#     inspector: Inspector,
# }
# 
# struct Inspector {
#     property_editors: PropertyEditorDefinitionContainer,
# }

#[derive(Reflect, Debug, AsRefStr, EnumString, EnumVariantNames, Clone)]
enum MyEnum {
    Foo(u32),
    Bar { baz: String, foobar: u32 },
}

# fn add_property_editor(editor: &Editor) {
editor
    .inspector
    .property_editors
    .insert(EnumPropertyEditorDefinition::<MyEnum>::new());
# }
```

As you can see, your enumeration needs a decent amount of trait implementations, hopefully all of them can be derived.

### Inheritable Properties

If your structure or enumeration needs to be inheritable (see more info about [property inheritance](../scene/inheritance.md)),
then you need one more step. In case of inheritable variables, your fields will be wrapped in `InheritableVariable<>` and
this fact requires you to register an appropriate property editor for this:

```rust,no_run
use fyrox::{
    core::{reflect::prelude::*, variable::InheritableVariable},
    gui::inspector::editors::{
        inherit::InheritablePropertyEditorDefinition,
        inspectable::InspectablePropertyEditorDefinition, PropertyEditorDefinitionContainer,
    },
};
# 
# struct Editor {
#     inspector: Inspector,
# }
# 
# struct Inspector {
#     property_editors: PropertyEditorDefinitionContainer,
# }

#[derive(Reflect, Debug)]
struct MyStruct {
    foo: u32,
    bar: String,
}

// An example script with inheritable field of custom structure.
struct MyScript {
    inheritable: InheritableVariable<MyStruct>,
}

# fn add_property_editor(editor: &Editor) {
editor
    .inspector
    .property_editors
    .insert(InspectablePropertyEditorDefinition::<MyStruct>::new());

// This is responsible for supporting inheritable properties in scripts.
editor
    .inspector
    .property_editors
    .insert(InheritablePropertyEditorDefinition::<MyStruct>::new());

// Alternatively, the two previous insertions could be replaced by a single call of helper
// method:
editor
    .inspector
    .property_editors
    .register_inheritable_inspectable::<MyStruct>();
# }
```

### Collections

If you have a vector of some custom structure (`Vec<MyStruct>`), then you also need to register a property editor for
it:

```rust
# extern crate fyrox;
use fyrox::{
    core::reflect::prelude::*,
    gui::inspector::editors::{
        collection::VecCollectionPropertyEditorDefinition,
        inspectable::InspectablePropertyEditorDefinition, PropertyEditorDefinitionContainer,
    },
};
# 
# struct Editor {
#     inspector: Inspector,
# }
# 
# struct Inspector {
#     property_editors: PropertyEditorDefinitionContainer,
# }

#[derive(Reflect, Debug)]
struct MyStruct {
    foo: u32,
    bar: String,
}

// An example script with Vec field of custom structure.
struct MyScript {
    inheritable: Vec<MyStruct>,
}

# fn add_property_editor(editor: &Editor) {
editor
    .inspector
    .property_editors
    .insert(InspectablePropertyEditorDefinition::<MyStruct>::new());

// VecCollectionPropertyEditorDefinition is used to create a property editor for Vec<MyStruct>,
// internally it uses a registered property editor for its generic argument (MyStruct).
editor
    .inspector
    .property_editors
    .insert(VecCollectionPropertyEditorDefinition::<MyStruct>::new());

// Alternatively, you can use a special helper method to replace the two blocks above by a
// single one.
editor
    .inspector
    .property_editors
    .register_inheritable_vec_collection::<MyStruct>();
# }
```

## Custom Property Editors

See [Inspector](../ui/inspector.md) widget chapter to learn how to create custom property editors.