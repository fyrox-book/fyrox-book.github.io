# User Interface 

Fyrox features an extremely powerful and flexible node-based user interface system. Power and flexibility comes with
a certain price: it has a steep learning curve. This chapter will cover user interface usage in the engine, explain
basic concepts, provide information about most commonly used widgets, and so on.

## User Data

It is possible to assign arbitrary data structure to the user interface (both from the editor and code). This ability is
very useful in cases where you need to have some "global" data for your user interface. The typical use case for user data is
to store handles to some widgets (for example, handles to buttons in your main menu), so these handles can be accessed 
directly from your plugin or scripts. User data is based on [dynamic types](../misc/dyn_type.md).

## Web Demo

You can explore UI system capabilities in this [web demo](https://fyrox.rs/assets/demo/ui/index.html). Keep in mind,
that it was designed to run on PC and wasn't tested on mobile devices.