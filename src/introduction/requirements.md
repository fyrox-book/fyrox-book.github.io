# System Requirements

As any other software, Fyrox has its own system requirements that will provide the best user experience.

> ⚠️ Do not try to run the engine on virtual machines, pretty much all of them have rudimentary support for
> graphics APIs which won't let you run the engine. If the editor fails to start, then it is most likely your video card
> does not support OpenGL 3.3+.

## Minimum Requirements

The following system requirements are just the bare minimum to be able to compile and run relatively small game with
simple graphics:

- **CPU** - at least 2 core CPU with 1.5 GHz per each core. The more is better.
- **GPU** - any relatively modern GPU with OpenGL 3.3+ support with at least 128 Mb of video memory.
- **RAM** - at least 512 Mb of RAM. The more is better.

The actual system requirements highly depend on your game.

## Recommended System Configuration for Development

The following system configurations is recommended for the development process:

- **CPU** - 6 core CPU with 3.6 GHz per each core.
- **GPU** - any relatively modern GPU with OpenGL 3.3+ support with 2 Gb of video memory.
- **RAM** - 16 Gb of RAM.
- **Monitor** - 1920x1080 or higher resolution, lower resolutions may work as well.

The crucial part for the development is the CPU, the better the CPU the faster the compilation will be. 

## Supported Platforms

| Platform    | Engine | Editor |
|-------------|--------|--------|
| Windows     | ✅      | ✅      |
| Linux       | ✅      | ✅      |
| macOS       | ✅¹     | ✅      |
| WebAssembly | ✅      | ❌²     |
| Android     | ✅      | ❌²     |
| iOS         | ❓      | ❌²     |

- ✅ - first-class support
- ❓ - theoretically compilable, but not tested to be runnable.
  See [this issue](https://github.com/FyroxEngine/Fyrox/issues/726) for more info.
- ❌ - not supported
- ¹ - macOS suffers from bad GPU performance on Intel chipsets, M1+ works well.
- ² - the editor works only on PC, it requires rich filesystem functionality as well as decent threading support.