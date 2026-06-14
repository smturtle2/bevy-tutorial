# 14. Handmade Map Geometry

[Index](index.md) | Previous: [Animation state](13-animation-state.md) | Next: [Game states](15-game-states.md)

Run:

```sh
cargo run --example 14_handmade_map_geometry
```

## Contract

You do not need a tilemap crate to learn map structure. This chapter builds a map from entities.

```text
floor tile = visual sprite only
wall       = Wall + Body + Transform + Sprite
player     = Player + Body + Velocity + Transform
```

Collision belongs to wall entities, not to the background art.

## Data Contract

`WallBundle::new(position, size)` creates rectangular wall geometry. The resolver compares the player's `Body` to each wall `Body` and pushes the player out on the shallowest axis.

## Rust Point

The wall list is an array of `(Vec3, Vec2)` tuples. That is enough for fixed tutorial geometry. A later real level format could replace this with loaded data.

## Bevy Point

The resolver reads wall transforms and writes the player transform. That means it must run after movement:

```text
Input -> Movement -> Collision
```

The example uses `SystemSet` to make that order visible.

## Common Mistakes

- Making every floor tile collide.
- Resolving collision before movement.
- Mixing decorative map pieces and blocking map pieces under the same marker.

## Change It

- Add another wall rectangle.
- Increase player size and observe collision behavior.
- Add an exit zone as a non-wall trigger.
