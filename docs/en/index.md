# Rust + Bevy Tutorial

<div align="center">

[Repository root](../../README.md) · [Korean](../ko/index.md)

<code>Rust 2024</code> · <code>Bevy 0.18.1</code> · <code>18 runnable examples</code> · <code>English/한국어</code>

</div>

---

This tutorial teaches Rust and Bevy as one track. You will build from a blank Bevy window to a small top-down RPG that includes camera follow, enemy waves, attack hitboxes, sprite assets, screen-space UI, animation state, handmade map geometry, game states, and save/load.

![RPG feature roadmap](../../assets/diagrams/rpg-feature-roadmap.png)

> [!IMPORTANT]
> This tutorial teaches Rust from the first Bevy example onward. Rust syntax appears in the order the examples need it, and each chapter is tied to one runnable example.

## Learning Contract

By the end, you should be able to:

- Read and write Bevy systems with `Commands`, `Res`, `ResMut`, `Query`, `Single`, `With`, and `Without`.
- Design components as small data types and spawn stable gameplay entities through bundles.
- Split frame behavior into explicit phases with `.chain()` and `SystemSet`.
- Explain the Rust behind Bevy code: function signatures, `struct`, tuple, tuple struct, `impl`, trait, derive, generic types, ownership, references, `Option`, `Result`, `match`, modules, and visibility.
- Load image assets and sprite sheets through `AssetServer`.
- Keep world-space gameplay, screen-space UI, and game state transitions separate.
- Save and load explicit progress data without pretending the whole Bevy world is a save file.

## Learning Path

| Phase | Chapters | Output |
|---|---:|---|
| Setup | 0 | A Cargo/Bevy project that builds |
| Rust + app basics | 1-2 | You can read Rust type syntax and Bevy app registration |
| ECS fundamentals | 3-5 | Components, resources, queries, bundles, plugins, and system order |
| Presentation | 6 | Assets, camera follow, and world text |
| RPG foundation | 7 | A compact loop with movement, AI, collision, score, and HUD |
| Required RPG systems | 8-17 | Camera smoothing, waves, attacks, UI, animation, map geometry, states, and save/load |

## Chapters

0. [Project setup](00-project-setup.md)
1. [Rust for Bevy](01-rust-for-bevy.md)
2. [The Bevy app model](02-bevy-app-model.md)
3. [ECS fundamentals](03-ecs-fundamentals.md)
4. [Input and movement](04-input-and-movement.md)
5. [Bundles, plugins, and sets](05-bundles-plugins-sets.md)
6. [Assets, camera, and UI](06-assets-camera-ui.md)
7. [RPG foundation slice](07-rpg-slice.md)
8. [Smooth camera follow](08-smooth-camera-follow.md)
9. [Enemy waves](09-enemy-waves.md)
10. [Attack hitboxes](10-attack-hitbox.md)
11. [Sprite assets](11-sprite-assets.md)
12. [Screen-space UI](12-screen-space-ui.md)
13. [Animation state](13-animation-state.md)
14. [Handmade map geometry](14-handmade-map-geometry.md)
15. [Game states](15-game-states.md)
16. [Save and load progress](16-save-load-progress.md)
17. [Complete RPG slice](17-complete-rpg-slice.md)

## Run The Track

> [!TIP]
> Read one chapter, run its example, change a small rule, then move on. Treat the final code as something to inspect and modify. Bevy becomes easier when you keep asking: "what does this system read, and what does it write?"

```sh
cargo run --example 01_empty_app
cargo run --example 02_spawn_sprite
cargo run --example 03_player_input
cargo run --example 04_velocity_body
cargo run --example 05_plugins_sets
cargo run --example 06_assets_camera_ui
cargo run --example 07_rpg_slice
cargo run --example 08_smooth_camera_follow
cargo run --example 09_enemy_waves
cargo run --example 10_attack_hitbox
cargo run --example 11_sprite_assets
cargo run --example 12_screen_space_ui
cargo run --example 13_animation_state
cargo run --example 14_handmade_map_geometry
cargo run --example 15_game_states
cargo run --example 16_save_load_progress
cargo run --example 17_complete_rpg_slice
```

## Mental Model

The core model stays stable:

```text
Entity    = an ID in the world
Component = data attached to an entity
System    = a Rust function that reads/writes ECS data
Resource  = one global value stored in the world
Plugin    = a registration unit for systems/resources/plugins
State     = a schedule-level mode such as Menu or Playing
```

When code feels scattered, look for the responsibility boundary:

```text
Component = data shape
Bundle    = spawn shape
System    = behavior
Resource  = global state
SystemSet = frame order
State     = which systems are allowed to run
Module    = source-code boundary
```

---

<div align="center">

[Start: Project setup →](00-project-setup.md)

</div>
