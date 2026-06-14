# 7. RPG Foundation Slice

<div align="center">

[Index](index.md) · [← Previous: Assets, camera, and UI](06-assets-camera-ui.md) · [Next: Smooth camera follow →](08-smooth-camera-follow.md)

</div>

---

## Outcome

At the end of this chapter, you have a compact playable RPG arena:

- player movement
- enemy chase AI
- arena bounds
- collectible pickup
- health damage with cooldown
- score and health display
- explicit system order

![The RPG foundation slice combines player movement, enemies, pickups, health, score, and HUD.](../../assets/screenshots/ch07-rpg-slice.png)

## Run

```sh
cargo run --example 07_rpg_slice
```

Move with WASD or arrow keys. Collect yellow items. Avoid red enemies.

## Build Step 1: Declare The Frame Phases

The slice has enough behavior that system order must be named:

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Ai,
    Movement,
    Collision,
    Display,
}
```

Registration turns that enum into a frame pipeline:

```rust
.configure_sets(
    Update,
    (
        GameSet::Input,
        GameSet::Ai,
        GameSet::Movement,
        GameSet::Collision,
        GameSet::Display,
    )
        .chain(),
)
```

The order is the gameplay rule:

```text
Input writes player velocity
AI writes enemy velocity
Movement writes transforms
Collision reads final positions
Display reads final game state
```

## Build Step 2: Define Shared Body Data

The collision and movement systems need common data:

```rust
#[derive(Component)]
struct Body {
    half_size: Vec2,
}

#[derive(Component)]
struct Velocity(Vec2);
```

The body bundle keeps spawn shape consistent:

```rust
#[derive(Bundle)]
struct BodyBundle {
    body: Body,
    velocity: Velocity,
    transform: Transform,
}

impl BodyBundle {
    fn new(position: Vec3, size: Vec2) -> Self {
        Self {
            body: Body {
                half_size: size / 2.0,
            },
            velocity: Velocity(Vec2::ZERO),
            transform: Transform::from_translation(position),
        }
    }
}
```

This is the foundation for player, enemy, and collectible entities.

## Build Step 3: Spawn Domain Bundles

The player bundle combines identity, body data, rendering, and health:

```rust
#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    body: BodyBundle,
    sprite: Sprite,
    health: Health,
}
```

Enemies and collectibles use the same body idea:

```rust
#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    body: BodyBundle,
    sprite: Sprite,
}

#[derive(Bundle)]
struct CollectibleBundle {
    collectible: Collectible,
    body: BodyBundle,
    sprite: Sprite,
}
```

The entity type is not inherited. An enemy is an entity with `Enemy`, `Body`, `Velocity`, `Transform`, and `Sprite`. A collectible is an entity with `Collectible`, `Body`, `Velocity`, `Transform`, and `Sprite`.

## Build Step 4: Move The Player

Player input writes velocity:

```rust
fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in &mut players {
        velocity.0 = direction.normalize_or_zero() * PLAYER_SPEED;
    }
}
```

The body movement system applies velocity to every body:

```rust
fn move_bodies(time: Res<Time>, mut bodies: Query<(&mut Transform, &Velocity), With<Body>>) {
    for (mut transform, velocity) in &mut bodies {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
    }
}
```

This makes movement reusable. Player input is only one possible source of velocity.

## Build Step 5: Add Enemy Chase AI

Enemy AI reads the player position and writes enemy velocity:

```rust
fn enemy_ai(
    player: Single<&Transform, With<Player>>,
    mut enemies: Query<(&Transform, &mut Velocity), With<Enemy>>,
) {
    let player_position = player.translation.truncate();

    for (transform, mut velocity) in &mut enemies {
        let to_player = player_position - transform.translation.truncate();
        velocity.0 = to_player.normalize_or_zero() * ENEMY_SPEED;
    }
}
```

`truncate()` converts `Vec3` to `Vec2` by dropping `z`. Movement direction is a 2D problem.

## Build Step 6: Clamp Bodies To The Arena

The arena clamp keeps bodies inside the frame:

```rust
fn clamp_to_arena(mut bodies: Query<(&mut Transform, &Body), With<Body>>) {
    for (mut transform, body) in &mut bodies {
        let min = -ARENA_HALF_SIZE + body.half_size;
        let max = ARENA_HALF_SIZE - body.half_size;
        transform.translation.x = transform.translation.x.clamp(min.x, max.x);
        transform.translation.y = transform.translation.y.clamp(min.y, max.y);
    }
}
```

The body half-size matters. The center of a large entity must stop sooner than the center of a small entity.

## Build Step 7: Detect AABB Collision

This tutorial uses axis-aligned bounding box collision:

```rust
fn overlaps(
    a_transform: &Transform,
    a_body: &Body,
    b_transform: &Transform,
    b_body: &Body,
) -> bool {
    let a = a_transform.translation.truncate();
    let b = b_transform.translation.truncate();
    let distance = (a - b).abs();
    let allowed = a_body.half_size + b_body.half_size;

    distance.x < allowed.x && distance.y < allowed.y
}
```

The rule is simple:

```text
if center distance on x is smaller than combined half width
and center distance on y is smaller than combined half height
then the rectangles overlap
```

## Build Step 8: Collect Items

Collectibles use `Commands` because collection removes entities:

```rust
fn collect_items(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player: Single<(&Transform, &Body), With<Player>>,
    collectibles: Query<(Entity, &Transform, &Body), With<Collectible>>,
) {
    let (player_transform, player_body) = *player;

    for (entity, transform, body) in &collectibles {
        if overlaps(player_transform, player_body, transform, body) {
            commands.entity(entity).despawn();
            score.0 += 1;
        }
    }
}
```

The query includes `Entity` because `Commands` needs the entity ID to despawn it.

## Build Step 9: Damage With A Cooldown

Enemy contact reduces health, but not every frame:

```rust
fn enemy_hits_player(
    time: Res<Time>,
    player: Single<(&Transform, &Body, &mut Health), With<Player>>,
    enemies: Query<(&Transform, &Body), With<Enemy>>,
    mut hit_cooldown: Local<f32>,
) {
    *hit_cooldown -= time.delta_secs();

    if *hit_cooldown > 0.0 {
        return;
    }

    let (player_transform, player_body, mut health) = player.into_inner();

    for (enemy_transform, enemy_body) in &enemies {
        if overlaps(player_transform, player_body, enemy_transform, enemy_body) {
            health.current = (health.current - 1).max(0);
            *hit_cooldown = 1.0;
            break;
        }
    }
}
```

`Local<f32>` is perfect here because only this system owns the cooldown.

## Build Step 10: Update Display Last

The display phase reads the final health and score:

```rust
fn update_hud_text(
    score: Res<Score>,
    player: Single<&Health, With<Player>>,
    mut health_text: Single<&mut Text2d, (With<HealthText>, Without<ScoreText>)>,
    mut score_text: Single<&mut Text2d, (With<ScoreText>, Without<HealthText>)>,
) {
    let health = *player;
    health_text.0 = format!("Health: {}/{}", health.current, health.max);
    score_text.0 = format!("Score: {}", score.0);
}
```

This should happen after collision so the HUD reflects the current frame.

## Rust Lens

This chapter uses Rust types as gameplay language:

```text
Health { current, max }      named fields for related facts
Score(u32)                   tuple struct for one global value
BodyBundle::new(...)         associated constructor
Local<f32>                   generic wrapper around private system state
Single<(&Transform, &Body)>  tuple of borrowed components
```

Notice that no class inheritance is required. Shared behavior comes from shared components and shared systems.

## Bevy Lens

The slice scales because each system has one job:

```text
player_input          keyboard -> player Velocity
enemy_ai              player Transform -> enemy Velocity
move_bodies           Velocity -> Transform
clamp_to_arena        Transform within bounds
collect_items         overlap -> despawn + score
enemy_hits_player     overlap -> health
update_hud_text       resources/components -> text
```

When a later feature feels large, place it in this pipeline first. Then write the component/resource data it needs.

## Check

Run:

```sh
cargo run --example 07_rpg_slice
```

Expected result:

- The player moves.
- Red enemies move toward the player.
- Yellow collectibles disappear on contact.
- Score increases after collection.
- Health decreases when enemies touch the player, with a short cooldown.
- Entities stay inside the arena frame.

## Change

Change enemy speed:

```rust
const ENEMY_SPEED: f32 = 30.0;
```

Expected result: enemies still chase the player, but the game becomes easier. No collision or HUD code needs to change.

---

<div align="center">

[← Previous: Assets, camera, and UI](06-assets-camera-ui.md) · [Index](index.md) · [Next: Smooth camera follow →](08-smooth-camera-follow.md)

</div>
