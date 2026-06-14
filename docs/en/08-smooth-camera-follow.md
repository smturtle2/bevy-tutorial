# 8. Smooth Camera Follow


<div align="center">

[Index](index.md) · [← Previous: RPG foundation slice](07-rpg-slice.md) · [Next: Enemy waves →](09-enemy-waves.md)

</div>

---

Run:

```sh
cargo run --example 08_smooth_camera_follow
```

![The smooth camera follow example shows the player inside a larger grid world while the camera eases toward the target entity.](../../assets/screenshots/ch08-smooth-camera-follow.png)

## Contract

The camera is an entity. It should not be treated as a hidden global viewport. In this chapter the camera has a `CameraFollow` component:

```text
CameraFollow = target entity + offset + smoothness
```

The player owns its own `Transform`. The camera reads that transform and moves toward it.

## Data Contract

```text
Player       marks the target entity
CameraFollow stores which entity to follow and how strongly
Transform    stores positions for both player and camera
```

`CameraFollow.target` is an `Entity`. That is the Rust value Bevy uses as an ID for an entity in the world.

## Rust Point

The camera system uses `let Ok(target_transform) = targets.get(follow.target) else { continue; };`.

That is Rust pattern matching for a fallible lookup. Entity lifetimes can change during gameplay, so the system gives the missing-target case an explicit branch and keeps the frame running.

## Bevy Point

The smoothing line is:

```rust
let blend = 1.0 - (-follow.smoothness * time.delta_secs()).exp();
camera_transform.translation = camera_transform.translation.lerp(target, blend);
```

`lerp` means linear interpolation. A higher `smoothness` moves the camera closer to the target each frame. The exponential form makes the feel stable across different frame rates.

## Frame Flow

```text
move_player          writes Player Transform
smooth_follow_camera reads Player Transform, writes Camera Transform
```

The systems are chained because the camera should follow the latest player position.

## Common Mistakes

- Copying the player position directly gives a hard camera snap, not follow smoothing.
- Storing the target as a resource is less flexible than a component when several cameras may exist.
- Use `Query::get` for target lookups so the system has a clear path when the target entity is gone.

## Change It

- Change `CAMERA_SMOOTHNESS` to `2.0`, then `20.0`.
- Add an offset such as `Vec3::new(120.0, 0.0, 0.0)`.
- Clamp the camera to the map bounds instead of clamping only the player.

---

<div align="center">

[← Previous: RPG foundation slice](07-rpg-slice.md) · [Index](index.md) · [Next: Enemy waves →](09-enemy-waves.md)

</div>
