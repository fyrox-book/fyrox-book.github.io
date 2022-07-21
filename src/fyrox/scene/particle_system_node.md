# Particle system 

Particle system is a scene node that is used to create complex visual effects (VFX). It operates on huge amount
of particles at once allowing you to do complex simulation that involves large amount of particles. Typically,
particle systems are used to create following visual effects: smoke, sparks, blood splatters, steam, etc. 

**Important:** Current particle system implementation is **not deterministic**, this means that the state of the
particles will be different at each run of your game. Also you **cannot** rewind the particle system, nor set
a particular position in time. This fact limits potential usages of the particle system, however it is still useful
for any effects that does not have to be deterministic, like sparks, smoke, steam, etc. This is a known issue, and
it will eventually be fixed by adding a new kind of particle systems. Tracking issue could be found 
[here](https://github.com/FyroxEngine/Fyrox/issues/120).

