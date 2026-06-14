# 8. 부드러운 카메라 추적

<div align="center">

[목차](index.md) · [← 이전: RPG 기초 예제](07-rpg-slice.md) · [다음: 적 웨이브 →](09-enemy-waves.md)

</div>

---

## 이 장에서 만들 것

이 장이 끝나면 카메라가 플레이어 위치로 즉시 순간이동하지 않고, 부드럽게 따라옵니다.

![카메라가 큰 맵 기준선을 보며 플레이어를 부드럽게 따라갑니다.](../../assets/screenshots/ch08-smooth-camera-follow.png)

## 실행

```sh
cargo run --example 08_smooth_camera_follow
```

WASD나 방향키로 움직입니다. 카메라는 처음에 플레이어와 떨어진 곳에서 시작하고, 목표 지점으로 미끄러지듯 따라옵니다.

## 구현 흐름 1: 카메라가 따라갈 대상을 저장하기

카메라는 어떤 엔티티를 따라갈지 기억해야 합니다.

```rust
#[derive(Component)]
struct CameraFollow {
    target: Entity,
    offset: Vec3,
    smoothness: f32,
}
```

이 컴포넌트는 플레이어가 아니라 카메라에 붙습니다. 따라가는 동작의 주체는 카메라이기 때문입니다.

## 구현 흐름 2: 플레이어 Entity ID 잡기

`commands.spawn(...).id()`는 방금 만든 엔티티 ID를 돌려줍니다.

```rust
let player = commands.spawn(PlayerBundle::new(&asset_server)).id();
```

카메라는 그 ID를 저장합니다.

```rust
commands.spawn((
    Camera2d,
    Transform::from_xyz(-420.0, 260.0, 0.0),
    CameraFollow {
        target: player,
        offset: Vec3::new(0.0, 0.0, 0.0),
        smoothness: CAMERA_SMOOTHNESS,
    },
));
```

전역 “플레이어 위치” 리소스를 만들 필요가 없습니다. 이 카메라가 저 엔티티를 따라간다는 관계를 컴포넌트에 직접 담습니다.

## 구현 흐름 3: 큰 맵 안에서 플레이어 움직이기

플레이어는 여전히 `Transform`으로 움직입니다. 다만 예제에서는 큰 맵 밖으로 나가지 않도록 clamp합니다.

```rust
player.translation.x = player
    .translation
    .x
    .clamp(-MAP_HALF_SIZE.x, MAP_HALF_SIZE.x);
```

맵의 격자선은 카메라 움직임을 보이게 하려고 둔 기준입니다. 배경 기준이 없으면 카메라가 움직이는지 알아보기 어렵습니다.

## 구현 흐름 4: 목표 지점으로 보간하기

카메라 추적 시스템은 대상 Transform을 읽고 카메라 Transform을 수정합니다.

```rust
fn smooth_follow_camera(
    time: Res<Time>,
    targets: Query<&Transform, Without<Camera2d>>,
    mut cameras: Query<(&CameraFollow, &mut Transform), With<Camera2d>>,
) {
    for (follow, mut camera_transform) in &mut cameras {
        let Ok(target_transform) = targets.get(follow.target) else {
            continue;
        };

        let target = Vec3::new(
            target_transform.translation.x,
            target_transform.translation.y,
            camera_transform.translation.z,
        ) + follow.offset;
        let blend = 1.0 - (-follow.smoothness * time.delta_secs()).exp();

        camera_transform.translation = camera_transform.translation.lerp(target, blend);
    }
}
```

`targets.get(follow.target)`은 특정 엔티티 하나의 transform을 가져옵니다. 그 엔티티가 사라졌다면 이 카메라는 이번 프레임에 건너뜁니다.

## 구현 흐름 5: 지수 보간 사용하기

blend 값은 이렇게 계산합니다.

```rust
let blend = 1.0 - (-follow.smoothness * time.delta_secs()).exp();
```

이 방식은 프레임레이트가 달라도 따라가는 느낌이 크게 흔들리지 않습니다. `smoothness`가 높으면 더 빨리 따라오고, 낮으면 더 묵직하게 따라옵니다.

`lerp`는 현재 카메라 위치에서 목표 위치로 보간합니다.

```rust
camera_transform.translation = camera_transform.translation.lerp(target, blend);
```

## Rust로 보면

여기서 `let else`가 나옵니다.

```rust
let Ok(target_transform) = targets.get(follow.target) else {
    continue;
};
```

`targets.get(...)`은 `Result`를 반환합니다. 성공하면 transform을 바인딩하고, 실패하면 다음 카메라로 넘어갑니다.

target이 없어질 수도 있는 구조에서는 `unwrap`보다 이런 처리가 더 분명합니다.

## Bevy로 보면

카메라도 평범한 엔티티입니다. 이 컴포넌트들을 가집니다.

```text
Camera2d
Transform
CameraFollow
```

카메라는 여러 개 있을 수 있습니다. 어떤 카메라를 어떤 순서로 어디에 렌더링할지는 별도의 설정으로 정합니다. 이 장에서는 하나의 메인 월드 카메라만 쓰지만, 설계상 플레이어가 카메라를 알 필요는 없습니다.

## 확인

실행합니다.

```sh
cargo run --example 08_smooth_camera_follow
```

기대 결과:

- 플레이어는 즉시 움직입니다.
- 카메라는 플레이어 위치로 부드럽게 따라옵니다.
- 격자선 덕분에 카메라 이동이 눈에 보입니다.

## 바꿔보기

이 값을:

```rust
const CAMERA_SMOOTHNESS: f32 = 9.0;
```

이렇게 바꿔 봅니다.

```rust
const CAMERA_SMOOTHNESS: f32 = 2.0;
```

기대 결과: 카메라가 플레이어 뒤에 더 길게 따라붙습니다.

---

<div align="center">

[← 이전: RPG 기초 예제](07-rpg-slice.md) · [목차](index.md) · [다음: 적 웨이브 →](09-enemy-waves.md)

</div>
