# NumericUpDown Widget

A widget that handles numbers of any machine type. Use this widget if you need to provide input field for a numeric
type.

## How to create

Use `NumericUpDownBuilder` to create a new instance of the `NumericUpDown` widget:

```rust
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle, gui::{numeric::NumericUpDownBuilder, widget::WidgetBuilder, BuildContext, UiNode}
# };
fn create_numeric_widget(ctx: &mut BuildContext) -> Handle<UiNode> {
 NumericUpDownBuilder::new(WidgetBuilder::new())
     .with_value(123.0f32)
     .build(ctx)
}
```

Keep in mind, that this widget is generic and can work with any numeric types. Sometimes you might get an "unknown type"
error message from the compiler (especially if your use `123.0` ambiguous numeric literals), in this case you need to
specify the type explicitly (`NumericUpDownBuilder::<f32>::new...`).

## Limits

This widget supports lower and upper limits for the values. It can be specified by `NumericUpDownBuilder::with_min_value`
and `NumericUpDownBuilder::with_max_value` (or changed at runtime using `NumericUpDownMessage::MinValue` and `NumericUpDownMessage::MaxValue`
messages):

```rust
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle, gui::{numeric::NumericUpDownBuilder, widget::WidgetBuilder, BuildContext, UiNode}
# };
fn create_numeric_widget(ctx: &mut BuildContext) -> Handle<UiNode> {
 NumericUpDownBuilder::new(WidgetBuilder::new())
     .with_value(123.0f32)
     .with_min_value(42.0)
     .with_max_value(666.0)
     .build(ctx)
}
```

The default limits for min and max are `NumericType::min_value` and `NumericType::max_value` respectively.

## Step

Since the value of the widget can be changed via up/down arrow buttons (also by dragging the cursor up or down on them), the widget
provides a way to set the step of the value (for increment and decrement at the same time):

```rust
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle, gui::{numeric::NumericUpDownBuilder, widget::WidgetBuilder, BuildContext, UiNode}
# };
fn create_numeric_widget(ctx: &mut BuildContext) -> Handle<UiNode> {
 NumericUpDownBuilder::new(WidgetBuilder::new())
     .with_value(125.0f32)
     .with_step(5.0)
     .build(ctx)
}
```

The default value of the step is `NumericType::one`.

## Precision

It is possible to specify **visual** rounding of the value up to desired decimal place (it does not change the way how
the actual value is rounded). For example, in some cases you might get irrational values such as `1/3 ~= 0.33333333`,
but you interested in only first two decimal places. In this case you can set the precision to `2`:

```rust
# extern crate fyrox;
# use fyrox::{
#     core::pool::Handle, gui::{numeric::NumericUpDownBuilder, widget::WidgetBuilder, BuildContext, UiNode}
# };
fn create_numeric_widget(ctx: &mut BuildContext) -> Handle<UiNode> {
 NumericUpDownBuilder::new(WidgetBuilder::new())
     .with_value(0.3333333f32)
     .with_precision(2)
     .build(ctx)
}
```