# 7. RPG 기초 예제

<div align="center">

[목차](index.md) · [← 이전: 에셋, 카메라, UI](06-assets-camera-ui.md) · [다음: 부드러운 카메라 추적 →](08-smooth-camera-follow.md)

</div>

---

## 이 장에서 만들 것

이 장이 끝나면 작은 RPG 아레나 하나가 완성됩니다.

- 플레이어 이동
- 플레이어를 따라오는 적 AI
- 아레나 경계
- 수집 아이템
- 쿨다운이 있는 피해 처리
- 점수와 체력 표시
- 명시적인 시스템 실행 순서

![RPG 기초 예제는 이동, 적, 수집, 체력, 점수, HUD를 한 장면에 묶습니다.](../../assets/screenshots/ch07-rpg-slice.png)

## 실행

```sh
cargo run --example 07_rpg_slice
```

WASD나 방향키로 움직입니다. 노란 아이템을 먹고, 빨간 적은 피합니다.

## 구현 흐름 1: 프레임 단계를 먼저 정하기

기능이 많아졌기 때문에 시스템 순서에 이름을 붙입니다.

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Ai,
    Movement,
    Collision,
    Display,
}
```

등록할 때 이 enum을 프레임 파이프라인으로 만듭니다.

```rust
.configure_sets(
    Update,
    (
        GameSet::Input,
        GameSet::Ai,
        GameSet::Movement,
        GameSet::Collision,
        GameSet::Display,
    )
        .chain(),
)
```

이 순서 자체가 게임 규칙입니다.

```text
Input은 플레이어 속도를 정합니다
AI는 적 속도를 정합니다
Movement는 transform을 바꿉니다
Collision은 최종 위치를 읽습니다
Display는 최종 게임 상태를 화면에 반영합니다
```

## 구현 흐름 2: 공통 Body 데이터 만들기

이동과 충돌 시스템은 공통 데이터가 필요합니다.

```rust
#[derive(Component)]
struct Body {
    half_size: Vec2,
}

#[derive(Component)]
struct Velocity(Vec2);
```

`BodyBundle`은 생성 모양을 일정하게 유지합니다.

```rust
#[derive(Bundle)]
struct BodyBundle {
    body: Body,
    velocity: Velocity,
    transform: Transform,
}

impl BodyBundle {
    fn new(position: Vec3, size: Vec2) -> Self {
        Self {
            body: Body {
                half_size: size / 2.0,
            },
            velocity: Velocity(Vec2::ZERO),
            transform: Transform::from_translation(position),
        }
    }
}
```

플레이어, 적, 수집 아이템이 모두 이 Body 개념을 공유합니다.

## 구현 흐름 3: 도메인별 Bundle 만들기

플레이어 bundle은 플레이어 표식, Body 데이터, 렌더링, 체력을 묶습니다.

```rust
#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    body: BodyBundle,
    sprite: Sprite,
    health: Health,
}
```

적과 수집 아이템도 같은 Body 구조를 씁니다.

```rust
#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    body: BodyBundle,
    sprite: Sprite,
}

#[derive(Bundle)]
struct CollectibleBundle {
    collectible: Collectible,
    body: BodyBundle,
    sprite: Sprite,
}
```

ECS에서는 필요한 컴포넌트 조합이 엔티티의 역할을 만듭니다. 적은 `Enemy`, `Body`, `Velocity`, `Transform`, `Sprite`를 가진 엔티티입니다. 수집 아이템은 `Collectible`, `Body`, `Velocity`, `Transform`, `Sprite`를 가진 엔티티입니다.

## 구현 흐름 4: 플레이어 움직이기

플레이어 입력은 속도를 씁니다.

```rust
fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in &mut players {
        velocity.0 = direction.normalize_or_zero() * PLAYER_SPEED;
    }
}
```

Body 이동 시스템은 모든 Body에 속도를 적용합니다.

```rust
fn move_bodies(time: Res<Time>, mut bodies: Query<(&mut Transform, &Velocity), With<Body>>) {
    for (mut transform, velocity) in &mut bodies {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
    }
}
```

이 구조 덕분에 이동을 재사용할 수 있습니다. 플레이어 입력은 속도를 만드는 한 가지 방법일 뿐입니다.

## 구현 흐름 5: 적 추적 AI 추가하기

적 AI는 플레이어 위치를 읽고 적 속도를 씁니다.

```rust
fn enemy_ai(
    player: Single<&Transform, With<Player>>,
    mut enemies: Query<(&Transform, &mut Velocity), With<Enemy>>,
) {
    let player_position = player.translation.truncate();

    for (transform, mut velocity) in &mut enemies {
        let to_player = player_position - transform.translation.truncate();
        velocity.0 = to_player.normalize_or_zero() * ENEMY_SPEED;
    }
}
```

`truncate()`는 `Vec3`에서 `z`를 버리고 `Vec2`로 만듭니다. 이동 방향은 2D 문제이기 때문입니다.

## 구현 흐름 6: 아레나 안에 묶기

아레나 제한 시스템은 Body가 경계 밖으로 나가지 않게 합니다.

```rust
fn clamp_to_arena(mut bodies: Query<(&mut Transform, &Body), With<Body>>) {
    for (mut transform, body) in &mut bodies {
        let min = -ARENA_HALF_SIZE + body.half_size;
        let max = ARENA_HALF_SIZE - body.half_size;
        transform.translation.x = transform.translation.x.clamp(min.x, max.x);
        transform.translation.y = transform.translation.y.clamp(min.y, max.y);
    }
}
```

`half_size`가 중요합니다. 큰 엔티티의 중심은 작은 엔티티의 중심보다 더 안쪽에서 멈춰야 합니다.

## 구현 흐름 7: AABB 충돌 검사하기

이 튜토리얼은 축에 맞춘 바운딩 박스(AABB) 충돌을 씁니다.

```rust
fn overlaps(
    a_transform: &Transform,
    a_body: &Body,
    b_transform: &Transform,
    b_body: &Body,
) -> bool {
    let a = a_transform.translation.truncate();
    let b = b_transform.translation.truncate();
    let distance = (a - b).abs();
    let allowed = a_body.half_size + b_body.half_size;

    distance.x < allowed.x && distance.y < allowed.y
}
```

규칙은 단순합니다.

```text
x축 중심 거리 < 두 half width의 합
그리고
y축 중심 거리 < 두 half height의 합
이면 두 사각형은 겹칩니다
```

## 구현 흐름 8: 아이템 수집하기

아이템 수집은 엔티티를 제거하므로 `Commands`가 필요합니다.

```rust
fn collect_items(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player: Single<(&Transform, &Body), With<Player>>,
    collectibles: Query<(Entity, &Transform, &Body), With<Collectible>>,
) {
    let (player_transform, player_body) = *player;

    for (entity, transform, body) in &collectibles {
        if overlaps(player_transform, player_body, transform, body) {
            commands.entity(entity).despawn();
            score.0 += 1;
        }
    }
}
```

`Commands`로 `despawn`하려면 엔티티 ID가 필요합니다. 그래서 쿼리에 `Entity`가 들어갑니다.

## 구현 흐름 9: 쿨다운 있는 피해 만들기

적에게 닿으면 체력이 줄지만, 매 프레임 줄어들면 너무 빠릅니다.

```rust
fn enemy_hits_player(
    time: Res<Time>,
    player: Single<(&Transform, &Body, &mut Health), With<Player>>,
    enemies: Query<(&Transform, &Body), With<Enemy>>,
    mut hit_cooldown: Local<f32>,
) {
    *hit_cooldown -= time.delta_secs();

    if *hit_cooldown > 0.0 {
        return;
    }

    let (player_transform, player_body, mut health) = player.into_inner();

    for (enemy_transform, enemy_body) in &enemies {
        if overlaps(player_transform, player_body, enemy_transform, enemy_body) {
            health.current = (health.current - 1).max(0);
            *hit_cooldown = 1.0;
            break;
        }
    }
}
```

쿨다운은 이 시스템만 쓰므로 `Local<f32>`가 딱 맞습니다.

## 구현 흐름 10: 화면 표시는 마지막에 갱신하기

표시 단계는 최종 체력과 점수를 읽습니다.

```rust
fn update_hud_text(
    score: Res<Score>,
    player: Single<&Health, With<Player>>,
    mut health_text: Single<&mut Text2d, (With<HealthText>, Without<ScoreText>)>,
    mut score_text: Single<&mut Text2d, (With<ScoreText>, Without<HealthText>)>,
) {
    let health = *player;
    health_text.0 = format!("Health: {}/{}", health.current, health.max);
    score_text.0 = format!("Score: {}", score.0);
}
```

충돌 처리가 끝난 뒤 실행되어야 HUD가 이번 프레임의 결과를 보여줍니다.

## Rust로 보면

이 장의 Rust 타입들은 그대로 게임 용어가 됩니다.

```text
Health { current, max }      이름 있는 필드로 체력 상태 표현
Score(u32)                   전역 값 하나를 tuple struct로 표현
BodyBundle::new(...)         타입에 붙은 생성 함수
Local<f32>                   시스템 내부 상태를 감싼 제네릭 타입
Single<(&Transform, &Body)>  컴포넌트 참조 tuple
```

상속은 필요하지 않습니다. 공유 동작은 공유 컴포넌트와 공유 시스템으로 만듭니다.

## Bevy로 보면

이 예제가 확장되는 이유는 시스템마다 일이 하나씩만 있기 때문입니다.

```text
player_input          키보드 입력 -> player Velocity
enemy_ai              player Transform -> enemy Velocity
move_bodies           Velocity -> Transform
clamp_to_arena        Transform을 경계 안으로 제한
collect_items         겹침 -> `despawn` + score
enemy_hits_player     겹침 -> health
update_hud_text       리소스/컴포넌트 -> text
```

나중에 기능이 커져도 먼저 이 파이프라인 안에서 위치를 잡고, 필요한 컴포넌트와 리소스를 정하면 됩니다.

## 확인

실행합니다.

```sh
cargo run --example 07_rpg_slice
```

기대 결과:

- 플레이어가 움직입니다.
- 빨간 적이 플레이어 쪽으로 움직입니다.
- 노란 수집 아이템은 닿으면 사라집니다.
- 수집하면 점수가 오릅니다.
- 적에게 닿으면 짧은 쿨다운을 두고 체력이 줄어듭니다.
- 엔티티는 아레나 경계 안에 머뭅니다.

## 바꿔보기

적 속도를 낮춰 봅니다.

```rust
const ENEMY_SPEED: f32 = 30.0;
```

기대 결과: 적은 여전히 플레이어를 따라오지만 게임이 쉬워집니다. 충돌 코드나 HUD 코드는 바꿀 필요가 없습니다.

---

<div align="center">

[← 이전: 에셋, 카메라, UI](06-assets-camera-ui.md) · [목차](index.md) · [다음: 부드러운 카메라 추적 →](08-smooth-camera-follow.md)

</div>
