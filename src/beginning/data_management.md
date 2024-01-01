# Data Management

The engine uses pools to store most objects (scene nodes in a 
graph, animations in an animation player, sound sources in an audio context, etc.). Since you'll use them quite often, reading and understanding this chapter is recommended. 

## Motivation

Rust ownership system and borrow checker, in particular, dictate the rules of data management. In game development, you
often have the need to reference objects from other objects. In languages like C, this is usually achieved by simply storing a raw 
pointer and calling it a day. That works, yet it's remarkably unsafe - you risk either forgetting to destroy an object and leaking 
memory or destroying an object still being referenced and then trying to access deallocated memory. Other languages, like C++, allow 
you to store _shared pointers_ to your data, which by keeping a reference count, ensures the previous doesn't happen at the cost of 
a, most often, negligible overhead. Rust counts with smart pointers similar to this, though not without their limitations. There is the `Rc/Arc` - they function like _shared pointers_, except they don't allow mutating their content, only 
reading it. If you want mutability, you use either a `RefCell` for a
single-threaded environment, or a `Mutex` for a multithreaded environment. That is where the problems begin. For 
types such as `Rc<RefCell>` or `Arc<Mutex>`, Rust enforces its borrowing rules at runtime, which are unlimited readers but 
a single writer. Any attempt to borrow mutably more than once at a time will lead to a runtime error.

Another problem with these shared references is that is very easy to accidentally create cyclical references
that prevent objects from ever being destroyed. While the previous could be lived with, the last problem is especially
severe in the case of games: the overhead of runtime checks. In the case of a `Rc<RefCell>`, it is a single 
reference counter for given accesses to the data, but in the case of a `Arc<Mutex>`, it is a mutex lock. 

The solution to these problems is far from ideal; it certainly has its own downfalls. Instead of scattering objects across memory 
and then having to manage the lifetime of each of them through reference counting, we can store all of the objects in a single
and contiguous memory block and then use indices to access each object. Such a structure is called a pool. 

## Technical Details

A pool is an efficient method of data management. A pool is a vector with entries that can be either vacant or occupied. Each 
entry, regardless of its status, also stores a number called a _generation_ number. This is used 
to understand whether an entry has changed over time or not. When an entry is reused, its generation number is 
increased, rendering all previously created handles leading to the entry invalid. This is a simple and efficient 
algorithm for tracking the lifetime of objects.

To access the data in the entries, the engine uses the previously mentioned _handles_. A handle is a pair of the index of an entry
and a generation number. When you put an object in the pool, this gives you the handle that leads to the object, as well as the 
entry's current generation number. The number remains valid until you "free" the object, which makes the entry vacant again.

## Advantages

- Since a pool is a contiguous memory block, it is far more CPU cache-friendly. This reduces the occurrences of CPU cache misses, which makes accesses to data blazingly fast.
- Almost every entity in Fyrox lives on its own pool, which makes it easy to create data structures
like graphs, where nodes refer to other nodes. In this case, nodes simply need to store a handle to refer to other nodes.
- Simple lifetime management. There is no way to leak memory since cross-references can only be done via handles.
- Fast random access with a constant complexity.
- Handles are the same size as a pointer on a 64-bit architecture, just 8 bytes.

## Disadvantages

- Pools can contain lots of gaps between currently used memory, which may lead to less efficient memory usage. 
- Handles are sort of weak references, but worse. Since they do not own any data nor even point to their data, you
need a reference to its pool instance in order to borrow the data a handle leads to.
- Handles introduce a level of indirection that can hurt performance in places with high loads that require random
access, though this is not too significant as random access is already somewhat slow because of potential 
CPU cache misses.

## Usage

You'll use `Handle` _a lot_ while working with Fyrox. So where are the main usages of pools and 
handles? The largest is in a [scene graph](../scene/graph.md). This stores all the nodes in a pool and gives handles
to each node. Each scene node stores a handle to their parent node and a set of handles to their children nodes. A scene graph
automatically ensures that such handles are valid. In [scripts](../scripting/script.md), you can also store handles
to scene nodes and assign them in the editor.

[Animation](../animation/animation.md) is another place that stores handles to animated scene nodes. 
[Animation Blending State Machine](../animation/blending.md) stores its own state graph using a pool; it also takes
handles to animations from an animation player in a scene. 

And the list could keep going for a long time. This is why you need to understand the basic concepts of data management, as to 
efficiently and fearlessly use Fyrox.

## Borrowing

Once an object is placed in a pool, you have to use its respective handle to get a reference to it. This can
be done with either `pool.borrow(handle)` or `pool.borrow_mute(handle)`, or by using the `Index` trait: `pool[handle]`. Note that 
these methods panic when the handle given is invalid. If you want to be safe, use the `try_borrow(handle)` or 
`try_borrow_mut(handle)` method.

```rust,no_run
# extern crate fyrox;
# use fyrox::core::pool::Pool;
#
# fn main() {
let mut pool = Pool::<u32>::new();
let handle = pool.spawn(1);

let obj = pool.borrow_mut(handle);
*obj = 11;

let obj = pool.borrow(handle);
assert_eq!(*obj, 11);
# }
```

## Freeing 

You can extract an object from a pool by calling `pool.free(handle)`. This will give you the object back and make all current 
handles to it invalid.

```rust,no_run
# extern crate fyrox;
# use fyrox::core::pool::Pool;
#
# fn main() {
let mut pool = Pool::<u32>::new();
let handle = pool.spawn(1);

pool.free(handle);

let obj = pool.try_borrow(handle);
assert_eq!(obj, None);
# }
```

## Take and Reserve

Sometimes you may want to temporarily extract an object from a pool, do something with it, and then put it back, yet not want 
to break every handle to the object in the process. There are three methods for this:

1) `take_reserve` + `try_take_reserve` - moves an object out of the pool but leaves the entry in an occupied state. This function returns a tuple with two values `(Ticket<T>, T)`. The latter one being your object, and the former one being a wrapper over its index that allows you to return the object once you're done with it. This is called a ticket. Note that attempting to borrow a moved object will cause a panic! 
2) `put_back` - moves the object back using the given ticket. The ticket contains information about where in the pool to return the object to. 
3) `forget_ticket` - makes the pool entry vacant again. Useful in cases where you move an object
out of the pool, and then decide you won't return it. If this is the case, you **must** call
this method, otherwise, the corresponding entry will remain unusable.

Reservation example:

```rust,no_run
# extern crate fyrox;
# use fyrox::core::pool::Pool;
#
# fn main() {
let mut pool = Pool::<u32>::new();
let handle = pool.spawn(1);

let (ticket, ref mut obj) = pool.take_reserve(handle);

*obj = 123;

// Attempting to fetch while there is an existing reservation, will fail.

let attempt_obj = pool.try_borrow(handle);
assert_eq!(attempt_obj, None);

// Put the object back, allowing borrowing again.

pool.put_back(ticket, *obj);

let obj = pool.borrow(handle);

assert_eq!(obj, &123);
# }
```

Forget example:

```rust,no_run
# extern crate fyrox;
# use fyrox::core::pool::Pool;
#
# fn main() {
let mut pool = Pool::<u32>::new();
let handle = pool.spawn(1);

let (ticket, _obj) = pool.take_reserve(handle);

pool.forget_ticket(ticket);

let obj = pool.try_borrow(handle);

assert_eq!(obj, None);
# }
```

## Iterators

There are a few possible iterators, each one serving its own purpose:

1) `iter/iter_mut` - creates an iterator over occupied pool entries, returning references to each object.
2) `pair_iter/pair_iter_mut` - creates an iterator over occupied pool entries, returning tuples of a handle and reference
to each object. 

```rust,no_run
# extern crate fyrox;
# use fyrox::core::pool::Pool;
#
# fn main() {
let mut pool = Pool::<u32>::new();
let _handle = pool.spawn(1);

let mut iter = pool.iter_mut();

let next_obj = iter.next().unwrap();

assert_eq!(next_obj, &1);

let next_obj = iter.next();

assert_eq!(next_obj, None);
# }
```

## Direct Access

You have the ability to get an object from a pool using only an index. The methods for that are `at` and `at_mut`.

## Validation
 
To check if a handle is valid, you can use the `is_valid_handle` method.

## Type-erased Handles

The pool module also offers type-erased handles that can be of use in some situations. Still, try to avoid using these, as they may introduce hard-to-reproduce bugs. Type safety is always good :3

A type-erased handle is called an `ErasedHandle` and can be created either manually or from a strongly-typed handle.
Both handle types are interchangeable; you can use the `From` and `Into` traits to convert from one to the other.

### Getting a Handle to an Object by its Reference

If you need to get a handle to an object from only having a reference to it, you can use the `handle_of` method.

### Iterate Over and Filter Out Objects

The `retain` method allows you to filter your pool's content using a closure provided by you.
