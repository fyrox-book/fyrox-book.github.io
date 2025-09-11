# Graphics Server

Graphics server is an abstraction that hides low-level graphics API (such as DirectX, OpenGL, Vulkan, etc.)
calls under a higher level interface that consists of a number of "building blocks" that allows you to 
render computer graphics in an easier way.

## OpenGL Graphics Server

The default graphics server is the OpenGL-based one (`GlGraphicsServer`). Why using a deprecated API in the 
modern world? The answer is simple: OpenGL is wide-spread, runs on pretty much all mainstream platforms 
(except consoles). Fyrox has a small developers team, which can't maintain a number of different GAPIs for
different platforms. Foreseeing the next potential question - why not just use wgpu? The answer is again 
simple: legacy. Fyrox is a relatively old game engine (compared to the entire Rust ecosystem, of course) 
and when its development was started, there was no such thing as wgpu. Transition to it nowadays is a 
challenging task, since the renderer of the engine is quite big. Anyway, the engine is still evolving and 
more support for other GAPIs will be added in the future.

## Custom Graphics Server

It is possible to write a graphics server with pretty much any API, since a graphics server 
(`GraphicsServer` trait) has quite a small number of functions. If you're familiar with other GAPIs, then 
it should be relatively straightforward for you to implement your own server.

## Usage

Typical use of the graphics server is shown extensively in the [render pass section](render_pass.md).