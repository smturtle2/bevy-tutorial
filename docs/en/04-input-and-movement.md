# 4. Input And Movement


<div align="center">

[Index](index.md) · [← Previous: ECS fundamentals](03-ecs-fundamentals.md) · [Next: Bundles, plugins, and sets →](05-bundles-plugins-sets.md)

</div>

---

This chapter turns a static sprite into a controllable entity, then refactors movement into a more reusable ECS shape.

## Walkthrough: `03_player_input`

Run:

```sh
cargo run --example 03_player_input
```

Use WASD or arrow keys. The blue square should move.

The example introduces a marker component and a resource:

```rust
#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PlayerSpeed(f32);
```

`Player` marks which entity receives input. `PlayerSpeed` stores one shared speed value:

```rust
.insert_resource(PlayerSpeed(280.0))
```

The setup system attaches `Player`, `Sprite`, and `Transform` to one entity:

```rust
commands.spawn((
    Player,
    Sprite::from_color(Color::srgb(0.25, 0.70, 1.0), Vec2::splat(80.0)),
    Transform::from_translation(Vec3::ZERO),
));
```

## Direct Movement System

The movement system reads input and directly mutates position:

```rust
fn move_player(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    speed: Res<PlayerSpeed>,
    mut players: Query<&mut Transform, With<Player>>,
) {
    let mut direction = Vec2::ZERO;
    // input checks...
    let movement = direction.normalize_or_zero() * speed.0 * time.delta_secs();

    for mut transform in &mut players {
        transform.translation += movement.extend(0.0);
    }
}
```

Read the signature as the system contract:

```text
Reads frame time.
Reads keyboard state.
Reads player speed.
Mutates Transform on entities with Player.
```

This is a good first version because every piece is visible in one place.

## Input Resource

`ButtonInput<KeyCode>` is a Bevy resource. It stores keyboard button state for the current frame:

```rust
keyboard.pressed(KeyCode::ArrowLeft)
keyboard.pressed(KeyCode::KeyA)
```

`pressed` stays true while the key is held. Bevy also has edge-style checks such as "just pressed", but held movement should use `pressed`.

## Direction Normalization

The system accumulates a direction vector:

```rust
let mut direction = Vec2::ZERO;
direction.x -= 1.0;
direction.y += 1.0;
```

Diagonal input produces a vector like `(-1.0, 1.0)`, which is longer than a horizontal vector. Normalize before applying speed:

```rust
direction.normalize_or_zero()
```

`normalize_or_zero` avoids invalid math when no keys are pressed and the vector is zero.

## Frame Independence

Movement uses delta time:

```rust
let movement = direction.normalize_or_zero() * speed.0 * time.delta_secs();
```

`time.delta_secs()` is the duration of the previous frame in seconds. Multiplying by it makes movement roughly "units per second" instead of "units per frame."

`Transform.translation` is a `Vec3`, while movement is a `Vec2`, so the code extends it:

```rust
transform.translation += movement.extend(0.0);
```

The new `z` value is `0.0`.

## Why Refactor?

The direct system couples keyboard input to position changes:

```text
keyboard -> Transform
```

That works for a single player square. It becomes limiting when enemies, knockback, scripted movement, or physics also need to move entities.

The next example splits intent from motion:

```text
input system    -> writes Velocity
movement system -> reads Velocity, writes Transform
```

## Walkthrough: `04_velocity_body`

Run:

```sh
cargo run --example 04_velocity_body
```

The controls feel similar, but the data model is different:

```rust
#[derive(Component)]
struct Body;

#[derive(Component)]
struct Velocity(Vec2);
```

`Body` marks entities that can be moved by the movement system. `Velocity` stores their movement direction.

The input system no longer touches `Transform`:

```rust
fn handle_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Velocity, With<Player>>,
) {
    // calculate direction...

    for mut velocity in &mut players {
        velocity.0 = direction.normalize_or_zero();
    }
}
```

The movement system no longer knows about keyboard input:

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

This is the first major ECS design step in the tutorial. Systems communicate through component data instead of directly calling each other.

## Ordering With `.chain()`

`04_velocity_body` registers the systems like this:

```rust
.add_systems(Update, (handle_player_input, move_bodies).chain())
```

The order matters:

```text
handle_player_input writes Velocity
move_bodies reads Velocity
```

`.chain()` tells Bevy to run them in that order. Without ordering, Bevy is free to run compatible systems in an order chosen by the scheduler.

Later, when systems live in different plugins, the tutorial switches to `SystemSet`.

## Exercise

In a local experiment:

1. Change `BodySpeed(220.0)` to `BodySpeed(80.0)`.
2. Remove `.chain()` and observe whether movement still appears correct.
3. Add a second spawned `PlayerBundle::new()` and think about what `Query<&mut Velocity, With<Player>>` will do.

The important question is not only "does it move?" The important question is "which entities does each system affect?"

## Common Mistakes

- Storing frame-scaled movement in `Velocity`, then multiplying by delta time again later.
- Forgetting to normalize diagonal input.
- Making `move_bodies` filter with `With<Player>`, which prevents enemies or other bodies from reusing it.
- Assuming systems run in registration order without `.chain()`, explicit ordering, or sets.

---

<div align="center">

[← Previous: ECS fundamentals](03-ecs-fundamentals.md) · [Index](index.md) · [Next: Bundles, plugins, and sets →](05-bundles-plugins-sets.md)

</div>
