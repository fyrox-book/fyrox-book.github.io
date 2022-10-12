# Saved Games (WIP)

Saved game is used to store progress made in a play-through of a game to disk or some other storage. It is very important
for pretty much every game and this chapter will help you to understand basic concepts of saved games in the engine.

## Saved Game Structure

This could sound weird, but saved game in most cases is just a scene with additional data. Let's understand why. At first,
when you're making a save file you need to take a "snapshot" of your game world. Essential way of storing such data is
a scene. Secondly, game plugins is also may store some data that should be saved. By these two facts, it is quite easy
to get a full picture: to make a save all you need to do is to serialize current scene, serialize some other data and 
just "dump" it to a file. You might ask: is this efficient to serialize the entire scene? In short: yes. A bit more
detailed answer: when you serialize a scene, it does not store everything, it only stores _changed_ fields and references
to external assets.

To load a save file you need to do pretty much the same, but instead of serializing things, you need to deserialize 
data from file into a scene and restore data for plugins. So how the engine is able to restore the data on load if it
does not store everything? If you carefully read the book, you might already know the answer -
[property inheritance](/src/fyrox/scene/inheritance.md). 


