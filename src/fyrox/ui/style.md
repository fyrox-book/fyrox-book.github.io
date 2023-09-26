# Styles 

The engine has an ability to customize the appearance of widgets, however it is not centralized, and has to be done 
per widget. Check `Style` section of each widget (keep in mind that some of the widgets does not support custom styles,
mainly because they were made in a hurry).

In general, styling of widgets could be performed by replacing parts of a widget with your own. For example, a button by default
uses [Decorator](./decorator.md) widget as its background, which in its turn uses simple set of brushes to control
internal [Border](./border.md) widget's parameters, such as background and foreground colors. This is ok if you're 
creating some tools, where you don't need bells and whistles. However, buttons in games could be of any shape, color,
have any kind of animations and so on. For this reason, `Button` widget allows you to change background widget with
your own. So, imagine that you have a button template with two images that represents its state - `Pressed` and `Normal`.
In this case you could create a custom widget, that will render different images depending on pressed state and use 
this widget as a background widget of your button.

The same applies to pretty much every other widget, for example [CheckBox](./checkbox/check_box.md) allows you to change
check marks for every of three states as well as a widget that is used as a background.