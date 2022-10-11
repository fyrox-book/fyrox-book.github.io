# Serialization

Serialization is a process that converts arbitrary objects into a set of bytes that can be stored to disk or to send
them over the network. An opposite to serialization - deserialization - is a process that restores objects from a given
set of bytes. Serialization often used to make save/load functionality in games.

Fyrox has built-in serializer that is used all over the place the engine and which is represented by a `Visit` trait. 
`Visit` name could be confusing, but it is called after well-known [Visitor](https://en.wikipedia.org/wiki/Visitor_pattern) 
design pattern. 

Serialization and deserialization itself is handled by `Visitor`, it can be created in two modes: read and write. See
mode info in respective sections below.

## Usage

There are two main ways to implement `Visit` trait, each way serves for specific cases. Let's understand which one to
use when.

### Proc-macro `#[derive(Visit)]`

The engine provides proc-macro, that uses code generation to implement `Visit` trait for you. All you need to do is 
to add `#[derive(Visit)]` to your struct/enum. Code generation in most cases is capable to generate typical 
implementation for serialization/deserialization. You should prefer proc-macro to manual implementation in most cases.

The macro supports few very useful attributes, that can be added to fields of a struct/enum:

- `#[visit(optional)]` - forces the engine to ignore any errors that may occur during deserialization, leaving a field's
value in default state. Very useful option if you're adding a new field to your structure, otherwise the engine will
refuse to continue loading of your struct. In case of scripts, deserialization will stop on missing field, and it will
be partially loaded.
- `#[visit(rename = "new_name")]` - replaces the name of a field with given value. Useful if you need to rename a field
in the code, but leave backward compatibility with previous versions.
- `#[visit(skip)]` - ignores a field completely. Useful if you don't want to serialize a field at all, or a field is not
serializable.

To use the macro, you must import all types related to `Visit` trait by `use fyrox::core::visitor::prelude::*;`. Here's
an example:

```rust,no_run
# extern crate fyrox;
use fyrox::core::visitor::prelude::*;

#[derive(Visit)]
struct MyStruct {
    foo: u32,

    #[visit(rename = "baz")]
    foobar: f32,

    #[visit(optional)]
    optional: String,

    #[visit(skip)]
    ignored: usize,
}
```

### Manual implementation

Manual implementation of the trait gives you an opportunity to fix compatibility issues, do some specific actions
during serialization (logging, for instance). Typical manual implementation could look like this:

```rust,no_run
# extern crate fyrox;
use fyrox::core::visitor::prelude::*;

struct MyStruct {
    foo: u32,
    foobar: f32,
    optional: String,
    ignored: usize,
}

impl Visit for MyStruct {
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
```

This code pretty much shows the result of macro expansion from the previous section. As you can see, proc-macro saves
you from writing tons of boilerplate code.

Implementing `Visit` trait is a first step, the next step is to either serialize an object or deserialize it. See
the following section for more info.

## Serialization and Deserialization

To serialize an object all you need to do is to create an instance of a Visitor in either read or write mode and use it
like so:

```rust,no_run
# extern crate fyrox;
use fyrox::core::visitor::prelude::*;
use std::path::Path;

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

async fn visit_my_structure(path: &Path, object: &mut MyStruct, write: bool) -> VisitResult {
    if write {
        let mut visitor = Visitor::new();
        object.visit("MyObject", &mut visitor)?;

        // Dump to the path.
        visitor.save_binary(path)
    } else {
        let mut visitor = Visitor::load_binary(path).await?;

        // Create default instance of an object.
        let mut my_object = MyStruct::default();

        // "Fill" it with contents from visitor.
        my_object.visit("MyObject", &mut visitor)
    }
}
```

The key function here is `visit_my_structure` which works in both serialization and deserialization modes depending on
`write` flag value. 

When `write` is true (serialization), we're creating a new empty visitor and filling it with values from our `object` 
and then "dump" its content to binary file.

When `write` is false (deserialization), we're loading contents of a file, creating the object in its default state and 
then "filling" it with values from the visitor.

## Environment

Sometimes there is a need to pass custom data to `visit` methods, one of the ways to do this is to use `environment` field
of the visitor:

```rust,no_run
# extern crate fyrox;
use fyrox::core::visitor::prelude::*;
use std::sync::Arc;

struct MyStruct {
    // Fields are intentionally omitted.
}

struct MyEnvironment {
    some_data: String,
}

impl Visit for MyStruct {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        if let Some(environment) = visitor
            .environment
            .as_ref()
            .and_then(|e| e.downcast_ref::<MyEnvironment>())
        {
            println!("{}", environment.some_data);
        }

        Ok(())
    }
}

fn serialize_with_environment() {
    let mut my_object = MyStruct {};

    let mut visitor = Visitor::new();

    visitor.environment = Some(Arc::new(MyEnvironment {
        some_data: "Foobar".to_owned(),
    }));

    my_object.visit("MyObject", &mut visitor).unwrap();
}
```

## Limitations

All fields of your structure must implement `Default` trait, this is essential limitation because deserialization must
have a way to create an instance of an object for you. 
