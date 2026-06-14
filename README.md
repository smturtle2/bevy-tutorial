# Rust + Bevy Tutorial

A practical, GitHub-friendly tutorial for learning Rust and Bevy together by building a small top-down RPG.

The project teaches Rust through Bevy instead of treating Rust as a separate prerequisite. Early chapters explain the Rust syntax that Bevy code relies on. Later chapters make the expected RPG systems first-class tutorial material, not optional "next steps."

![Complete RPG slice preview](assets/screenshots/complete-rpg-slice-preview.png)

## Start Here

- English: [Tutorial index](docs/en/index.md)
- Korean: [한국어 튜토리얼 목차](docs/ko/index.md)
- Target Bevy version: `0.18.1`
- Target Rust edition: `2024`

If you are new to Rust but know another language, start with [Rust for Bevy](docs/en/01-rust-for-bevy.md). If you already know Rust, start with [The Bevy app model](docs/en/02-bevy-app-model.md).

## Run

```sh
cargo run
```

The root app demonstrates the architectural shape used throughout the tutorial:

- `GamePlugin` owns high-level app setup.
- `BodyPlugin` owns shared body movement.
- `PlayerPlugin` owns player spawning and input.
- `GameSet` orders input before movement.

## Run Chapter Examples

Every chapter example is designed to compile independently:

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

## Chapters

| # | English | Korean |
|---|---|---|
| 0 | [Project setup](docs/en/00-project-setup.md) | [프로젝트 설정](docs/ko/00-project-setup.md) |
| 1 | [Rust for Bevy](docs/en/01-rust-for-bevy.md) | [Bevy를 위한 Rust](docs/ko/01-rust-for-bevy.md) |
| 2 | [The Bevy app model](docs/en/02-bevy-app-model.md) | [Bevy 앱 모델](docs/ko/02-bevy-app-model.md) |
| 3 | [ECS fundamentals](docs/en/03-ecs-fundamentals.md) | [ECS 기본](docs/ko/03-ecs-fundamentals.md) |
| 4 | [Input and movement](docs/en/04-input-and-movement.md) | [입력과 이동](docs/ko/04-input-and-movement.md) |
| 5 | [Bundles, plugins, and sets](docs/en/05-bundles-plugins-sets.md) | [번들, 플러그인, 세트](docs/ko/05-bundles-plugins-sets.md) |
| 6 | [Assets, camera, and UI](docs/en/06-assets-camera-ui.md) | [에셋, 카메라, UI](docs/ko/06-assets-camera-ui.md) |
| 7 | [RPG foundation slice](docs/en/07-rpg-slice.md) | [RPG 기초 조각](docs/ko/07-rpg-slice.md) |
| 8 | [Smooth camera follow](docs/en/08-smooth-camera-follow.md) | [부드러운 카메라 추적](docs/ko/08-smooth-camera-follow.md) |
| 9 | [Enemy waves](docs/en/09-enemy-waves.md) | [적 웨이브](docs/ko/09-enemy-waves.md) |
| 10 | [Attack hitboxes](docs/en/10-attack-hitbox.md) | [공격 hitbox](docs/ko/10-attack-hitbox.md) |
| 11 | [Sprite assets](docs/en/11-sprite-assets.md) | [sprite asset](docs/ko/11-sprite-assets.md) |
| 12 | [Screen-space UI](docs/en/12-screen-space-ui.md) | [screen-space UI](docs/ko/12-screen-space-ui.md) |
| 13 | [Animation state](docs/en/13-animation-state.md) | [애니메이션 상태](docs/ko/13-animation-state.md) |
| 14 | [Handmade map geometry](docs/en/14-handmade-map-geometry.md) | [직접 만든 맵 지오메트리](docs/ko/14-handmade-map-geometry.md) |
| 15 | [Game states](docs/en/15-game-states.md) | [게임 상태](docs/ko/15-game-states.md) |
| 16 | [Save and load progress](docs/en/16-save-load-progress.md) | [진행 저장/불러오기](docs/ko/16-save-load-progress.md) |
| 17 | [Complete RPG slice](docs/en/17-complete-rpg-slice.md) | [완성 RPG 조각](docs/ko/17-complete-rpg-slice.md) |

## Tutorial Contract

- Every listed core feature has a runnable example and a chapter.
- Domain gameplay entities are spawned through `Bundle + impl new()` when they have a stable shape.
- Runtime behavior belongs in systems.
- Shared behavior belongs in shared plugins/modules once repetition becomes real.
- Query filters should match system names.
- Ordering that matters must be explicit with `.chain()` or `SystemSet`.
- Game state transitions must be represented with `States` and `NextState`, not hidden booleans.
- Save files store explicit progress data, not the whole Bevy world.

## Image Contract

Images under `assets/screenshots/` are rendered previews that mirror runnable examples. Images under `assets/diagrams/` are explanatory diagrams. Do not treat either as a replacement for compiling and running the examples.

## License

This tutorial is licensed under the [MIT License](LICENSE). Code, documentation, and included tutorial assets are MIT-licensed unless a file explicitly states otherwise.

## References

- [Bevy getting started](https://bevy.org/learn/quick-start/getting-started/)
- [Bevy 0.18 release notes](https://bevy.org/news/bevy-0-18/)
- [Bevy ECS docs](https://docs.rs/bevy_ecs/latest/bevy_ecs/)
- [Bevy crate docs](https://docs.rs/bevy/0.18.1/bevy/)
