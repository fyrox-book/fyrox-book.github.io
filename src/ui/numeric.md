# NumericUpDown Widget

![numeric up down](numeric_up_down.gif)

A widget that handles numbers of any machine type. Use this widget if you need to provide input field for a numeric
type.

## How to create

Use `NumericUpDownBuilder` to create a new instance of the `NumericUpDown` widget:

```rust,no_run
{{#include ../code/snippets/src/ui/numeric.rs:create_numeric_widget}}
```

Keep in mind, that this widget is generic and can work with any numeric types. Sometimes you might get an "unknown type"
error message from the compiler (especially if your use `123.0` ambiguous numeric literals), in this case you need to
specify the type explicitly (`NumericUpDownBuilder::<f32>::new...`).

## Limits

This widget supports lower and upper limits for the values. It can be specified by `NumericUpDownBuilder::with_min_value`
and `NumericUpDownBuilder::with_max_value` (or changed at runtime using `NumericUpDownMessage::MinValue` and `NumericUpDownMessage::MaxValue`
messages):

```rust,no_run
{{#include ../code/snippets/src/ui/numeric.rs:create_numeric_widget_with_limits}}
```

The default limits for min and max are `NumericType::min_value` and `NumericType::max_value` respectively.

## Step

Since the value of the widget can be changed via up/down arrow buttons (also by dragging the cursor up or down on them), the widget
provides a way to set the step of the value (for increment and decrement at the same time):

```rust,no_run
{{#include ../code/snippets/src/ui/numeric.rs:create_numeric_widget_with_step}}
```

The default value of the step is `NumericType::one`.

## Precision

It is possible to specify **visual** rounding of the value up to desired decimal place (it does not change the way how
the actual value is rounded). For example, in some cases you might get irrational values such as `1/3 ~= 0.33333333`,
but you interested in only first two decimal places. In this case you can set the precision to `2`:

```rust,no_run
{{#include ../code/snippets/src/ui/numeric.rs:create_numeric_widget_with_precision}}
```