# Design Philosophy and Goals

Let's talk a bit about design philosophy and goals of the engine. Development of the engine started in the beginning
of 2019 as a hobby project to learn Rust, and it quickly showed that Rust can be a game changer in the game development
industry. Initially, the engine was just a port of [an engine](https://github.com/mrDIMAS/DmitrysEngine) that is written 
in C. At the beginning, it was very interesting to build such complex thing as game engine in such low level language without
any safety guarantees. After a year of development it became annoying to fix memory related issues (memory corruption,
leaks, etc.), luckily at that time Rust's popularity grew, and it showed on my radar. I ([@mrDIMAS(https://github.com/mrDIMAS)]) 
was able to port the engine to it in less than a year. Stability has improved dramatically, no more random crashes, 
performance was at the same or better levels - time invested in learning new language was paid off. Development speed 
does not degrade over time as it was in C, it is very easy to manage growing project.

## Safety

One of the main goals of in the development of the engine is to provide high level of safety. What does this mean? 
In short: protect from memory-unsafety bugs. This does not include any logic errors, but when your game is free 
of random crashes due to memory unsafety it is much easier to fix logic bugs, because you don't have to think about
potentially corrupted memory.

Safety is also dictates architecture design decisions of your game, typical callback hell, that is possible to do in
many other languages, is very tedious to implement in Rust. It is possible, but it requires quite a lot of manual work
and quickly tell you that you're doing it wrong.

## Performance

Game engines usually built using system-level programming languages, that provides peak performance levels. Fyrox is not
an exception. One if its design goals is to provide high levels of performance by default, leaving an opportunity for
adding custom solutions for performance-critical places.

## Ease of use

Other very important part is that the engine should be friendly to newcomers. It should lower entry threshold, not make
it worse. Fyrox uses well known and battle-tested concepts, thus making it easier to make games with it. On other hand,
it still can be extended with anything you need - it tries to be as good for veterans of the game industry as for 
newcomers.

## Battle-tested

Fyrox has large projects built on it, that helps to understand real needs for general-purpose game engine. This helps
in revealing weak spots in design and fix them.