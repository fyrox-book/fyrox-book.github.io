# Borrow Checker

Rust has a [famous borrow checker](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html), which has become a 
sort of horror story for newcomers. It is often treated like an enemy that prevents you from writing anything 
useful in the ways you may be used to in other languages. In fact, it is a very useful part of Rust that proves the correctness 
of your program and prevents you from doing nasty things like [memory corruption](https://en.wikipedia.org/wiki/Memory_corruption), 
[data races](https://en.wikipedia.org/wiki/Race_condition#Data_race), etc. This chapter explains how Fyrox solves 
the most common borrowing issues and makes game development as easy as in any other game engine.

## Multiple Borrowing

When writing a script logic, there is often a need to do a multiple borrowing of some data, usually it is other scene
nodes. In normal circumstances, you can borrow each node one-by-one, but in other cases you can't do an action 
without borrowing two or more nodes simultaneously. In this case, you can use multi-borrowing:

```rust
{{#include ../code/snippets/src/borrowck/mod.rs:synthetic_example}}
```

As you can see, you can borrow multiple nodes at once with no compilation errors. Borrowing rules in this case
are enforced at runtime. They're the same as standard Rust borrowing rules:

1) You can have infinite number of immutable references to the same object.
2) You can have only one mutable reference to the same object.

Multi-borrow context provides detailed error messages for cases when borrowing has failed. For example, it will 
tell you if you're trying to mutably borrow an object that is already borrowed as immutable (and vice versa).
It also provides handle validation and will tell you what's wrong with it. It could be either an invalid index
or generation. The latter means that the object at the handle was changed and therefore the handle is invalid.

The previous example looks kinda synthetic and does not show the real-world code that could lead to borrowing 
issues. Let's fix this. Imagine that you're making a shooter, and that you have bots that can follow and attack 
targets. Then the code could look like this: 

```rust
{{#include ../code/snippets/src/borrowck/mod.rs:bot_example}}
```

As you can see, for this code to compile we need to borrow at least two nodes simultaneously: the node with `Bot`
script and the `target` node. This is because we're calculating distance between the two nodes to switch 
animations accordingly (attack if the target is close enough).

As with pretty much any approach, this one is not ideal and comes with its own pros and cons. The pros are quite 
simple:

- No compilation errors - sometimes Rust is too strict about borrowing rules, and valid code does not pass its
checks.
- Better ergonomics - no need to juggle with temporary variable here and there to perform an action.

The cons are:

- Multi-borrowing is slightly slower (~1-4% depending on your use case) - this happens because the 
multi-borrowing context checks borrowing rules at runtime.


## Message Passing

Sometimes the code becomes so convoluted that it is hard to maintain and understand what it is doing. 
This happens when code coupling gets to a certain point, which requires very broad context for the code to
be executed. For example, if bots in your game have weapons it is so tempting to just borrow the weapon 
and call something like `weapon.shoot(..)`. When your weapon is simple, then it might work fine. However, when 
your game gets bigger and weapons get new features, a simple `weapon.shoot(..)` might be not enough. It could be
because the `shoot` method gets more and more arguments or some other reason. This is quite common. In
general, when your code becomes tightly coupled, it becomes hard to maintain it, and, more importantly, can
easily result in issues with the borrow checker. To illustrate this, let's look at
this code:

```rust
{{#include ../code/snippets/src/borrowck/without_message_passing.rs:without_message_passing}}
```

This is probably one of the typical implementations of shooting in games - you cast a ray from the weapon
and if it hits a bot, you apply some damage to it. In this case, bots can also shoot, and this is where
borrow checker again gets in our way. If you try to uncomment the 
`// weapon.shoot(ctx.handle, &mut ctx.scene.graph);` line, you'll get a compilation error that tells you that 
`ctx.scene.graph` is already borrowed. It seems like we are stuck, and we need to somehow fix this issue.
We can't use multi-borrowing in this case because it still enforces borrowing rules and instead of compilation
error, you'll runtime error.

To solve this, you can use the well-known message passing mechanism. The core idea is to not call methods
immediately, but to collect all the needed data for the call and send it an object, so it can do the call later.
Here's how it will look:

```rust
{{#include ../code/snippets/src/borrowck/message_passing.rs:message_passing}}
```

The weapon now subscribes to `ShootMessage` and listens to it in `on_message` method and from there it can
perform the actual shooting without any borrowing issues. The bot now just sends the `ShootMessage` instead of
borrowing the weapon trying to call `shoot` directly. The messages do not add any one-frame delay as you might
think, they're processed in the same frame, so there's no one-or-more frames desynchronization.

This approach with messages has its own pros and cons. The pros are quite significant: 

- Decoupling - coupling is now very loose and done mostly on the message side.
- Easy to refactor - since the coupling is loose, you can refactor the internals with a low chance of breaking
existing code, that could otherwise be done because of intertwined and convoluted code.
- No borrowing issues - the method calls are done in different places and thus there are no lifetime collisions.
- Easy to write unit and integration tests - this comes from loose coupling. 

The cons are the following: 

- Message passing is slightly slower than direct method calls (~1-7% depending on your use case) - you should 
keep message granularity at a reasonable level. Do not use message passing for tiny changes, as it will most likely make 
your game slower.
