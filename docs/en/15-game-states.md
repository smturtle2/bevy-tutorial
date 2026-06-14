# 15. Game States


<div align="center">

[Index](index.md) · [← Previous: Handmade map geometry](14-handmade-map-geometry.md) · [Next: Save and load progress →](16-save-load-progress.md)

</div>

---

Run:

```sh
cargo run --example 15_game_states
```

![The game state example starts in Menu, where only menu input systems are active until the player starts a run.](../../assets/screenshots/ch15-game-state-menu.png)

## Contract

Game states control which systems are allowed to run.

```text
Menu     menu input only
Playing  gameplay systems
Paused   pause input only; gameplay remains visible
GameOver restart/menu input only
```

A state is not just a boolean. Use a Rust enum with Bevy's `States` derive.

## Data Contract

`GameState` is initialized with:

```rust
.init_state::<GameState>()
```

Transitions are requested with `NextState<GameState>`. State-specific UI uses marker components such as `MenuUi`, `PauseUi`, and `GameOverUi`.

## Rust Point

The `#[default]` enum variant tells Rust which state to use when Bevy initializes the state resource.

## Bevy Point

Systems are gated with:

```rust
menu_input.run_if(in_state(GameState::Menu))
move_player.run_if(in_state(GameState::Playing))
```

`OnEnter` and `OnExit` are schedules. They are the right place to spawn and clean up state-specific UI.

## Common Mistakes

- Running gameplay movement while paused.
- Despawning gameplay on pause when pause should only freeze logic.
- Mutating `State<GameState>` directly instead of using `NextState`.

## Change It

- Add a settings state.
- Make Escape return to the menu from Playing.
- Add state-specific background colors.

---

<div align="center">

[← Previous: Handmade map geometry](14-handmade-map-geometry.md) · [Index](index.md) · [Next: Save and load progress →](16-save-load-progress.md)

</div>
