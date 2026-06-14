# 6. Assets, Camera, And UI

<div align="center">

[Index](index.md) · [← Previous: Bundles, plugins, and sets](05-bundles-plugins-sets.md) · [Next: RPG foundation slice →](07-rpg-slice.md)

</div>

---

## Outcome

At the end of this chapter, the player uses a real image asset, the camera follows the player, and a world-space label displays the player's position.

![A sprite asset, a camera-following view, and world-space HUD text.](../../assets/screenshots/ch06-assets-camera-ui.png)

## Run

```sh
cargo run --example 06_assets_camera_ui
```

Move with WASD or arrow keys. The player stays centered because the camera follows the player.

## Build Step 1: Load A Sprite With `AssetServer`

The player bundle receives the asset server:

```rust
impl PlayerBundle {
    fn new(asset_server: &AssetServer) -> Self {
        Self {
            player: Player,
            sprite: Sprite::from_image(asset_server.load("player.png")),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
        }
    }
}
```

`asset_server.load("player.png")` returns a handle. The handle is cheap to clone and store. Bevy loads the actual image through the asset system.

The path is relative to the `assets/` directory:

```text
assets/player.png -> asset_server.load("player.png")
```

## Build Step 2: Spawn A Background

The example adds a large colored sprite behind the player:

```rust
commands.spawn((
    Sprite::from_color(Color::srgb(0.18, 0.22, 0.28), Vec2::new(900.0, 540.0)),
    Transform::from_xyz(0.0, 0.0, 0.0),
));
```

The player is at `z = 1.0`, so it draws above the background at `z = 0.0`.

In 2D, larger `z` generally means drawn later and therefore visually on top.

## Build Step 3: Use `Single` For The Player

The movement system expects exactly one player:

```rust
fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    player.translation +=
        (direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs()).extend(0.0);
}
```

`Single` is a clear statement of intent. This example is not written for a party of players. It is written for one player entity.

## Build Step 4: Follow With The Camera

The camera follow system uses two `Single` queries:

```rust
fn follow_player(
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}
```

`Without<Camera2d>` keeps the player query separate from the camera query. Both deal with `Transform`, so filters matter.

The system copies only `x` and `y`. The camera keeps its own `z`.

## Build Step 5: Add World-Space Text

The position label is a world entity:

```rust
commands.spawn((
    HudText,
    Text2d::new("Position: 0, 0"),
    TextFont::from_font_size(24.0),
    TextColor(Color::srgb(0.86, 0.91, 0.98)),
    Transform::from_xyz(0.0, 230.0, 2.0),
));
```

The text is positioned with `Transform`, so it lives in the game world. The example moves it near the player each frame:

```rust
hud.translation.x = player.translation.x;
hud.translation.y = player.translation.y + 230.0;
```

Chapter 12 will build screen-space UI that stays fixed to the window instead.

## Rust Lens

This constructor borrows `AssetServer`:

```rust
fn new(asset_server: &AssetServer) -> Self
```

The bundle does not own the asset server. It only uses it to request a handle.

`format!` creates a `String`:

```rust
hud.0 = format!(
    "Position: {:.0}, {:.0}",
    player.translation.x, player.translation.y
);
```

`{:.0}` formats the number with zero decimal places.

## Bevy Lens

There are two coordinate spaces in this chapter:

```text
World space    Sprite, Transform, Text2d, camera movement
Screen space   Node, Text, fixed HUD, menus
```

`Text2d` is world-space text. `Text` plus `Node` is UI text. You will use both in the RPG.

## Check

Run:

```sh
cargo run --example 06_assets_camera_ui
```

Expected result:

- The player image appears, not a colored square.
- The camera snaps to the player position as you move.
- The position text follows the player and updates its numbers.

## Change

In `follow_player`, add an offset:

```rust
camera.translation.x = player.translation.x + 120.0;
camera.translation.y = player.translation.y + 60.0;
```

Expected result: the player no longer sits exactly in the center. The camera tracks a point offset from the player.

---

<div align="center">

[← Previous: Bundles, plugins, and sets](05-bundles-plugins-sets.md) · [Index](index.md) · [Next: RPG foundation slice →](07-rpg-slice.md)

</div>
