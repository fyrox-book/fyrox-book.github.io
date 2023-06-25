# Frequently Asked Questions

This chapter contains answers for frequently asked questions.

## Which graphics API the engine uses?

Fyrox uses quite old OpenGL 3.3 on PC and OpenGL ES 3.0 on WebAssembly. Why? Mainly due to historical reasons, back in 
the day (Q4 of 2018), there wasn't any good alternatives to it with wide amount of supported platforms. For example `wgpu`
[wasn't even exist](https://crates.io/crates/wgpu/0.1.0), since its first version was released in January 2019. Other
crates did their first baby steps and weren't ready for production.

Why not use alternatives now? There's no need for this, the engine has **a lot** of stuff that needs to be implemented,
and graphics is not a top-priority task.

## Does the engine based on ECS?

No, the engine uses mixed composition-based, object-oriented design with message passing and other different approaches
that fits the most for a particular task. Why not ECS for everything, though? Pragmatism. Use a right tool for the job,
don't use a microscope to hammer nails.

## What kinds of games can I make using Fyrox?

Pretty much any kinds of games, excepts maybe games with vast open-worlds (since there's no built-in world streaming).
In general, it depends on your game development experience.