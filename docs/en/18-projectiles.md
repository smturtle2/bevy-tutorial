# 18. Projectiles

<div align="center">

[Index](index.md) · [← Previous: Integrated RPG slice](17-complete-rpg-slice.md) · [Next: Inventory →](19-inventory.md)

</div>

---

## Outcome

At the end of this chapter, the player fires moving projectiles. A projectile has its own position, velocity, collision body, damage, and lifetime. It can hit enemies or disappear when its timer runs out.

![Projectiles travel from the player toward enemies.](../../assets/screenshots/ch18-projectiles.png)

## Run

```sh
cargo run --example 18_projectiles
```

Move with WASD or arrow keys. Press Space to fire in the last movement direction.

## Build Step 1: Treat A Projectile As An Entity

A projectile is not a flag on the player. It is a short-lived entity:

```rust
#[derive(Component)]
struct Projectile {
    lifetime: Timer,
    damage: i32,
}
```

The entity also receives `Body`, `Velocity`, `Transform`, and `Sprite`. That gives it the same basic movement and collision vocabulary as the player and enemies.

```text
Projectile entity = Projectile + Body + Velocity + Transform + Sprite
```

This is the rule for RPG objects: if something has independent position and lifetime, make it an entity.

## Build Step 2: Spawn From Player Position And Facing

The player stores the direction it last moved:

```rust
#[derive(Component)]
struct Facing(Vec2);
```

When Space is pressed, the fire system reads the player transform and facing direction:

```rust
let (transform, facing) = *player;
let start = transform.translation + (facing.0 * 34.0).extend(1.0);

commands.spawn(ProjectileBundle::new(start, facing.0));
```

The extra `34.0` places the projectile in front of the player instead of inside the player body. The `extend(1.0)` turns the 2D direction offset into a `Vec3` and raises the projectile above the player layer.

## Build Step 3: Give The Projectile Velocity And Rotation

The bundle constructor turns direction into movement:

```rust
velocity: Velocity(direction * PROJECTILE_SPEED),
```

It also rotates the sprite:

```rust
let angle = direction.y.atan2(direction.x);

Transform {
    translation: position,
    rotation: Quat::from_rotation_z(angle),
    ..default()
}
```

The projectile can now use the shared movement system:

```rust
fn move_bodies(time: Res<Time>, mut bodies: Query<(&mut Transform, &Velocity), With<Body>>) {
    for (mut transform, velocity) in &mut bodies {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
    }
}
```

## Build Step 4: Expire Missed Projectiles

A projectile that never hits anything still needs a cleanup rule:

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

Without this system, missed projectiles would stay in the world forever.

## Build Step 5: Apply Damage On Collision

The collision system compares every projectile with every enemy:

```rust
if overlaps(projectile_transform, projectile_body, enemy_transform, enemy_body) {
    health.0 -= projectile.damage;
    commands.entity(projectile_entity).despawn();
    stats.hits += 1;

    if health.0 <= 0 {
        commands.entity(enemy_entity).despawn();
    }
}
```

The projectile despawns after a hit. That makes it a non-piercing projectile. If you want piercing arrows later, the rule changes here.

## Build Step 6: Put The Systems In Order

Projectiles need a stable frame order:

```text
Input      fire projectile
Movement   move projectile and tick lifetime
Collision  apply projectile hits
Ui         show counts
```

If collision runs before movement, a projectile hits using last frame's position. If cleanup never runs, missed projectiles accumulate. The order is part of the feature contract.

## Rust Lens

This chapter uses a tuple struct for single-field components:

```rust
struct Velocity(Vec2);
struct Health(i32);
```

`velocity.0` and `health.0` access the inner value. Tuple structs are useful when the component has one obvious value and the type name carries the meaning.

The direction code uses `normalize_or_zero`:

```rust
let normalized = direction.normalize_or_zero();
```

Normalizing a zero vector is unsafe math. `normalize_or_zero` returns `Vec2::ZERO` for the zero case, so firing and movement code can stay simple.

## Bevy Lens

Projectiles are a good ECS fit because they are independent world facts:

```text
where it is      Transform
how it moves     Velocity
what it hits     Body
what it does     Projectile { damage, lifetime }
how it looks     Sprite
```

The firing system only creates intent. The movement and collision systems do the rest. That separation is why adding projectiles does not require rewriting player movement.

## Check

Run:

```sh
cargo run --example 18_projectiles
```

Expected result:

- Space fires a projectile in the player's facing direction.
- Projectiles move without the player carrying them.
- Enemies lose health and disappear after enough hits.
- Missed projectiles disappear after a short time.
- The UI counters change as shots are fired and hits land.

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
