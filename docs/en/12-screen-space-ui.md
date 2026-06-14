# 12. Screen-Space UI


<div align="center">

[Index](index.md) · [← Previous: Sprite assets](11-sprite-assets.md) · [Next: Animation state →](13-animation-state.md)

</div>

---

Run:

```sh
cargo run --example 12_screen_space_ui
```

![The screen-space UI example shows fixed HUD text and a health bar over a camera-following world scene.](../../assets/screenshots/ch12-screen-space-ui.png)

## Contract

World-space text lives in the game world. Screen-space UI is fixed to the screen.

```text
Text2d + Transform = world-space text
Text + Node        = screen-space UI
```

This chapter uses `Text`, `Node`, `TextFont`, `TextColor`, and `BackgroundColor`.

## Data Contract

The player owns `Health`. `Score` is a resource. HUD entities are marked with components such as `HealthText`, `ScoreText`, and `HealthBarFill`.

The UI system reads gameplay data and writes UI components:

```text
read Health + Score
write Text + Node width
```

## Rust Point

The UI system uses separate marker components and `Without` filters for distinct text entities. That keeps Bevy's borrow rules clear when one system mutates several `Text` components.

## Bevy Point

`Node` uses UI layout values:

```rust
Node {
    position_type: PositionType::Absolute,
    top: px(82),
    left: px(16),
    width: px(200),
    height: px(16),
    ..default()
}
```

These values are screen UI layout, not world coordinates.

## Frame Flow

```text
move_player               changes world position
follow_player_with_camera moves camera
debug_change_stats        changes Health/Score
update_screen_space_ui    writes fixed HUD
```

The HUD stays fixed even while the camera follows the player.

## Common Mistakes

- Using `Text2d` for health bars that should stay on screen.
- Moving HUD entities with the camera.
- Mixing UI `Node` positions with world `Transform` positions.

## Change It

- Move the HUD to the top right.
- Add a stamina bar.
- Change the health bar color when health is low.

---

<div align="center">

[← Previous: Sprite assets](11-sprite-assets.md) · [Index](index.md) · [Next: Animation state →](13-animation-state.md)

</div>
