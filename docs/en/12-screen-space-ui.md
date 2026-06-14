# 12. Screen-Space UI

<div align="center">

[Index](index.md) · [← Previous: Sprite assets](11-sprite-assets.md) · [Next: Animation state →](13-animation-state.md)

</div>

---

## Outcome

At the end of this chapter, health and score UI stay fixed to the screen while the camera follows the player through the world.

![Screen-space UI stays fixed while the player and camera move.](../../assets/screenshots/ch12-screen-space-ui.png)

## Run

```sh
cargo run --example 12_screen_space_ui
```

Move with WASD/arrows. Press `H` and `J` to change health. Press Space to increase score.

## Build Step 1: Mark The Main Camera

The camera receives a marker:

```rust
#[derive(Component)]
struct MainCamera;

commands.spawn((Camera2d, MainCamera));
```

The marker lets camera systems query the game camera without confusing it with other entities.

## Build Step 2: Spawn UI Text With `Node`

Screen-space UI uses `Text` plus layout components:

```rust
commands.spawn((
    HealthText,
    Text::new("Health: 5/5"),
    TextFont::from_font_size(24.0),
    TextColor(Color::srgb(0.94, 0.97, 1.0)),
    Node {
        position_type: PositionType::Absolute,
        top: px(48),
        left: px(16),
        ..default()
    },
));
```

This entity has no `Transform`. Its position is UI layout data, not world position data.

## Build Step 3: Build A UI Health Bar

The background bar and fill bar are UI nodes:

```rust
commands.spawn((
    Node {
        position_type: PositionType::Absolute,
        top: px(82),
        left: px(16),
        width: px(200),
        height: px(16),
        ..default()
    },
    BackgroundColor(Color::srgb(0.22, 0.24, 0.30)),
));
```

The fill has a marker:

```rust
#[derive(Component)]
struct HealthBarFill;
```

That marker lets the update system mutate only the fill width.

## Build Step 4: Prove It Is Screen-Space

The camera follows the player:

```rust
fn follow_player_with_camera(
    player: Single<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}
```

The UI stays fixed because it is laid out in screen space through `Node`.

World-space text from chapter 6 moved with `Transform`. Screen-space text ignores camera movement.

## Build Step 5: Update UI From Resources And Components

The debug system changes score and health:

```rust
if keyboard.just_pressed(KeyCode::Space) {
    score.0 += 1;
}

if keyboard.just_pressed(KeyCode::KeyH) {
    player.current = (player.current - 1).max(0);
}
```

The UI system reads the data and writes text/node width:

```rust
fn update_screen_space_ui(
    score: Res<Score>,
    player: Single<&Health, With<Player>>,
    mut health_text: Single<&mut Text, (With<HealthText>, Without<ScoreText>)>,
    mut score_text: Single<&mut Text, (With<ScoreText>, Without<HealthText>)>,
    mut health_bar: Single<&mut Node, With<HealthBarFill>>,
) {
    let health = *player;
    let health_fraction = health.current as f32 / health.max as f32;

    health_text.0 = format!("Health: {}/{}", health.current, health.max);
    score_text.0 = format!("Score: {}", score.0);
    health_bar.width = px(200.0 * health_fraction);
}
```

## Rust Lens

`as f32` converts integer health into a float for division:

```rust
let health_fraction = health.current as f32 / health.max as f32;
```

Without conversion, integer division would not produce the fractional bar width you want.

The filters in the text queries avoid ambiguity:

```rust
(With<HealthText>, Without<ScoreText>)
```

They tell Bevy the two mutable text queries target different entities.

## Bevy Lens

Use this split:

```text
Text2d + Transform    label in the game world
Text + Node           fixed interface on the screen
Sprite + Transform    rendered world object
Node + BackgroundColor UI rectangle
```

Most RPG HUDs should use screen-space UI. Floating damage numbers and nameplates often use world-space text.

## Check

Run:

```sh
cargo run --example 12_screen_space_ui
```

Expected result:

- Moving the player moves the camera.
- The HUD stays in the top-left corner.
- `H` lowers health and shrinks the bar.
- `J` restores health.
- Space increases score.

## Change

Move the HUD to the right:

```rust
left: px(16),
```

to:

```rust
right: px(16),
```

Expected result: the UI pins to the right side of the screen. Apply the same change to each HUD node you want aligned together.

---

<div align="center">

[← Previous: Sprite assets](11-sprite-assets.md) · [Index](index.md) · [Next: Animation state →](13-animation-state.md)

</div>
