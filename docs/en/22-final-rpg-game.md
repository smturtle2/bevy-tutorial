# 22. Final RPG Game

<div align="center">

[Index](index.md) · [← Previous: Scene loading](21-scene-loading.md) · [Contribute →](https://github.com/smturtle2/bevy-tutorial)

</div>

---

## Outcome

This is the final chapter. The separate systems from the tutorial are now one playable RPG game: loaded scenes, player movement, smooth camera, map collision, enemy waves, melee hitboxes, projectiles, inventory, NPC dialogue, audio events, screen-space HUD, menu, pause, game over, and save/load progress.

![The final RPG game combines combat, inventory, dialogue, audio events, scene data, HUD, and saving.](../../assets/screenshots/ch22-final-rpg-game.png)

## Run

```sh
cargo run --example 22_final_rpg_game
```

Controls:

```text
Enter       start from menu
WASD        move
Space       melee attack / continue dialogue
F           fire projectile
E           talk near an NPC
1 / 2       load another scene
P           pause
Esc         close dialogue / return to menu from pause
F5 / F9     save and load progress
```

## Final Game Contract

The final example is not another isolated feature demo. It is the integration point for the tutorial.

```text
State       owns app mode: Menu, Playing, Dialogue, Paused, GameOver
Scene       loads JSON and spawns Player, Wall, InventoryItem, Npc, Body, Transform
Combat      uses melee hitboxes and projectile entities against the same Health model
Inventory   records collected Gem, Key, and Potion items
Dialogue    pauses movement with GameState::Dialogue and DialogueState
Audio       receives typed GameAudioEvent messages from gameplay rules
HUD         reads resources/components and renders screen-space status
Progress    saves best score and unlocked wave explicitly
```

The key rule is ownership of responsibility:

```text
Gameplay systems decide what happened.
Resources remember run-wide state.
Messages announce cross-cutting events.
UI systems display state.
Scene loading creates entities from data.
```

## Integration Map

The final game keeps the earlier features as real systems, not as comments or future work.

| Earlier feature | Final game owner |
| --- | --- |
| Smooth camera follow | `smooth_follow_camera` reads the player transform and lerps the camera |
| Enemy waves | `WaveSpawner` and `spawn_enemy_waves` create timed waves |
| Attack hitbox | `AttackHitboxBundle`, `expire_attack_hitboxes`, and `attack_hits_enemies` |
| Projectiles | `ProjectileBundle`, `fire_projectile`, `tick_projectile_lifetime`, and `projectiles_hit_enemies` |
| Sprite assets | `SpriteAssets` loads player sheet, enemy, gem, and slash images |
| Screen-space UI | HUD text and health bar entities use `Node` with absolute positioning |
| Animation state | `PlayerAnimation` and `PlayerAnimState` drive texture atlas frames |
| Map geometry | JSON scene walls become `Wall + Body` entities |
| Game states | `Menu`, `Playing`, `Dialogue`, `Paused`, and `GameOver` gate systems |
| Save/load | `Progress` serializes best score and unlocked wave to JSON |

This table is a contract for the final chapter. If a feature is listed here, the final example contains the code path that makes it run.

## System Schedule

The final game uses one ordered schedule for the live gameplay frame:

```text
Input
-> Wave
-> Ai
-> Movement
-> Collision
-> Animation
-> Ui
```

That order is part of the game design:

```text
Input      creates intent, attacks, projectiles, dialogue requests, scene switches
Wave       creates new enemies before AI reads the world
Ai         writes enemy velocity
Movement   moves bodies and resolves wall collision
Collision  applies pickups, damage, cleanup, and game-over checks
Animation  updates visible player frames from gameplay state
Ui         renders the resulting state for the player
```

The important habit is that each phase consumes the previous phase's results. This keeps the final game readable even though many systems now run together.

## Build Step 1: Final States

The game uses five states:

```rust
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Dialogue,
    Paused,
    GameOver,
}
```

`Dialogue` is a real game state, not a boolean flag inside player input. Normal movement, AI, collision, and wave spawning run in `Playing`; dialogue input runs in `Dialogue`.

## Build Step 2: Load A Scene Into Gameplay Components

The final game reads `assets/scenes/arena_a.json` and `assets/scenes/arena_b.json`. The JSON file decides layout and content. The Rust systems still decide behavior.

```text
scene file -> SceneData
SceneData  -> Player, Wall, InventoryItem, Npc entities
systems    -> movement, collision, collection, dialogue, combat
```

That separation matters. Data decides what exists. Systems decide what it means.

## Build Step 3: Keep Combat Shared

Melee and ranged attacks both target the same enemy contract:

```text
Enemy = Enemy + Body + Velocity + Health + Transform + Sprite
```

The difference is only how the attack exists:

```text
melee       short-lived AttackHitbox near the player
projectile  moving Projectile entity with its own lifetime
```

Both systems mutate `Health`. The rest of the game does not need two health models.

## Build Step 4: Inventory Is Run State

World items are entities. The player inventory is a resource:

```rust
#[derive(Resource, Default)]
struct Inventory {
    gems: u32,
    keys: u32,
    potions: u32,
}
```

Collection removes an `InventoryItem` entity, updates `Inventory`, adds score, and emits `GameAudioEvent::Pickup`.

## Build Step 5: Dialogue Owns Conversation State

NPCs own their text:

```text
Npc { name: String, lines: Vec<String> }
```

The current conversation lives in a resource:

```text
DialogueState { active_npc, line_index }
```

This keeps entity data and current UI flow separate.

## Build Step 6: Audio Is Event-Driven

Gameplay systems emit typed messages:

```rust
#[derive(Message, Debug, Clone, Copy)]
enum GameAudioEvent {
    Attack,
    Projectile,
    Pickup,
    Hurt,
    Dialogue,
}
```

One audio system reads those messages and spawns short-lived `AudioPlayer` entities. Attack code does not know frequencies, handles, or playback settings.

## Build Step 7: HUD Reads The Game

The final HUD is a reader, not a second source of truth:

```text
Health text      reads Health
Score text       reads RunStats + Progress
Wave text        reads RunStats + WaveSpawner
Inventory text   reads Inventory
Scene text       reads CurrentScene
Save text        reads SaveStatus
Dialogue panel   reads DialogueState + Npc
```

When the HUD is wrong, the fix is usually in the resource/component update path, not in the text itself.

## Rust Lens

The final example puts the earlier Rust concepts together:

```text
struct        named game data with clear ownership
enum          finite state and event choices
derive        trait contracts for Bevy, serde, debug, copy, equality, hashing
Option        active dialogue may or may not exist
Result        scene/save I/O may fail
Vec<T>        loaded dialogue lines and scene lists
String        text loaded at runtime
&T / &mut T   systems borrow ECS data through queries/resources
```

## Check

Run the final game and verify the full loop:

```sh
cargo run --example 22_final_rpg_game
```

Expected result:

```text
Enter starts the game.
WASD moves through a loaded scene.
Space attacks nearby enemies.
F fires a projectile.
Items update score and inventory.
E opens NPC dialogue.
1/2 swaps the loaded scene.
P pauses the game.
F5 writes progress and F9 reloads it.
```

## Change

Open `assets/scenes/arena_a.json` and add one item:

```json
{ "kind": "Potion", "x": 80.0, "y": -170.0 }
```

Run the final game again. The item appears without adding a new Rust spawn call because the final game loads scene content from data.

---

<div align="center">

[← Previous: Scene loading](21-scene-loading.md) · [Index](index.md) · [Contribute →](https://github.com/smturtle2/bevy-tutorial)

</div>
