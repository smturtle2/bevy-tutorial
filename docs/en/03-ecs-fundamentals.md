# 3. ECS Fundamentals

[Index](index.md) | Previous: [The Bevy app model](02-bevy-app-model.md) | Next: [Input and movement](04-input-and-movement.md)

ECS means Entity Component System.

![ECS overview](../../assets/diagrams/ecs-overview.png)

This chapter explains the vocabulary you need before the movement examples become interesting.

## Entity

An entity is an ID in the world.

It does not inherently have position, health, velocity, rendering, or behavior. It only has the components you attach to it.

In `examples/02_spawn_sprite.rs`, this line creates one entity:

```rust
commands.spawn((
    Sprite::from_color(Color::srgb(0.25, 0.70, 1.0), Vec2::splat(80.0)),
    Transform::from_translation(Vec3::ZERO),
));
```

That entity is renderable because it has `Sprite` and placeable because it has `Transform`.

## Component

A component is data attached to an entity.

```rust
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);
```

`Player` is a marker component. It identifies an entity as the player.

`Velocity` is a data component. It stores movement direction or speed as a `Vec2`.

The final example uses both marker and data components:

```rust
#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Body {
    half_size: Vec2,
}
```

`Enemy` identifies a role. `Body` stores collision size.

## Spawning Component Tuples

`commands.spawn` accepts a bundle. A tuple of components is a simple bundle:

```rust
commands.spawn((
    Player,
    Velocity(Vec2::ZERO),
    Transform::from_translation(Vec3::ZERO),
));
```

This creates one entity and attaches three components.

The tuple order is not the identity of the entity. The component types are. A query asks for entities by component shape, not by spawn order.

## Query Basics

A query selects entities by component shape:

```rust
Query<&mut Transform, With<Player>>
```

Read it as:

```text
Find entities with Player.
From each one, mutably borrow Transform.
```

Multiple components can be queried from the same entity:

```rust
Query<(&mut Transform, &Velocity), With<Body>>
```

Read it as:

```text
Find entities with Body, Transform, and Velocity.
Mutate Transform.
Read Velocity.
```

The data access is visible:

- `&Transform`: read component data.
- `&mut Transform`: write component data.
- `With<Player>`: require a marker/component but do not borrow it.
- `Without<Enemy>`: exclude entities with a marker/component.

## Query Filters And Conflicts

Filters narrow which entities match:

```rust
Query<&mut Velocity, With<Player>>
```

Only entities that have both `Velocity` and `Player` match. The query mutates `Velocity`; it only uses `Player` as a filter.

Bevy also checks whether system parameters can safely run together. This matters when two queries access the same component and at least one access is mutable.

For a camera follow system, this shape is ambiguous:

```rust
Query<&Transform, With<Player>>
Query<&mut Transform, With<Camera2d>>
```

Both queries access `Transform`. Bevy cannot assume the player is never also the camera. Add filters to prove the sets are separate:

```rust
Query<&Transform, (With<Player>, Without<Camera2d>)>
Query<&mut Transform, (With<Camera2d>, Without<Player>)>
```

The examples mostly avoid this by keeping systems focused.

## Resource, `Res`, And `ResMut`

A resource is one global value stored in the world.

```rust
#[derive(Resource)]
struct BodySpeed(f32);
```

Register it:

```rust
app.insert_resource(BodySpeed(220.0));
```

Read it:

```rust
fn move_bodies(speed: Res<BodySpeed>) {}
```

Mutate it:

```rust
fn collect_items(mut score: ResMut<Score>) {
    score.0 += 1;
}
```

Use a component for per-entity data. Use a resource for one shared value.

In this tutorial:

- `BodySpeed` is a shared movement speed in `examples/04_velocity_body.rs`.
- `Score` stores one shared score value in `examples/07_rpg_slice.rs`.
- `Time`, `ButtonInput<KeyCode>`, and `AssetServer` are Bevy-provided resources.

## `Local`

`Local<T>` stores system-local state. It is not a component and not a global resource. Each system that requests a `Local<T>` gets its own persisted value.

The final example uses it for hit cooldown:

```rust
fn enemy_hits_player(
    time: Res<Time>,
    mut hit_cooldown: Local<f32>,
) {
    *hit_cooldown -= time.delta_secs();
}
```

The `*` dereferences the local wrapper so the inner `f32` can be changed.

Use `Local` when the state belongs only to one system and does not need to be inspected by other systems.

## `Single`

`Single` is a query parameter for cases where exactly one entity should match:

```rust
fn enemy_ai(
    player: Single<&Transform, With<Player>>,
    mut enemies: Query<(&Transform, &mut Velocity), With<Enemy>>,
) {
    let player_position = player.translation.truncate();
}
```

This is a strong contract: there should be exactly one `Player` with a `Transform`. If your game can temporarily have zero or multiple matches, use `Query` with `single()` and handle the `Result`:

```rust
let Ok(player) = players.single() else {
    return;
};
```

## System Parameters Are The Contract

Look at this system from `examples/07_rpg_slice.rs`:

```rust
fn move_bodies(time: Res<Time>, mut bodies: Query<(&mut Transform, &Velocity), With<Body>>) {
    for (mut transform, velocity) in &mut bodies {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
    }
}
```

Before reading the body, you know:

```text
Reads Time.
Finds entities with Body, Transform, and Velocity.
Mutates Transform.
Reads Velocity.
Does not spawn or despawn entities.
Does not read keyboard input.
Does not touch health or score.
```

That is why Bevy code becomes easier when systems stay small.

## Checkpoint

For each system below, write down what it reads and writes before looking at the implementation:

- `player_input` in `examples/07_rpg_slice.rs`
- `collect_items` in `examples/07_rpg_slice.rs`
- `update_health_bar` in `examples/07_rpg_slice.rs`

## Common Mistakes

- Putting data in a resource when each entity needs its own value.
- Querying a marker as `&Player` when `With<Player>` would be enough.
- Forgetting that `Commands` are deferred.
- Writing one large system that reads input, moves entities, handles collision, updates score, and updates display all at once.
