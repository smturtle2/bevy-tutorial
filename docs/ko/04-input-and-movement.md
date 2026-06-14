# 4. 입력과 이동

<div align="center">

[목차](index.md) · [← 이전: ECS 기본](03-ecs-fundamentals.md) · [다음: 번들, 플러그인, 세트 →](05-bundles-plugins-sets.md)

</div>

---

## 이 장에서 만들 것

이 장이 끝나면 키보드 입력으로 플레이어를 움직이는 방식을 두 가지로 볼 수 있습니다.

1. 바로 `Transform`을 수정해서 이동합니다.
2. 입력은 `Velocity`만 쓰고, 별도의 Body 이동 시스템이 `Transform`을 수정합니다.

두 번째 방식이 재사용 가능한 게임플레이 코드의 시작입니다.

![플레이어 입력은 Velocity를 쓰고, Body 이동은 Transform을 씁니다.](../../assets/screenshots/ch04-velocity-body.png)

## 실행

```sh
cargo run --example 03_player_input
cargo run --example 04_velocity_body
```

두 예제 모두 WASD나 방향키로 움직입니다.

## 구현 흐름 1: 키보드 상태 읽기

입력 리소스는 이렇게 생겼습니다.

```rust
Res<ButtonInput<KeyCode>>
```

직접 이동하는 예제는 키 입력으로 방향 벡터를 만듭니다.

```rust
let mut direction = Vec2::ZERO;

if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
    direction.x -= 1.0;
}
if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
    direction.x += 1.0;
}
if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
    direction.y -= 1.0;
}
if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
    direction.y += 1.0;
}
```

`pressed`는 키를 누르고 있는 동안 계속 true입니다. 나중에 공격이나 메뉴에서는 `just_pressed`를 씁니다. `just_pressed`는 키가 눌린 바로 그 프레임에만 true입니다.

## 구현 흐름 2: 대각선 속도 보정하기

정규화를 하지 않으면 대각선 이동이 더 빨라집니다. `(1, 1)`의 길이가 `(1, 0)`보다 길기 때문입니다.

```rust
let movement = direction.normalize_or_zero() * speed.0 * time.delta_secs();
```

`normalize_or_zero()`는 대각선 속도를 맞추고, 방향이 0일 때도 안전하게 0을 돌려줍니다.

## 구현 흐름 3: 프레임 독립 이동 만들기

`time.delta_secs()`는 이전 프레임 이후 지난 시간입니다.

```rust
transform.translation += movement.extend(0.0);
```

공식은 이렇습니다.

```text
이번 프레임 이동량 = 방향 * 초당 픽셀 수 * 이번 프레임에 지난 초
```

그래서 프레임레이트가 달라도 이동 속도가 크게 달라지지 않습니다.

## 구현 흐름 4: `Transform`을 직접 움직이기

직접 이동 버전은 플레이어의 `Transform`을 가져옵니다.

```rust
fn move_player(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    speed: Res<PlayerSpeed>,
    mut players: Query<&mut Transform, With<Player>>,
) {
    let movement = direction.normalize_or_zero() * speed.0 * time.delta_secs();

    for mut transform in &mut players {
        transform.translation += movement.extend(0.0);
    }
}
```

플레이어 하나만 움직이는 작은 예제에서는 단순하고 좋습니다. 다만 입력 처리와 이동 물리가 한 시스템 안에 섞입니다.

## 구현 흐름 5: `Velocity` 도입하기

리팩터링 버전은 이동 데이터를 따로 둡니다.

```rust
#[derive(Component)]
struct Velocity(Vec2);
```

입력 시스템은 이동 의도만 `Velocity`에 기록합니다.

```rust
fn handle_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in &mut players {
        velocity.0 = direction.normalize_or_zero();
    }
}
```

이동 시스템은 속도를 적용합니다.

```rust
fn move_bodies(
    time: Res<Time>,
    speed: Res<BodySpeed>,
    mut bodies: Query<(&mut Transform, &Velocity), With<Body>>,
) {
    let movement_scale = speed.0 * time.delta_secs();

    for (mut transform, velocity) in &mut bodies {
        transform.translation += (velocity.0 * movement_scale).extend(0.0);
    }
}
```

이제 적, 발사체, NPC도 `Body`, `Velocity`, `Transform`을 가지면 같은 `move_bodies` 시스템으로 움직일 수 있습니다.

## 구현 흐름 6: `.chain()`으로 순서 정하기

이 구조에서는 입력이 이동보다 먼저 실행되어야 합니다.

```rust
.add_systems(
    Update,
    (handle_player_input, move_bodies, capture_velocity_body_pose).chain(),
)
```

`.chain()`은 tuple 안의 시스템을 순서대로 실행하게 만듭니다.

순서를 정하지 않으면 Bevy는 가능한 순서로 시스템을 실행할 수 있습니다. 병렬 실행에는 좋지만, 한 시스템이 쓴 값을 같은 프레임의 다른 시스템이 읽어야 한다면 순서를 명시해야 합니다.

## Rust로 보면

이 줄은 Rust 관점에서 세 가지를 합니다.

```rust
for mut transform in &mut players {
```

```text
&mut players       쿼리를 수정 가능하게 빌려 순회
transform          매칭된 Transform을 가리키는 지역 바인딩
mut transform      mutable reference를 통해 값을 쓸 수 있음
```

튜플 반복문은 쿼리 데이터와 모양이 같습니다.

```rust
for (mut transform, velocity) in &mut bodies {
```

첫 번째 값은 `&mut Transform`을 요청했기 때문에 수정 가능하고, 두 번째 값은 `&Velocity`를 요청했기 때문에 읽기 전용입니다.

## Bevy로 보면

리팩터링의 핵심은 책임 분리입니다.

```text
Player input system     keyboard -> Velocity
Body movement system    Velocity + Time -> Transform
```

그래서 시스템 이름을 `update_players`처럼 뭉뚱그리면 금방 흐려집니다. 시스템 이름은 맡은 동작을 말해야 합니다. `handle_player_input`, `move_bodies`, `enemy_ai`, `collect_items`, `update_hud_text`처럼요.

## 확인

두 예제를 실행합니다.

```sh
cargo run --example 03_player_input
cargo run --example 04_velocity_body
```

기대 결과:

- 두 예제 모두 WASD/방향키로 움직입니다.
- 대각선 이동이 직선 이동보다 눈에 띄게 빠르지 않습니다.
- `04_velocity_body`에서는 입력 시스템이 `Transform`을 직접 건드리지 않는데도 플레이어가 움직입니다.

## 바꿔보기

`examples/04_velocity_body.rs`에서 이 값을 바꿔 봅니다.

```rust
.insert_resource(BodySpeed(220.0))
```

```rust
.insert_resource(BodySpeed(80.0))
```

기대 결과: 입력 시스템을 바꾸지 않아도 플레이어가 느려집니다. 속도 책임이 키보드 입력이 아니라 Body 이동 쪽에 있다는 뜻입니다.

---

<div align="center">

[← 이전: ECS 기본](03-ecs-fundamentals.md) · [목차](index.md) · [다음: 번들, 플러그인, 세트 →](05-bundles-plugins-sets.md)

</div>
