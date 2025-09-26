# Basic concepts

This chapter should help you understand basic concepts lying in the foundation of the GUI in the engine.

## Stateful

Stateful UI means that we can create and destroy widgets when we need to, it is the opposite approach of
immediate-mode or stateless UIs when you don't have long-lasting state for your widgets
(usually stateless UI hold its state only for one or two frames).

Stateful UI is much more powerful and flexible, it allows you to have complex layout system without having to
create hacks to create complex layout as you'd do in immediate-mode UIs. It is also much faster in terms of
performance.

Stateful UI is a must for complex user interfaces that requires rich layout and high performance. I'm not telling
that you _can't_ do it in immediate mode UI, you can, but using tons of hacks. See [Layout](#layout) section for
more info.

## Model-View-Controller

The UI system is designed to be used in a classic model-view-controller MVC approach. Model in this case is your game
state, view is the UI system, controller is your event handlers. In other words - the UI shows what happens in your game
and does not store any game-related information. This is quite old, yet powerful mechanism that decouples UI code from
game code very efficiently and allows you to change game code and UI code independently.

## Node-based architecture

Every user interface could be represented as a set of small blocks that have hierarchical bonding between each
other. For example a button could be represented using two parts: a background and a foreground. Usually the background
is just a simple rectangle (either a vector or bitmap), and a foreground is a text. The text (the foreground widget)
is a child object of the rectangle (the background widget). These two widgets forms another, more complex widget that
we call button. Graphically it will look like this:

![Button](./button.svg)

On the right side of the image we can see the generic button and on the left side, we can see its hierarchical
structure. Such approach allows us to modify the look of the button as we wish, we can create a button with
image background, or with any vector image, or even other widgets. The foreground can be anything too, it can also
contain its own complex hierarchy, like a pair of an icon with a text and so on.

## Composition

Every widget in the engine uses composition to build more complex widgets. All widgets (and respective builders)
contains
`Widget` instance inside, it provides basic functionality the widget such as layout information, hierarchy, default
foreground and background brushes (their usage depends on derived widget), render and layout transform and so on.

## Component Querying

Many widgets provide component querying functionality - you can get an immutable reference to inner component by its
type. It is
used instead of type casting in many places. Component querying is much more flexible compared to direct type casting.
For example, you may want to build a custom [Tree](../tree.md) widget, you want your CustomTree to inherit all the
functionality from the Tree, but add something new. The Tree widget can manage its children subtrees, but it needs to
somehow get required data from subtree. Direct type casting would fail in this case, because now you have something
like this:

```rust,no_run
# extern crate fyrox;
# use fyrox::gui::tree::Tree;
struct CustomTree {
    tree: Tree,
    my_data: u32
}
```

On other hand, component querying will work fine, because you can query inner component (Tree in our case). Please note
that this has nothing similar with ECS and stuff, it is made to circumvent Rust's lack of inheritance.

## Message passing

The engine uses message passing mechanism for any UI logic. What does that mean? Let's see at the button from the
previous section and imagine we want to change its text. To do that we need to explicitly "tell" the button's text
widget to change its content to something new. This is done by sending a message to the widget.

There is no classic callbacks to handle various types of messages, which may come from widgets. Instead, you should
write
your own message dispatcher where you'll handle all messages. Why so? At first - decoupling, in this case business logic
is decoupled from the UI. You just receive messages one-by-one and do specific logic. The next reason is that any
callback would require context capturing which could be somewhat restrictive - since you need to share context with the
UI, it would force you to wrap it in `Rc<RefCell<..>>`/`Arc<Mutex<..>>`.

Message dispatcher is very easy to write, all you need to do is to handle UI messages in `Plugin::on_ui_message` method:

```rust,no_run
{{#include ../../code/snippets/src/ui/mod.rs:message_passing}}
```

As you can see, all you need to do is to check type of incoming message and message destination, which is a node handle
from which a message was come from. Then you do any actions you want.

### Message routing strategies

Message passing mechanism works in pair with various routing strategies that allows you to define how the message
will "travel" across the tree of nodes.

1. Bubble - a message starts its way from a widget and goes up on hierarchy until it reaches root node of hierarchy.
   Nodes that lies outside that path won't receive the message. This is the most important message routing strategy,
   that
   is used for **every** node by default.
2. Direct - a message passed directly to every node that are capable to handle it. There is actual routing in this
   case. Direct routing is used in rare cases when you need to catch a message outside its normal "bubble" route.

Bubble message routing is used to handle complex hierarchies of widgets with ease. Let's take a look at the button
example above - it has text widget as a content and when, for instance, you hover a mouse over the text widget the UI
system creates a "mouse moved" message and sends it to the text. Once it was processed by the text, it "floats" one
level of hierarchy up - to the button widget itself. This way the button widget can process mouse events as well.

## Layout

The UI systems uses complex, yet powerful layout system that allows you to build complex user interfaces with
complex layout. Layout pass has two _recursive_ sub-passes:

1. Measurement - the sub-pass is used to fetch the desired size of each widget in hierarchy. Each widget in the
   hierarchy
   "asked" for its desired size with the constraint from a parent widget. This step is recursive - to know a desired
   size
   of a widget root of some hierarchy you need to recursively fetch the desired sizes of every descendant.
2. Arrangement - the sub-pass is used to set final position and size of each widget in hierarchy. It uses desired size
   of every widget from the previous step to set the final size and relative position. This step is recursive.

Such separation in two passes is required because we need to know desired size of each node in hierarchy before we can
actually do an arrangement.

## Code-first and Editor-first approaches

The UI system supports both ways of making a UI - code- and editor-based approaches.

### Code-first approach

Code-first approach is used when your user interface is procedural and its appearance is heavily depends on
your game logic. In this case you need to use various widget builder to create UIs.

In case of code-first approach you should prefer so-called _fluent syntax_: this means that you can create your
widget in series of nested call of other widget builders. In code, it looks something like this:

```rust,no_run
{{#include ../../code/snippets/src/ui/mod.rs:create_fancy_button}}
```

This code snippet creates a button with an image and a text. Actually it creates **three** widgets, that forms
complex hierarchy. The topmost widget in hierarchy is the `Button` widget itself, it has two children widgets:
background image and a text. Background image is set explicitly by calling image widget builder with specific
texture. The text is created implicitly, the button builder creates `Text` widget for you and attaches it to
the button. The structure of the button can contain _any_ number of nodes, for example you can create a button
that contains text with some icon. To do that, replace `.with_text("My Button")` with this:

```rust,no_run
{{#include ../../code/snippets/src/ui/mod.rs:create_fancy_button_with_text}}
```

Quite often you need to store a handle to a widget in a variable, there is one neat trick to do that preserving
the fluent syntax:

```rust,no_run
{{#include ../../code/snippets/src/ui/mod.rs:create_fancy_button_with_shortcut}}
```

### Editor-first approach

Editor-first approach is used when you have relatively static (animations does not count) user interface,
that almost does not change in time. In this case you can use built-in WYSIWYG (what-you-see-is-what-you-get)
editor. See [Editor](../editor/editor.md) chapter for more info.

When a user interface is ready to be loaded in your game, you can do this like so:

```rust,no_run
{{#include ../../code/snippets/src/ui/load.rs:load_ui}}
```

This code loads the given user interface and sets it as primary user interface of your game. You can have multiple
user interfaces and they can be added using `let ui_handle = ctx.user_interfaces.add(ui);`. The `ui_handle` is then
can be used in `on_ui_message`, to understand from which UI the message has come from.

## Limitations

UI system uses completely different kind of scenes - UI scenes, which are fully decoupled from game scenes. This means
that you can't incorporate UI widgets in a game scene. As a consequence, you don't have an ability to attach scripts to
widgets - their logic is strictly defined in their backing code. This limitation is intentional, and it is here
only for one reason - decoupling of UI code from game logic. Currently, there's only one right approach to make UIs -
to create widgets in your game plugin and sync the state of the widgets with game entities manually.
