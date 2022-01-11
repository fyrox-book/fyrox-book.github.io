# fyrox

Fyrox is a feature-rich, general purpose game engine that is suitable for any kind of games. It is capable to power
games with small- or medium-sized worlds, large-sized world most likely will require some manual work. This chapter
of the book will help you to get familiar with core engine concepts and will help you start writing your first game!

Let's briefly get over some basic concepts of the engine, there's not much, but all of them are crucial to understand
design decisions made in the engine.

## ECS and classic OOP

The engine **does not** use ECS, instead it uses **classic OOP** with composition over inheritance. More specifically,
complex objects in the engine can be constructed using simpler objects. As a very simple example of this, let's 
check the scene node. The base object in for every scene node is a `Base` node, it contains a transform, a list
of children, etc. The more complex object, that _extends_ functionality of the `Base` node stores an instance of
`Base` inside of them. For example, a `Mesh` node is a `Base` node _plus_ some specific info (a list of surfaces,
material, etc.). The "hierarchy" depth is unlimited - a `Light` node in the engine is an enumeration of three 
possible types of light source. `Directional`, `Point`, and `Spot` light sources both use `BaseLight` node, 
which in its turn contains `Base` node inside. Graphically it can be represented like so:

```text
`Point`
|__ Point Light Properties (radius, etc.)
|__`BaseLight`
   |__ Base Light Properties (color, etc.)
   |__`Base`
      |__ Base Node Properties (transform, children nodes, etc.)
```

As you can see, this forms the nice tree that shows what the object contains. This is very natural way of describing
scene nodes, it gives you the full power of building an object of any complexity. On the other hand, in ECS, all
you have is just a set of components that you have to query in your systems, the structure becomes flat, and you lose
all "relationships" between object parts.

### Performance

Theoretically, the ECS approach _can_ give you better performance, but let's at first see where ECS is beneficial,
and why classic approach is still viable. The ECS is beneficial _only_ in cases where you have to process 
**ten or hundreds thousands** objects every frame, the performance gain of cache friendliness can be significant
in such cases. But let's stop for a second and ask ourselves again: how _often_ games have such huge amount of objects 
that has to be processed every frame? There are very few examples of such games:

- Strategy games - at some extent, because there are very few games that allows you to control tens of thousands
units at the same time. More often you have a range from five hundreds up to few thousands.
- Sandboxes - there could be lots of tiny objects that has to be processed every frame.
- Specific genres - games with destructible environment and so on.

Note that the list does not include games with vast worlds, why so? The reason is that such games does **not**
process every tiny object in the world at once, instead they split the world in small chunks and process only
few chunks at once, those where the player is present. 

The rest of genres operate on a tiny amount of object compared to those up above, maybe a few hundreds at max.
One might say - hey, each object could contain lots of tiny "moving parts", what's about them? Usually each 
object contains up to 10-15 sub-parts, which leads us to few thousands of "atomic" object. Is it much? Not really.

### Architecture

One might also think that ECS is a silver bullet for borrow checker in Rust, which "shuts its noisy mouth" once
and for all leaving you only with your game code. That's not quite true, it somewhat solves the problem of unique
mutable access to data, but interaction between systems can still be painful. Standard OOP-approach is always being
criticized by allowing you to create spaghetti-code for which borrow checker will yell at you (which is indeed 
reasonable). We should consider borrow checker not as our enemy, that prevents us from writing code, but as 
our friend that tells us - "dude, this won't work without potential crashes, memory issues, etc.". What borrow
checker tries to tell us, it that we need to re-think the architecture of our game. 

So how the engine solves the problem of unique mutable access? It forces you to use a "top-down" flow in your game.
What does that mean? In short, you have to change the data only by going from top to bottom on a call tree. But 
isn't that too restrictive, what if I want to call some higher-level function while being in lower-level function?
This is a very good question, and a short answer for it: _no_. It isn't restrictive at all, because you can always
invert the "bottom-to-top" flow to "top-down". The "bottom-to-top" calls are prohibited, because they're violating
unique mutable borrow rules.

The flow can be easily inverted by _deferring_ actions for later, not for a next frame, but for a moment after 
the place where "bottom-to-top" call was required. How this can be achieved? All you should do is to collect the
info that is needed to perform inverted "bottom-to-top" call and do a call right after that place where it was 
required, but starting from the top level of your game. One of the most common approaches for this is to use 
message passing with Rust's channels (MPSC queue). The receiver should be polled at the top level of your game
and every other place that needs "bottom-to-top" call should just queue desired actions by providing required info
in respective message. 

This is a very simple, yet powerful mechanism to satisfy make your code clearer and satisfy borrow checker. One
may argue that such approach has some performance impact. It is indeed has performance impact, but it is tiny, in
most cases it can't be even measured.

Borrowing issues cannot be fully prevented, even the right architecture can't help borrow checker to prove that 
your code is safe in some cases (graph data structure for example). To solve this problem, the engine uses
generational arenas (_pool_ in fyrox's terminology) and handles. Instead of storing the objects in various places,
you put all your objects in a pool, and it gives you handles which can later be used to borrow a reference to
that object. This approach allows you to build any data structures that may hold "references" to other objects.
The references replaced with handles, which can be treated (very roughly) as just an index. See
[separate chapter](./beginning/data_management.md) in the book for more info.

### Can I use ECS anyways?

Indeed, you can! You can use it for your game code with no problems, all you should do is to write some glue
code that will provide the engine with required information. Please check examples for your favourite ECS crate
to understand how to do that.

### Afterword

ECS and classic OOP are _just tools_, pick one which suits you the most, don't be a blind zealot, choose wisely!