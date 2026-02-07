pub mod save;
mod save_custom;

use fyrox::core::visitor::prelude::*;
use std::path::Path;
use std::sync::Arc;

// ANCHOR: my_struct
#[derive(Visit, Default)]
struct MyStruct {
    foo: u32,

    #[visit(rename = "baz")]
    foobar: f32,

    #[visit(optional)]
    optional: String,

    #[visit(skip)]
    ignored: usize,
}
// ANCHOR_END: my_struct

// ANCHOR: my_struct_with_manual_visit
struct MyStructWithManualVisit {
    foo: u32,
    foobar: f32,
    optional: String,
    ignored: usize,
}

impl Visit for MyStructWithManualVisit {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        // Create a region first.
        let mut region = visitor.enter_region(name)?;

        // Add fields to it.
        self.foo.visit("Foo", &mut region)?;

        // Manually rename the field for serialization.
        self.foobar.visit("Baz", &mut region)?;

        // Ignore result for option field.
        let _ = self.optional.visit("Baz", &mut region);

        // Ignore `self.ignored`

        Ok(())
    }
}
// ANCHOR_END: my_struct_with_manual_visit

// ANCHOR: visit_my_structure
async fn visit_my_structure(path: &Path, object: &mut MyStruct, write: bool) -> VisitResult {
    if write {
        let mut visitor = Visitor::new();
        object.visit("MyObject", &mut visitor)?;

        // Dump to the path.
        visitor.save_binary_to_file(path)
    } else {
        let mut visitor = Visitor::load_binary_from_file(path).await?;

        // Create default instance of an object.
        let mut my_object = MyStruct::default();

        // "Fill" it with contents from visitor.
        my_object.visit("MyObject", &mut visitor)
    }
}
// ANCHOR_END: visit_my_structure

// ANCHOR: environment
struct MyStructWithEnv {
    // ...
}

struct MyEnvironment {
    some_data: String,
}

impl Visit for MyStructWithEnv {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        if let Some(environment) = visitor.blackboard.get::<MyEnvironment>() {
            println!("{}", environment.some_data);
        }

        Ok(())
    }
}

fn serialize_with_environment() {
    let mut my_object = MyStructWithEnv {
        // ...
    };

    let mut visitor = Visitor::new();

    visitor.blackboard.register(Arc::new(MyEnvironment {
        some_data: "Foobar".to_owned(),
    }));

    my_object.visit("MyObject", &mut visitor).unwrap();
}
// ANCHOR_END: environment
