# Bots and AI (WIP)

**WORK IN PROGRESS - DO NOT USE**

In this tutorial we'll add bots and a simple AI system to our 2D platformer. 

## Bot Prefab

Let's start by creating a prefab for our bots. At first, we need a sprite sheet for the bot, we'll use 
[this one](https://astrobob.itch.io/animated-pixel-art-skeleton). It contains attack, hit, death, walk, idle animations 
exactly what we need:

![skeleton](skeleton.png)

Open the editor and create a new scene, right-lick on the `__ROOT__` scene node and do `Replace With -> Physics 2D -> Rigid Body`.
Rename this node to `Skeleton` and then create a `Rectangle` child node by right-clicking on the `Skeleton` node and doing
`Create Child -> 2D -> Rectangle`, select the new rectangle node and set its scale to `2.0, 2.0, 1.0` (default scale of 1.0 is too
small and the skeleton will be half of the height of our player). Now let's apply the texture to the rectangle, find 
`skeleton.png` in the asset browser, select it, set its properties like on the screenshot below - all filtration modes 
to `Nearest` (to make its pixels sharp, not blurry) and wrapping to `Clamp To Edge` (to prevent potential seams on the 
edges). Find the `Material` property in the inspector and open the material editor, drag the `skeleton.png` texture from 
the asset browser to `diffuseTexture` property in the material editor. Set the `UV Rect -> Size` property to `0.077; 0.2` 
and you should see something similar to this:

![skeleton prefab](skeleton_prefab.png)

If you look closely at the world viewer, you should notice a small warning sign near the rigid body - the editor tells 
us that we've forgotten to add a collider to the rigid body. Let's fix this by right-clicking on the rigid body, then 
select `Create Child -> Physics 2D -> Collider`. Select the collider and set its shape to `Capsule` in the properties 
like so:

![capsule](capsule.png)

We're almost finished with our prefab, the last step is to configure properties of the rigid body properly. Currently, we
have a simple rigid body, that will rotate freely during collisions and will also "sleep" on inactivity. Let's fix this
by selecting the rigid body in the inspector and disable rotational movement and prevent it from sleeping:

![rigid body](rigid_body.png)

## Script

Now onto code part, run the following command in the root folder of your game: `fyrox-template script --name=bot` and add
the `mod bot;` line at the beginning of `lib.rs` of the `game` package. The code for the script will look something like 
this:

```rust,no_run
{{#include ../../code/tutorials/platformer/game/src/residuals.rs:bot_stub_script}}
```

Register the script by adding `script_constructors.add::<Bot>("Bot");` line near the `script_constructors.add::<Player>("Player");`
line in `lib.rs` (as we did in the previous part of the tutorial).

### Patrol

By default, when there's no target nearby the bot will patrol in available bounds. Basically, it will walk from one "wall"
to another. Add the following fields to the `Bot` script:

```rust
{{#include ../../code/tutorials/platformer/game/src/bot.rs:movement_fields}}
```

`speed` field will define overall movement speed of the bot and `direction` will be used to alternate movement direction
along X axis.

### Target Searching

When the bot is patrolling, it will search for a target to attack. Bots will be able to attack only the player, so we just 
need to check if the player is in front of a bot and close enough to it. We need a way to get player's handle, we could just
iterate over the scene and search for it at every frame, but that's inefficient and there's a better way. All we need to 
do is to slightly modify the plugin and the player script. Add the following field to the plugin:

```rust
{{#include ../../code/tutorials/platformer/game/src/lib.rs:player_field}}
```

Now we need to set this handle somehow, the ideal place for it is `on_start` method of the `Player` script:

```rust
{{#include ../../code/tutorials/platformer/game/src/lib.rs:set_player_field}}
```

Great, now when the player script is created and initialized, it will register itself in the plugin. Now we can use this handle
in the bot's target searching routine. Add the following code to the `impl Bot`:

```rust
{{#include ../../code/tutorials/platformer/game/src/bot.rs:search_target}}
```

This code is very straightforward - at first, we're fetching a reference to the plugin (in which we've just stored player's
handle). Then we're getting self position of the bot and player's position. Finally, to check if the bot can "see" the 
player we're calculating horizontal distance between the player and the bot, checking its absolute value to be less than 
some sensible threshold and also checking the sign of the distance. If the sign of the distance equals with the sign of 
the direction, the bot can see the player. As the last step, call this method in the `on_update` method:

```rust
{{#include ../../code/tutorials/platformer/game/src/bot.rs:search_target_call}}
```

### Target Following

If there's a target, then the bot will follow it and try to attack when it is close enough.

### Ground Checks

At this moment, our bot can move and follow targets, but it can easily fall of into "abyss" and die. Let's prevent that 
by adding ground check, that will be used to switch movement direction also. How will we check for ground presence anyway?
We'll do this using simple ray casting. At first, add the following fields to the bot script:

```rust
{{#include ../../code/tutorials/platformer/game/src/bot.rs:ground_probe_fields}}
```

`ground_probe` field will be used to store a handle of a point scene node, that will be used as a starting point for ray
casting. `ground_probe_distance` field is used to define maximum distance, after which ray casting considered failed.
Now add the following code in the `impl Bot`:

```rust
{{#include ../../code/tutorials/platformer/game/src/bot.rs:has_ground_in_front}}
```

Open the skeleton prefab and create the ground probe like so:

![ground probe](ground_probe.png)

Do not forget to assign its handle to the bot script as well.

## Animations

Our bot can patrol, search and attack targets, but all of this is not properly visualized since we're not using any animations
for such actions. Let's fix this. Add the following fields to the `Bot` structure:

```rust
{{#include ../../code/tutorials/platformer/game/src/bot.rs:animation_fields}}
```

As with the player from the previous tutorial, we'll use sprite sheet animations. Open the bot prefab and select the rigid
body, add five animations and fill every slot. For example, attack animation will look like this:

![attack animation](attack_animation.png)