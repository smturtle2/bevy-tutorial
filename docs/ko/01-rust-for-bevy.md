# 1. Bevy에 필요한 Rust

<div align="center">

[목차](index.md) · [← 이전: 프로젝트 설정](00-project-setup.md) · [다음: Bevy 앱 모델 →](02-bevy-app-model.md)

</div>

---

## 이 장에서 만들 것

이 장이 끝나면 초반 Bevy 예제에 나오는 Rust 문법을 읽을 수 있습니다. 함수, 바인딩, 가변성, struct, enum, `impl`, 참조, 소유권, 제네릭, `Option`, `Result`, 모듈, 시스템 매개변수를 게임 코드와 연결해서 봅니다.

목표는 실전적입니다. Bevy 시스템 시그니처를 봤을 때, 이 시스템이 어떤 데이터를 받고 어떤 데이터를 바꿀 수 있는지 말할 수 있으면 됩니다.

![ECS 개요 다이어그램](../../assets/diagrams/ecs-overview.png)

## 실행

```sh
cargo run --example 01_empty_app
```

이 장을 읽으면서 `examples/01_empty_app.rs`를 같이 봅니다.

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.11)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
```

작은 파일이지만 앞으로 계속 볼 Rust 모양이 대부분 들어 있습니다.

## 이름, 값, 타입

Rust는 `let`으로 지역 이름을 만듭니다.

```rust
let score = 0;
let speed: f32 = 280.0;
let direction = Vec2::ZERO;
```

형태는 이렇게 읽습니다.

```text
let name = value;        값을 보고 타입을 추론합니다
let name: Type = value;  타입을 직접 적습니다
```

지역 변수는 오른쪽 값이 분명하면 타입을 생략하는 경우가 많습니다.

```rust
let mut direction = Vec2::ZERO;
```

반대로 함수 매개변수, 특히 Bevy 시스템 매개변수는 타입을 명확히 적습니다. 그 타입이 시스템의 접근 권한이기 때문입니다.

```rust
fn move_player(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Transform, With<Player>>,
) {
}
```

이 시그니처는 이렇게 읽습니다. 이 시스템은 시간을 읽고, 키보드 입력을 읽고, `Player`가 붙은 엔티티의 `Transform`을 수정합니다.

## 가변성

Rust의 바인딩은 기본적으로 바꿀 수 없습니다.

```rust
let direction = Vec2::ZERO;
direction.x += 1.0; // 컴파일 오류
```

값을 바꿔야 하면 `mut`를 붙입니다.

```rust
let mut direction = Vec2::ZERO;
direction.x += 1.0;
```

`mut`가 어디에 붙는지에 따라 의미가 달라집니다.

```rust
let mut value = 10;         // value 바인딩을 바꿀 수 있음
let reference = &mut value; // value를 독점적으로 빌림
```

이런 코드도 가능합니다.

```rust
let a = 10;
let b = 20;
let mut r = &a;
r = &b;
```

여기서 `r`은 다른 값을 가리키도록 바꿀 수 있지만, 타입은 여전히 `&i32`입니다. 즉 `i32`를 읽기 전용으로 빌린 참조입니다.

읽기 전용 참조에서 값을 복사해서 별도의 `i32`로 바꾸고 싶다면 이렇게 씁니다.

```rust
let a = 10;
let r = &a;
let mut copied: i32 = *r;
copied += 1;
```

`i32`는 `Copy` 타입이라 `*r`로 값을 복사할 수 있습니다.

## 함수와 세미콜론

함수는 `fn`으로 정의합니다.

```rust
fn add_score(current: u32, amount: u32) -> u32 {
    current + amount
}
```

읽는 규칙은 이렇습니다.

```text
current: u32    current라는 매개변수의 타입은 u32
-> u32          반환 타입은 u32
세미콜론 없음   블록의 마지막 표현식이 반환값
```

세미콜론을 붙이면 표현식이 문장이 됩니다.

```rust
fn add_score(current: u32, amount: u32) -> u32 {
    current + amount;
    // 컴파일 오류: u32를 반환해야 하는데 ()가 나옴
}
```

`()`는 유닛 타입입니다. 의미 있는 값이 없다는 뜻입니다. Bevy 시스템은 대부분 `()`를 반환합니다.

```rust
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
```

## `::`와 `.`

Rust에서 `::`는 타입이나 모듈 아래에 있는 이름을 부를 때 씁니다.

```rust
App::new()
Vec2::ZERO
Vec3::new(0.0, 0.0, 1.0)
Transform::from_xyz(0.0, 0.0, 1.0)
```

`.`은 이미 있는 값의 메서드나 필드에 접근할 때 씁니다.

```rust
direction.normalize_or_zero()
transform.translation
velocity.0
```

Bevy의 체이닝도 같은 문법입니다.

```rust
App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Update, move_player)
    .run();
```

`App::new()`가 먼저 값을 만들고, 뒤의 줄들은 그 값에 메서드를 이어서 호출합니다.

## Struct

`struct`는 새 타입을 만듭니다. Bevy의 컴포넌트와 리소스는 대부분 Rust struct입니다.

필드가 없는 표식 컴포넌트는 이렇게 씁니다.

```rust
#[derive(Component)]
struct Player;
```

엔티티에 `Player`가 붙어 있으면 플레이어고, 없으면 플레이어가 아닙니다. 존재 자체가 데이터입니다.

값 하나에 의미를 붙이고 싶을 때는 tuple struct가 좋습니다.

```rust
#[derive(Component)]
struct Velocity(Vec2);
```

접근은 숫자 필드로 합니다.

```rust
velocity.0 = Vec2::X * 260.0;
```

여러 정보를 이름으로 다루고 싶으면 이름 있는 필드의 struct를 씁니다.

```rust
#[derive(Component)]
struct Body {
    half_size: Vec2,
}
```

선택 기준은 이렇게 잡으면 됩니다.

```text
표식만 필요함                 -> unit struct
값 하나에 도메인 이름이 필요함 -> tuple struct
이름 있는 여러 정보가 필요함   -> named-field struct
생성 모양을 재사용해야 함      -> bundle struct
```

## Tuple과 Spawn

Rust의 tuple은 값을 묶습니다.

```rust
let pair = (10, 20);
let x = pair.0;
```

Bevy에서는 여러 컴포넌트를 한 엔티티에 붙일 때 tuple을 자주 씁니다.

```rust
commands.spawn((
    Player,
    Sprite::from_color(Color::srgb(0.25, 0.70, 1.0), Vec2::splat(80.0)),
    Transform::from_translation(Vec3::ZERO),
));
```

바깥 `spawn(...)`은 함수 호출이고, 안쪽 `(...)`은 컴포넌트 tuple입니다.

## Derive와 Trait

Trait는 타입이 만족해야 하는 동작 규칙입니다. Bevy는 derive macro로 필요한 trait 구현을 많이 만들어 줍니다.

```rust
#[derive(Component)]
struct Player;

#[derive(Resource, Default)]
struct Score(u32);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
}
```

`Component`는 이 타입을 엔티티에 붙일 수 있다는 뜻입니다. `Resource`는 월드에 하나 저장할 수 있다는 뜻입니다. `Default`는 기본값을 만들 수 있다는 뜻입니다.

## `impl`과 `Self`

`impl` block은 타입에 함수나 메서드를 붙입니다.

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

읽는 규칙은 이렇습니다.

```text
BodyBundle::new(...)  BodyBundle 타입에 붙은 함수 호출
Self                  지금 impl 중인 타입
Self { ... }          그 타입의 값을 생성
```

Bevy 예제에서 bundle에 `new`를 붙이는 이유는 spawn 코드를 단순하게 만들기 위해서입니다.

```rust
commands.spawn(PlayerBundle::new(&asset_server));
```

호출하는 쪽에서는 무엇을 생성하는지만 보이고, 필요한 컴포넌트 조립은 bundle 안에 들어갑니다.

## Enum과 `match`

Enum은 가능한 상태가 정해져 있을 때 씁니다.

```rust
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum PlayerAnimState {
    Idle,
    Run,
    Attack,
}
```

`match`는 모든 경우를 처리합니다.

```rust
match animation.state {
    PlayerAnimState::Idle => atlas.index = 0,
    PlayerAnimState::Run => atlas.index = animation.run_frame,
    PlayerAnimState::Attack => atlas.index = 3,
}
```

컴파일러가 enum variant를 빠뜨렸는지 확인해 줍니다. 애니메이션, 게임 상태, 메뉴 상태, 저장/불러오기 분기에서 특히 유용합니다.

## 소유권, 빌림, Bevy 시스템

Rust는 대부분의 값에 명확한 소유자를 둡니다. 소유한 값을 넘기면 이동이 일어날 수 있습니다.

```rust
let name = String::from("player");
let other = name; // name이 other로 이동
```

작은 숫자나 수학 타입은 `Copy`인 경우가 많아서 대입하면 복사됩니다.

```rust
let a = Vec2::X;
let b = a; // Vec2는 복사됨
```

빌림(borrowing)은 값을 잠깐 빌려 쓰는 것입니다.

```rust
fn length(v: &Vec2) -> f32 {
    v.length()
}

fn push_right(v: &mut Vec2) {
    v.x += 1.0;
}
```

Bevy 시스템 매개변수는 Rust borrowing을 타입으로 드러낸 것입니다.

```text
Res<T>       리소스를 읽기 전용으로 빌림
ResMut<T>    리소스를 수정 가능하게 빌림
&T           컴포넌트를 읽기 전용으로 빌림
&mut T       컴포넌트를 수정 가능하게 빌림
```

어떤 시스템이 `Query<&mut Transform, With<Player>>`를 요청하면, Bevy는 이 시스템이 플레이어의 `Transform`을 쓴다는 것을 압니다. 이 타입 정보가 안전한 스케줄링의 재료가 됩니다.

## 제네릭과 Query 문법

제네릭은 어떤 타입 안에 다른 타입을 넣는 문법입니다.

```rust
Res<Time>
Res<ButtonInput<KeyCode>>
Query<&mut Transform, With<Player>>
Query<(&mut Transform, &Velocity), With<Body>>
```

안쪽부터 읽으면 됩니다.

```text
ButtonInput<KeyCode>              키보드 키 상태
Res<ButtonInput<KeyCode>>         그 상태를 리소스로 읽음
With<Player>                      Player가 붙은 엔티티만 선택
Query<&mut Transform, With<Player>> Player 엔티티의 Transform을 수정 가능하게 가져옴
```

`Query`의 데이터 튜플은 같은 엔티티에서 여러 컴포넌트를 가져온다는 뜻입니다.

```rust
for (mut transform, velocity) in &mut bodies {
    transform.translation += velocity.0.extend(0.0);
}
```

쿼리가 tuple을 요청했기 때문에 반복문 변수도 tuple입니다.

## `Option`, `Result`, 빠른 반환

`Option<T>`는 값이 있을 수도 있고 없을 수도 있다는 뜻입니다.

```rust
let Some(atlas) = &mut sprite.texture_atlas else {
    return;
};
```

스프라이트에 texture atlas가 없으면 이 시스템은 여기서 끝납니다.

`Result<T, E>`는 성공하거나 실패할 수 있는 작업을 나타냅니다.

```rust
fn save_progress_to_disk(progress: &Progress) -> Result<(), String> {
    let json = serde_json::to_string_pretty(progress).map_err(|error| error.to_string())?;
    fs::write(SAVE_PATH, json).map_err(|error| error.to_string())
}
```

`?`는 오류가 나면 바로 반환합니다. 저장/불러오기 장에서 다시 씁니다.

## Module, `use`, `pub`

`use`는 이름을 현재 scope로 가져옵니다.

```rust
use bevy::prelude::*;
```

모듈은 코드를 나누는 단위입니다.

```rust
mod body;
mod player;
```

공개 여부는 `pub`로 정합니다.

```rust
pub struct BodyPlugin;
```

처음에는 한 예제 파일에서 시작해도 됩니다. 책임이 안정되면 그때 모듈로 빼면 됩니다. 예를 들면 Body 이동, 플레이어 동작, 에셋, UI, 저장/불러오기 같은 단위입니다.

## 시스템 시그니처 읽기

Bevy 시스템을 볼 때는 항상 이 질문으로 읽습니다.

```rust
fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&mut Velocity, &mut Facing), With<Player>>,
) {
}
```

```text
읽는 리소스는?          keyboard
쓰는 리소스는?          없음
읽는 컴포넌트는?        Facing은 읽기 전용이 아니라 수정 대상
쓰는 컴포넌트는?        Velocity, Facing
어떤 엔티티를 고르나?    Player, Velocity, Facing이 있는 엔티티
월드 구조를 바꾸나?      Commands가 없으므로 spawn/despawn은 하지 않음
```

이 습관 하나만 있어도 Bevy 코드가 훨씬 덜 복잡해 보입니다.

## 확인

다음 표현을 설명할 수 있으면 다음 장으로 넘어가도 됩니다.

```text
App::new()
commands.spawn(Camera2d)
Transform::from_xyz(0.0, 0.0, 1.0)
velocity.0
Query<(&mut Transform, &Velocity), With<Body>>
```

## 바꿔보기

`examples/03_player_input.rs`에서 이 코드를 찾아봅니다.

```rust
struct PlayerSpeed(f32);
```

이렇게 바꿔 봅니다.

```rust
struct PlayerSpeed(pub f32);
```

이 예제는 한 파일 안에 있으므로 그대로 동작합니다. 나중에 타입이 모듈 밖으로 나가면 `pub`가 다른 모듈에서 필드에 접근할 수 있는지를 결정합니다.

---

<div align="center">

[← 이전: 프로젝트 설정](00-project-setup.md) · [목차](index.md) · [다음: Bevy 앱 모델 →](02-bevy-app-model.md)

</div>
