# 6. Assets, Camera, And UI


<div align="center">

[Index](index.md) · [← Previous: Bundles, plugins, and sets](05-bundles-plugins-sets.md) · [Next: RPG foundation slice →](07-rpg-slice.md)

</div>

---

The early examples use colored sprites because they keep asset loading out of the first ECS lessons. This chapter adds three common presentation features while keeping the game logic small:

- load an image through `AssetServer`
- follow the player with a camera
- display world-space HUD text with `Text2d`

Run:

```sh
cargo run --example 06_assets_camera_ui
```

![The assets, camera, and UI example shows an image sprite, a world-space HUD label, and a camera-following scene.](../../assets/screenshots/ch06-assets-camera-ui.png)

You should see an image-based player, a large background rectangle, and text that follows above the player while reporting position.

## Walkthrough: `06_assets_camera_ui`

The example has two marker components:

```rust
#[derive(Component)]
struct Player;

#[derive(Component)]
struct HudText;
```

`Player` marks the movable sprite. `HudText` marks the text entity that displays the player's position.

The app registers one startup system and four ordered update systems:

```rust
.add_systems(Startup, setup)
.add_systems(
    Update,
    (
        move_player,
        follow_player,
        update_hud_text,
        position_hud_text,
    )
        .chain(),
)
```

The order is intentional:

```text
move_player      changes the player Transform
follow_player    moves the camera to the new player position
update_hud_text  updates the text contents
position_hud_text moves the text relative to the player
```

Without `.chain()`, Bevy may run compatible systems in another order. For this example, the HUD and camera should use the latest player position, so the chain is part of the behavior.

## Loading A Sprite With `AssetServer`

The setup system asks for `AssetServer` as a resource:

```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle::new(&asset_server));
}
```

The bundle owns the spawn shape:

```rust
#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    sprite: Sprite,
    transform: Transform,
}

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

By default, Bevy resolves asset paths under the repository's `assets/` directory. This code loads:

```text
assets/player.png
```

`asset_server.load("player.png")` returns a handle. Asset loading can complete asynchronously; systems normally store and use handles instead of manually reading image files.

Use the two sprite constructors for different stages:

```rust
Sprite::from_color(color, size)        // fast prototype visual
Sprite::from_image(asset_server.load("player.png")) // image asset
```

The example still uses a colored sprite for the background:

```rust
Sprite::from_color(Color::srgb(0.18, 0.22, 0.28), Vec2::new(900.0, 540.0))
```

That is a normal mix: assets where identity matters, colored primitives where a simple shape is enough.

## `Single` For Exactly One Player

The movement system uses `Single`:

```rust
fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    // ...
}
```

`Single<&mut Transform, With<Player>>` means:

```text
There must be exactly one entity with Player and Transform.
Give this system mutable access to that Transform.
```

That is a stronger contract than `Query<&mut Transform, With<Player>>`, which can match zero, one, or many entities. Use `Single` when the example or feature really does require exactly one match.

## Camera Follow

The camera is just another entity:

```rust
commands.spawn(Camera2d);
```

The follow system copies the player's x/y position into the camera:

```rust
fn follow_player(
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}
```

`Without<Camera2d>` tells Bevy that the player query excludes the camera entity. This matters because both parameters access `Transform`, and one of them is mutable.

The camera's z value is left alone. In 2D, x/y controls where the camera looks, while z and projection settings control how the view is rendered.

## World-Space HUD With `Text2d`

The HUD text is spawned as a normal ECS entity:

```rust
commands.spawn((
    HudText,
    Text2d::new("Position: 0, 0"),
    TextFont::from_font_size(24.0),
    TextColor(Color::srgb(0.86, 0.91, 0.98)),
    Transform::from_xyz(0.0, 230.0, 2.0),
));
```

This is not screen-space UI. It is 2D text in the world. Two systems keep it useful: one updates the text content, and one moves the text above the player.

```rust
fn update_hud_text(
    player: Single<&Transform, With<Player>>,
    mut hud: Single<&mut Text2d, With<HudText>>,
) {
    hud.0 = format!(
        "Position: {:.0}, {:.0}",
        player.translation.x, player.translation.y
    );
}

fn position_hud_text(
    player: Single<&Transform, (With<Player>, Without<HudText>)>,
    mut hud: Single<&mut Transform, (With<HudText>, Without<Player>)>,
) {
    hud.translation.x = player.translation.x;
    hud.translation.y = player.translation.y + 230.0;
}
```

The `Without` filters are not decorative. `position_hud_text` reads a player `Transform` and mutates the HUD `Transform`; the filters prove to Bevy that those two queries cannot access the same entity.

`Text2d` is a tuple struct, so the string is stored in `text.0`.

The text transform is also updated. Since the camera follows the player, this keeps the HUD-like text visually near the top of the view.

## Exercise

Try these small changes:

1. Change the HUD offset from `+ 230.0` to `+ 120.0`.
2. Remove `.chain()` and think about which systems might read the previous frame's position.
3. Replace `Sprite::from_image(...)` with `Sprite::from_color(...)` and confirm that movement, camera follow, and HUD text do not depend on the asset.

## Common Mistakes

- Using `"assets/player.png"` instead of `"player.png"` with `AssetServer::load`.
- Forgetting the camera and seeing nothing.
- Using `Single` when zero or multiple entities are valid for that moment.
- Creating two mutable queries over `Transform` without filters that prove they cannot match the same entity.
- Expecting `Text2d` to behave like fixed screen-space UI. In this example it is world-space text.

---

<div align="center">

[← Previous: Bundles, plugins, and sets](05-bundles-plugins-sets.md) · [Index](index.md) · [Next: RPG foundation slice →](07-rpg-slice.md)

</div>
