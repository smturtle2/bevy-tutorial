# 17. 완성된 RPG 조각


<div align="center">

[목차](index.md) · [← 이전: 진행 저장/불러오기](16-save-load-progress.md) · [다음: 목차 →](index.md)

</div>

---

실행:

```sh
cargo run --example 17_complete_rpg_slice
```

이 장의 계약은 앞 장들의 기능을 하나의 작동하는 작은 RPG 루프로 합치는 것입니다. 메뉴에서 시작하고, 플레이 중에는 이동, 적 웨이브, 공격, 수집, 벽 충돌, 카메라, 애니메이션, HUD, 저장이 같은 ECS 스케줄 안에서 협력합니다.

![완성 RPG 조각은 카메라 추적, 적 웨이브, 공격 히트박스, 스프라이트 애니메이션, 화면 고정 HUD, 맵 지오메트리, 진행 저장이 함께 동작하는 장면을 보여줍니다.](../../assets/screenshots/ch17-complete-rpg-gameplay.png)

## 핵심 ECS 계약

- `GameState`: `Menu`, `Playing`, `Paused`, `GameOver` 흐름을 제어합니다.
- `GameSet`: `Input -> Wave -> Ai -> Movement -> Collision -> Animation -> Ui` 실행 순서를 고정합니다.
- `SpriteAssets`: 플레이어 시트, 적, 보석, 공격 이미지 핸들과 아틀라스 레이아웃을 모은 리소스입니다.
- `RunStats`: 현재 플레이의 점수와 웨이브입니다.
- `Progress`: 디스크에 저장되는 최고 점수와 해금 웨이브입니다.
- `GameplayEntity`: 한 판이 끝날 때 정리할 엔티티 경계입니다.

기능이 많아져도 기본 규칙은 같습니다. 데이터는 컴포넌트/리소스에 있고, 시스템은 읽기/쓰기 계약을 시그니처로 드러냅니다.

`Progress.unlocked_wave`는 표시만 하는 값이 아닙니다. 새 run은 저장된 해금 웨이브에서 시작합니다. 현재 run 기록은 `RunStats`, 영구 기록은 `Progress`로 분리합니다.

## Rust 포인트

이 예제는 작은 타입을 많이 씁니다. `Health`, `Facing`, `RunStats`, `SaveStatus`, `SpriteAssets`는 모두 단순하지만 서로 다른 의미를 가진 데이터입니다. Rust에서는 이런 타입 분리가 컴파일러가 실수를 잡을 수 있는 경계를 만듭니다.

`start_run(commands, assets, stats, spawner)`는 새 게임 시작에 필요한 초기화를 한곳에 모읍니다. `*stats = RunStats::default();`처럼 mutable 참조가 가리키는 값을 교체하고, `spawner.reset()`으로 웨이브 리소스를 초기 상태로 되돌립니다.

## Bevy 포인트

시작 시 `setup_camera_and_assets`가 카메라와 `SpriteAssets` 리소스를 준비합니다. 실제 플레이 엔티티는 메뉴에서 Enter를 누를 때 생성됩니다. 따라서 에셋 준비와 한 판의 수명은 분리되어 있습니다.

상태와 세트가 함께 쓰입니다. `run_if(in_state(GameState::Playing))`은 플레이 중에만 시스템을 돌리고, `GameSet` 체인은 플레이 중 시스템들의 순서를 보장합니다.

HUD는 화면 UI이고, 플레이어/적/벽/보석은 월드 엔티티입니다. 카메라는 월드를 따라가지만 HUD는 화면에 고정됩니다.

## 프레임 흐름

1. 메뉴에서 Enter를 누르면 `start_run`이 플레이어, 맵, HUD, 보석을 생성합니다.
2. 입력 세트가 이동 방향과 공격 히트박스를 만듭니다.
3. 웨이브 세트가 타이머에 따라 적을 생성합니다.
4. AI 세트가 적의 속도를 플레이어 방향으로 정합니다.
5. 이동 세트가 위치, 벽 충돌, 카메라 추적을 처리합니다.
6. 충돌 세트가 보석 수집, 공격 판정, 적 접촉 데미지, 사망 체크를 처리합니다.
7. 애니메이션 세트가 플레이어 아틀라스 프레임을 바꿉니다.
8. UI 세트가 저장 단축키와 HUD 갱신을 처리합니다.
9. `F5`는 현재 run 기록을 `Progress`에 반영한 뒤 저장하고, `F9`는 디스크에서 진행도를 다시 읽습니다.
10. 게임오버에 들어가면 진행도를 갱신하고 `target/tutorial-save/complete-progress.json`에 저장합니다.

## 흔한 실수

- 새 플레이 엔티티에 `GameplayEntity`를 빼면 게임오버나 메뉴 복귀 때 정리되지 않습니다.
- 상태 조건 없이 시스템을 등록하면 메뉴에서도 전투, 웨이브, 충돌이 실행됩니다.
- `SpriteAssets`를 준비하기 전에 `start_run`을 호출하면 스프라이트 핸들을 받을 수 없습니다.
- 세트 순서를 풀어버리면 공격, 충돌, HUD가 이전 프레임 데이터를 볼 수 있습니다.
- 저장되는 `Progress`와 현재 판의 `RunStats`를 섞지 마세요. 현재 판 기록은 `RunStats`, 영구 기록은 `Progress`입니다.

## 읽는 순서

이 파일을 처음부터 끝까지 한 번에 외우려 하지 마세요. 먼저 `main`의 상태/세트 등록을 읽고, 다음으로 `start_run`이 어떤 엔티티를 만드는지 확인하세요. 그다음 각 세트의 시스템을 순서대로 따라가면 전체 루프가 보입니다.

## 작게 바꿔보기

- 적 투사체를 추가하세요.
- 두 번째 collectible 타입을 추가하세요.
- 게임오버 저장 시 더 많은 진행도 필드를 저장하세요.
- 경계가 충분히 익숙해지면 최종 예제를 모듈로 분리하세요.

---

<div align="center">

[← 이전: 진행 저장/불러오기](16-save-load-progress.md) · [목차](index.md) · [다음: 목차 →](index.md)

</div>
