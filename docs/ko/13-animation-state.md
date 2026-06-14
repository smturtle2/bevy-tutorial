# 13. 애니메이션 상태

<div align="center">

[목차](index.md) · [← 이전: 화면 고정 UI](12-screen-space-ui.md) · [다음: 직접 만든 맵 구조 →](14-handmade-map-geometry.md)

</div>

---

## 이 장에서 만들 것

이 장이 끝나면 플레이어 스프라이트가 스프라이트 시트의 프레임을 바꿔 가며 대기, 달리기, 공격 상태를 표현합니다.

![스프라이트 시트가 대기, 달리기, 공격 애니메이션을 표현합니다.](../../assets/screenshots/ch13-animation-attack.png)

## 실행

```sh
cargo run --example 13_animation_state
```

WASD/방향키로 움직이고, Space로 공격합니다.

## 구현 흐름 1: 애니메이션 상태 정의하기

플레이어 애니메이션 상태는 enum으로 표현합니다.

```rust
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum PlayerAnimState {
    Idle,
    Run,
    Attack,
}
```

가능한 상태는 정확히 세 개입니다. 나중에 `match`가 이 세 경우를 모두 처리합니다.

## 구현 흐름 2: 애니메이션 타이머 저장하기

애니메이션 컴포넌트는 현재 상태와 프레임 타이밍을 저장합니다.

```rust
#[derive(Component)]
struct PlayerAnimation {
    state: PlayerAnimState,
    frame_timer: Timer,
    attack_timer: Timer,
    run_frame: usize,
}
```

`Default`는 기본 애니메이션 상태를 만듭니다.

```rust
impl Default for PlayerAnimation {
    fn default() -> Self {
        Self {
            state: PlayerAnimState::Idle,
            frame_timer: Timer::from_seconds(0.14, TimerMode::Repeating),
            attack_timer: Timer::from_seconds(0.20, TimerMode::Once),
            run_frame: 1,
        }
    }
}
```

달리기 애니메이션은 프레임 `1`과 `2`를 번갈아 씁니다. 공격은 프레임 `3`을 씁니다.

## 구현 흐름 3: Texture atlas layout 만들기

setup은 스프라이트 시트를 로드합니다.

```rust
let texture = asset_server.load("player_sheet.png");
let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None);
let texture_atlas_layout = texture_atlas_layouts.add(layout);
```

이 시트는 가로로 4프레임, 세로로 1줄이고, 각 프레임은 `32x32`입니다.

스프라이트는 이미지와 atlas layout을 같이 저장합니다.

```rust
Sprite {
    image: texture.clone(),
    texture_atlas: Some(TextureAtlas {
        layout: texture_atlas_layout.clone(),
        index: 0,
    }),
    ..default()
}
```

`index`를 바꾸면 렌더링되는 프레임이 바뀝니다.

## 구현 흐름 4: 입력으로 상태 고르기

입력 시스템은 속도와 애니메이션 상태를 같이 갱신합니다.

```rust
if keyboard.just_pressed(KeyCode::Space) {
    animation.state = PlayerAnimState::Attack;
    animation.attack_timer.reset();
} else if animation.state != PlayerAnimState::Attack {
    animation.state = if normalized == Vec2::ZERO {
        PlayerAnimState::Idle
    } else {
        PlayerAnimState::Run
    };
}
```

공격은 우선순위가 높습니다. 공격 타이머가 끝나기 전에는 이동 상태가 공격을 끊지 않습니다.

## 구현 흐름 5: `match`로 atlas index 바꾸기

애니메이션 시스템은 현재 상태를 읽습니다.

```rust
match animation.state {
    PlayerAnimState::Idle => {
        atlas.index = 0;
    }
    PlayerAnimState::Run => {
        animation.frame_timer.tick(time.delta());

        if animation.frame_timer.just_finished() {
            animation.run_frame = if animation.run_frame == 1 { 2 } else { 1 };
        }

        atlas.index = animation.run_frame;
    }
    PlayerAnimState::Attack => {
        atlas.index = 3;
        animation.attack_timer.tick(time.delta());

        if animation.attack_timer.is_finished() {
            animation.state = if velocity.0 == Vec2::ZERO {
                PlayerAnimState::Idle
            } else {
                PlayerAnimState::Run
            };
        }
    }
}
```

핵심은 단순합니다. 상태가 프레임 동작을 고르고, 타이머가 프레임이나 상태를 바꿀 시점을 정합니다.

## Rust로 보면

atlas에 접근할 때는 `Option`이 나옵니다.

```rust
let Some(atlas) = &mut sprite.texture_atlas else {
    return;
};
```

모든 sprite가 texture atlas를 갖는 것은 아닙니다. 이 시스템은 atlas가 있는 스프라이트에만 애니메이션을 적용합니다. 없으면 바로 빠져나갑니다.

enum은 `Copy`를 derive했기 때문에 상태 대입이 가볍습니다.

```rust
animation.state = PlayerAnimState::Attack;
```

## Bevy로 보면

애니메이션은 컴포넌트 데이터와 시스템 하나로 볼 수 있습니다.

```text
Sprite.texture_atlas.index       지금 보이는 프레임
PlayerAnimation.state            현재 동작 상태
PlayerAnimation timers           언제 프레임을 넘길지
animate_player system            상태 -> atlas index
```

렌더링 코드에 애니메이션 판단을 섞지 않습니다. 게임플레이 시스템이 상태를 정하고, 렌더러는 sprite를 그립니다.

## 확인

실행합니다.

```sh
cargo run --example 13_animation_state
```

기대 결과:

- 가만히 있으면 프레임 0을 씁니다.
- 움직이면 달리기 프레임이 번갈아 보입니다.
- Space를 누르면 잠깐 공격 프레임으로 바뀝니다.
- 라벨에 현재 애니메이션 상태가 표시됩니다.

## 바꿔보기

달리기 타이머를 바꿔 봅니다.

```rust
Timer::from_seconds(0.14, TimerMode::Repeating)
```

```rust
Timer::from_seconds(0.06, TimerMode::Repeating)
```

기대 결과: 달리기 애니메이션이 더 빠르게 재생됩니다. 이동 속도는 바뀌지 않습니다.

---

<div align="center">

[← 이전: 화면 고정 UI](12-screen-space-ui.md) · [목차](index.md) · [다음: 직접 만든 맵 구조 →](14-handmade-map-geometry.md)

</div>
