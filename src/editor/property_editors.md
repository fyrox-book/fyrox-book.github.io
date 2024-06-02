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
{{#include ../code/snippets/src/editor/prop_editors.rs:add_property_editor}}
```

Keep in mind, that your structure must implement `Reflect` trait, otherwise you'll get a compilation error.

### Enumerations

Enumerations are a bit trickier to support, than simple structures, because they require a bit more traits to be 
implemented for your enumeration. At first, make sure that your `editor` project has the following dependencies:

```toml
#[dependencies]
strum = "0.26.0"
strum_macros = "0.26.0"
```

These two crates responsible for enum to string (and vice versa) conversions which will be very useful for us. The 
following example shows a typical usage:

```rust,no_run
{{#include ../code/snippets/src/editor/prop_editors.rs:add_enum_property_editor}}
```

As you can see, your enumeration needs a decent amount of trait implementations, hopefully all of them can be derived.

### Inheritable Properties

If your structure or enumeration needs to be inheritable (see more info about [property inheritance](../scene/inheritance.md)),
then you need one more step. In case of inheritable variables, your fields will be wrapped in `InheritableVariable<>` and
this fact requires you to register an appropriate property editor for this:

```rust,no_run
{{#include ../code/snippets/src/editor/prop_editors.rs:inheritable}}
```

### Collections

If you have a vector of some custom structure (`Vec<MyStruct>`), then you also need to register a property editor for
it:

```rust,no_run
{{#include ../code/snippets/src/editor/prop_editors.rs:collections}}
```

## Custom Property Editors

See [Inspector](../ui/inspector.md) widget chapter to learn how to create custom property editors.