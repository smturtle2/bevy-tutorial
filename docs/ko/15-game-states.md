# 15. 게임 상태

[목차](index.md) | 이전: [직접 만든 맵 지오메트리](14-handmade-map-geometry.md) | 다음: [진행 저장/불러오기](16-save-load-progress.md)

실행:

```sh
cargo run --example 15_game_states
```

이 장의 계약은 메뉴, 플레이, 일시정지, 게임오버를 하나의 enum 상태로 관리하는 것입니다. 상태는 어떤 시스템이 실행되는지, 어떤 UI가 생성/정리되는지 결정합니다.

## 핵심 ECS 계약

- `GameState`: `Menu`, `Playing`, `Paused`, `GameOver` 중 하나입니다.
- `NextState<GameState>`: 다음 상태 전환 요청을 쓰는 리소스입니다.
- `OnEnter`, `OnExit`: 상태에 들어가거나 나갈 때 한 번 실행되는 스케줄입니다.
- `run_if(in_state(...))`: 특정 상태에서만 시스템을 실행합니다.
- `GameplayEntity`, `MenuUi`, `PauseUi`, `GameOverUi`: 정리 대상을 구분하는 마커입니다.

상태 전환은 즉시 모든 코드를 점프시키는 명령이 아닙니다. 시스템은 `next_state.set(...)`으로 요청하고, Bevy가 스케줄 경계에서 상태를 바꿉니다.

## Rust 포인트

`#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]`는 Bevy 상태 타입의 계약입니다. 상태 enum은 복사 가능하고 비교 가능하며 해시 가능해야 합니다. `#[default]`는 `init_state::<GameState>()`가 처음 사용할 값을 지정합니다.

`cleanup_entities<T: Component>`는 제네릭 시스템입니다. `MenuUi`, `PauseUi`, `GameOverUi`처럼 타입만 다른 정리 로직을 하나의 함수로 재사용합니다.

## Bevy 포인트

`OnEnter(GameState::Menu)`에서 메뉴 UI를 만들고 `OnExit(GameState::Menu)`에서 제거합니다. 이렇게 하면 Update 시스템이 매 프레임 UI 존재 여부를 직접 검사하지 않아도 됩니다.

플레이 중인 엔티티에는 `GameplayEntity`를 붙입니다. 메뉴로 돌아가거나 게임오버가 될 때 이 마커로 한 번에 정리합니다.

## 프레임 흐름

1. 앱은 `Menu` 상태로 시작합니다.
2. Enter를 누르면 플레이 엔티티를 스폰하고 `Playing`으로 전환합니다.
3. `Playing`에서만 이동, 데미지, 사망 체크가 실행됩니다.
4. P를 누르면 `Paused`로 전환하고 일시정지 UI를 생성합니다.
5. 체력이 0이면 `GameOver`로 전환하고 플레이 엔티티를 정리합니다.

## 흔한 실수

- 상태별 UI를 정리하지 않으면 메뉴, 일시정지, 게임오버 텍스트가 겹칩니다.
- `NextState` 대신 현재 상태 리소스를 직접 바꾸려고 하면 Bevy 상태 스케줄과 어긋납니다.
- `run_if(in_state(GameState::Playing))`를 빼면 메뉴나 일시정지 중에도 이동 시스템이 실행됩니다.
- 게임플레이 엔티티에 `GameplayEntity`를 빠뜨리면 상태 전환 정리에서 남습니다.

## 작게 바꿔보기

- Settings 상태를 하나 추가해보세요.
- Playing에서 Escape를 누르면 Menu로 돌아가게 바꿔보세요.
- 상태마다 다른 배경색을 쓰게 해보세요.
