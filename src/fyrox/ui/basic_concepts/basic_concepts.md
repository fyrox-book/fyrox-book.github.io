# Basic concepts

This chapter should help you understand basic concepts lying in the foundation of the GUI in the engine.

## Stateful

**Stateful UI* means that we can create and destroy widgets when we need to, it is the opposite approach of 
**immediate-mode** or **stateless UIs** when you don't have long-lasting state for your widgets
(usually stateless UI hold its state only for one or two frames). 

Stateful UI is much more powerful and flexible, it allows you to have complex layout system without having to 
create hacks to create complex layout as you'd do in immediate-mode UIs. It is also much faster in terms of 
performance.

Stateful UI is a must for complex user interfaces that requires rich layout and high performance. I'm not telling
that you _can't_ do it in immediate mode UI, you can, but using tons of hacks. See [Layout](#layout) section for
more info.

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

Every widget in the engine uses composition to build more complex widgets. All widgets (and respective builders) contains
`Widget` instance inside, it provides basic functionality the widget such as layout information, hierarchy, default
foreground and background brushes (their usage depends on derived widget), render and layout transform and so on. 

## Component Querying

Many widgets provide component querying functionality - you can get an immutable reference to inner component by its type. It is 
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

There is no classic callbacks to handle various types of messages, which may come from widgets. Instead, you should write
your own message dispatcher where you'll handle all messages. Why so? At first - decoupling, in this case business logic
is decoupled from the UI. You just receive messages one-by-one and do specific logic. The next reason is that any 
callback would require context capturing which could be somewhat restrictive - since you need to share context with the 
UI, it would force you to wrap it in `Rc<RefCell<..>>`/`Arc<Mutex<..>>`.

Message dispatcher is very easy to write, all you need to do is to handle UI messages in `Plugin::on_ui_message` method:

```rust
# extern crate fyrox;
# use fyrox::{
#     core::{pool::Handle, uuid::Uuid},
#     event_loop::ControlFlow,
#     gui::{button::ButtonMessage, message::UiMessage, UiNode},
#     plugin::{Plugin, PluginContext},
# };
# 
struct MyPlugin {
    button: Handle<UiNode>,
}

impl Plugin for MyPlugin {
#     fn id(&self) -> Uuid {
#         todo!()
#     }
# 
    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        message: &UiMessage,
        _control_flow: &mut ControlFlow,
    ) {
        if let Some(ButtonMessage::Click) = message.data() {
            if message.destination() == self.button {
                println!("The button was clicked!");
            }
        }
    }
}
```

As you can see, all you need to do is to check type of incoming message and message destination, which is a node handle
from which a message was come from. Then you do any actions you want.

### Message routing strategies

Message passing mechanism works in pair with various routing strategies that allows you to define how the message 
will "travel" across the tree of nodes.

1. Bubble - a message starts its way from a widget and goes up on hierarchy until it reaches root node of hierarchy.
Nodes that lies outside that path won't receive the message. This is the most important message routing strategy, that
is used for **every** node by default.
2. Direct - a message passed directly to every node that are capable to handle it. There is actual routing in this 
case. Direct routing is used in rare cases when you need to catch a message outside its normal "bubble" route.

## Layout

The engine uses very complex, yet powerful layout system that allows you to build complex user interfaces with 
complex layout. Layout pass has two _recursive_ sub-passes:

1. Measurement - the sub-pass is used to fetch the desired size of each node in hierarchy.  
2. Arrangement - the sub-pass is used to set final position and size of each node in hierarchy.

Such split is required because we need to know desired size of each node in hierarchy before we can actually do an
arrangement.  
