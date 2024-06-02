# Grid

Grids are one of several methods to position multiple widgets in relation to each other. A Grid Widget, as the name 
implies, is able to position children widgets into a grid of specifically sized rows and columns. 

Here is a simple example that positions several text widgets into a 2 by 2 grid:

```rust,no_run
{{#include ../code/snippets/src/ui/grid.rs:create_text_grid}}
```

As with other UI widgets, Grids are created via the GridBuilder struct. Each widget whose position should be controlled
by the Grid should be added as a child of the GridBuilder's base widget.

You then need to tell each child what row and column it belongs to via the on\_column and on\_row functions of their base
widget. By default, all children will be placed into row 0, column 0. 

After that you need to provide sizing constraints for each row and column to the GridBuilder by using the add\_row and 
add\_column functions while providing a GridDimension instance to the call. GridDimension can be constructed with the 
following functions:

* `GridDimension::auto()` - Sizes the row or column so it's just large enough to fit the largest child's size.
* `GridDimension::stretch()` - Stretches the row or column to fill the parent's available space, if multiple rows or 
columns have this option the size is evenly distributed between them.
* `GridDimension::strict(f32)` - Sets the row or column to be exactly the given value of pixels long. So a row will only 
be the given number of pixels wide, while a column will be that many pixels tall.

You can add any number of rows and columns to a grid widget, and each grid cell does **not** need to have a UI widget 
in it to be valid. For example, you can add a column and set it to a specific size via strict to provide spacing between
two other columns.

