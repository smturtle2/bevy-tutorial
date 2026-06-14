# 17. Complete RPG Slice


<div align="center">

[Index](index.md) · [← Previous: Save and load progress](16-save-load-progress.md) · [Next: Index →](index.md)

</div>

---

Run:

```sh
cargo run --example 17_complete_rpg_slice
```

![Complete RPG slice preview](../../assets/screenshots/complete-rpg-slice-preview.png)

## Contract

This chapter does not introduce a new trick. It proves the required systems can coexist:

- smooth camera follow
- enemy wave spawning
- attack hitboxes
- sprite assets and sprite-sheet animation
- screen-space UI
- handmade map geometry
- `Menu`, `Playing`, `Paused`, `GameOver`
- save/load progress data

## Data Contract

The final example keeps responsibility explicit:

```text
Bundles       spawn stable gameplay shapes
Components    hold per-entity data
Resources     hold progress, run stats, wave spawning, assets
SystemSet     orders frame phases
GameState     decides which systems may run
```

`GameplayEntity` marks entities that should be removed when leaving a run. Pause does not remove them; it only stops gameplay systems.

`Progress.unlocked_wave` is not only displayed. New runs start at the saved unlocked wave. `RunStats` remains the current run record, and `Progress` remains the durable record.

## Frame Flow

```text
Input      player movement and attack intent
Wave       spawn enemies
Ai         chase player
Movement   apply velocity, resolve walls, follow camera
Collision  collect gems, hit enemies, damage player, end game
Animation  choose player sprite frame
Ui         save hotkey and HUD update
```

That order is the final tutorial contract. If a feature needs another order, make it explicit.

## Rust Point

The final file combines several Rust concepts:

- `enum` for `GameState` and `PlayerAnimState`
- `impl Default` for reusable starting values
- `Result` for save errors
- `Option` for path handling
- typed tuple structs such as `Velocity(Vec2)` and `Facing(Vec2)`

## Bevy Point

The app uses both state gating and system sets. These solve different problems:

```text
State     = should this system run at all?
SystemSet = when does this system run inside the frame?
```

Do not replace one with the other.

## Controls

```text
Enter   start from menu
WASD    move
Space   attack
P       pause/resume
F5      fold current run stats into progress, then save progress
F9      load progress from disk
R       restart from game over
Esc     return to menu from pause/game over
```

## Common Mistakes

- Letting menu systems and gameplay systems run at the same time.
- Saving `RunStats` as permanent progress without deciding what should persist.
- Spawning gameplay directly inside `OnEnter(Playing)` when pause/resume would duplicate it.
- Adding a new system without assigning it to a frame phase.

## Change It

- Add enemy projectiles.
- Add a second collectible type.
- Save more progress on game over.
- Split the final example into modules once the boundaries feel stable.

---

<div align="center">

[← Previous: Save and load progress](16-save-load-progress.md) · [Index](index.md) · [Next: Index →](index.md)

</div>
