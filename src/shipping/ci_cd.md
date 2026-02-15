# CI/CD Integration

It is also possible to configure continuous integration/continuous delivery (CI/CD) for games made with Fyrox. Each
project built using the project manager has a special crate called `export-cli`. This crate offers a command line
interface (CLI) for the same functionality that the editor uses when you export the game from it.

## Options

There are number of useful options that can be used to simplify testing of the produced builds.

- `-h`, `--help` - show the detailed usage instructions for the CLI.
- `-V`, `--version` - prints version of the CLI.
- `--target_platform <pc/wasm/android>` - the target platform to build the game to. Must be one of: `pc, android, wasm`.
  Keep in mind, that you must also set the appropriate `build_target` parameter if you're using cross compilation (for
  example, creating a WebAssembly or Android build from PC). Default value is `pc`.
- `--build_target <target triple>` - the name of the build target. The `default` value forces to compile the game to the
  default target of the current toolchain. Usually this parameter can be left unchanged, unless you need
  cross-compilation to some specific platform and architecture (see below). WebAssembly builds requires this parameter
  to be `wasm32-unknown-unknown`. Android builds require one of the following: `armv7-linux-androideabi` for 32-bit and
  `aarch64-linux-android` for 64-bit. The full list of build
  targets [can be found here](https://doc.rust-lang.org/nightly/rustc/platform-support.html). Default value is
  `default`.
- `--destination_folder <path>` - the destination folder for the build. Default value is `./build/`.
- `--include_used_assets` - a flag, that defines whether the project exporter should include only used assets in the
  final build or not. If specified, then this flag essentially forces the exporter to scan all the assets for
  cross-links and if there's at least one usage then such asset will be included in the final build. This option could
  be useful if your project has a lot of "dangling" resources, and you don't want to search all the used resources
  yourself. Use this option carefully, because it won't include assets that you manually load from code bypassing the
  resource manager. In this case, the project manager will simply ignore such "unknown" files. This option is disabled
  by default, because it may produce unwanted side effects in some projects.
- `--ignored_extensions <ext1,ext2,..,extN>` - specifies a set of file extensions that should be ignored. Each extension
  must be separated by a comma. For example: `log,txt,iml`. Default value is `log`, which excludes Fyrox log file.
- `-r`, `--run_after_build` - if specified, the exporter will try to run the exported project after the successful
  build. This option is disabled by default.
- `-o`, `--open_destination_folder` - if specified, the exporter will try to open the build folder in the default file
  manager of your OS after the successful build. This option is disabled by default.
- `-c`, `--convert_assets` - if specified, the exporter will try to convert all supported assets to their "shipping"
  version. For example, native game scenes and UIs will be converted from ASCII to binary if this option is specified.
  This option is enabled by default.
- `-e`, `--enable_optimization` - if specified, enables all possible optimizations for the build. This option is enabled
  by default.

## Examples

The typical usage varies on the target platform. Keep in mind that you can also add all the arguments listed above
for the commands below.

### PC

When building for Windows/Linux/macOS on the same OS (Windows → Windows, etc.) then all you need is to run the tool
with default arguments:

```shell
cargo run --package export-cli
```

Cross-compilation requires build target to be specified, for example if you're making Linux build from Windows you can
do something like this:

```shell
cargo run --package export-cli -- --build-target x86_64-unknown-linux-gnu
```

### WebAssembly

WebAssembly builds usually performed on Windows/Linux/macOS machine so it essentially requires cross-compilation and a
typical command to build the game can be something like this:

```shell
cargo run --package export-cli -- --target-platform android --build-target wasm32-unknown-unknown
```

Keep in mind that WebAssembly builds were tested only on `wasm32-unknown-unknown` build target.

### Android

Android builds usually performed on Windows/Linux/macOS machine so it essentially requires cross-compilation and a
typical command to build the game can be something like this for 64-bit version:

```shell
cargo run --package export-cli -- --target-platform android --build-target aarch64-linux-android
```

32-bit version requires different build target:

```shell
cargo run --package export-cli -- --target-platform android --build-target armv7-linux-androideabi
```

## CI/CD Script

The actual script depends on the runner, but the general logic should be the same: run the `export-cli` tool, copy
produced build from the build folder (default is `./build/`) to the build storage and that's pretty much it. The runner
may require some additional setup steps, like those specified in the installation guide of the engine.