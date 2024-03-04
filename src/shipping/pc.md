# PC

PC builds can be created using either the automatic approach using the editor or manual. This chapter covers both
ways.

## Automatic

![PC Build](pc_build.png)

The editor provides a special tool that can create a build for shipping in a few clicks. It can be opened by going to
`File -> Export Project...`. At first, you need to select a target platform from the list of available platforms.
Then specify the data folders, ignored extensions of assets, data folders, etc. Finally, click `Export` and wait until
your game build is done. It can take from few minutes to tens of minutes, depending on the size of your game. 

## Manual

Manual build consists of three main steps:

- Building the game for desired platform.
- Copying assets.
- Bundling everything together.

Your game can be built pretty easily by a single `cargo` command:

```shell
cargo build --package executor --release
```

This command will create an executable file of your game in `target/release` folder. Go to this folder and copy the
`executor` file (it can have different extension depending on your platform). Create a folder for your final game build
and copy the `executor` file there.

Now go to the root directory of your game and copy all assets folders (for example, `data` folder) and paste it in the
folder with your executable. This is pretty much all you need to create a simple build. However, this solution is far
from optimal, because it clones all the assets, even those that aren't actually used in the final build.