# Reflection

The engine offers quite powerful static [reflection](https://en.wikipedia.org/wiki/Reflective_programming). Reflection
allows you to get information about fields of structures, variants of an enumeration and its fields. It is also possible
to iterate over all fields, and iterate in-depth to get information about the entire "tree".

## Code Generation

The derive macro is available under `#[reflect(...)]` attribute that can be placed on both
the type and its fields.

### Type attributes

- `#[reflect(hide_all)]` - hide all fields from reflection.
- `#[reflect(bounds)]` - add type boundary for `Reflect` impl, for example
  `#[reflect(bounds = "T: Reflect + Clone")]`
- `#[reflect(non_cloneable)]` - prevent the macro from generating an implementation of
  [`Self::try_clone_box`] trait for your type. Could be useful for non-cloneable types.
- `#[reflect(derived_type = "Type")]` - marks the type for which the attribute is added as a
  subtype for the `Type`.

### Field attributes

- `#[reflect(hidden)]` - hides the field from reflection.
- `#[reflect(setter = "foo")]` - set the desired method that will be used by [`Self::set_field`]
  default implementation.
- `#[reflect(deref)]` - delegate the field access with `deref` + `deref_mut` calls. Could be
  useful for new-type objects.
- `#[reflect(field = "foo")]` - sets the desired method, that will be used to access
  the field.
- `#[reflect(field_mut = "foo")]` - sets the desired method, that will be used to access
  the field.
- `#[reflect(name = "name")]` - overrides the name of the field.
- `#[reflect(display_name = "name")]` - sets the human-readable name for the field.
- `#[reflect(tag = "tag")]` - sets some arbitrary string tag of the field. It could be used to
  group properties by a certain criteria or to find a specific property by its tag.
- `#[reflect(read_only)]` - the field is not meant to be editable. This flag does not prevent
  the reflection API from changing the actual value, it is just an instruction for external
  users (editors, tools, etc.)
- `[#reflect(immutable_collection)]` - only for dynamic collections (`Vec`, etc.) - means that its
  size cannot be changed, however the _items_ of the collection can still be changed.
- `#[reflect(min_value = "0.0")]` - minimal value of the field. Works only for numeric fields!
- `#[reflect(max_value = "1.0")]` - maximal value of the field. Works only for numeric fields!
- `#[reflect(step = "0.1")]` - increment/decrement step of the field. Works only for numeric fields!
- `#[reflect(precision = "3")]` - maximum amount of decimal places for a numeric property.

### Clone

By default, the proc macro adds an implementation of [`Self::try_clone_box`] with the assumption
that your type implements the [`Clone`] trait. Not all types can implement this trait, in this
case, add `#[reflect(non_cloneable)]` attribute for your type. This will force the implementation
of [`Self::try_clone_box`] to return `None`.

## Additional Trait Bounds

`Reflect` restricted to types that implement `Debug` trait, this is needed to convert the actual value
to string. `Display` isn't used here, because it can't be derived and it is very tedious to implement it
for every type that should support `Reflect` trait. It is a good compromise between development speed
and the quality of the string output.