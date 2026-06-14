# 12. 화면 고정 UI

[목차](index.md) | 이전: [sprite asset](11-sprite-assets.md) | 다음: [애니메이션 상태](13-animation-state.md)

실행:

```sh
cargo run --example 12_screen_space_ui
```

이 장의 계약은 월드 오브젝트와 화면 UI를 분리하는 것입니다. 플레이어와 배경은 카메라 이동의 영향을 받지만, `Node` 기반 UI는 화면 좌표에 고정됩니다.

## 핵심 ECS 계약

- `MainCamera`: 추적할 카메라를 찾는 마커입니다.
- `Health { current, max }`: 플레이어 체력 데이터입니다.
- `Score(u32)`: 점수 리소스입니다.
- `HealthText`, `ScoreText`, `HealthBarFill`: 각각 갱신할 UI 엔티티를 찾는 마커입니다.
- `update_screen_space_ui`: 게임 데이터를 읽고 UI 컴포넌트를 씁니다.

UI 갱신 시스템은 `Score`, 플레이어 `Health`, 텍스트 `Text`, 체력바 `Node`를 함께 다룹니다. 규칙은 명확합니다. 게임 상태는 데이터 컴포넌트/리소스에 있고, UI는 매 프레임 그 상태를 표시합니다.

## Rust 포인트

`Without` 필터는 같은 컴포넌트 타입을 여러 쿼리에서 mutable로 빌릴 때 충돌을 피합니다.

```rust
Single<&mut Text, (With<HealthText>, Without<ScoreText>)>
Single<&mut Text, (With<ScoreText>, Without<HealthText>)>
```

둘 다 `Text`를 mutable로 요구하지만 서로 다른 엔티티라는 사실을 필터로 Bevy에 알려줍니다.

`health.current as f32 / health.max as f32`는 정수 나눗셈을 피하기 위한 변환입니다. 체력바 너비 계산은 부동소수점 비율이어야 합니다.

## Bevy 포인트

`Node { position_type: PositionType::Absolute, top: px(...), left: px(...) }`는 UI를 화면 기준 위치에 둡니다. 카메라가 플레이어를 따라 움직여도 이 UI는 월드 좌표가 아니라 화면 레이아웃 시스템에 의해 배치됩니다.

체력바는 배경 노드와 채움 노드를 같은 위치에 겹쳐 만들고, 채움 노드의 `width`만 바꿉니다.

## 프레임 흐름

1. 플레이어가 이동합니다.
2. 카메라가 플레이어 위치로 이동합니다.
3. 디버그 입력이 체력과 점수를 바꿉니다.
4. UI 시스템이 텍스트와 체력바 너비를 현재 데이터에 맞춥니다.

## 흔한 실수

- HUD를 `Sprite`로 만들면 카메라 이동에 같이 흔들립니다. 화면 고정 정보는 `Node`/`Text` UI로 만드세요.
- 같은 `Text` 타입을 여러 `Single<&mut Text>`로 빌리면서 `Without`을 빼면 쿼리 충돌이 납니다.
- 체력 비율 계산에서 정수 나눗셈을 하면 0 또는 1만 나옵니다.
- UI는 표시일 뿐입니다. 점수나 체력의 원본 데이터를 UI 텍스트에서 다시 파싱하지 마세요.

## 작게 바꿔보기

- HUD를 화면 오른쪽 위로 옮겨보세요.
- stamina bar를 하나 추가하세요.
- 체력이 낮을 때 health bar 색을 빨간색으로 바꿔보세요.
