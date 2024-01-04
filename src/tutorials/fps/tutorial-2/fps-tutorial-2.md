# Weapons (WIP)

In the [previous tutorial](../tutorial-1/fps-tutorial.md) we've added basic character controller, but what is a 
first-person shooter without weapons? Let's add them. As usual, we need a script that will "drive" our weapons:

```shell
fyrox-template script --name=weapon
```

Add the `weapon` mod to the `lib.rs` module using `pub mod weapon;`.

## Conclusion

In this tutorial part we've added weapons that can shoot projectiles, which in their turn can interact with the 
environment.