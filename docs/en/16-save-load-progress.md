# 16. Save And Load Progress

[Index](index.md) | Previous: [Game states](15-game-states.md) | Next: [Complete RPG slice](17-complete-rpg-slice.md)

Run:

```sh
cargo run --example 16_save_load_progress
```

The save file is written to:

```text
target/tutorial-save/progress.json
```

## Contract

Do not save the whole Bevy world. Save explicit progress data.

```text
Progress = best_score + unlocked_wave + player_name
```

The rest of the world can be rebuilt from code, assets, and level data.

## Data Contract

`Progress` is both a Bevy `Resource` and a serializable Rust struct:

```rust
#[derive(Resource, Serialize, Deserialize)]
struct Progress { ... }
```

The app inserts `load_progress_from_disk()` before startup systems run.

## Rust Point

`Result<(), String>` is used for saving because writing a file can fail. The system turns that result into a user-visible status message.

`Option` appears in:

```rust
let Some(parent) = Path::new(SAVE_PATH).parent() else { ... };
```

The code handles the missing-parent case explicitly.

## Bevy Point

Progress is a resource because there is one current progress record. UI text reads that resource every frame.

## Controls

```text
S       increase score in memory
W       increase unlocked wave in memory
F5      save JSON
F9      load JSON
Delete  reset progress and remove the file
```

## Common Mistakes

- Saving transient entities instead of durable progress.
- Ignoring file write errors.
- Writing save files into the repository root.

## Change It

- Add `gold`.
- Add `last_selected_character`.
- Save on game over instead of only on F5.
