# 5. 번들, 플러그인, 세트

<div align="center">

[목차](index.md) · [← 이전: 입력과 이동](04-input-and-movement.md) · [다음: 에셋, 카메라, UI →](06-assets-camera-ui.md)

</div>

---

## 이 장에서 만들 것

이 장이 끝나면 스폰에 필요한 데이터, 기능 등록, 프레임 실행 순서가 `main` 안에 뒤섞이지 않습니다.

![GamePlugin이 BodyPlugin과 PlayerPlugin을 등록하고 Input을 Movement보다 먼저 실행합니다.](../../assets/screenshots/ch05-plugins-sets.png)

## 실행

```sh
cargo run --example 05_plugins_sets
```

화면에서 보이는 동작은 여전히 움직이는 플레이어입니다. 중요한 변화는 코드 구조입니다.

## 구현 흐름 1: 반복되는 스폰 데이터를 Bundle로 묶기

Bundle은 함께 붙일 컴포넌트 묶음입니다.

```rust
#[derive(Bundle)]
struct BodyBundle {
    body: Body,
    velocity: Velocity,
    transform: Transform,
}
```

생성자는 스폰할 때 기본으로 넣을 값을 정합니다.

```rust
impl BodyBundle {
    fn new(position: Vec3) -> Self {
        Self {
            body: Body,
            velocity: Velocity(Vec2::ZERO),
            transform: Transform::from_translation(position),
        }
    }
}
```

이제 body를 만들 때마다 세 컴포넌트를 매번 외워서 적지 않아도 됩니다.

## 구현 흐름 2: Bundle 안에 Bundle 넣기

Bundle은 다른 Bundle을 포함할 수 있습니다.

```rust
#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    body: BodyBundle,
    sprite: Sprite,
}
```

플레이어 생성자는 이렇게 됩니다.

```rust
impl PlayerBundle {
    fn new() -> Self {
        Self {
            player: Player,
            body: BodyBundle::new(Vec3::ZERO),
            sprite: Sprite::from_color(Color::srgb(0.25, 0.70, 1.0), Vec2::splat(80.0)),
        }
    }
}
```

스폰하는 쪽은 짧고 분명합니다.

```rust
commands.spawn(PlayerBundle::new());
```

“같은 변수를 두 번 선언하는 느낌”을 줄이는 Rust다운 방법이 이겁니다. 컴포넌트 타입은 작게 분리해 두고, 생성 레시피는 Bundle이 맡습니다.

## 구현 흐름 3: Plugin을 등록 단위로 쓰기

Plugin은 한 기능 영역의 등록을 맡습니다.

```rust
struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BodySpeed(220.0))
            .add_systems(Update, move_bodies.in_set(GameSet::Movement));
    }
}
```

플레이어 플러그인은 플레이어 생성과 입력을 맡습니다.

```rust
struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, handle_player_input.in_set(GameSet::Input));
    }
}
```

플러그인 안에서 다른 플러그인을 등록할 수도 있습니다.

```rust
struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BodyPlugin)
            .add_plugins(PlayerPlugin);
    }
}
```

Bevy에서 자연스러운 구조입니다. 최상위 game plugin이 기능별 plugin을 조립합니다.

## 구현 흐름 4: `SystemSet`으로 프레임 단계를 이름 붙이기

예제는 프레임 단계를 enum으로 정의합니다.

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
}
```

그리고 순서를 정합니다.

```rust
.configure_sets(Update, (GameSet::Input, GameSet::Movement).chain())
```

각 시스템은 자기 단계에 들어갑니다.

```rust
handle_player_input.in_set(GameSet::Input)
move_bodies.in_set(GameSet::Movement)
```

결과적으로 프레임 순서가 이렇게 읽힙니다.

```text
Input -> Movement
```

## Rust로 보면

`impl Plugin for BodyPlugin`은 `BodyPlugin`이 Bevy의 `Plugin` trait를 구현한다는 뜻입니다.

```rust
impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
    }
}
```

`&self`는 plugin 값을 읽기 전용으로 빌리고, `&mut App`은 app 등록 객체를 수정 가능하게 빌립니다.

Bundle 생성자의 `Self`는 지금 `impl` 중인 타입입니다.

```rust
fn new() -> Self
```

`impl PlayerBundle` 안에서는 `Self`가 `PlayerBundle`입니다.

## Bevy로 보면

책임 경계는 이렇게 나눕니다.

```text
Component   엔티티에 붙는 데이터 타입
Bundle      재사용할 스폰 레시피
System      동작
Plugin      등록 경계
SystemSet   실행 단계
```

여기서부터 Bevy 코드가 긴 `App` 체인 하나처럼 보이지 않기 시작합니다. 체인은 여전히 있지만, 각 plugin이 자기 영역의 등록을 맡습니다.

## 확인

실행합니다.

```sh
cargo run --example 05_plugins_sets
```

기대 결과:

- 플레이어가 움직입니다.
- `GamePlugin`이 `BodyPlugin`과 `PlayerPlugin`을 등록합니다.
- `GameSet::Input`이 `GameSet::Movement`보다 먼저 실행됩니다.

## 바꿔보기

set 순서를 바꿔 봅니다.

```rust
.configure_sets(Update, (GameSet::Movement, GameSet::Input).chain())
```

기대 결과: Movement 단계가 이전 프레임의 Velocity를 쓰게 되어 조작이 한 프레임 늦게 느껴질 수 있습니다. 확인한 뒤 원래 순서로 되돌립니다.

---

<div align="center">

[← 이전: 입력과 이동](04-input-and-movement.md) · [목차](index.md) · [다음: 에셋, 카메라, UI →](06-assets-camera-ui.md)

</div>
