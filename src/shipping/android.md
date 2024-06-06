# Android

Android builds requires a lot of preparation steps which include:

- Android build target installation
- `cargo-apk` installation
- Android Studio
- Android SDK installation (at least API level 26)
- NDK installation
- CMake installation
- JRE installation

Install `cargo-apk` first:

```shell
cargo install cargo-apk
```

Install Android build target, for example `armv7-linux-androideabi`:

```shell
rustup target add armv7-linux-androideabi
```

You should install appropriate target for your target device (or emulator), it could also be `x86_64-linux-android`.

[Install Android Studio](https://developer.android.com/studio/index.html) first. Then install NDK by following
these [instructions](https://developer.android.com/studio/projects/install-ndk).

Setup environment variables, you need to set two of them to correct paths: `ANDROID_HOME` and `ANDROID_NDK_ROOT`.
Follow these [instructions](https://developer.android.com/tools/variables)

Install Java Runtime Environment from [here](https://www.java.com/ru/download/manual.jsp) and add `bin` folder
of it to your `PATH` variable. On Windows it could be `C:\Program Files\Java\jre-1.8\bin`.

Now you can build your game by running the following command from `executor-android` folder:

```shell
cargo-apk apk build --target=armv7-linux-androideabi
```

## Automation

Use the [project exporter](shipping.md) for automated builds.