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

## Message passing

The engine uses message passing mechanism for any UI logic. What does that mean? Let's see at the button from the
previous section and imagine we want to change its text. To do that we need to explicitly "tell" the button's text
widget to change its content to something new. This is done by sending a message to the widget.

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
