# Data management

The engine uses generational arenas (pools in engine's terminology) to store most of the objects (scene nodes in a 
graph, animations in an animation player, sound sources in an audio context, etc.). You'll use them quite often, so it 
read this chapter thoroughly. 

## Motivation

Rust ownership system and borrow checker in particular, dictates the rules of data management. In game development you
often have a need to have some sort of reference to an object in some other object. In languages like C you usually 
store a raw pointer and consider it done. This works, but it is very unsafe - an object could die (its memory can become
deallocated), and you'll know about this only when try to access that object. In more advanced languages like C++, you
could store `shared_ptr<Foo>` which ensures that the pointer will be valid until at least one `shared_ptr<Foo>` instance
is held alive. In Rust, however, it is possible too, but with some limitations and quirks. At first, we won't consider 
raw pointers and leave this to C or C++. As for smart pointers such as `Rc/Arc` - they will work, but they do not allow
to mutate the content, only share it with multiple readers. To mutate the inner data, we can use either `RefCell` (for
single-threaded environment) or `Mutex` (for multithreaded env) to wrap the data. This is where the "fun" begins: for 
such types (`Rc<RefCell<Foo>>`, `Arc<Mutex<Foo>>`) Rust enforces borrowing rules (multiple readers, one writer) at runtime
which will stab you in the back with panic in unexpected time - any attempt to borrow the inner data mutably twice will
raise panic. 

The next problem with such shared references is the fact that it is very easy to accidentally create cyclical references
which will prevent objects from dying (deallocating). While this is more or less ok, the next problem is much more 
critical for games - overhead for runtime checks. In case of `Rc<RefCell<Foo>>` it is a single reference counter for
given references to the data, and in case of `Arc<Mutex<Foo>>` it is a mutex lock. 

The solution to these problems is not ideal, it has its own pros and cons. Instead of storing objects in random places
of memory and having to manage their lifetime by reference counting, we can put all objects in a single storage with 
a contiguous memory block and use indices to access the objects. Such structure is called - pool. 

## Technical details

Pool is an efficient way data management. Pool is a vector with entries that can be either vacant or occupied. Each 
entry, no matter occupied or vacant, also stores a special number called _generation_. The generation number is used 
to understand whether an entry has been changed over time or not. When an entry is reused, its generation number is 
increased leaving all previously created handle leading to the entry invalid. This is a very simple and efficient 
algorithm for tracking the "lifetime" of the objects.

To access the data in entries, the engine uses _handles_. The handle is a pair of index of an entry and a
_generation_ number. When you put an object in the pool, it gives you the handle that "leads" to the object.
At this moment the generation of the handle matches the generation of the corresponding entry so the handle
is valid. It will remain valid until you "free" the object, which will make the entry vacant again.

## Advantages

- Since the pool is just a contiguous memory block, it is much more CPU cache-friendly. This means that in most
cases the data portions will be loaded in CPU caches, making the access to the data blazing fast.
- Almost every entity in the engine "lives" in its own pool, this make it easy to create such data structures
like graphs, where a node refers to other nodes. In this case scene nodes stores just handles (which is just
8 bytes of memory) to other nodes.
- Easy lifetime management; no way to create memory leaks since cross-references could be done only via handles.
- Fast random access with constant complexity.
- Handles are the same size as pointer on 64-bit arch (8 bytes).

## Disadvantages

- Pool could be considered as a sparse array, which could contains a lot of gaps. This may lead to less efficient memory
usage. 
- Handles is a sort of weak references, but worse. Since they do not own any data (or even pointer to data), you will
need a pool instance to borrow a data a handle "points" to. 
- Handles introduce a level of indirection, that could hurt performance in places with high load that requires random
access (as a counterargument it is worth to mention, that random access is usually relatively slow because of potential 
CPU cache misses).

## Usage

You'll use `Handle<Something>` _a lot_ while developing a game using Fyrox. So where are the main usages of Pool and 
handles? The largest is indeed a [scene graph](../scene/graph.md); it stores all the nodes in a pool and gives handles
to nodes. Each scene node stores a handle to a parent node and a set of handle to children nodes. Scene graph
auto-magically ensures that such handles are valid. In [scripts](../scripting/script.md), you can also store handles
to scene nodes and assign them in the editor.

[Animation](../animation/animation.md) is another place which stores handles to animated scene nodes. 
[Animation Blending State Machine](../animation/blending.md) stores its own state graph using pool; it also takes
handles to animations from an animation player in a scene. 

This list can be extended for long; this is why you need to understand basic concepts of data management to efficiently
and fearlessly use the engine.

## Borrowing

Once an object was placed in a pool, you have to use respective handle to get a reference to it. This could 
be done either with `.borrow[_mut](handle)` or by using `Index` trait: `pool[handle]`. These methods panic
when handle is invalid, if you want to prevent that, use `try_borrow[_mut](handle)` method.

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

You can extract an object from a pool by calling `pool.free(handle)`, it will give you the object back, making
all handles to the object invalid.

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

## Take & reserve

Sometimes you may need to temporarily extract an object from a pool, do something with it and then put it back
while preserving handles to that object. There are three special methods for that:

1) `take_reserve` + `try_take_reserve` - moves object out of the pool, but leaves the entry in "occupied" state. This function returns
a tuple with two values `(Ticket<T>, T)`. The latter value is obviously your object, but the former is 
more interesting. It is a special wrapper over object index that allows you to return the object back. It is used
in `put_back` method. **Caveat:** an attempt to borrow moved object in the pool will cause panic! 
2) `put_back` - moves the object back in the pool using given ticket. Ticket says where to put the object in the 
pool. 
3) `forget_ticket` - makes the entry of the pool vacant again. It is useful in situations when you've moved object
out of the pool, but for some reason you don't want to return it back in pool, in this case you **must** call
this method, otherwise the corresponding entry will be unusable.

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

// Put back, allowing borrowing again.

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

There are few possible iterators, each one is useful for a particular purpose:

1) `iter/iter_mut` - creates an iterator that iterates over occupied pool entries returning references to an 
object associated with an entry.
2) `pair_iter/pair_iter_mut` - creates an iterator that iterates over occupied pool entries returning tuples with
two elements `(handle, reference)`. 

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

## Direct access

There is ability to get an object from a pool using only indices, there are two methods for that `at` and `at_mut`.

## Validation

Sometimes you may need to check if a handle is valid, to do that use `is_valid_handle` method.

## Type-erased handles

The pool module offers type-erased handles that could be useful for some situations. Try to avoid using type-erased
handles, because they may introduce hardly-reproducible bugs. Type safety is always good :)

Type-erased handle is called `ErasedHandle` and it can be created either manually, or from strongly-typed handles.
Both handle types are interchangeable, you can use `From` and `Into` traits to convert them one into another.

## Special 

Uncategorized stuff.

### Getting a handle of an object by its reference

Sometimes you may need to get a handle of an object having only a reference to it, there is a `handle_of` method
exactly for that.

### Iterate over and discard unnecessary objects

There is a `retain` method for that, it allows you to "filter" your pool using a closure with custom filtering
logic.