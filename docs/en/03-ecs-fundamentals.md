# 3. ECS Fundamentals

<div align="center">

[Index](index.md) · [← Previous: The Bevy app model](02-bevy-app-model.md) · [Next: Input and movement →](04-input-and-movement.md)

</div>

---

## Outcome

At the end of this chapter, you can separate entity identity, component data, resources, systems, queries, filters, `Local`, and `Single`. This is the vocabulary used by every later gameplay feature.

![ECS overview diagram](../../assets/diagrams/ecs-overview.png)

## Run

```sh
cargo run --example 02_spawn_sprite
```

Use the sprite example as the smallest ECS scene: one camera entity and one sprite entity.

## Build Step 1: Entity Means ID

An entity is an ID in Bevy's world. It is not a class instance and it does not own methods.

When you write:

```rust
commands.spawn((
    Player,
    Sprite::from_color(Color::srgb(0.25, 0.70, 1.0), Vec2::splat(80.0)),
    Transform::from_translation(Vec3::ZERO),
));
```

Bevy creates an entity ID and attaches three components to that ID:

```text
Entity 42
  Player
  Sprite
  Transform
```

You usually do not care about the numeric ID unless another component or resource needs to remember a specific entity.

## Build Step 2: Component Means Typed Data

A component is a Rust type attached to an entity:

```rust
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Body {
    half_size: Vec2,
}
```

The type is the key. Each entity can have at most one component of a given type. An entity can have both `Player` and `Body`, but not two separate `Body` components.

## Build Step 3: System Means Function Over ECS Data

A system is a Rust function Bevy can schedule:

```rust
fn move_bodies(
    time: Res<Time>,
    mut bodies: Query<(&mut Transform, &Velocity), With<Body>>,
) {
    for (mut transform, velocity) in &mut bodies {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
    }
}
```

The function signature is the important part. It says:

```text
read resource: Time
match entities: Body + Transform + Velocity
write component: Transform
read component: Velocity
```

## Build Step 4: Query Selects Matching Entities

`Query<&mut Transform, With<Player>>` means:

```text
for every entity that has Player and Transform,
give this system mutable access to Transform.
```

Tuple query data means “get several components from the same entity”:

```rust
Query<(&mut Transform, &Velocity), With<Body>>
```

Filters narrow the set:

```rust
With<Player>       entity must have Player
Without<Camera2d>  entity must not have Camera2d
```

`Without` is often how you make two queries disjoint when both could otherwise touch `Transform`.

## Build Step 5: Resource Means One Global Value

A resource is one typed value in the world:

```rust
#[derive(Resource)]
struct PlayerSpeed(f32);
```

Insert it:

```rust
.insert_resource(PlayerSpeed(280.0))
```

Read it:

```rust
fn move_player(speed: Res<PlayerSpeed>) {
    let pixels_per_second = speed.0;
}
```

Mutate it:

```rust
fn add_score(mut score: ResMut<Score>) {
    score.0 += 1;
}
```

Use resources for global state: score, wave spawner, save progress, loaded asset handles, or configuration.

## Build Step 6: `Local` Means Per-System Memory

`Local<T>` stores private state for one system:

```rust
fn enemy_hits_player(
    time: Res<Time>,
    mut hit_cooldown: Local<f32>,
) {
    *hit_cooldown -= time.delta_secs();
}
```

Use `Local` when the state belongs only to that system. Use a resource when multiple systems need to read or write the state.

## Build Step 7: `Single` Means Exactly One Match

`Single` is useful when the design requires one matching entity:

```rust
fn follow_player(
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}
```

Use `Single` for one player, one main camera, or one HUD text. Use `Query` when there may be zero, one, or many matches.

## Rust Lens

The `for` loop is not Bevy magic:

```rust
for (mut transform, velocity) in &mut bodies {
}
```

`&mut bodies` asks the query for mutable iteration. The loop variable mirrors the query data tuple. `mut transform` means the local binding can write through the mutable component reference.

Tuple struct access also appears often:

```rust
velocity.0
score.0
```

That `.0` is Rust tuple-field syntax, not Bevy syntax.

## Bevy Lens

ECS is not “one object with all variables.” It is a set of typed data columns. Systems declare which columns they need, and Bevy runs compatible systems in an order that satisfies borrowing and scheduling rules.

The practical design rule:

```text
Marker component     answers “what kind of entity is this?”
Data component       answers “what facts belong to this entity?”
Resource             answers “what one global fact exists?”
System               answers “what changes this frame?”
```

## Check

You can move on when you can explain this signature without looking it up:

```rust
fn collect_items(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player: Single<(&Transform, &Body), With<Player>>,
    collectibles: Query<(Entity, &Transform, &Body), With<Collectible>>,
) {
}
```

Expected reading:

```text
can despawn entities
can mutate the global score
expects exactly one player with Transform and Body
iterates every collectible with Entity, Transform, and Body
```

## Change

In `examples/04_velocity_body.rs`, remove `With<Body>` from the `move_bodies` query:

```rust
mut bodies: Query<(&mut Transform, &Velocity)>,
```

Expected result: behavior stays the same in this example because only body entities have `Velocity`. The filter becomes important later when more entities share components.

---

<div align="center">

[← Previous: The Bevy app model](02-bevy-app-model.md) · [Index](index.md) · [Next: Input and movement →](04-input-and-movement.md)

</div>
