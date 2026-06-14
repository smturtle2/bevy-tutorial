# 18. 발사체

<div align="center">

[목차](index.md) · [← 이전: 통합 RPG 예제](17-complete-rpg-slice.md) · [다음: 인벤토리 →](19-inventory.md)

</div>

---

## 이 장에서 만들 것

17장의 전투 루프에 원거리 공격을 추가합니다. `Space`는 그대로 근접 베기 공격이고, `F`를 누르면 플레이어가 바라보는 방향으로 발사체를 쏩니다. 두 공격은 같은 체력 모델, 충돌 크기 모델, 게임플레이 정리 마커, 시스템 순서 계약을 사용합니다.

![플레이어가 적을 향해 발사체를 쏘는 장면](../../assets/screenshots/ch18-projectiles.png)

## 실행

```sh
cargo run --example 18_projectiles
```

조작:

```text
WASD / 방향키   이동
Space           근접 베기
F               발사체 발사
```

## 이 장의 기준

이 예제는 발사체 규칙만 분리해서 보여줍니다. 그 규칙이 실제로 만지는 기준만 유지합니다.

```text
GameState::Playing      게임플레이 시스템 실행 조건
GameSet                 Input -> Movement -> Collision -> Ui
GameplayEntity          생성된 게임플레이 엔티티 표시
Body                    충돌 크기
Velocity                이동 벡터
Facing                  마지막으로 움직인 방향
Health { current, max } 적 체력 데이터 모양
Space 베기              근접 공격으로 유지
```

새로 추가되는 전투 오브젝트는 이것입니다.

```text
Projectile entity = GameplayEntity + Projectile + Body + Velocity + Transform + Sprite
```

## 구현 흐름 1: 발사체 컴포넌트 추가하기

발사체는 스프라이트만 있는 물체가 아니라 게임 규칙을 가진 엔티티입니다.

```rust
#[derive(Component)]
struct Projectile {
    lifetime: Timer,
    damage: i32,
}
```

`lifetime`은 빗나간 발사체를 정리하기 위한 시간입니다. `damage`는 근접 공격과 같은 방식으로 적 체력을 깎기 위한 값입니다.

## 구현 흐름 2: 근접 공격과 원거리 공격 입력 분리하기

통합 예제에서 `Space`는 이미 베기 히트박스 입력입니다. 그래서 발사체는 `F`에 둡니다.

```rust
if !keyboard.just_pressed(KeyCode::KeyF) {
    return;
}

let (transform, facing) = *player;
let start = transform.translation + (facing.0 * 34.0).extend(1.0);

commands.spawn(ProjectileBundle::new(start, facing.0));
```

발사 시스템은 플레이어 위치와 `Facing`을 읽고, 시작 데이터가 들어간 새 엔티티를 만듭니다. 이동은 이동 시스템의 책임입니다.

## 구현 흐름 3: 이동 규칙을 번들에 넣기

발사체 번들은 방향을 속도와 회전으로 바꿉니다.

```rust
velocity: Velocity(direction * PROJECTILE_SPEED),
rotation: Quat::from_rotation_z(direction.y.atan2(direction.x)),
```

그래서 발사체는 공용 이동 시스템을 그대로 씁니다.

```rust
fn move_bodies(time: Res<Time>, mut bodies: Query<(&mut Transform, &Velocity), With<Body>>) {
    for (mut transform, velocity) in &mut bodies {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
    }
}
```

## 구현 흐름 4: 빗나간 발사체 정리하기

아무것도 맞히지 못한 발사체도 정리 규칙이 있어야 합니다.

```rust
fn tick_projectile_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(Entity, &mut Projectile)>,
) {
    for (entity, mut projectile) in &mut projectiles {
        projectile.lifetime.tick(time.delta());

        if projectile.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
```

발사체 수명 관리는 발사체 기능의 책임입니다. 플레이어 입력이나 적 로직 안에 숨기지 않습니다.

## 구현 흐름 5: 같은 체력 계약 재사용하기

발사체 충돌은 통합 예제의 `Health { current, max }` 구조를 그대로 수정합니다.

```rust
if overlaps(projectile_transform, projectile_body, enemy_transform, enemy_body) {
    health.current -= projectile.damage;
    commands.entity(projectile_entity).despawn();

    if health.current <= 0 {
        commands.entity(enemy_entity).despawn();
    }
}
```

이 예제의 발사체는 한 번 맞히면 사라집니다. 관통 발사체를 만들고 싶다면 이 규칙을 바꾸면 됩니다.

## 통합 지점

이 기능은 17장의 전투 루프에서 필요한 단계만 사용합니다.

```text
Input       F 입력을 읽고 ProjectileBundle 생성
Movement    발사체 이동과 수명 타이머 진행
Collision   발사체와 적 Body를 비교하고 Health 감소
Ui          발사 수, 명중 수, 활성 발사체, 적 체력 표시
```

순서는 기능의 일부입니다. 이동 뒤에 충돌해야 현재 프레임 위치로 판정하고, 수명 정리가 있어야 빗나간 발사체가 쌓이지 않습니다.

## Rust로 보면

`Projectile`은 두 정보를 가지므로 이름 있는 필드 struct가 맞습니다.

```rust
struct Projectile {
    lifetime: Timer,
    damage: i32,
}
```

`Velocity(Vec2)`와 `Facing(Vec2)`은 값 하나를 감싸는 tuple struct입니다. 타입 이름이 의미를 설명하므로 필드 이름을 따로 만들 필요가 적습니다.

상태와 세트의 derive는 각각 역할이 있습니다.

```rust
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Playing,
}
```

`States`는 이 enum을 Bevy 앱 상태로 저장할 수 있게 합니다. `Default`는 시작 상태를 정합니다. `Eq`와 `Hash`는 상태 값을 식별값으로 쓰게 합니다. `Clone`과 `Copy`는 작은 상태 값을 싸게 전달하게 합니다. `Debug`는 진단 메시지에 상태 이름을 출력할 수 있게 합니다.

이동 계산에는 `normalize_or_zero`를 씁니다.

```rust
let normalized = direction.normalize_or_zero();
```

0 벡터를 정규화하면 `NaN` 같은 잘못된 숫자 방향이 나올 수 있습니다. `normalize_or_zero`는 입력이 0일 때 결과도 0으로 정해서 이동과 발사 코드가 예측 가능하게 동작합니다.

## 확인

실행합니다.

```sh
cargo run --example 18_projectiles
```

확인 기준:

- `Space`를 누르면 짧은 근접 히트박스가 생깁니다.
- `F`를 누르면 플레이어가 바라보는 방향으로 발사체가 나갑니다.
- 발사체는 플레이어와 독립적으로 움직입니다.
- 근접 공격과 발사체 모두 적 체력을 줄입니다.
- 빗나간 발사체는 수명이 끝나면 사라집니다.
- UI가 발사 수, 발사체 명중 수, 베기 수, 근접 명중 수, 활성 발사체, 적 체력을 보여줍니다.

## 바꿔보기

다음을:

```rust
const PROJECTILE_LIFETIME: f32 = 0.9;
```

이렇게 바꿉니다.

```rust
const PROJECTILE_LIFETIME: f32 = 2.0;
```

기대 결과: 발사체가 더 멀리 날아간 뒤 사라집니다.

---

<div align="center">

[← 이전: 통합 RPG 예제](17-complete-rpg-slice.md) · [목차](index.md) · [다음: 인벤토리 →](19-inventory.md)

</div>
