# Animation

The engine offers a quite powerful animation system that will help you to animate your game characters. It built around
skeletal animation and pose blending. Next chapters will help you to understand what this means.

## What is animation?

Animation is a timeline with tracks, which in their turn have key frames, each key frame holds information about how a 
parameter should change at a moment in time. When playback time changes, the animation calculates intermediate values
between key frames using various sorts of interpolation:

- Step - value changes from keyframe to keyframe immediately.
- Linear - value changes linearly from keyframe to keyframe.
- Cubic - value changes from keyframe to keyframe using cubic interpolation.

Animation can have multiple tracks, where each track is responsible for animation of a single node (see `Limitations`
section). 

Animations usually prepared in special 3D or 2D modelling software, and then loaded in the engine. Animation loading 
have no difference to a model resource loading. Model resource can hold any number of animations which will be 
instantiated to a scene either by calling `Model::instantiate` or by `Model::retarget_animations`.

## Limitations

Current version of the engine does **not** support animation of arbitrary object properties. At the moment, you can 
animate only position, scaling, rotation part of a node's transform. While this may seem too limiting, it covers 90%
of animation use cases. 

Arbitrary property animation however, allows you to animate any property of an object. For example, you may have an 
animation that animates color of an object. Such functionality is planned for the future versions along with the
animation editor.

Morphing animation is not supported too, but can be done manually by transforming vertices in a model's vertex buffer. 