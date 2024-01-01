# System Requirements

As any other software, Fyrox has its own system requirements that will provide the best user experience. 

- **CPU** - at least 2 core CPU with 1.5 GHz per each core. The more is better.
- **GPU** - any relatively modern GPU with OpenGL 3.3+ support. If the editor fails to start, then it is most likely your
video card does not support OpenGL 3.3+. Do **not** try to run the editor on virtual machines, pretty much all of them
have rudimentary support for graphics APIs which won't let you run the editor.
- **RAM** - at least 1 Gb of RAM. The more is better.
- **VRAM** - at least 256 Mb of video memory. It highly depends on your game. 

## Supported Platforms

| Platform    | Engine | Editor |
|-------------|--------|--------|
| Windows     | ✅      | ✅      |
| Linux       | ✅      | ✅      |
| macOS       | ✅¹     | ✅      |
| WebAssembly | ✅      | ❌²     |
| Android     | ✅      | ❌²     |

- ✅ - first-class support
- ❌ - not supported
- ¹ - macOS suffers from bad GPU performance on Intel chipsets, M1+ works well.
- ² - the editor works only on PC, it requires rich filesystem functionality as well as decent threading support.