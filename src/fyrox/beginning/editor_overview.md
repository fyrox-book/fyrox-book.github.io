# FyroxEd Overview

FyroxEd - is the native editor of Fyrox, it is made for one purpose - be integrated game development environment,
that allows you to build your game from start to finish with relatively low effort.

You'll be spending a lot of time in the editor (FyroxEd), so you should get familiar with it and learn how to use its 
basic functionality. This chapter will guide you through basics, advanced topics will be covered in respective chapters.

## Windows

Once the editor is open for the first time, it will confuse you by amount of windows, buttons, lists, etc. Each window
serves for different purpose, but all of them works together to help you make your game. Let's look at the editor's
screenshot and learn what parts of it responsible for what (please note that it can change over time, because the 
development is quite rapid and image can become outdated pretty easily):

![Windows](./overview.png)

- **World viewer** - shows every object in the scene and their relations. It allows you to inspect and edit the 
contents of the scene in hierarchical form.
- **Scene preview** - renders your scene with debug info and various editor-specific objects (like gizmos and
stuff). Allows you to select, move, rotate, scale, delete, etc. various entities. On the left side it has a **Toolbar**,
it shows available context-dependent tools.
- **Inspector** - allows you to modify various properties of selected object.
- **Message Log** - shows the important messages from the editor.
- **Navmesh Panel** - allows you to create/delete and edit navigational meshes.
- **Command Stack** - shows the most recent actions you've done, allows you to undo and redo the changes on demand.
- **Asset Browser** - inspects the assets of your game, allows you to instantiate resources in the scene and so on.
- **Audio Context** - allows you to edit settings of scene's sound context, such as global volume, available sound 
effects, etc.

## Creating (Loading) a Scene

FyroxEd works with scenes - scene is a container for game entities, you can create and edit one scene at a time. You
must have a scene created (or loaded) to begin working with the editor. To create a scene click `File -> New Scene`.

To load existing scene, go to `File -> Load` and select desired scene using file browser. It is may be very useful
to use list of recently opened scenes, to load them quickly without a need to go `File -> Load`, search a scene, etc.
Just go to `File -> Recent Scenes` and click one to load.

## Populating Your Scene

Scene can be filled with various objects, there are two equivalent ways of creating game entities:

- By clicking `Create` main menu item and selecting desired entity.
- By right-clicking on a game entity in `World Viewer` and selecting desired entity from `Add Child` sub-menu.

Complex objects usually made in 3D modelling software (Blender, 3Ds Max, Maya, etc.) and saved in various formats, Fyrox
supports FBX format, which is supported by pretty much any 3D modelling software. You can add such object to your scene
by drag'n'dropping a desired asset for Asset Browser. Find one in the browser, click on it, hold the button and drag it
to the Scene Viewer and then release the button. 

You can do the same with other scenes made in the editor (`rgs` files), for example you can create a scene with a few 
objects in it with some scripts and re-use it across any other scenes. Such scene is called [prefab](../scene/prefab.md).

## Saving Scene

To save your work, go to `File -> Save`. For the first time (for unsaved scene), the editor will ask you to specify 
file name and path to a folder where the scene will be saved. A scene loaded from a file will be automatically saved 
to the path it was loaded from.

## Undo/redo

FyroxEd remembers your actions, and it is possible to undo (or redo undone) your changes to fix some stuff in the scene.
You can undo or redo your changes either by clicking `Edit -> Undo/Redo` or by standard shortcuts: `Ctrl+Z` - undo,
`Ctrl+Y` - redo.

## Controls

There are number of control keys that you'll be using most of the time, pretty much all of them works in `Scene Preview`
window:

### Editor camera movement
Click and hold `[Right Mouse Button]` within `Scene Preview` window to use movement controls:
  - `[W][S][A][D]` - Move camera forward/backward/left/right
  - `[Space][Q]/[E]` - Raise/Lower Camera
  - `[Ctrl]` - speed up
  - `[Shift]`- slowdown
### Others
- `[Left Mouse Button]` - Select
- `[Middle Mouse Button]` to pan camera in viewing plane
- `[1]` - Select interaction mode
- `[2]` - Move interaction mode
- `[3]` - Scale interaction mode
- `[4]` - Rotate interaction mode
- `[Ctrl]+[Z]` - Undo
- `[Ctrl]+[Y]` - Redo
- `[Delete]` - deletes current selection.

## Play Mode

One of the key features of the editor is that it allows you to run your game from it. Use `Play/Stop` button at the
top of `Scene Preview` window to enter (or leave) Play Mode. Keep in mind, that editor UI will be locked while you're
in the Play Mode. 

Play Mode can be activated only for projects that made with `fyrox-template` (or have similar structure). The editor
calls `cargo` commands to build and run your game in a separate process. Running the game in a separate process ensures
that the editor won't crash if the game does, it also provides excellent isolation between the game and the editor, not
giving a chance to break the editor by running the game.

## Additional Utilities

There are also number of powerful utilities that will make your life easier, which works without any scene, they can be
found under `Utils` section of main menu: 

- Animation Editor - allows you to create and editor animation blending state machines which are responsible for 
animation mixing.
- Curve Editor - create and edit curve resources to make complex laws for game parameters.
- Path Fixer - helps you fix incorrect resource references in your scenes.