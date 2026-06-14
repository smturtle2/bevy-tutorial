# 4. Input And Movement

<div align="center">

[Index](index.md) · [← Previous: ECS fundamentals](03-ecs-fundamentals.md) · [Next: Bundles, plugins, and sets →](05-bundles-plugins-sets.md)

</div>

---

## Outcome

At the end of this chapter, keyboard input moves the player in two versions:

1. Direct movement writes `Transform`.
2. Refactored movement writes `Velocity`, then a body movement system writes `Transform`.

That refactor is the first step toward reusable gameplay code.

![Player input writes velocity, and body movement writes transform.](../../assets/screenshots/ch04-velocity-body.png)

## Run

```sh
cargo run --example 03_player_input
cargo run --example 04_velocity_body
```

Use WASD or arrow keys in both examples.

## Build Step 1: Read Keyboard State

The input resource is:

```rust
Res<ButtonInput<KeyCode>>
```

The direct movement example builds a direction vector:

```rust
let mut direction = Vec2::ZERO;

if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
    direction.x -= 1.0;
}
if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
    direction.x += 1.0;
}
if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
    direction.y -= 1.0;
}
if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
    direction.y += 1.0;
}
```

`pressed` stays true while the key is held. Later, attacks and menus use `just_pressed`, which is true only on the frame the key changed from up to down.

## Build Step 2: Normalize Diagonal Movement

Without normalization, diagonal movement is faster because `(1, 1)` has a longer length than `(1, 0)`.

```rust
let movement = direction.normalize_or_zero() * speed.0 * time.delta_secs();
```

`normalize_or_zero()` keeps diagonal speed equal and safely returns zero when the direction is zero.

## Build Step 3: Make Movement Frame Independent

`time.delta_secs()` is the number of seconds since the previous frame:

```rust
transform.translation += movement.extend(0.0);
```

The formula is:

```text
pixels this frame = direction * pixels per second * seconds this frame
```

That keeps movement roughly consistent across different frame rates.

## Build Step 4: Move Through `Transform`

The direct version queries player transforms:

```rust
fn move_player(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    speed: Res<PlayerSpeed>,
    mut players: Query<&mut Transform, With<Player>>,
) {
    let movement = direction.normalize_or_zero() * speed.0 * time.delta_secs();

    for mut transform in &mut players {
        transform.translation += movement.extend(0.0);
    }
}
```

This is simple and correct for one player. The downside is that input and movement physics live in the same system.

## Build Step 5: Introduce `Velocity`

The refactored example adds movement data:

```rust
#[derive(Component)]
struct Velocity(Vec2);
```

Input now writes intent:

```rust
fn handle_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in &mut players {
        velocity.0 = direction.normalize_or_zero();
    }
}
```

Movement applies velocity:

```rust
fn move_bodies(
    time: Res<Time>,
    speed: Res<BodySpeed>,
    mut bodies: Query<(&mut Transform, &Velocity), With<Body>>,
) {
    let movement_scale = speed.0 * time.delta_secs();

    for (mut transform, velocity) in &mut bodies {
        transform.translation += (velocity.0 * movement_scale).extend(0.0);
    }
}
```

Now enemies, projectiles, and NPCs can all move with the same `move_bodies` system if they have `Body`, `Velocity`, and `Transform`.

## Build Step 6: Chain The Order

The refactor needs input before movement:

```rust
.add_systems(
    Update,
    (handle_player_input, move_bodies, capture_velocity_body_pose).chain(),
)
```

`.chain()` makes those systems run in tuple order.

Without an order, Bevy may run systems in any valid order. That is good for parallelism, but gameplay code must declare dependencies when one system writes data another system reads in the same frame.

## Rust Lens

This line is doing three Rust things:

```rust
for mut transform in &mut players {
```

```text
&mut players       borrow the query mutably for iteration
transform          local binding for each matched Transform
mut transform      this binding can write through the mutable reference
```

This tuple loop mirrors query data:

```rust
for (mut transform, velocity) in &mut bodies {
```

The first item is mutable because the query asked for `&mut Transform`. The second is read-only because the query asked for `&Velocity`.

## Bevy Lens

The design improvement is responsibility separation:

```text
Player input system     keyboard -> Velocity
Body movement system    Velocity + Time -> Transform
```

This is why `update_players` is usually too vague as a system name. A system name should say which behavior it owns: `handle_player_input`, `move_bodies`, `enemy_ai`, `collect_items`, `update_hud_text`.

## Check

Run both examples:

```sh
cargo run --example 03_player_input
cargo run --example 04_velocity_body
```

Expected result:

- Both examples move with WASD/arrows.
- Diagonal movement does not visibly outrun straight movement.
- In `04_velocity_body`, the player still moves even though input no longer touches `Transform` directly.

## Change

In `examples/04_velocity_body.rs`, change:

```rust
.insert_resource(BodySpeed(220.0))
```

to:

```rust
.insert_resource(BodySpeed(80.0))
```

Expected result: the player moves slower without changing the input system. That proves speed belongs to body movement, not keyboard reading.

---

<div align="center">

[← Previous: ECS fundamentals](03-ecs-fundamentals.md) · [Index](index.md) · [Next: Bundles, plugins, and sets →](05-bundles-plugins-sets.md)

</div>
