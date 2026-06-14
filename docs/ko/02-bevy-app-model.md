# 2. Bevy 앱 모델


<div align="center">

[목차](index.md) · [← 이전: Bevy를 위한 Rust](01-rust-for-bevy.md) · [다음: ECS 기본 →](03-ecs-fundamentals.md)

</div>

---

첫 예제를 실행합니다.

```sh
cargo run --example 01_empty_app
```

어두운 배경의 Bevy 창이 보여야 합니다. 이 첫 창 안에 이미 핵심 앱 형태가 들어 있습니다.

## 둘러보기: `01_empty_app`

예제는 Bevy prelude로 시작합니다.

```rust
use bevy::prelude::*;
```

그다음 `main`이 앱을 만들고 실행합니다.

```rust
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.11)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .run();
}
```

체인을 위에서 아래로 읽으세요.

```text
App::new()                  앱 빌더를 만든다
insert_resource(...)        월드에 전역 값 하나를 저장한다
add_plugins(DefaultPlugins) Bevy의 기본 엔진 플러그인을 추가한다
add_systems(Startup, ...)   startup 시스템 하나를 등록한다
run()                       엔진 루프에 들어간다
```

`DefaultPlugins`는 일반적인 엔진 구성요소를 추가합니다. 창, 렌더링, 입력, 에셋, 로깅과 관련 기본값이 여기에 들어갑니다. 튜토리얼 예제의 표준 시작점입니다.

1장에서 배운 Rust 문법으로 보면 `App::new()`는 `App` 타입의 연관 함수이고, 뒤의 `.insert_resource(...)`, `.add_plugins(...)`, `.add_systems(...)`, `.run()`은 만들어진 앱 빌더 값을 이어받는 메서드 호출입니다.

## `App`은 게임플레이를 등록합니다

`App`은 Bevy가 실행할 데이터와 동작을 등록하는 곳입니다. 이동 로직, AI, 충돌, UI는 시스템 안에 있고, `App`은 그 시스템들을 스케줄에 등록합니다.

중요한 구분은 다음입니다.

```text
App 설정 = 플러그인, 리소스, 시스템, 스케줄을 등록한다
시스템   = ECS 데이터를 읽고 쓰는 일을 한다
```

## Startup과 Update

Bevy 시스템은 스케줄에 등록되는 평범한 Rust 함수입니다.

```rust
.add_systems(Startup, setup_camera)
.add_systems(Update, move_bodies)
```

스케줄이 실행 시점을 정합니다.

```text
Startup = 앱이 시작될 때 한 번 실행
Update  = 매 프레임 실행
```

등록 위치가 실행 타이밍을 정합니다. `setup`이라는 이름의 함수도 `Update`에 등록하면 매 프레임 실행됩니다.

## Commands와 지연된 월드 변경

`01_empty_app`의 startup 시스템은 다음과 같습니다.

```rust
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
```

`Commands`는 ECS 월드 변경을 큐에 넣습니다. 엔티티 생성은 월드 구조를 바꾸므로 `Commands`를 거칩니다.

흔한 `Commands` 사용:

- `commands.spawn(...)`: 컴포넌트를 가진 엔티티를 만듭니다.
- `commands.entity(entity).despawn()`: 엔티티를 제거합니다.
- `commands.entity(entity).insert(...)`: 컴포넌트를 추가합니다.
- `commands.entity(entity).remove::<T>()`: 컴포넌트 타입을 제거합니다.

Commands는 지연됩니다. 시스템 안에서 `commands.spawn(...)`을 호출하면 구조 변경이 큐에 쌓입니다. Bevy는 스케줄 경계와 정의된 sync point에서 큐에 쌓인 commands를 적용하고, 쿼리는 그 적용 지점 뒤의 구조를 봅니다. 이렇게 해서 시스템 실행을 안전하고 병렬화 가능하게 유지합니다.

경험칙:

```text
Commands = 어떤 엔티티/컴포넌트가 존재하는지 바꾼다
Query    = 이미 존재하는 컴포넌트 값을 읽거나 수정한다
```

## 둘러보기: `02_spawn_sprite`

실행합니다.

```sh
cargo run --example 02_spawn_sprite
```

![두 번째 앱 모델 예제는 Camera2d, Sprite, Transform으로 파란 스프라이트 엔티티를 렌더링합니다.](../../assets/screenshots/ch02-spawn-sprite.png)

setup 시스템은 엔티티 두 개를 만듭니다.

```rust
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(Color::srgb(0.25, 0.70, 1.0), Vec2::splat(80.0)),
        Transform::from_translation(Vec3::ZERO),
    ));
}
```

카메라 엔티티에는 `Camera2d` 컴포넌트가 있습니다. 사각형 엔티티에는 `Sprite`와 `Transform`이 있습니다.

이 사각형은 컴포넌트로 구성된 ECS 엔티티입니다.

```text
Entity
  Sprite
  Transform
```

`Sprite`는 무엇을 그릴지 제어합니다. `Transform`은 어디에 그릴지 제어합니다.

## Plugin은 등록 단위입니다

플러그인은 앱 등록을 묶습니다.

```rust
struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}
```

`build`는 플러그인이 추가될 때 호출됩니다. 매 프레임 동작은 플러그인이 등록한 시스템 안에 둡니다.

이후 예제들은 플러그인을 사용해 책임을 나눕니다.

```text
GamePlugin   = 최상위 게임 설정과 실행 순서
BodyPlugin   = 이동 데이터와 이동 시스템
PlayerPlugin = 플레이어 생성과 입력 시스템
```

## 체크포인트

직접 실험하면서 `examples/02_spawn_sprite.rs`를 수정하고 질문에 답해 보세요.

- `commands.spawn(Camera2d)`를 제거하면 어떻게 되는가?
- `Vec2::splat(80.0)`를 `Vec2::splat(30.0)`로 바꾸면 어떻게 되는가?
- `Transform::from_translation(Vec3::ZERO)`를 `Transform::from_translation(Vec3::new(200.0, 0.0, 0.0))`로 바꾸면 어떻게 되는가?

기대하는 교훈: 렌더링도 ECS 데이터입니다. 보이는 것을 만들려면 올바른 컴포넌트를 가진 엔티티를 spawn합니다.

## 흔한 실수

- 시스템을 `Startup`에 등록해 놓고 매 프레임 실행되리라 기대함.
- 기존 컴포넌트 값을 바꾸는 데 `Query<&mut T>`가 더 명확한데도 `Commands`를 사용함.
- `DefaultPlugins`를 잊고 창이나 렌더링이 왜 없는지 고민함.
- 카메라 없이 sprite를 spawn하고 빈 창만 봄.

---

<div align="center">

[← 이전: Bevy를 위한 Rust](01-rust-for-bevy.md) · [목차](index.md) · [다음: ECS 기본 →](03-ecs-fundamentals.md)

</div>
