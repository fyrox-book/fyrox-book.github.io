# Frequently Asked Questions

This chapter contains answers for frequently asked questions.

## Which graphics API does the engine use?

Fyrox uses OpenGL 3.3 on PC and OpenGL ES 3.0 on WebAssembly. Why? Mainly due to historical reasons. Back in the day 
(Q4 of 2018), there weren't any good alternatives to it with a wide range of supported platforms. For example, `wgpu` 
[didn't even exist](https://crates.io/crates/wgpu/0.1.0), as its first version was released in January 2019. Other crates were taking their first baby steps and weren't ready for production.

### Why not use alternatives now?

There is no need for it. The current implementation works and is more than good enough. So instead of focusing on 
replacing something that works for little to no benefit, the current focus is on adding features that are missing as 
well as improving existing features when needed. If you have some specific demands, you can implement your own graphics
server with a GAPI that you want. See [this chapter](../rendering/server.md) for more info.

## Is the engine based on ECS?

No, the engine uses a mixed composition-based, object-oriented design with message passing and other different approaches 
that fit the most for a particular task. Why not use ECS for everything, though? Pragmatism. Use the right tool for the job. 
Don't use a microscope to hammer nails. As was mentioned in the previous chapter, it is possible to use ECS, but you need
to manually synchronize the state of your entities with respective engine entities.

## What kinds of games can I make using Fyrox?

Pretty much any kind of games, except maybe games with vast open-worlds (since there's no built-in world streaming).
In general, it depends on your game development experience.

## What if I don't need the editor?

You can completely ignore the editor if you need, you can even delete it from your project without any consequences.
It is also possible to initialize the engine manually, as an ordinary Rust crate. See 
[this chapter](../engine/manual_initialization.md) for more info.