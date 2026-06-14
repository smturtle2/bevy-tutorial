# 9. Enemy Waves

<div align="center">

[Index](index.md) · [← Previous: Smooth camera follow](08-smooth-camera-follow.md) · [Next: Attack hitboxes →](10-attack-hitbox.md)

</div>

---

## Outcome

At the end of this chapter, enemies spawn over time in waves. The game tracks the current wave, how many enemies are queued, where the next enemy spawns, and when the next spawn is allowed.

![The wave spawner tracks alive enemies and queued spawns.](../../assets/screenshots/ch09-enemy-waves.png)

## Run

```sh
cargo run --example 09_enemy_waves
```

Enemies spawn from arena corners, chase the player, expire after a short lifetime, and advance to the next wave.

## Build Step 1: Put Wave State In A Resource

Wave state is global gameplay state, so it belongs in a resource:

```rust
#[derive(Resource)]
struct WaveSpawner {
    wave: u32,
    remaining_to_spawn: u32,
    spawn_index: usize,
    timer: Timer,
}
```

`Default` gives the first wave:

```rust
impl Default for WaveSpawner {
    fn default() -> Self {
        Self {
            wave: 1,
            remaining_to_spawn: 4,
            spawn_index: 0,
            timer: Timer::from_seconds(0.35, TimerMode::Repeating),
        }
    }
}
```

Register it:

```rust
.init_resource::<WaveSpawner>()
```

`init_resource` creates the default value if the resource is not already present.

## Build Step 2: Give Enemies A Lifetime

The example removes enemies automatically so waves can finish:

```rust
#[derive(Component)]
struct EnemyLifetime(Timer);
```

Each enemy gets one timer:

```rust
lifetime: EnemyLifetime(Timer::from_seconds(2.5, TimerMode::Once)),
```

The expiration system despawns finished enemies:

```rust
fn expire_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut enemies: Query<(Entity, &mut EnemyLifetime), With<Enemy>>,
) {
    for (entity, mut lifetime) in &mut enemies {
        lifetime.0.tick(time.delta());

        if lifetime.0.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
```

## Build Step 3: Start The Next Wave

The spawner checks two conditions:

```rust
if spawner.remaining_to_spawn == 0 && enemies.iter().count() == 0 {
    spawner.wave += 1;
    spawner.remaining_to_spawn = spawner.wave + 3;
    spawner.timer.reset();
}
```

The wave advances only when nothing is queued and no enemies are alive.

## Build Step 4: Spawn On Timer Ticks

The spawner ticks its timer every frame:

```rust
spawner.timer.tick(time.delta());

if !spawner.timer.just_finished() {
    return;
}
```

Only a finished tick spawns an enemy:

```rust
let spawn = SPAWN_POINTS[spawner.spawn_index % SPAWN_POINTS.len()];
spawner.spawn_index += 1;
spawner.remaining_to_spawn -= 1;

commands.spawn(EnemyBundle::new(spawn.extend(2.0), asset_server.as_ref()));
```

The modulo wraps spawn points so waves can be larger than the number of spawn locations.

## Build Step 5: Display Wave State

The UI text reads the resource and the enemy query:

```rust
fn update_wave_text(
    spawner: Res<WaveSpawner>,
    enemies: Query<(), With<Enemy>>,
    mut text: Single<&mut Text, With<WaveText>>,
) {
    text.0 = format!(
        "Wave {} | alive: {} | queued: {}",
        spawner.wave,
        enemies.iter().count(),
        spawner.remaining_to_spawn
    );
}
```

`Query<(), With<Enemy>>` means “I only need to count matching entities; I do not need their component data.”

## Rust Lens

This constant is an array:

```rust
const SPAWN_POINTS: [Vec2; 4] = [
    Vec2::new(-470.0, 260.0),
    Vec2::new(470.0, 260.0),
    Vec2::new(-470.0, -260.0),
    Vec2::new(470.0, -260.0),
];
```

`[Vec2; 4]` means exactly four `Vec2` values.

`usize` is used for indexing:

```rust
spawn_index: usize
```

Rust array indexing requires a `usize`.

## Bevy Lens

Wave spawning state lives in a resource because it must survive across frames:

```text
temporary calculation inside one frame -> local variable
private memory for one system           -> Local<T>
global state across systems/frames      -> Resource
```

## Check

Run:

```sh
cargo run --example 09_enemy_waves
```

Expected result:

- The wave text starts at wave 1.
- Enemies appear from corners over time.
- Alive count changes as enemies spawn and expire.
- A later wave starts after the current enemies are gone.

## Change

Change the timer:

```rust
Timer::from_seconds(0.35, TimerMode::Repeating)
```

to:

```rust
Timer::from_seconds(0.08, TimerMode::Repeating)
```

Expected result: enemies spawn much faster, but the wave rules stay the same.

---

<div align="center">

[← Previous: Smooth camera follow](08-smooth-camera-follow.md) · [Index](index.md) · [Next: Attack hitboxes →](10-attack-hitbox.md)

</div>
