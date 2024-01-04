# Performance

This section of the book covers very specific cases of extreme performance, that is suitable for some exceptional cases.
For the vast majority of cases, standard engine approaches are perfectly fine.

## ECS

Theoretically, the ECS approach _can_ give you better performance, but lets at first see where ECS is beneficial,
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
checker tries to tell us is that we need to re-think the architecture of our game.

So how does Fyrox solve the problem of unique mutable access? It forces you to use a "top-down" flow in your game.
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
generational arenas (_pool_ in Fyrox's terminology) and handles. Instead of storing the objects in various places,
you put all your objects in a pool, and it gives you handles which can later be used to borrow a reference to
that object. This approach allows you to build any data structures that may hold "references" to other objects.
The references replaced with handles, which can be treated (very roughly) as just an index. See
[separate chapter](../beginning/data_management.md) in the book for more info.
