# 3. ECS 기본

<div align="center">

[목차](index.md) · [← 이전: Bevy 앱 모델](02-bevy-app-model.md) · [다음: 입력과 이동 →](04-input-and-movement.md)

</div>

---

## 이 장에서 만들 것

이 장이 끝나면 엔티티의 정체성, 컴포넌트 데이터, 리소스, 시스템, 쿼리, 필터, `Local`, `Single`을 구분할 수 있습니다. 이후 모든 게임플레이 기능은 이 개념들 위에서 만들어집니다.

![ECS 개요 다이어그램](../../assets/diagrams/ecs-overview.png)

## 실행

```sh
cargo run --example 02_spawn_sprite
```

이 예제는 가장 작은 ECS 장면입니다. 카메라 엔티티 하나, 스프라이트 엔티티 하나가 있습니다.

## 구현 흐름 1: Entity는 ID다

엔티티는 Bevy 월드 안의 ID입니다. 클래스 인스턴스가 아니고, 메서드를 들고 있는 객체도 아닙니다.

이렇게 쓰면:

```rust
commands.spawn((
    Player,
    Sprite::from_color(Color::srgb(0.25, 0.70, 1.0), Vec2::splat(80.0)),
    Transform::from_translation(Vec3::ZERO),
));
```

Bevy는 엔티티 ID를 만들고 그 ID에 컴포넌트 세 개를 붙입니다.

```text
Entity 42
  Player
  Sprite
  Transform
```

보통 숫자 ID 자체는 신경 쓰지 않습니다. 특정 엔티티를 나중에 다시 찾아야 할 때만 ID를 저장합니다.

## 구현 흐름 2: Component는 타입 있는 데이터다

컴포넌트는 엔티티에 붙는 Rust 타입입니다.

```rust
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Body {
    half_size: Vec2,
}
```

핵심은 타입입니다. 한 엔티티는 같은 타입의 컴포넌트를 하나만 가질 수 있습니다. `Player`와 `Body`를 동시에 가질 수는 있지만, `Body`를 두 개 붙일 수는 없습니다.

## 구현 흐름 3: System은 ECS 데이터를 다루는 함수다

시스템은 Bevy가 실행할 수 있는 Rust 함수입니다.

```rust
fn move_bodies(
    time: Res<Time>,
    mut bodies: Query<(&mut Transform, &Velocity), With<Body>>,
) {
    for (mut transform, velocity) in &mut bodies {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
    }
}
```

중요한 것은 함수 본문보다 시그니처입니다. 이 시그니처는 이렇게 말합니다.

```text
읽는 리소스: Time
대상 엔티티: Body + Transform + Velocity
쓰는 컴포넌트: Transform
읽는 컴포넌트: Velocity
```

## 구현 흐름 4: Query는 조건에 맞는 엔티티를 고른다

`Query<&mut Transform, With<Player>>`는 이렇게 읽습니다.

```text
Player와 Transform이 있는 모든 엔티티에 대해
Transform을 수정 가능하게 가져온다.
```

튜플로 쓴 쿼리 데이터는 같은 엔티티에서 여러 컴포넌트를 가져옵니다.

```rust
Query<(&mut Transform, &Velocity), With<Body>>
```

Filter는 대상을 좁힙니다.

```rust
With<Player>       Player가 있어야 함
Without<Camera2d>  Camera2d가 없어야 함
```

`Without`은 두 쿼리가 같은 `Transform`을 건드릴 가능성을 끊을 때 자주 씁니다.

## 구현 흐름 5: 리소스는 전역 값 하나다

리소스는 월드에 하나만 저장되는, 타입이 정해진 값입니다.

```rust
#[derive(Resource)]
struct PlayerSpeed(f32);
```

등록은 이렇게 합니다.

```rust
.insert_resource(PlayerSpeed(280.0))
```

읽을 때는:

```rust
fn move_player(speed: Res<PlayerSpeed>) {
    let pixels_per_second = speed.0;
}
```

수정할 때는:

```rust
fn add_score(mut score: ResMut<Score>) {
    score.0 += 1;
}
```

점수, 웨이브 스포너, 저장 데이터, 로드된 에셋 핸들, 설정값처럼 전역으로 하나 있으면 되는 데이터에 리소스를 씁니다.

## 구현 흐름 6: `Local`은 시스템 혼자 쓰는 기억이다

`Local<T>`는 한 시스템만 쓰는 상태를 저장합니다.

```rust
fn enemy_hits_player(
    time: Res<Time>,
    mut hit_cooldown: Local<f32>,
) {
    *hit_cooldown -= time.delta_secs();
}
```

그 상태를 한 시스템만 쓴다면 `Local`이 좋습니다. 여러 시스템이 같이 읽거나 써야 하면 리소스로 올립니다.

## 구현 흐름 7: `Single`은 정확히 하나를 기대한다

`Single`은 설계상 대상이 하나여야 할 때 씁니다.

```rust
fn follow_player(
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}
```

플레이어 하나, 메인 카메라 하나, HUD 텍스트 하나처럼 “정확히 하나”가 맞는 곳에 씁니다. 0개일 수도 있고 여러 개일 수도 있으면 `Query`가 맞습니다.

## Rust로 보면

이 `for`문은 Bevy 전용 문법이 아닙니다.

```rust
for (mut transform, velocity) in &mut bodies {
}
```

`&mut bodies`는 쿼리를 가변 순회로 빌립니다. 반복문 변수의 모양은 쿼리가 요청한 튜플 구조와 같습니다. `mut transform`은 이 바인딩을 통해 수정 가능한 컴포넌트 참조를 쓸 수 있다는 뜻입니다.

Tuple struct 접근도 자주 나옵니다.

```rust
velocity.0
score.0
```

이 `.0`은 Rust tuple field 문법입니다. Bevy 문법이 아닙니다.

## Bevy로 보면

ECS는 “변수를 모두 가진 객체 하나” 모델이 아닙니다. 타입 있는 데이터 열이 있고, 시스템이 필요한 데이터 열을 선언해서 가져가는 모델입니다. Bevy는 그 선언을 보고 같이 실행해도 되는 시스템을 판단합니다.

실전 기준은 이렇게 잡으면 됩니다.

```text
표식 컴포넌트        이 엔티티가 무엇인지 표시합니다
데이터 컴포넌트      이 엔티티가 가진 사실을 저장합니다
Resource             전역으로 하나인 사실을 저장합니다
System               이번 프레임에 무엇을 바꿀지 정의합니다
```

## 확인

다음 시그니처를 보고 설명할 수 있으면 됩니다.

```rust
fn collect_items(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player: Single<(&Transform, &Body), With<Player>>,
    collectibles: Query<(Entity, &Transform, &Body), With<Collectible>>,
) {
}
```

읽으면 이렇습니다.

```text
엔티티를 `despawn`할 수 있음
전역 score를 수정할 수 있음
Transform과 Body를 가진 플레이어 하나를 기대함
모든 collectible의 Entity, Transform, Body를 순회함
```

## 바꿔보기

`examples/04_velocity_body.rs`에서 `move_bodies` 쿼리의 `With<Body>`를 지워 봅니다.

```rust
mut bodies: Query<(&mut Transform, &Velocity)>,
```

기대 결과: 이 예제에서는 동작이 그대로입니다. `Velocity`가 있는 엔티티가 전부 body이기 때문입니다. 하지만 나중에 더 많은 엔티티가 생기면 필터가 중요한 설계 도구가 됩니다.

---

<div align="center">

[← 이전: Bevy 앱 모델](02-bevy-app-model.md) · [목차](index.md) · [다음: 입력과 이동 →](04-input-and-movement.md)

</div>
