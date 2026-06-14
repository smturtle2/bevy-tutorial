# 2. Bevy 앱 모델

<div align="center">

[목차](index.md) · [← 이전: Bevy에 필요한 Rust](01-rust-for-bevy.md) · [다음: ECS 기본 →](03-ecs-fundamentals.md)

</div>

---

## 이 장에서 만들 것

이 장이 끝나면 Bevy의 `App` 설정을 위에서 아래로 읽을 수 있습니다. 플러그인, 리소스, startup system, update system, commands가 어디에 들어가는지 잡습니다.

![두 번째 예제는 Camera2d, Sprite, Transform으로 파란 스프라이트를 그립니다.](../../assets/screenshots/ch02-spawn-sprite.png)

## 실행

```sh
cargo run --example 01_empty_app
cargo run --example 02_spawn_sprite
```

첫 예제는 어두운 창을 엽니다. 두 번째 예제는 파란 사각형을 하나 그립니다.

## 구현 흐름 1: App 만들기

`examples/01_empty_app.rs`는 이런 체인으로 시작합니다.

```rust
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.11)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .run();
}
```

등록 목록처럼 읽으면 됩니다.

```text
App::new()                  앱을 만듭니다
insert_resource(...)        ECS 월드에 전역 값을 하나 저장합니다
add_plugins(DefaultPlugins) 창, 렌더링, 입력, 에셋, 로그 등 기본 엔진 기능을 추가합니다
add_systems(Startup, ...)   한 번 실행할 시스템을 등록합니다
run()                       Bevy 엔진 루프를 시작합니다
```

`App`은 동작을 등록하는 곳입니다. 실제 동작은 시스템 함수 안에 있습니다.

## 구현 흐름 2: Startup 시스템 추가하기

startup system은 카메라를 만듭니다.

```rust
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
```

`Startup`에 등록하면 앱 시작 시 한 번 실행됩니다.

```rust
.add_systems(Startup, setup_camera)
```

`Update`에 등록하면 매 프레임 실행됩니다.

```rust
.add_systems(Update, move_player)
```

함수 이름이 실행 시점을 정하는 것이 아닙니다. 어떤 스케줄에 등록했는지가 실행 시점을 정합니다.

## 구현 흐름 3: 구조 변경은 Commands로 하기

`Commands`는 ECS 월드 구조를 바꾸는 요청을 예약합니다.

```rust
commands.spawn(Camera2d);
commands.spawn((Sprite::from_color(...), Transform::from_translation(...)));
```

자주 쓰는 명령은 이렇습니다.

```text
spawn(...)                  엔티티 생성
entity(id).despawn()        엔티티 제거
entity(id).insert(...)      컴포넌트 추가
entity(id).remove::<T>()    컴포넌트 타입 하나 제거
```

Commands는 월드를 바로 바꾸는 방식이 아니라 예약 방식입니다. 시스템은 구조 변경 요청을 기록하고, Bevy가 정해진 지점에서 그 요청을 적용합니다. 이 덕분에 시스템들이 안전하게 병렬 실행될 수 있습니다.

기준은 이렇게 잡으면 됩니다.

```text
Commands = 어떤 엔티티와 컴포넌트가 존재하는지 바꿈
Query    = 이미 존재하는 컴포넌트 값을 읽거나 수정함
```

## 구현 흐름 4: Sprite 엔티티 만들기

`examples/02_spawn_sprite.rs`는 엔티티를 두 개 만듭니다.

```rust
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(Color::srgb(0.25, 0.70, 1.0), Vec2::splat(80.0)),
        Transform::from_translation(Vec3::ZERO),
    ));
}
```

카메라 엔티티에는 이것이 붙습니다.

```text
Camera2d
```

사각형 엔티티에는 이것이 붙습니다.

```text
Sprite       무엇을 그릴지
Transform    어디에 그릴지
```

렌더링도 ECS 데이터입니다. 2D에서 보이는 무언가를 만들려면 그릴 컴포넌트와 위치 컴포넌트가 필요하고, 월드에는 그것을 볼 카메라가 있어야 합니다.

## Rust로 보면

스프라이트를 생성할 때는 tuple을 씁니다.

```rust
commands.spawn((
    Sprite::from_color(...),
    Transform::from_translation(...),
));
```

이 tuple은 클래스가 아닙니다. 한 엔티티에 붙일 컴포넌트 값 묶음입니다.

색상과 벡터를 만드는 코드는 타입에 붙은 함수입니다.

```rust
Color::srgb(0.25, 0.70, 1.0)
Vec2::splat(80.0)
Vec3::ZERO
```

각 호출은 값을 만들고, Bevy는 그 값을 컴포넌트 데이터로 저장합니다.

## Bevy로 보면

`DefaultPlugins`는 기본 엔진 기능을 켭니다. 이것이 없으면 일반적인 창, 렌더러, 입력, 에셋 로더, 로그 설정을 요청하지 않은 것입니다.

`ClearColor`는 리소스입니다. 앱 전체의 배경색은 하나만 있으면 되기 때문입니다.

```rust
.insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.11)))
```

리소스는 타입이 정해진 전역 값입니다. 컴포넌트는 엔티티에 붙습니다.

## 확인

스프라이트 예제를 실행합니다.

```sh
cargo run --example 02_spawn_sprite
```

어두운 창 가운데에 파란 사각형이 보여야 합니다.

그다음 하나씩 바꿔 봅니다.

- `commands.spawn(Camera2d);`를 지우면 앱은 실행되지만 사각형이 보이지 않습니다.
- `Vec2::splat(80.0)`을 `Vec2::splat(30.0)`으로 바꾸면 사각형이 작아집니다.
- `Vec3::ZERO`를 `Vec3::new(200.0, 0.0, 0.0)`으로 바꾸면 사각형이 오른쪽으로 이동합니다.

## 바꿔보기

두 번째 사각형을 추가해 봅니다.

```rust
commands.spawn((
    Sprite::from_color(Color::srgb(1.0, 0.82, 0.25), Vec2::splat(40.0)),
    Transform::from_xyz(120.0, 0.0, 1.0),
));
```

기대 결과: 오른쪽에 작은 노란 사각형이 생깁니다. `z` 값이 더 크므로 파란 사각형과 겹치면 위에 그려집니다.

---

<div align="center">

[← 이전: Bevy에 필요한 Rust](01-rust-for-bevy.md) · [목차](index.md) · [다음: ECS 기본 →](03-ecs-fundamentals.md)

</div>
