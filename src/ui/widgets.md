# Widgets

The subsections of this chapter explains how to use every widget built into Fyrox. The widgets in the table of contents to the left are ordered in alphebetical order. However, below we will order them by primary function to help introduce them to new users.

## Containers

The Container widgets primary purpose is to contain other widgets. They are mostly used as a tool to layout the UI in visually different ways.

* [Stack panel](./stack_panel.md): The Stack Panel arranges widgets in a linear fashion, either vertically or horizontally depending on how it's setup.
* [Wrap Panel](./wrap_panel.md): The Wrap Panel arranges widgets in a linear fashion but if it overflows the widgets are continued adjacent to the first line. Can arrange widgets either vertically or horizontally depending on how it's setup.
* [Grid](./grid.md): The Grid arranges widgets into rows and columns with given size constraints.
* [Canvas](./canvas.md): The Canvas arranges widgets with pixel perfect precision.\
* [Window](./window.md): The Window holds other widgets in a panel that can be configured at setup to be move-able, expanded and contracted via user input, exited, and have a displayed label. The window has a title bar to assist with these features.
    * [Message Box](./message_box.md): The Message Box is a Window that has been streamlined to show standard confirmation/information dialogues, for example, closing a document with unsaved changes. It has a title, some text, and a fixed set of buttons (Yes, No, Cancel in different combinations).
* [Menu](./menu.md): The Menu is a root container for Menu Items, an example could be a menu strip with File, Edit, View, etc items.
* [Popup](./popup.md): The Popup is a panel that locks input to its content while it is open. A simple example of it could be a context menu.
* [Scroll Viewer](./scroll_viewer.md): The ScrollViewer is a wrapper for Scroll Panel that adds two scroll bars to it.
    * [Scroll Panel](./scroll_panel.md): The Scroll Panel is a panel that allows you apply some offset to children widgets. It is used to create "scrollable" area in conjunction with the Scroll Viewer.
* [Expander](./expander.md): The Expander handles hiding and showing multiple panels of widgets in an accordian style UI element. Multiple panels can be shown or hidden at any time based on user input.
* [Tab Control](./tab_control.md): The Tab Control handles hiding several panels of widgets, only showing the one that the user has selected.
* [Docking Manager](./dock.md): The Docking manager allows you to dock windows and hold them in-place.
* [Tree](./tree.md): The Tree allows you to create views for hierarchical data.

## Visual

The Visual widgets primary purpose is to provide the user feedback generally without the user directly interacting with them.

* [Text](./text.md): The Text widget is used to display a string to the user.
* [Image](./image.md): The Image widget is used to display a pixel image to the user.
* [Vector Image](./vector_image.md): The Vector Image is used to render vector instructions as a graphical element.
* [Rect](./rect.md): The Rect allows you to specify numeric values for X, Y, Width, and Height of a rectangle.
* [Progress Bar](./progress_bar.md): The Progress Bar shows a bar whoes fill state can be adjusted to indicate visually how full something is, for example how close to 100% is a loading process.
* [Decorator](./decorator.md): The Decorator is used to style any widget. It has support for different styles depending on various events like mouse hover or click.
    * [Border](./border.md): The Border widget is used in conjunction with the Decorator widget to provide configurable boarders to any widget for styling purposes.

## Controls

Control widgets primary purpose is to provide users with intractable UI elements to control some aspect of the program.

* [Button](./button.md): The Button provides a press-able control that can contain other UI elements, for example a Text or Image Widget.
* [Check Box](./checkbox/check_box.md): The Check Box is a toggle-able control that can contain other UI elements, for example a Text or Image Widget.
* [Text Box](./text_box.md): The Text Box is a control that allows the editing of text.
* [Scroll Bar](./scroll_bar.md): The Scroll Bar provides a scroll bar like control that can be used on it's own as a data input or with certain other widgets to provide content scrolling capabilities.
* [Numeric Field](./numeric.md): The Numeric Field provides the ability to adjust a number via increment and decrement buttons or direct input. The number can be constrained to remain inside a specific range or have a specific step.
* [Range](./range.md): The Range allows the user to edit a numeric range - specify its begin and end values.
* [List View](./list_view.md): The List View provides a control where users can select from a list of items.
* [Dropdown List](./dropdown_list.md): The Drop-down List is a control which shows the currently selected item and provides a drop-down list to select an item.
* [File Browser](./file_browser.md): The File Browser is a tree view of the file system allowing the user to select a file or folder.
* [Curve Editor](./curve_editor.md): The CurveEditor allows editing parametric curves - adding points, and setting up transitions (constant, linear, cubic) between them.
* [Inspector](./inspector.md): The Inspector automatically creates and handles the input of UI elements based on a populated Inspector Context given to it allowing the user to adjust values of a variety of models without manually creating UI's for each type.
