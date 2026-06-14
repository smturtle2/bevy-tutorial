# 17. Complete RPG Slice

<div align="center">

[Index](index.md) · [← Previous: Save and load progress](16-save-load-progress.md) · [Next: Contribute →](https://github.com/smturtle2/bevy-tutorial)

</div>

---

## Outcome

At the end of this chapter, the previous systems are integrated into one small RPG slice: menu, gameplay, pause, game over, image assets, animated player, smooth camera, enemy waves, attack hitboxes, map collision, screen-space HUD, and progress saving.

![The complete RPG slice combines the tutorial systems into one playable example.](../../assets/screenshots/ch17-complete-rpg-gameplay.png)

## Run

```sh
cargo run --example 17_complete_rpg_slice
```

Controls:

```text
Enter     start from menu
WASD      move
Space     attack
P         pause/resume
F5        save progress while playing
F9        load progress while playing
Esc       return to menu from pause or game over
R         restart from game over
```

## Build Step 1: Keep The Integration Pipeline Explicit

The final example has more systems, so the frame phases are explicit:

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Wave,
    Ai,
    Movement,
    Collision,
    Animation,
    Ui,
}
```

The order is:

```text
Input -> Wave -> Ai -> Movement -> Collision -> Animation -> Ui
```

That order is a game design decision. For example, collision should happen after movement, and UI should happen after gameplay changes.

## Build Step 2: Store Shared Assets Once

The final example stores image handles and the player atlas layout in a resource:

```rust
#[derive(Resource, Clone)]
struct SpriteAssets {
    player_sheet: Handle<Image>,
    player_layout: Handle<TextureAtlasLayout>,
    enemy: Handle<Image>,
    gem: Handle<Image>,
    slash: Handle<Image>,
}
```

Setup loads them once:

```rust
commands.insert_resource(SpriteAssets {
    player_sheet: asset_server.load("player_sheet.png"),
    player_layout,
    enemy: asset_server.load("enemy.png"),
    gem: asset_server.load("gem.png"),
    slash: asset_server.load("slash.png"),
});
```

Bundles then receive `&SpriteAssets` instead of loading paths repeatedly.

## Build Step 3: Use Bundles As Spawn Contracts

The final slice uses separate bundles for different entity shapes:

```text
PlayerBundle          player, body, facing, health, animation, atlas sprite
EnemyBundle           enemy, body, health, sprite
CollectibleBundle     gem body and sprite
WallBundle            static wall body and sprite
AttackHitboxBundle    temporary damage body and slash sprite
```

This keeps `start_run` readable:

```rust
commands.spawn(PlayerBundle::new(assets));
spawn_map(commands);
spawn_hud(commands);
commands.spawn(CollectibleBundle::new(position, assets));
```

The call site shows game objects, not component plumbing.

## Build Step 4: Start And Reset A Run

Starting a run resets runtime stats and wave state:

```rust
fn start_run(
    commands: &mut Commands,
    assets: &SpriteAssets,
    progress: &Progress,
    stats: &mut RunStats,
    spawner: &mut WaveSpawner,
) {
    *stats = RunStats::default();
    stats.wave = progress.unlocked_wave.max(1);
    spawner.reset_to_wave(stats.wave);

    commands.spawn(PlayerBundle::new(assets));
    spawn_map(commands);
    spawn_hud(commands);
}
```

Progress influences the starting wave, but runtime enemies and HUD are spawned fresh.

## Build Step 5: Gate Gameplay By State

Gameplay systems run only while playing:

```rust
.add_systems(
    Update,
    (player_input, spawn_attack_hitbox)
        .chain()
        .in_set(GameSet::Input)
        .run_if(in_state(GameState::Playing)),
)
```

Menu, pause, and game-over systems have their own state gates.

This is how the final example avoids movement while paused and avoids spawning enemies in the menu.

## Build Step 6: Combine Collision Rules

The collision phase handles several gameplay events:

```rust
(
    collect_gems,
    attack_hits_enemies,
    enemy_hits_player,
    expire_attack_hitboxes,
    end_game_if_dead,
)
    .chain()
    .in_set(GameSet::Collision)
```

The chain says what happens first:

```text
collect gems
apply player attacks
apply enemy contact damage
remove expired hitboxes
enter game over if player health is zero
```

These systems are separate because each rule has a different responsibility.

## Build Step 7: Save Progress At Natural Boundaries

Progress is updated on game over:

```rust
progress.best_score = progress.best_score.max(stats.score);
progress.unlocked_wave = progress.unlocked_wave.max(stats.wave);
```

It is also manually saved with F5 while playing:

```rust
if keyboard.just_pressed(KeyCode::F5) {
    progress.best_score = progress.best_score.max(stats.score);
    progress.unlocked_wave = progress.unlocked_wave.max(stats.wave);
    save_status.0 = match save_progress_to_disk(&progress) {
        Ok(()) => format!("Saved progress to {SAVE_PATH}"),
        Err(error) => format!("Save failed: {error}"),
    };
}
```

The final game does not serialize temporary enemies, hitboxes, or UI. It saves long-lived progress.

## Rust Lens

The final example combines the Rust concepts from the track:

```text
struct               components, resources, bundles
tuple struct         Velocity(Vec2), SaveStatus(String)
enum                 GameState, GameSet, PlayerAnimState
impl                 constructors and reset methods
generic functions    cleanup_entities::<MenuUi>
Option               texture atlas access
Result               save/load IO
borrowing            Res, ResMut, Query, Single
```

The code is not “less Rust” because Bevy injects parameters. Bevy system signatures are Rust type contracts.

## Bevy Lens

The final slice is a composition of small ECS rules:

```text
states decide which systems run
system sets decide frame order
resources store global game state
components store per-entity facts
bundles make spawning repeatable
commands create and remove entities
queries apply behavior to matching entities
```

This is the complete architecture for the tutorial track. The systems from earlier chapters now run together inside one game loop.

## Check

Run:

```sh
cargo run --example 17_complete_rpg_slice
```

Acceptance checks:

- Menu appears first.
- Enter starts gameplay.
- Player moves and animates.
- Camera follows smoothly.
- Enemies spawn in waves and chase the player.
- Space creates slash hitboxes that damage enemies.
- Gems increase score.
- Walls block movement.
- HUD stays fixed to the screen.
- P pauses and resumes.
- Game over saves best score and unlocked wave.
- Restarting can load saved progress.

## Change

Add one more gem in `start_run`:

```rust
Vec3::new(0.0, 210.0, 3.0),
```

Expected result: the new gem appears in the map, uses existing collision, increases score when collected, and requires no new collection system.

## Main Track Completion

This tutorial includes all of these systems in the main track:

```text
camera follow      smooth camera movement that follows the player
enemy waves        wave-based spawning and enemy pursuit
attack hitbox      short-lived melee hitboxes
sprite assets      image and sprite-sheet driven presentation
screen-space UI    HUD elements fixed to the screen
animation state    idle, moving, and attack animation states
map geometry       handmade walls and collision structure
game states        menu, playing, paused, and game over
save/load          best score and unlocked wave persistence
```

The final chapter connects these systems in one frame order and shows how each responsibility stays clear.

---

<div align="center">

[← Previous: Save and load progress](16-save-load-progress.md) · [Index](index.md) · [Contribute →](https://github.com/smturtle2/bevy-tutorial)

</div>
