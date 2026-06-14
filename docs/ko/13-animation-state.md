# 13. 애니메이션 상태

[목차](index.md) | 이전: [screen-space UI](12-screen-space-ui.md) | 다음: [직접 만든 맵 지오메트리](14-handmade-map-geometry.md)

실행:

```sh
cargo run --example 13_animation_state
```

이 장의 계약은 애니메이션을 단순한 프레임 번호가 아니라 상태 기계로 다루는 것입니다. 플레이어는 `Idle`, `Run`, `Attack` 중 하나의 상태를 갖고, 상태별로 텍스처 아틀라스 인덱스를 선택합니다.

## 핵심 ECS 계약

- `PlayerAnimState`: 가능한 애니메이션 상태입니다.
- `PlayerAnimation`: 현재 상태, 반복 프레임 타이머, 공격 타이머, 달리기 프레임을 저장합니다.
- `Velocity(Vec2)`: 이동 중인지 판단하는 입력입니다.
- `Sprite.texture_atlas`: 현재 표시할 프레임 인덱스를 가집니다.
- `AnimationLabel`: 현재 상태를 보여줄 텍스트입니다.

`player_input`은 입력과 상태 전환을 담당합니다. `animate_player`는 상태를 읽고 스프라이트 프레임을 바꿉니다. 상태 결정과 프레임 선택을 분리하면 나중에 공격 판정, 사운드, 이펙트를 붙이기 쉽습니다.

## Rust 포인트

`#[derive(Debug, Clone, Copy, PartialEq, Eq)]`가 붙은 enum은 값 비교와 디버그 출력이 가능합니다. `animation.state != PlayerAnimState::Attack`처럼 비교하려면 `PartialEq`가 필요하고, `format!("{:?}", player.state)`에는 `Debug`가 필요합니다.

`let Some(atlas) = &mut sprite.texture_atlas else { return; };`는 아틀라스가 없는 스프라이트를 조용히 건너뜁니다. `Option`을 패턴 매칭으로 풀어 안전하게 접근합니다.

## Bevy 포인트

`TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None)`는 32x32 프레임 4개가 가로로 놓인 스프라이트 시트를 설명합니다. 실제 이미지는 `Handle<Image>`이고, 레이아웃은 `Assets<TextureAtlasLayout>`에 등록한 핸들입니다.

`Sprite { image, texture_atlas: Some(TextureAtlas { layout, index: 0 }), ..default() }`에서 `index`만 바꾸면 같은 이미지 안의 다른 프레임이 표시됩니다.

## 프레임 흐름

1. 입력이 이동 방향과 공격 키를 읽습니다.
2. 공격 키가 눌리면 상태를 `Attack`으로 바꾸고 공격 타이머를 리셋합니다.
3. 공격 중이 아니면 속도에 따라 `Idle` 또는 `Run`으로 바꿉니다.
4. 애니메이션 시스템이 상태별 아틀라스 인덱스를 정합니다.
5. UI 라벨이 현재 상태를 표시합니다.

## 흔한 실수

- 공격 중에 이동 입력으로 바로 `Run` 상태를 덮어쓰면 공격 프레임이 보이지 않습니다.
- `TextureAtlasLayout`을 `Assets`에 추가하지 않고 지역 변수로만 두면 스프라이트가 참조할 핸들이 없습니다.
- 타이머를 `tick`하지 않으면 달리기 프레임과 공격 종료가 진행되지 않습니다.
- 프레임 인덱스가 이미지의 실제 칸 수를 넘으면 의도하지 않은 영역을 참조합니다.

## 작게 바꿔보기

- 공격 타이머를 더 길게 바꿔보세요.
- 달리기 프레임을 하나 더 추가해보세요.
- 왼쪽으로 움직일 때 sprite를 좌우 반전해보세요.
