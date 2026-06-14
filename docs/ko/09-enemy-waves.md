# 9. 적 웨이브


<div align="center">

[목차](index.md) · [← 이전: 부드러운 카메라 추적](08-smooth-camera-follow.md) · [다음: 공격 히트박스 →](10-attack-hitbox.md)

</div>

---

실행:

```sh
cargo run --example 09_enemy_waves
```

이 장의 계약은 웨이브 진행 상태를 리소스로 두고, 타이머가 끝날 때마다 적을 생성하는 것입니다. 적은 플레이어 위치를 읽어 속도를 정하고, 이동 시스템은 속도를 위치에 반영합니다.

이 장은 전투 장이 아닙니다. 그래서 적은 `EnemyLifetime`이 끝나면 자동으로 사라집니다. 이 데모용 클리어 규칙 덕분에 2웨이브 이상이 실제로 도달 가능합니다.

## 핵심 ECS 계약

- `WaveSpawner`: 현재 웨이브, 남은 생성 수, 생성 지점 인덱스, 반복 `Timer`를 가진 리소스입니다.
- `Enemy`: 적 엔티티를 구분하는 마커 컴포넌트입니다.
- `EnemyLifetime(Timer)`: 전투를 배우기 전에 웨이브가 실제로 넘어가도록 하는 데모용 수명입니다.
- `Body { half_size }`: 충돌/경계 계산에 쓰는 크기 계약입니다.
- `Velocity(Vec2)`: 이동 의도입니다. 입력과 AI는 이 값을 쓰고, 이동 시스템은 이 값을 읽습니다.
- `WaveText`: 화면 텍스트 엔티티를 찾기 위한 마커입니다.

시스템 세트는 `Input -> Wave -> Ai -> Movement -> Ui` 순서로 체인됩니다. 웨이브 세트는 먼저 오래된 적을 제거하고, 그 다음 적을 만듭니다. AI가 새 적까지 포함해 속도를 계산하고, 이동이 위치를 바꾼 뒤 UI가 최종 숫자를 표시합니다.

## Rust 포인트

`impl Default for WaveSpawner`는 초기 웨이브 규칙을 타입 가까이에 둡니다. `.init_resource::<WaveSpawner>()`는 이 `Default` 구현을 호출해 리소스를 월드에 넣습니다.

`SPAWN_POINTS[spawner.spawn_index % SPAWN_POINTS.len()]`는 고정 배열을 순환합니다. 인덱스를 직접 증가시키되, 접근할 때 나머지 연산으로 범위를 보장합니다.

## Bevy 포인트

`Timer::from_seconds(0.35, TimerMode::Repeating)`은 매 프레임 자동으로 움직이지 않습니다. 시스템 안에서 `spawner.timer.tick(time.delta())`를 호출해야 시간이 흐릅니다. `just_finished()`는 이번 프레임에 타이머가 끝났는지 알려주므로 생성 타이밍에 적합합니다.

적 수는 `Query<(), With<Enemy>>`로 셉니다. 컴포넌트 데이터가 필요 없고 존재 여부만 필요할 때 빈 튜플 조회가 가볍고 의도가 분명합니다.

## 프레임 흐름

1. 플레이어 입력이 `Velocity`를 씁니다.
2. 웨이브 리소스가 남은 생성 수와 살아있는 적 수를 확인합니다.
3. 수명이 끝난 적을 제거하고, 타이머가 끝났으면 다음 생성 지점에 적을 스폰합니다.
4. 적 AI가 플레이어 방향으로 적의 `Velocity`를 씁니다.
5. 모든 `Body` 엔티티가 이동하고 경기장 안으로 제한됩니다.
6. UI가 웨이브, 생존 적 수, 대기 생성 수를 표시합니다.

## 흔한 실수

- `Timer`를 `tick`하지 않으면 `just_finished()`는 참이 되지 않습니다.
- `remaining_to_spawn == 0`만 보고 다음 웨이브를 시작하면 아직 살아있는 적과 새 웨이브가 겹칩니다.
- 적을 제거하는 클리어 규칙이 없으면 2웨이브에 도달할 수 없습니다.
- AI와 이동 순서가 바뀌면 적이 이전 프레임 속도로 움직입니다.
- `Query<Entity, With<Enemy>>`가 필요하지 않은데 엔티티를 읽으면 코드 의도가 흐려집니다. 카운트만 필요하면 `Query<(), With<Enemy>>`로 충분합니다.

## 작게 바꿔보기

- 웨이브마다 생성되는 적 수를 더 크게 바꿔보세요.
- `SPAWN_POINTS`에 새 생성 지점을 추가하세요.
- 히트박스 장을 끝낸 뒤 `EnemyLifetime` 대신 실제 전투로 웨이브를 clear하게 바꿔보세요.

---

<div align="center">

[← 이전: 부드러운 카메라 추적](08-smooth-camera-follow.md) · [목차](index.md) · [다음: 공격 히트박스 →](10-attack-hitbox.md)

</div>
