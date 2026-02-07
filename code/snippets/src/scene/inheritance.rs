use fyrox::core::reflect::prelude::*;
use fyrox::core::variable::InheritableVariable;
use fyrox::core::visitor::prelude::*;

// ANCHOR: my_script
#[derive(Reflect, Visit, Default, Clone, Debug)]
struct MyScript {
    foo: InheritableVariable<f32>,
}
// ANCHOR_END: my_script

// ANCHOR: complex_inheritance
#[derive(Reflect, Clone, PartialEq, Eq, Debug)]
struct SomeComplexData {
    foo: InheritableVariable<u32>,
    bar: InheritableVariable<String>,
}

#[derive(Reflect, Clone, Debug)]
struct MyEntity {
    some_field: InheritableVariable<f32>,

    // This field won't be inherited correctly - at first it will take parent's value and then
    // will try to inherit inner fields, but its is useless step, because inner data is already
    // a full copy of parent's field value.
    incorrectly_inheritable_data: InheritableVariable<SomeComplexData>,

    // Subfields of this field will be correctly inherited, because the field itself is not inheritable.
    inheritable_data: SomeComplexData,
}
// ANCHOR_END: complex_inheritance
