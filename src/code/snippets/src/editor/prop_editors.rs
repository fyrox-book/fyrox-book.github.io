use fyrox::core::variable::InheritableVariable;
use fyrox::gui::inspector::editors::collection::VecCollectionPropertyEditorDefinition;
use fyrox::gui::inspector::editors::enumeration::EnumPropertyEditorDefinition;
use fyrox::gui::inspector::editors::inherit::InheritablePropertyEditorDefinition;
use fyrox::{
    core::reflect::prelude::*, core::type_traits::prelude::*,
    gui::inspector::editors::inspectable::InspectablePropertyEditorDefinition,
};
use fyroxed_base::Editor;
use strum_macros::{AsRefStr, EnumString, VariantNames};

// ANCHOR: add_property_editor
#[derive(Reflect, Clone, Debug)]
struct MyStruct {
    foo: u32,
    bar: String,
}

fn add_property_editor(editor: &Editor) {
    editor
        .property_editors
        .insert(InspectablePropertyEditorDefinition::<MyStruct>::new());
}
// ANCHOR_END: add_property_editor

// ANCHOR: add_enum_property_editor
#[derive(Reflect, Default, Debug, AsRefStr, EnumString, VariantNames, TypeUuidProvider, Clone)]
#[type_uuid(id = "31311d8b-f956-4ae9-a633-1e45b755f322")]
enum MyEnum {
    #[default]
    Baz,
    Foo(u32),
    Bar {
        baz: String,
        foobar: u32,
    },
}

fn add_enum_property_editor(editor: &Editor) {
    editor
        .property_editors
        .insert(EnumPropertyEditorDefinition::<MyEnum>::new());
}
// ANCHOR_END: add_enum_property_editor

// ANCHOR: inheritable
#[derive(Reflect, Debug, TypeUuidProvider, Default, Clone)]
#[type_uuid(id = "31311d8b-f956-4ae9-a633-1e45b755f323")]
struct MyOtherStruct {
    foo: u32,
    bar: String,
}

// An example script with inheritable field of custom structure.
struct MyScript {
    inheritable: InheritableVariable<MyOtherStruct>,
}

fn add_inheritable_property_editor(editor: &Editor) {
    editor
        .property_editors
        .insert(InspectablePropertyEditorDefinition::<MyOtherStruct>::new());

    // This is responsible for supporting inheritable properties in scripts.
    editor
        .property_editors
        .insert(InheritablePropertyEditorDefinition::<MyOtherStruct>::new());

    // Alternatively, the two previous insertions could be replaced by a single call of helper
    // method:
    editor
        .property_editors
        .register_inheritable_inspectable::<MyStruct>();
}
// ANCHOR_END: inheritable

// ANCHOR: collections

// An example script with Vec field of custom structure.
struct MyOtherScript {
    inheritable: Vec<MyOtherStruct>,
}

fn add_collection_property_editor(editor: &Editor) {
    editor
        .property_editors
        .insert(InspectablePropertyEditorDefinition::<MyOtherStruct>::new());

    // VecCollectionPropertyEditorDefinition is used to create a property editor for Vec<MyStruct>,
    // internally it uses a registered property editor for its generic argument (MyStruct).
    editor
        .property_editors
        .insert(VecCollectionPropertyEditorDefinition::<MyOtherStruct>::new());

    // Alternatively, you can use a special helper method to replace the two blocks above by a
    // single one.
    editor
        .property_editors
        .register_inheritable_vec_collection::<MyOtherStruct>();
}
// ANCHOR_END: collections
