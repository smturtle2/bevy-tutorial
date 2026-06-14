# 8. 부드러운 카메라 추적


<div align="center">

[목차](index.md) · [← 이전: RPG 기초 조각](07-rpg-slice.md) · [다음: 적 웨이브 →](09-enemy-waves.md)

</div>

---

실행:

```sh
cargo run --example 08_smooth_camera_follow
```

![부드러운 카메라 추적 예제는 큰 그리드 월드 안에서 카메라가 대상 엔티티를 향해 보간되는 장면을 보여줍니다.](../../assets/screenshots/ch08-smooth-camera-follow.png)

이 장의 계약은 간단합니다. 플레이어는 입력으로 움직이고, 카메라는 플레이어 엔티티를 직접 소유하지 않고 `Entity` ID로 참조합니다. 카메라 시스템은 그 ID로 대상 `Transform`을 조회한 뒤 현재 위치에서 목표 위치로 조금씩 보간합니다.

## 핵심 ECS 계약

- `Player`: 입력을 받는 대상입니다.
- `CameraFollow { target, offset, smoothness }`: 카메라가 따라갈 엔티티와 추적 규칙입니다.
- `move_player`: `ButtonInput<KeyCode>`, `Time`, `Single<&mut Transform, With<Player>>`를 사용해 플레이어 위치를 갱신합니다.
- `smooth_follow_camera`: 모든 `CameraFollow` 카메라를 순회하고, `Query<&Transform, Without<Camera2d>>`에서 `target`의 위치를 읽습니다.

`CameraFollow.target`은 `commands.spawn(...).id()`로 얻은 플레이어 엔티티입니다. 이 방식은 부모-자식 관계를 만들지 않고도 한 엔티티가 다른 엔티티를 참조하게 합니다.

## Rust 포인트

`let Ok(target_transform) = targets.get(follow.target) else { continue; };`는 실패 가능한 조회를 명시적으로 처리합니다. ECS에서는 엔티티 수명이 런타임에 바뀌므로, 대상이 없는 경우도 시스템 흐름 안에서 다루고 다음 카메라로 넘어갑니다.

`Vec3::lerp(target, blend)`는 현재 값과 목표 값 사이의 값을 만듭니다. `blend`는 `1.0 - (-smoothness * delta).exp()`로 계산되어 프레임 시간이 달라도 비슷한 감속 느낌을 냅니다.

## Bevy 포인트

`Camera2d`도 엔티티에 붙는 컴포넌트입니다. 따라서 카메라에 `Transform`과 `CameraFollow`를 함께 붙이면 일반 게임 오브젝트처럼 시스템에서 움직일 수 있습니다.

시스템은 `(move_player, smooth_follow_camera).chain()`으로 등록됩니다. 같은 프레임에서 플레이어 이동을 먼저 반영한 뒤 카메라가 그 결과를 읽습니다.

## 프레임 흐름

1. 입력을 읽어 플레이어 이동 벡터를 만듭니다.
2. `time.delta_secs()`를 곱해 프레임 독립 이동을 적용합니다.
3. 플레이어 위치를 맵 범위 안으로 `clamp`합니다.
4. 카메라가 대상 엔티티의 `Transform`을 조회합니다.
5. 카메라 위치를 목표 위치로 보간합니다.

## 흔한 실수

- `CameraFollow.target`에 카메라 자신의 엔티티를 넣으면 카메라가 플레이어를 따라가지 않습니다.
- `smooth_follow_camera`를 `move_player`보다 먼저 실행하면 카메라가 한 프레임 늦게 반응합니다.
- 카메라의 z값까지 플레이어 z값으로 바꾸면 렌더링 순서가 의도와 달라질 수 있습니다. 예제는 카메라의 기존 z값을 유지합니다.
- 대상 엔티티 조회는 `Query::get`으로 처리하면 상태 전환이나 게임오버 정리 뒤에도 시스템 흐름이 명확합니다.

## 작게 바꿔보기

- `CAMERA_SMOOTHNESS`를 `2.0`, `20.0`으로 바꿔 추적 느낌을 비교하세요.
- `CameraFollow.offset`에 `Vec3::new(120.0, 0.0, 0.0)`을 넣어보세요.
- 플레이어가 아니라 카메라 위치를 맵 경계 안으로 제한해보세요.

---

<div align="center">

[← 이전: RPG 기초 조각](07-rpg-slice.md) · [목차](index.md) · [다음: 적 웨이브 →](09-enemy-waves.md)

</div>
