# Custom Widget

It is possible to create your own widgets, that could solve a specific task that couldn't be solved easily with the
widgets that the engine provides.

Let's say, for instance, that we need to have a custom button with specific visual effects. It will have a border and
a text, and it will also react to mouse events:

![custom widget](custom_widget.gif)

A "skeleton" of such widget could be something like this (for now it does nothing):  

```rust,no_run
{{#include ../code/snippets/src/ui/custom.rs:widget_skeleton}}
```

Every widget in the engine must have an instance of `Widget` (`widget: Widget` field) type in them and implement the 
`Control` trait with two required methods. `query_component` is used for dynamic component fetching and could be used 
to support behavior mix-ins and support derived widgets, that based on some other widgets. There's a lot more of 
available methods in the `Control` trait, however for simplicity we won't use it in this chapter. 

`handle_routed_message` is used to react to various messages, but only in the `child -> parent -> parent_of_parent -> ...`
chain. For example, if some of the child widget of our button will receive a message, it will also be passed to our button,
then to a parent of our button (if any), etc. This routing strategy is called "bubble" routing (it acts like a bubble of
air in the water; it always goes up). See [this section](basic_concepts/basic_concepts.md#message-routing-strategies) for
more info.

## Custom Logic

Now let's add some logic to the button, that will handle various mouse events. The full version of our button widget's 
logic will be like so:

```rust ,no_run
{{#include ../code/snippets/src/ui/custom.rs:my_button}}
```

As you can see, the most of the code was placed in `handle_routed_message`, we using it to respond for four messages:
`MouseDown` + `MouseUp`, `MouseEnter` + `MouseLeave`. Let's look closely at each pair of the messages.

The first two messages is used to handle clicks and send appropriate message to the outside world if a click has happened.
When you're sending a message, it is not immediately processed, instead it is put in the common message queue and will 
be processed by the engine later. You can react to such messages to perform a desired action, see 
[the section below](custom.md#reacting-to-click-messages) for more info. 

The last two events are used to alter the visual appearance of the button by changing the colors of both the border and 
the text. The source code above is very simple and straightforward, despite the look of it.

## Builder

Did you notice, that we didn't assign anything to `border` and `text` handles in our button widget? It is because, we
haven't made a respective builder for our button. Builder is a separate structure, that collects all the info from
the outside world and "compiles" it into a finished widget. Usually, widgets contains a bunch of children widgets, which
in their turn could have their own children and so on. In our case, the button will have two child widgets: a border and
a text.

```rust,no_run
{{#include ../code/snippets/src/ui/custom.rs:my_button_builder}}
```

This is how a button is created, at first we're creating a border widget instance with a text widget as a child of it.
Text widget uses the actual text string from our builder, and also it sets the desired alignment in parent border's 
bounds. Finally, we're initializing an instance of `MyButton` with the handles of the widget we've just made and as
the last step we're adding the widget to the user interface.

## Using the Builder

The widget could be created using the builder we've just made like so:

```rust,no_run
{{#include ../code/snippets/src/ui/custom.rs:my_button_builder_usage}}
```

## Reacting to Click Messages

Our button sends a `Click` message every time when it was pressed, and we can use this message to perform some actions
in an application. All you need to do is to catch `MyButtonMessage::Click` in `Plugin::on_ui_message` and do something
in response:

```rust,no_run
{{#include ../code/snippets/src/ui/custom.rs:reacting_to_click_messages}}
```

## Custom widget or composition of widgets. 
 
When do you need a custom widget? The answer depends on the use case, but the general rules here is quite simple:

- If your widget exist in a single instance, then there is no need to create a custom widget for it.
- If you need to create multiple instances of your widget, and each widget will carry some specific data, then you
  definitely need a custom widget.

Custom widgets have some limitations that could be limiting, one of them is that custom widgets do not have
access to your code, since they're "living" inside UI and know nothing about the "environment" where they're
being used.

## Source Code and Web Demo

Full source code for this chapter can be found [here](https://github.com/FyroxEngine/Fyrox-demo-projects/blob/main/ui/game/src/custom.rs)
, and you can also run [web demo](https://fyrox.rs/assets/demo/ui/index.html) to see it in action.