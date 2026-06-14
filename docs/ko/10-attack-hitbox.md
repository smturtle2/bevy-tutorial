# 10. 공격 히트박스

[목차](index.md) | 이전: [적 웨이브](09-enemy-waves.md) | 다음: [sprite asset](11-sprite-assets.md)

실행:

```sh
cargo run --example 10_attack_hitbox
```

이 장의 계약은 공격을 즉시 데미지 함수로 처리하지 않고, 짧게 존재하는 히트박스 엔티티로 표현하는 것입니다. 공격도 ECS 데이터가 되면 시각 효과, 충돌, 수명, 데미지를 분리해서 다룰 수 있습니다.

## 핵심 ECS 계약

- `Facing(Vec2)`: 플레이어가 마지막으로 바라본 방향입니다.
- `AttackHitbox { lifetime, damage }`: 공격 판정의 수명과 데미지입니다.
- `Health { current }`: 적의 생존 상태입니다.
- `HitCount`: 성공한 타격 수를 저장하는 리소스입니다.
- `Body { half_size }`: 플레이어, 적, 히트박스가 공유하는 AABB 크기입니다.

`spawn_attack_hitbox`는 Space가 눌린 프레임에만 히트박스를 생성합니다. `attack_hits_enemies`는 히트박스와 적의 `Body`를 비교해 겹치면 체력을 줄이고, 필요하면 적과 히트박스를 제거합니다.

## Rust 포인트

`Facing(Vec2)`와 `Velocity(Vec2)`는 tuple struct입니다. 같은 `Vec2`라도 의미가 다르면 다른 타입으로 감싸는 편이 안전합니다. 시스템 시그니처에서 `&mut Facing`과 `&mut Velocity`가 분리되므로 읽는 사람도 의도를 바로 압니다.

`atan2`로 방향 벡터를 회전 각도로 바꾸고 `Quat::from_rotation_z(angle)`에 넣습니다. 2D에서도 Bevy의 `Transform.rotation`은 3D 회전 타입인 `Quat`를 사용합니다.

## Bevy 포인트

`Commands`로 엔티티를 `despawn()`해도 현재 루프가 즉시 중단되는 것은 아닙니다. 명령은 스케줄 경계에서 적용됩니다. 그래서 이 예제는 히트박스가 무언가를 맞췄는지 `hit_anything`에 기록한 뒤 루프 끝에서 제거 명령을 냅니다.

충돌 함수 `overlaps`는 회전된 스프라이트 모양을 계산하지 않습니다. `Transform` 위치와 `Body.half_size`로 축 정렬 박스만 비교합니다. 초반 액션 RPG 판정에는 이 단순 계약이 디버그하기 쉽습니다.

## 프레임 흐름

1. 입력이 플레이어 `Velocity`와 `Facing`을 갱신합니다.
2. Space 입력이 있으면 바라보는 방향 앞에 히트박스를 스폰합니다.
3. 이동 시스템이 속도를 위치에 반영합니다.
4. 히트박스가 적과 겹치면 데미지를 주고 타격 수를 올립니다.
5. 수명이 끝난 히트박스를 제거합니다.
6. UI가 타격 수를 표시합니다.

## 흔한 실수

- 정지 중에도 공격 방향이 필요합니다. 입력이 없을 때 `Facing`을 `Vec2::ZERO`로 덮어쓰지 마세요.
- `just_pressed` 대신 `pressed`를 쓰면 Space를 누르고 있는 동안 매 프레임 히트박스가 생깁니다.
- 히트박스에 `Body`를 붙이지 않으면 충돌 시스템이 찾을 수 없습니다.
- `AttackHitbox` 수명 제거를 빼면 보이지 않는 판정이 계속 남습니다.

## 작게 바꿔보기

- `AttackHitbox`의 수명을 늘려 판정이 오래 남게 해보세요.
- `damage`를 2로 바꿔 적이 한 번에 죽는지 확인하세요.
- 히트박스를 첫 적에게만 맞게 할지, 여러 적을 관통하게 할지 규칙을 바꿔보세요.
