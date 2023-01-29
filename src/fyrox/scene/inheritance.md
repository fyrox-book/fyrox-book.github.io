# Property Inheritance

**Important:** This functionality is available on [latest engine](../beginning/scripting.md#using-latest-engine-version)
from `master` branch only. It will be available in next major release (0.28).

Property inheritance is used to propagate changes of unmodified properties from a prefab to its instances. For example,
you can change scale of a node in a prefab and its instances will have the same scale too, unless the scale is
set explicitly in an instance. Such feature allows you to tweak instances, add some unique details to them, but take
generic properties from parent prefabs.

Property inheritance works for prefab hierarchies of any depth, this means that you can create something like this:
a room prefab can have multiple instances of various furniture prefabs in it, while the furniture prefabs can also be
constructed from other prefabs and so on. In this case if you modify a property in one of the prefabs in the chain, 
all instance will immediately sync their unmodified properties. 

## How To Create Inheritable Properties

It is possible to use property inheritance for script variables. To make a property of your script inheritable, all you
need is to wrap its value using `InheritableVariable` wrapper.

```rust,no_run
# extern crate fyrox;
# use fyrox::core::variable::InheritableVariable;
# use fyrox::core::visitor::prelude::*;
# use fyrox::core::reflect::prelude::*;
#[derive(Reflect, Visit, Default, Clone, Debug)]
struct MyScript {
    foo: InheritableVariable<f32>
}
```

The engine will automatically resolve the correct value for the property when a scene with the script is loaded. If your
property was modified, then its value will remain the same, it won't be overwritten by parent's value. Keep in mind,
that the type of the inheritable variable must be cloneable and support reflection.

`InheritableVariable` implements the `Deref<Target = T> + DerefMut` traits, this means that any access via the `DerefMut` trait
will mark the property as modified. This could be undesired in some cases so `InheritableVariable` supports special `xxx_silent` 
methods that don't touch the internal modifiers and allows you to substitute the value with some other "silently" -
without marking the variable as modified.

## Which Fields Should Be Inheritable?

Inheritable variables intended to be "atomic" - it means that the variable stores some simple variable (`f32`, `String`,
`Handle<Node>`, etc.). While it is possible to store "compound" variables (`InheritableVariable<YourStruct>`), it is
not advised because of inheritance mechanism. When the engine sees inheritable variable, it searches the same variable
in a parent entity and copies its value to the child, thus completely replacing its content. In this case, even if you
have inheritable variables inside compound field, they won't be inherited correctly. Let's demonstrate this in the
following code snippet:

```rust,no_run
# extern crate fyrox;
# use fyrox::core::reflect::prelude::*;
# use fyrox::core::variable::InheritableVariable;
# 
#[derive(Reflect, Clone, PartialEq, Eq, Debug)]
struct SomeComplexData {
    foo: InheritableVariable<u32>,
    bar: InheritableVariable<String>,
}

#[derive(Reflect, Debug)]
struct MyEntity {
    some_field: InheritableVariable<f32>,

    // This field won't be inherited correctly - at first it will take parent's value and then
    // will try to inherit inner fields, but its is useless step, because inner data is already
    // a full copy of parent's field value.
    incorrectly_inheritable_data: InheritableVariable<SomeComplexData>,

    // Subfields of this field will be correctly inherited, because the field itself is not inheritable.
    inheritable_data: SomeComplexData,
}
```

This code snippet should clarify, that inheritable fields should contain some "simple" data, and almost never - complex
structs.

## Editor

The editor wraps all inheritable properties in a special widget that supports property reversion. Reversion allows you
to drop current changes and take the parent's property value. This is useful if you want a property to inherit its parent's 
value. In the Inspector it looks like this:

![revert](./revert.png)

Clicking on the `<` button will take the value from the parent prefab and the property won't be marked as modified anymore. In case
there is no parent prefab, the button will just drop `modified` flag.
