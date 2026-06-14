# 15. Game States

<div align="center">

[Index](index.md) · [← Previous: Handmade map geometry](14-handmade-map-geometry.md) · [Next: Save and load progress →](16-save-load-progress.md)

</div>

---

## Outcome

At the end of this chapter, the app has four explicit modes: menu, playing, paused, and game over. Systems run only in the modes where they make sense.

![The game state example starts in a menu and switches into gameplay.](../../assets/screenshots/ch15-game-state-menu.png)

## Run

```sh
cargo run --example 15_game_states
```

Controls:

```text
Enter     start from menu
WASD      move while playing
H         take debug damage
P         pause/resume
Esc       return to menu from pause or game over
R         restart from game over
```

## Build Step 1: Define The State Enum

Bevy states are Rust enums:

```rust
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    GameOver,
}
```

Register the state type:

```rust
.init_state::<GameState>()
```

The `#[default]` variant is the starting state.

## Build Step 2: Spawn And Clean Up State UI

Menu UI is created when entering the menu:

```rust
.add_systems(OnEnter(GameState::Menu), spawn_menu)
```

It is removed when leaving the menu:

```rust
.add_systems(OnExit(GameState::Menu), cleanup_entities::<MenuUi>)
```

The marker component identifies menu entities:

```rust
#[derive(Component)]
struct MenuUi;
```

This pattern repeats for pause UI and game-over UI.

## Build Step 3: Gate Systems With `run_if`

Menu input should not run while playing:

```rust
.add_systems(Update, menu_input.run_if(in_state(GameState::Menu)))
```

Gameplay systems should only run while playing:

```rust
.add_systems(
    Update,
    (move_player, debug_take_damage, game_over_when_dead)
        .chain()
        .run_if(in_state(GameState::Playing)),
)
```

This is cleaner than adding `if current_state == ...` checks inside every system.

## Build Step 4: Transition With `NextState`

Systems request transitions through `NextState`:

```rust
fn menu_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        spawn_gameplay(&mut commands, &asset_server);
        next_state.set(GameState::Playing);
    }
}
```

The transition is applied by Bevy's state system. The request is explicit and typed.

## Build Step 5: Mark Gameplay Entities

Gameplay entities get a shared marker:

```rust
#[derive(Component)]
struct GameplayEntity;
```

Returning to menu or entering game over can remove all gameplay entities:

```rust
for entity in &gameplay {
    commands.entity(entity).despawn();
}
```

UI markers and gameplay markers keep cleanup precise.

## Build Step 6: Use Generic Cleanup

The cleanup system works for any marker component:

```rust
fn cleanup_entities<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
```

That is why registration can write:

```rust
cleanup_entities::<MenuUi>
cleanup_entities::<PauseUi>
cleanup_entities::<GameOverUi>
```

## Rust Lens

`cleanup_entities::<MenuUi>` is explicit generic syntax. It tells Rust which component type `T` is for that system.

The state enum derives several traits because Bevy must store, compare, hash, clone, and debug-print state values:

```rust
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
```

Derives are not decoration. They satisfy concrete trait requirements.

## Bevy Lens

State controls schedules:

```text
OnEnter(Menu)      run once when entering Menu
OnExit(Menu)       run once when leaving Menu
Update + run_if    run every frame only while condition is true
NextState<T>       request a transition
```

Use states for app modes: menu, playing, paused, game over, loading, cutscene.

## Check

Run:

```sh
cargo run --example 15_game_states
```

Expected result:

- The app starts in the menu.
- Enter spawns gameplay and switches to playing.
- P shows pause UI and stops movement.
- Esc from pause removes gameplay and returns to menu.
- Pressing H until health reaches zero enters game over.
- R restarts from game over.

## Change

Change the player starting health:

```rust
Health(3)
```

to:

```rust
Health(1)
```

Expected result: one press of `H` during play triggers game over.

---

<div align="center">

[← Previous: Handmade map geometry](14-handmade-map-geometry.md) · [Index](index.md) · [Next: Save and load progress →](16-save-load-progress.md)

</div>
