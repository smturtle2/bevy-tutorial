# 3. ECS 기본

[목차](index.md) | 이전: [Bevy 앱 모델](02-bevy-app-model.md) | 다음: [입력과 이동](04-input-and-movement.md)

ECS는 Entity Component System입니다.

![ECS 개요](../../assets/diagrams/ecs-overview.png)

이 장은 이동 예제가 흥미로워지기 전에 필요한 어휘를 설명합니다.

## Entity

엔티티는 월드 안의 ID입니다.

엔티티 자체가 위치, 체력, 속도, 렌더링, 행동을 본질적으로 갖는 것은 아닙니다. 우리가 붙인 컴포넌트만 가집니다.

`examples/02_spawn_sprite.rs`에서 이 줄은 엔티티 하나를 만듭니다.

```rust
commands.spawn((
    Sprite::from_color(Color::srgb(0.25, 0.70, 1.0), Vec2::splat(80.0)),
    Transform::from_translation(Vec3::ZERO),
));
```

그 엔티티는 `Sprite`가 있어서 렌더링 가능하고, `Transform`이 있어서 배치 가능합니다.

## Component

컴포넌트는 엔티티에 붙는 데이터입니다.

```rust
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);
```

`Player`는 marker component입니다. 엔티티가 플레이어임을 표시합니다.

`Velocity`는 data component입니다. 이동 방향이나 속도를 `Vec2`로 저장합니다.

최종 예제는 marker component와 data component를 모두 씁니다.

```rust
#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Body {
    half_size: Vec2,
}
```

`Enemy`는 역할을 표시합니다. `Body`는 충돌 크기를 저장합니다.

## 컴포넌트 튜플 Spawn

`commands.spawn`은 bundle을 받습니다. 컴포넌트 튜플은 간단한 bundle입니다.

```rust
commands.spawn((
    Player,
    Velocity(Vec2::ZERO),
    Transform::from_translation(Vec3::ZERO),
));
```

이 코드는 엔티티 하나를 만들고 컴포넌트 세 개를 붙입니다.

튜플 순서가 엔티티의 정체성이 아닙니다. 컴포넌트 타입이 정체성입니다. 쿼리는 spawn 순서가 아니라 컴포넌트 형태로 엔티티를 찾습니다.

## Query 기본

쿼리는 컴포넌트 형태로 엔티티를 선택합니다.

```rust
Query<&mut Transform, With<Player>>
```

이렇게 읽습니다.

```text
Find entities with Player.
From each one, mutably borrow Transform.
```

같은 엔티티에서 여러 컴포넌트를 가져올 수도 있습니다.

```rust
Query<(&mut Transform, &Velocity), With<Body>>
```

이렇게 읽습니다.

```text
Find entities with Body, Transform, and Velocity.
Mutate Transform.
Read Velocity.
```

데이터 접근이 드러납니다.

- `&Transform`: 컴포넌트 데이터를 읽습니다.
- `&mut Transform`: 컴포넌트 데이터를 씁니다.
- `With<Player>`: marker/component가 있어야 하지만 빌리지는 않습니다.
- `Without<Enemy>`: marker/component가 있는 엔티티를 제외합니다.

## Query Filter와 Conflict

필터는 어떤 엔티티가 매칭되는지 좁힙니다.

```rust
Query<&mut Velocity, With<Player>>
```

`Velocity`와 `Player`를 모두 가진 엔티티만 매칭됩니다. 이 쿼리는 `Velocity`를 수정하고, `Player`는 필터로만 사용합니다.

Bevy는 시스템 파라미터들이 안전하게 함께 실행될 수 있는지도 검사합니다. 같은 컴포넌트에 두 쿼리가 접근하고, 그중 하나 이상이 가변 접근이면 중요합니다.

카메라 follow 시스템에서 다음 형태는 애매합니다.

```rust
Query<&Transform, With<Player>>
Query<&mut Transform, With<Camera2d>>
```

두 쿼리 모두 `Transform`에 접근합니다. Bevy는 플레이어가 카메라가 아니라고 가정할 수 없습니다. 두 집합이 분리되어 있음을 증명하려면 필터를 추가합니다.

```rust
Query<&Transform, (With<Player>, Without<Camera2d>)>
Query<&mut Transform, (With<Camera2d>, Without<Player>)>
```

예제들은 대체로 시스템을 작게 유지해서 이 문제를 피합니다.

## Resource, `Res`, `ResMut`

리소스는 월드에 저장된 전역 값 하나입니다.

```rust
#[derive(Resource)]
struct BodySpeed(f32);
```

등록:

```rust
app.insert_resource(BodySpeed(220.0));
```

읽기:

```rust
fn move_bodies(speed: Res<BodySpeed>) {}
```

수정:

```rust
fn collect_items(mut score: ResMut<Score>) {
    score.0 += 1;
}
```

엔티티마다 필요한 데이터에는 컴포넌트를 쓰고, 하나만 공유하는 값에는 리소스를 씁니다.

이 튜토리얼에서:

- `BodySpeed`는 `examples/04_velocity_body.rs`의 공유 이동 속도입니다.
- `Score`는 `examples/07_rpg_slice.rs`의 공유 점수 값입니다.
- `Time`, `ButtonInput<KeyCode>`, `AssetServer`는 Bevy가 제공하는 리소스입니다.

## `Local`

`Local<T>`는 시스템 로컬 상태를 저장합니다. 컴포넌트도 아니고 전역 리소스도 아닙니다. `Local<T>`를 요청하는 각 시스템은 자기만의 지속 값을 받습니다.

최종 예제는 hit cooldown에 이것을 씁니다.

```rust
fn enemy_hits_player(
    time: Res<Time>,
    mut hit_cooldown: Local<f32>,
) {
    *hit_cooldown -= time.delta_secs();
}
```

`*`는 local wrapper를 역참조해서 내부 `f32`를 바꿀 수 있게 합니다.

다른 시스템이 검사할 필요가 없고 한 시스템에만 속한 상태라면 `Local`을 쓰세요.

## `Single`

`Single`은 정확히 하나의 엔티티가 매칭되어야 하는 경우에 쓰는 쿼리 파라미터입니다.

```rust
fn enemy_ai(
    player: Single<&Transform, With<Player>>,
    mut enemies: Query<(&Transform, &mut Velocity), With<Enemy>>,
) {
    let player_position = player.translation.truncate();
}
```

이것은 강한 계약입니다. `Transform`을 가진 `Player`가 정확히 하나 있어야 합니다. 게임이 일시적으로 0개나 여러 개를 가질 수 있다면 `Query`와 `single()`을 쓰고 `Result`를 처리하세요.

```rust
let Ok(player) = players.single() else {
    return;
};
```

## 시스템 파라미터가 계약입니다

`examples/07_rpg_slice.rs`의 이 시스템을 보세요.

```rust
fn move_bodies(time: Res<Time>, mut bodies: Query<(&mut Transform, &Velocity), With<Body>>) {
    for (mut transform, velocity) in &mut bodies {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
    }
}
```

본문을 읽기 전에도 알 수 있습니다.

```text
Reads Time.
Finds entities with Body, Transform, and Velocity.
Mutates Transform.
Reads Velocity.
Does not spawn or despawn entities.
Does not read keyboard input.
Does not touch health or score.
```

그래서 Bevy 코드는 시스템이 작을수록 쉬워집니다.

## 체크포인트

아래 시스템 각각이 무엇을 읽고 쓰는지 구현을 보기 전에 적어 보세요.

- `player_input` in `examples/07_rpg_slice.rs`
- `collect_items` in `examples/07_rpg_slice.rs`
- `update_health_bar` in `examples/07_rpg_slice.rs`

## 흔한 실수

- 엔티티마다 필요한 데이터를 리소스에 넣음.
- marker를 `With<Player>`로 충분히 필터링할 수 있는데 `&Player`로 쿼리함.
- `Commands`가 지연된다는 사실을 잊음.
- 입력 읽기, 엔티티 이동, 충돌 처리, 점수 갱신, 표시 갱신을 모두 한 큰 시스템에 넣음.
