# Data management

The engine uses generation arenas (pools in engine's terminology) for efficient data management. Pool is a
vector with entries that can be either vacant or occupied. Each entry, no matter occupied it or vacant, also
stores a special number called _generation_. The generation number is used to understand whether an entry has
been changed over time or not. When an entry is reused, its generation number is increased leaving all previously
created handle leading to the entry invalid. This is a very simple and efficient algorithm for tracking the
"lifetime" of the objects.

To access the data in entries, the engine uses _handles_. The handle is a pair of index of an entry and a
_generation_ number. When you put an object in the pool, it gives you the handle that "leads" to the object.
At this moment the generation of the handle matches the generation of the corresponding entry so the handle
is valid. It will remain valid util you "free" the object, which will make the entry vacant again.

## Advantages

Since the pool is just a contiguous memory block, it is much more CPU cache-friendly. This means that in most
cases the data portions will be loaded in CPU caches, making the access to the data blazing fast.

Almost every entity in the engine "lives" in its own pool, this make it easy to create such data structures
like graphs, where a node refers to other nodes. In this case scene nodes stores just handles (which is just
8 bytes of memory) to other nodes.

## Borrowing

Once an object was placed in a pool, you have to use respective handle to get a reference to it. This could 
be done either with `.borrow[_mut](handle)` or by using `Index` trait: `pool[handle]`. These methods panic
when handle is invalid, if you want to prevent that, use `try_borrow[_mut](handle)` method.

```rust,norun
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

## Take & reserve

Sometimes you may need to temporarily extract an object from a pool, do something with it and then put it back
while preserving handles to that object. There are three special methods for that:

1) `take_reserve` + `try_take_reserve` - moves object out of the pool, but leaves the entry in "occupied" state. This function returns
a tuple with two values `(Ticket<T>, T)`. The latter value is obviously is your object, but the first object is 
more interesting. It is a special wrapper over object index that allows you to return the object back. It is used
in `put_back` method. **Caveat:** an attempt to borrow moved object in the pool will cause panic! 
2) `put_back` - moves the object back in the pool using given ticket. Ticket says where to put the object in the 
pool. 
3) `forget_ticket` - makes the entry of the pool vacant again. It is useful in situations when you've moved object
out of the pool, but for some reason you don't want to return it back in pool, in this case you **must** call
this method, otherwise the corresponding entry will be unusable.

## Iterators

There are few possible iterators, each one is useful for particular purpose:

1) `iter/iter_mut` - creates an iterator that iterates over occupied pool entries returning references to an 
object associated with an entry.
2) `pair_iter/pair_iter_mut` - creates an iterator that iterates over occupied pool entries returning tuples with
two elements `(handle, reference)`. 

## Direct access

There is ability to get an object from a pool using only indices, there are two methods for that `at` and `at_mut`.

## Validation

Sometimes you may need to check if a handle is valid, to do that use `is_valid_handle` method.

## Type-erased handles

The pool module offers type-erased handles that could be useful for some situations. Try to avoid using type-erased
handles, because they may introduce hardly-reproducible bugs and type safety is always good :)

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