# Rust + Bevy Tutorial

<div align="center">

[Repository root](../../README.md) · [Korean](../ko/index.md)

<code>Rust 2024</code> · <code>Bevy 0.18.1</code> · <code>23 chapters</code> · <code>22 runnable examples</code> · <code>English/한국어</code>

</div>

---

This is a community-made path for learning Rust and Bevy by building a small top-down RPG. The track starts with a window and a sprite, then grows into movement, ECS data design, plugins, assets, camera follow, waves, hitboxes, UI, animation, map geometry, game states, save/load, projectiles, inventory, dialogue, audio events, and scene loading.

You are not reading a reference manual. You are building one working slice at a time, running it, changing one rule, and then learning the Rust and Bevy concepts that made that slice work.

![Integrated RPG tutorial preview](../../assets/screenshots/ch17-complete-rpg-gameplay.png)

## How This Tutorial Teaches

Every chapter follows the same learning loop:

1. **Outcome**: the visible behavior or code structure you will have at the end.
2. **Run**: the exact example command for the chapter.
3. **Build**: the important systems, components, resources, or bundles that create the result.
4. **Rust lens**: the Rust syntax that matters in that chapter.
5. **Bevy lens**: the engine rule or ECS rule behind the code.
6. **Check**: observable proof that the chapter is working.
7. **Change**: a small modification with a clear expected result.

The English version is the primary source text. The Korean version mirrors the same structure so both languages teach the same path.

## Reference Trail

The writing style and learning order are informed by public Rust and Bevy learning material:

- [Bevy Quick Start](https://bevy.org/learn/quick-start/getting-started/) for short runnable app steps.
- [Bevy official examples](https://bevy.org/examples/) for small feature-focused programs.
- [Bevy ECS docs](https://docs.rs/bevy_ecs/latest/bevy_ecs/) for the entity/component/system model.
- [The Rust Programming Language](https://doc.rust-lang.org/book/) for ownership, structs, enums, and error handling.
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/) for runnable syntax examples.
- [Tainted Coders Bevy guides](https://taintedcoders.com/bevy/queries) for deeper feature-by-feature explanations.

This tutorial uses its own code and its own RPG project, but it keeps the same public-doc discipline: small examples, concrete outputs, and precise terminology.

## Learning Map

| Phase | Chapters | You build |
|---|---:|---|
| Setup | 0 | A Cargo project pinned to Bevy `0.18.1` |
| Rust first pass | 1 | Enough Rust to read Bevy systems instead of copying them blindly |
| App and ECS base | 2-5 | App setup, commands, entities, components, resources, queries, bundles, plugins, order |
| Presentation | 6 | Image assets, camera follow, and world-space text |
| RPG foundation | 7 | A compact playable arena with movement, enemies, pickups, health, score, and HUD |
| RPG systems | 8-16 | Camera smoothing, waves, attacks, sprite assets, fixed HUD, animation, map collision, states, save/load |
| Core integration | 17 | A playable RPG checkpoint using the combat-loop systems from the track |
| RPG content systems | 18-22 | Projectiles, inventory, dialogue, audio events, and scene loading |

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
17. [Integrated RPG slice](17-complete-rpg-slice.md)
18. [Projectiles](18-projectiles.md)
19. [Inventory](19-inventory.md)
20. [Dialogue](20-dialogue.md)
21. [Audio events](21-audio-events.md)
22. [Scene loading](22-scene-loading.md)

## Run The Examples

Each chapter has one example command. Run the command, observe the result, then read the chapter with the example file open.

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
cargo run --example 18_projectiles
cargo run --example 19_inventory
cargo run --example 20_dialogue
cargo run --example 21_audio_events
cargo run --example 22_scene_loading
```

## Core Mental Model

The core model stays stable for the whole track:

```text
Entity    = an ID in the world
Component = typed data attached to an entity
Resource  = one typed value stored globally in the world
System    = a Rust function scheduled by Bevy
Query     = typed access to matching entities
Commands  = deferred structural changes
Plugin    = registration unit for systems, resources, and other plugins
State     = app mode that controls which systems run
```

When a chapter feels busy, reduce it to two questions:

```text
What data exists?
Which system reads or writes that data this frame?
```

---

<div align="center">

[Start: Project setup →](00-project-setup.md)

</div>
