# 13. Animation State


<div align="center">

[Index](index.md) · [← Previous: Screen-space UI](12-screen-space-ui.md) · [Next: Handmade map geometry →](14-handmade-map-geometry.md)

</div>

---

Run:

```sh
cargo run --example 13_animation_state
```

![The animation state example shows the Attack state, atlas frame 3, and the sprite-sheet frames used by the player animation system.](../../assets/screenshots/ch13-animation-attack.png)

## Contract

Animation is gameplay presentation state. It should be explicit data, not hidden in input code.

```text
PlayerAnimState = Idle | Run | Attack
PlayerAnimation = current state + timers + frame index
Sprite atlas    = image handle + atlas layout + frame index
```

## Data Contract

Input updates velocity and requested animation state. The animation system updates the sprite frame.

```text
player_input   writes Velocity and PlayerAnimation.state
animate_player writes Sprite.texture_atlas.index
```

## Rust Point

`PlayerAnimState` is an `enum`. Rust enums are useful when exactly one of several named states is valid. The compiler forces each `match` arm to consider every variant.

## Bevy Point

The example creates a `TextureAtlasLayout` from a 4-frame sprite sheet:

```rust
TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None)
```

The frame is selected by changing `atlas.index`.

## Frame Flow

```text
input     decide Idle/Run/Attack
animate   tick timers and choose sprite frame
label     show current animation state
```

Attack has its own timer so the attack frame is visible even if the player releases the key immediately.

## Common Mistakes

- Treating animation state as the same thing as game state.
- Updating atlas frames in the input system.
- Use a timer to exit the attack state.

## Change It

- Add a longer attack timer.
- Add a third run frame.
- Flip the sprite when moving left.

---

<div align="center">

[← Previous: Screen-space UI](12-screen-space-ui.md) · [Index](index.md) · [Next: Handmade map geometry →](14-handmade-map-geometry.md)

</div>
