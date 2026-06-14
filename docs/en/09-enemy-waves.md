# 9. Enemy Waves


<div align="center">

[Index](index.md) · [← Previous: Smooth camera follow](08-smooth-camera-follow.md) · [Next: Attack hitboxes →](10-attack-hitbox.md)

</div>

---

Run:

```sh
cargo run --example 09_enemy_waves
```

## Contract

Enemy wave generation is global game state, so it belongs in a resource. Enemies are actual gameplay objects, so they are entities with components.

```text
WaveSpawner resource = wave number + remaining spawn count + timer
Enemy entity         = Enemy + EnemyLifetime + Body + Velocity + Sprite
```

## Data Contract

`WaveSpawner` decides when to queue a new wave and where the next enemy appears. `EnemyBundle::new` owns the spawn shape for an enemy. The wave system does not manually assemble random component tuples.

This chapter is about spawning waves, not combat. Enemies have `EnemyLifetime(Timer)` so the example can clear a wave and demonstrate wave 2 without introducing attack systems early.

## Rust Point

`WaveSpawner` implements `Default`. That lets the app use:

```rust
.init_resource::<WaveSpawner>()
```

The rule is simple: if a resource has a valid default starting state, implement `Default` and let Bevy initialize it.

## Bevy Point

The spawn system uses a repeating `Timer`:

```rust
spawner.timer.tick(time.delta());
if spawner.timer.just_finished() {
    commands.spawn(EnemyBundle::new(...));
}
```

The timer is data. The system advances it. `Commands` queues the spawn.

## Frame Flow

```text
Input     player velocity
Wave      expire old enemies, then make enemy spawn decisions
Ai        enemy velocity toward player
Movement  apply velocity
Ui        show current wave status
```

Enemy expiry runs before spawning, then spawning happens before AI so newly spawned enemies can receive velocity in the same frame.

## Common Mistakes

- Putting the wave number on each enemy makes it hard to know the global wave.
- Spawning the whole wave in one frame removes pacing.
- Starting the next wave while old enemies are alive changes the design; this example waits until the current wave is cleared.
- Forgetting a clear mechanic makes wave 2 unreachable.

## Change It

- Increase `remaining_to_spawn` per wave.
- Add more `SPAWN_POINTS`.
- Replace `EnemyLifetime` with real combat after finishing the hitbox chapter.

---

<div align="center">

[← Previous: Smooth camera follow](08-smooth-camera-follow.md) · [Index](index.md) · [Next: Attack hitboxes →](10-attack-hitbox.md)

</div>
