# Rust + Bevy 튜토리얼

<div align="center">

[저장소](../../README.md) · [English](../en/index.md)

<code>Rust 2024</code> · <code>Bevy 0.18.1</code> · <code>23장</code> · <code>실행 가능한 예제 22개</code> · <code>English/한국어</code>

</div>

---

이 튜토리얼은 작은 탑다운 RPG를 만들면서 Rust와 Bevy를 같이 익히는 커뮤니티 문서입니다. 처음에는 창 하나와 스프라이트 하나로 시작하고, 점점 이동, ECS 데이터 설계, 플러그인, 에셋, 카메라 추적, 적 웨이브, 공격 히트박스, UI, 애니메이션, 맵 구조, 게임 상태, 저장/불러오기, 발사체, 인벤토리, 대화, 오디오 이벤트, 씬 로딩까지 붙여갑니다.

레퍼런스 문서를 읽는 흐름이 아닙니다. 매 장마다 작게 만들고, 실행해 보고, 규칙 하나를 바꿔 보고, 그 코드에 들어간 Rust와 Bevy 개념을 다시 짚는 방식으로 진행합니다.

![통합 RPG 튜토리얼 미리보기](../../assets/screenshots/ch17-complete-rpg-gameplay.png)

## 진행 방식

각 장은 같은 흐름으로 읽습니다.

1. **이 장에서 만들 것**: 끝났을 때 화면이나 코드 구조가 어떻게 달라지는지 먼저 확인합니다.
2. **실행**: 해당 장의 예제를 바로 실행합니다.
3. **구현 흐름**: 필요한 시스템, 컴포넌트, 리소스, 번들을 순서대로 봅니다.
4. **Rust로 보면**: 이 장의 코드에서 꼭 알아야 할 Rust 문법을 짚습니다.
5. **Bevy로 보면**: 엔진이나 ECS 관점에서 왜 그렇게 짜는지 정리합니다.
6. **확인**: 제대로 동작하는지 눈으로 확인할 기준을 둡니다.
7. **바꿔보기**: 작은 수정을 해 보고 결과가 어떻게 달라지는지 봅니다.

두 언어판은 같은 학습 흐름을 따릅니다. 한국어판은 한국어 독자가 바로 읽을 수 있는 설명체로 따로 다듬었습니다.

## 참고한 공개 자료

문서의 흐름과 설명 방식은 공개 Rust/Bevy 학습 자료를 참고했습니다.

- [Bevy Quick Start](https://bevy.org/learn/quick-start/getting-started/): 짧게 실행해 보는 앱 흐름.
- [Bevy 공식 예제](https://bevy.org/examples/): 기능 하나에 집중한 작은 예제 구성.
- [Bevy ECS 문서](https://docs.rs/bevy_ecs/latest/bevy_ecs/): 엔티티, 컴포넌트, 시스템 모델.
- [The Rust Programming Language](https://doc.rust-lang.org/book/): 소유권, struct, enum, 오류 처리 같은 Rust 기본기.
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/): 실행 가능한 Rust 문법 예제.
- [Tainted Coders Bevy 가이드](https://taintedcoders.com/bevy/queries): Bevy 기능별 깊은 설명 방식.

코드와 RPG 예제는 이 튜토리얼 자체의 흐름에 맞춰 작성했습니다. 다만 공개 문서들이 가진 장점인 작은 예제, 명확한 결과, 정확한 용어 사용은 그대로 가져옵니다.

## 학습 지도

| 구간 | 장 | 만드는 것 |
|---|---:|---|
| 준비 | 0 | Bevy `0.18.1`에 맞춘 Cargo 프로젝트 |
| Rust 첫 감각 | 1 | Bevy 시스템을 읽기 위한 Rust 기본 문법 |
| App과 ECS 기초 | 2-5 | App 설정, commands, entity, component, resource, query, bundle, plugin, 실행 순서 |
| 화면 표현 | 6 | 이미지 에셋, 카메라 추적, 월드 공간 텍스트 |
| RPG 기초 | 7 | 이동, 적, 수집, 체력, 점수, HUD가 있는 작은 아레나 |
| RPG 시스템 | 8-16 | 부드러운 카메라, 웨이브, 공격, 스프라이트, 고정 HUD, 애니메이션, 맵 충돌, 상태, 저장 |
| 핵심 통합 | 17 | 전투 루프 시스템을 합친 플레이 가능한 RPG 체크포인트 |
| RPG 콘텐츠 시스템 | 18-22 | 발사체, 인벤토리, 대화, 오디오 이벤트, 씬 로딩 |

## 목차

0. [프로젝트 설정](00-project-setup.md)
1. [Bevy에 필요한 Rust](01-rust-for-bevy.md)
2. [Bevy 앱 모델](02-bevy-app-model.md)
3. [ECS 기본](03-ecs-fundamentals.md)
4. [입력과 이동](04-input-and-movement.md)
5. [번들, 플러그인, 세트](05-bundles-plugins-sets.md)
6. [에셋, 카메라, UI](06-assets-camera-ui.md)
7. [RPG 기초 예제](07-rpg-slice.md)
8. [부드러운 카메라 추적](08-smooth-camera-follow.md)
9. [적 웨이브](09-enemy-waves.md)
10. [공격 히트박스](10-attack-hitbox.md)
11. [스프라이트 에셋](11-sprite-assets.md)
12. [화면 고정 UI](12-screen-space-ui.md)
13. [애니메이션 상태](13-animation-state.md)
14. [직접 만든 맵 구조](14-handmade-map-geometry.md)
15. [게임 상태](15-game-states.md)
16. [진행도 저장과 불러오기](16-save-load-progress.md)
17. [통합 RPG 예제](17-complete-rpg-slice.md)
18. [발사체](18-projectiles.md)
19. [인벤토리](19-inventory.md)
20. [대화](20-dialogue.md)
21. [오디오 이벤트](21-audio-events.md)
22. [씬 로딩](22-scene-loading.md)

## 예제 실행

각 장은 하나의 예제 명령을 갖습니다. 먼저 실행하고, 화면을 확인한 뒤, 예제 파일을 열어 둔 상태로 문서를 읽는 흐름을 권합니다.

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

## 핵심 감각

전체 튜토리얼에서 이 모델은 계속 유지됩니다.

```text
Entity    = 월드 안의 ID
Component = 엔티티에 붙는 타입 있는 데이터
Resource  = 월드에 하나만 저장되는 타입 있는 값
System    = Bevy가 실행하는 Rust 함수
Query     = 조건에 맞는 엔티티를 타입으로 요청하는 방법
Commands  = 엔티티 생성/삭제 같은 구조 변경 예약
Plugin    = 시스템, 리소스, 다른 플러그인을 등록하는 단위
State     = 어떤 시스템을 실행할지 가르는 앱 모드
```

어떤 장이 복잡하게 느껴지면 질문을 두 개로 줄이면 됩니다.

```text
어떤 데이터가 있는가?
이번 프레임에 어떤 시스템이 그 데이터를 읽거나 쓰는가?
```

---

<div align="center">

[시작: 프로젝트 설정 →](00-project-setup.md)

</div>
