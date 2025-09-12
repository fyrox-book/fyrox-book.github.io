# Design Philosophy and Goals

Let's talk a bit about design philosophy and goals of the engine. Development of the engine started in the beginning
of 2019 as a hobby project to learn Rust, and it quickly showed that Rust can be a game changer in the game development
industry. Initially, the engine was just a port of [an engine](https://github.com/mrDIMAS/DmitrysEngine) that is written 
in C. At the beginning, it was very interesting to build such complex thing as game engine in such low level language without
any safety guarantees. After a year of development, it became annoying to fix memory related issues (memory corruption,
leaks, etc.), luckily at that time Rust's popularity grew, and it showed on my radar. I ([@mrDIMAS](https://github.com/mrDIMAS)) 
was able to port the engine to it in less than a year. Stability has improved dramatically, no more random crashes, 
performance was at the same or better levels - time invested in learning new language was paid off. Development speed 
does not degrade over time as it was in C, it is very easy to manage growing project.

## Safety

One of the main goals in the development of the engine is to provide a high level of safety. What does this mean? 
In short: protection from memory-safety related bugs. This does not include any logic errors, but when your game is free 
of random crashes due to memory unsafety, it is much easier to fix logic bugs, because you don't have to think about
potentially corrupted memory.

Safety also dictates the architectural design decisions of your game. The typical callback hell, that is possible to do in
many other languages, is very tedious to implement in Rust. It is possible, but it requires quite a lot of manual work
which quickly tells you that you're doing it wrong.

## Performance

Game engines are usually built using system-level programming languages, which provide peak performance levels. Fyrox is not
an exception. One of its design goals is to provide high levels of performance by default, leaving an opportunity for
adding custom solutions for performance-critical places.

## Ease of use

Another very important part is that the engine should be friendly to newcomers. It should lower the entry threshold, not make
it worse. Fyrox uses well known and battle-tested concepts, thus making it easier to make games with it. On the other hand,
it can still be extended with anything you need - it tries to be as good for veterans of the game industry as it is for 
newcomers.

## Battle-tested

Fyrox has large projects built on it, which helps with understanding the real needs of a general-purpose game engine. It also helps
reveal weak spots in the design and fix them.
