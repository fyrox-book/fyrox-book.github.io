# Border

The Border widget provides a stylized, static border around its child widget. Below is an example of creating a 1 pixel
thick border around a button widget:

```rust,no_run
{{#include ../code/snippets/src/ui/border.rs:create_border_with_button}}
```

As with other UI elements, we create the border using the BorderBuilder helper struct. The widget that should have a
border around it is added as a child of the base WidgetBuilder, and the border thickness can be set by providing a 
Thickness struct to the BorderBuilder's *with_stroke_thickness* function. This means you can set different thicknesses 
for each edge of the border.

You can style the border by creating a Brush and setting the border's base WidgetBuilder's foreground or background. 
The foreground will set the style of the boarder itself, while setting the background will color the whole area within 
the border. Below is an example of a blue border and a red background with white text inside.

```rust,no_run
{{#include ../code/snippets/src/ui/border.rs:create_blue_border_with_red_background}}
```
