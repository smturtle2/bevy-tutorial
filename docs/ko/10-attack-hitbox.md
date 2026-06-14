# 10. 공격 히트박스

<div align="center">

[목차](index.md) · [← 이전: 적 웨이브](09-enemy-waves.md) · [다음: 스프라이트 에셋 →](11-sprite-assets.md)

</div>

---

## 이 장에서 만들 것

이 장이 끝나면 Space를 눌렀을 때 플레이어 앞에 짧게 살아 있는 공격 히트박스가 생깁니다. 히트박스는 충돌 데이터, 베기 스프라이트, 피해량, 타이머를 가진 평범한 엔티티입니다.

![플레이어 앞에 베기 스프라이트와 공격 히트박스가 나타납니다.](../../assets/screenshots/ch10-attack-hitbox.png)

## 실행

```sh
cargo run --example 10_attack_hitbox
```

WASD/방향키로 움직이고, Space로 공격합니다.

## 구현 흐름 1: 바라보는 방향 저장하기

플레이어는 마지막 이동 방향을 기억해야 합니다.

```rust
#[derive(Component)]
struct Facing(Vec2);
```

입력 시스템은 플레이어가 움직일 때만 `Facing`을 갱신합니다.

```rust
let normalized = direction.normalize_or_zero();
velocity.0 = normalized * PLAYER_SPEED;

if normalized != Vec2::ZERO {
    facing.0 = normalized;
}
```

이렇게 하면 멈춘 상태에서도 마지막으로 바라보던 방향으로 공격할 수 있습니다.

## 구현 흐름 2: Hitbox 컴포넌트 정의하기

공격은 게임플레이 데이터입니다.

```rust
#[derive(Component)]
struct AttackHitbox {
    lifetime: Timer,
    damage: i32,
}
```

히트박스는 여기에 `Body`, `Sprite`, `Transform`도 같이 가집니다. 특별한 충돌 모드가 아니라 일반 엔티티입니다.

## 구현 흐름 3: `just_pressed`로 한 번만 생성하기

공격 시스템은 매 프레임 실행되지만, 키를 누른 프레임에만 동작합니다.

```rust
if !keyboard.just_pressed(KeyCode::Space) {
    return;
}
```

그다음 플레이어 앞에 히트박스 위치를 잡습니다.

```rust
let (player_transform, facing) = *player;
let position = player_transform.translation + (facing.0 * HITBOX_DISTANCE).extend(1.0);
let angle = facing.0.y.atan2(facing.0.x);
```

`atan2`는 방향 벡터를 베기 스프라이트의 회전 각도로 바꿉니다.

## 구현 흐름 4: Hitbox 엔티티 생성하기

실제 스폰은 이렇습니다.

```rust
commands.spawn((
    AttackHitbox {
        lifetime: Timer::from_seconds(0.14, TimerMode::Once),
        damage: 1,
    },
    Body {
        half_size: HITBOX_SIZE / 2.0,
    },
    Sprite::from_image(asset_server.load("slash.png")),
    Transform {
        translation: position,
        rotation: Quat::from_rotation_z(angle),
        ..default()
    },
));
```

짧은 수명이 중요합니다. 히트박스는 영구적인 무기 오브젝트가 아니라, 한 번의 공격에서 피해 판정이 살아 있는 구간입니다.

## 구현 흐름 5: 피해 적용하기

피해 시스템은 모든 히트박스와 모든 적을 비교합니다.

```rust
for (hitbox_entity, hitbox_transform, hitbox_body, hitbox) in &hitboxes {
    let mut hit_anything = false;

    for (enemy_entity, enemy_transform, enemy_body, mut health) in &mut enemies {
        if overlaps(hitbox_transform, hitbox_body, enemy_transform, enemy_body) {
            health.current -= hitbox.damage;
            hit_count.0 += 1;
            hit_anything = true;

            if health.current <= 0 {
                commands.entity(enemy_entity).despawn();
            }
        }
    }

    if hit_anything {
        commands.entity(hitbox_entity).despawn();
    }
}
```

무언가를 맞춘 히트박스는 사라집니다. 체력이 0 이하가 된 적도 사라집니다.

## 구현 흐름 6: 빗나간 공격 제거하기

아무것도 맞히지 못한 히트박스는 타이머로 제거합니다.

```rust
fn expire_attack_hitboxes(
    mut commands: Commands,
    time: Res<Time>,
    mut hitboxes: Query<(Entity, &mut AttackHitbox)>,
) {
    for (entity, mut hitbox) in &mut hitboxes {
        hitbox.lifetime.tick(time.delta());

        if hitbox.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
```

이렇게 해야 월드에 오래된 히트박스가 쌓이지 않습니다.

## Rust로 보면

피해 시스템에는 `Vec::new()`가 나옵니다.

```rust
let mut defeated_enemies = Vec::new();
```

이번 시스템 실행 중 이미 쓰러진 적을 기억해서, 같은 프레임의 다른 히트박스가 같은 적을 또 처리하지 않게 합니다.

`..default()`는 나머지 필드를 기본값으로 채웁니다.

```rust
Transform {
    translation: position,
    rotation: Quat::from_rotation_z(angle),
    ..default()
}
```

필요한 필드만 직접 정하고, 나머지는 기본 `Transform`을 쓰는 방식입니다.

## Bevy로 보면

공격에 위치, 수명, 렌더링, 충돌 정보가 필요하다면 엔티티로 표현하는 것이 자연스럽습니다.

```text
Player input     -> AttackHitbox 엔티티 생성
Combat collision -> AttackHitbox와 Enemy body 비교
Timer 정리       -> 만료된 AttackHitbox `despawn`
```

공격도 다른 게임 오브젝트와 같은 ECS 모델 안에 들어갑니다.

## 확인

실행합니다.

```sh
cargo run --example 10_attack_hitbox
```

기대 결과:

- Space를 누르면 바라보는 방향으로 slash가 생깁니다.
- slash가 적과 겹치면 hit count가 증가합니다.
- 적은 충분히 맞으면 사라집니다.
- 빗나간 slash는 금방 사라집니다.

## 바꿔보기

이 값을:

```rust
const HITBOX_DISTANCE: f32 = 48.0;
```

이렇게 바꿔 봅니다.

```rust
const HITBOX_DISTANCE: f32 = 90.0;
```

기대 결과: 공격이 플레이어에게서 더 멀리 생성됩니다. 가까이 붙은 적은 오히려 빗나가기 쉬워집니다.

---

<div align="center">

[← 이전: 적 웨이브](09-enemy-waves.md) · [목차](index.md) · [다음: 스프라이트 에셋 →](11-sprite-assets.md)

</div>
