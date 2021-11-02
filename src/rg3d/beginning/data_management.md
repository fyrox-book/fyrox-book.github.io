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

Since the pool is just a contiguous memory block, it is much more CPU cache-friendly. This means that in most
cases the data portions will be loaded in CPU caches, making the access to the data blazing fast.

Almost every entity in the engine "lives" in its own pool, this make it easy to create such data structures
like graphs, where a node refers to other nodes. In this case scene nodes stores just handles (which is just
8 bytes of memory) to other nodes.