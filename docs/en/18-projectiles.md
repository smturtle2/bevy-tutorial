# 18. Projectiles

<div align="center">

[Index](index.md) · [← Previous: Integrated RPG slice](17-complete-rpg-slice.md) · [Next: Inventory →](19-inventory.md)

</div>

---

## Outcome

This chapter extends the chapter 17 combat loop with a ranged attack. `Space` remains the melee slash. `F` fires a projectile in the player's facing direction. Both attacks use the same health model, collision body model, gameplay cleanup marker, and system-order contract.

![Projectiles travel from the player toward enemies.](../../assets/screenshots/ch18-projectiles.png)

## Run

```sh
cargo run --example 18_projectiles
```

Controls:

```text
WASD / Arrow keys   move
Space               melee slash
F                   fire projectile
```

## Chapter Contract

This example isolates the projectile rule and preserves only the contracts that the rule touches:

```text
GameState::Playing      gates gameplay systems
GameSet                 Input -> Movement -> Collision -> Ui
GameplayEntity          marks spawned gameplay objects
Body                    collision size
Velocity                movement vector
Facing                  last non-zero movement direction
Health { current, max } enemy health data shape
Space slash             remains a melee attack
```

The new feature adds one more combat object:

```text
Projectile entity = GameplayEntity + Projectile + Body + Velocity + Transform + Sprite
```

## Build Step 1: Add A Projectile Component

A projectile has gameplay rules, not only a sprite:

```rust
#[derive(Component)]
struct Projectile {
    lifetime: Timer,
    damage: i32,
}
```

`lifetime` controls cleanup for missed shots. `damage` lets projectile collision use the same health mutation as melee collision.

## Build Step 2: Keep Melee And Ranged Inputs Separate

The integrated slice already uses `Space` for slash hitboxes. This chapter keeps that contract and puts projectiles on `F`:

```rust
if !keyboard.just_pressed(KeyCode::KeyF) {
    return;
}

let (transform, facing) = *player;
let start = transform.translation + (facing.0 * 34.0).extend(1.0);

commands.spawn(ProjectileBundle::new(start, facing.0));
```

The firing system reads the player position and `Facing`, then creates a new entity with the right starting data. Movement remains the movement system's job.

## Build Step 3: Put Movement In The Bundle

The projectile bundle turns a direction into velocity and rotation:

```rust
velocity: Velocity(direction * PROJECTILE_SPEED),
rotation: Quat::from_rotation_z(direction.y.atan2(direction.x)),
```

That makes projectiles compatible with the shared movement system:

```rust
fn move_bodies(time: Res<Time>, mut bodies: Query<(&mut Transform, &Velocity), With<Body>>) {
    for (mut transform, velocity) in &mut bodies {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
    }
}
```

## Build Step 4: Expire Missed Shots

Projectiles that never collide still need an owner for cleanup:

```rust
fn tick_projectile_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(Entity, &mut Projectile)>,
) {
    for (entity, mut projectile) in &mut projectiles {
        projectile.lifetime.tick(time.delta());

        if projectile.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
```

The cleanup rule belongs to the projectile feature. It should not be hidden inside player input or enemy logic.

## Build Step 5: Reuse The Health Contract

Projectile collision mutates the same `Health { current, max }` structure used by enemies in the integrated slice:

```rust
if overlaps(projectile_transform, projectile_body, enemy_transform, enemy_body) {
    health.current -= projectile.damage;
    commands.entity(projectile_entity).despawn();

    if health.current <= 0 {
        commands.entity(enemy_entity).despawn();
    }
}
```

The projectile despawns after one hit. Piercing projectiles would change this exact rule.

## Integration Points

The feature uses the frame phases it needs from the chapter 17 combat loop:

```text
Input       read F and spawn ProjectileBundle
Movement    move projectiles and tick projectile lifetime
Collision   compare projectiles with enemy bodies and apply Health damage
Ui          show shots, hits, active projectiles, and enemy health
```

The order matters. Movement before collision means the hit test uses the projectile's current-frame position. Lifetime cleanup prevents missed shots from accumulating.

## Rust Lens

`Projectile` is a named-field component because it owns two facts:

```rust
struct Projectile {
    lifetime: Timer,
    damage: i32,
}
```

`Velocity(Vec2)` and `Facing(Vec2)` are tuple structs because each wraps one value and the type name supplies the meaning.

The state and set derives each unlock a scheduling role:

```rust
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Playing,
}
```

`States` lets Bevy store the enum as app state. `Default` chooses the starting state. `Eq` and `Hash` let Bevy use values as state identities. `Clone` and `Copy` make small state values easy to pass. `Debug` supports diagnostics and error messages.

The movement code uses `normalize_or_zero`:

```rust
let normalized = direction.normalize_or_zero();
```

Normalizing a zero vector can produce an invalid numeric direction such as `NaN`. `normalize_or_zero` makes the zero-input case explicit and keeps movement and firing code deterministic.

## Check

Run:

```sh
cargo run --example 18_projectiles
```

Expected result:

- `Space` creates a short melee hitbox.
- `F` fires a projectile in the player's facing direction.
- Projectiles move independently from the player.
- Enemy health decreases from both melee and projectile hits.
- Missed projectiles disappear after their lifetime.
- The UI shows shots, projectile hits, slash counts, melee hits, active projectiles, and enemy health.

## Change

Change:

```rust
const PROJECTILE_LIFETIME: f32 = 0.9;
```

to:

```rust
const PROJECTILE_LIFETIME: f32 = 2.0;
```

Expected result: projectiles travel farther before despawning.

---

<div align="center">

[← Previous: Integrated RPG slice](17-complete-rpg-slice.md) · [Index](index.md) · [Next: Inventory →](19-inventory.md)

</div>
