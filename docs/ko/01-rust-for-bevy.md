# 1. Bevy를 위한 Rust

[목차](index.md) | 이전: [프로젝트 설정](00-project-setup.md) | 다음: [Bevy 앱 모델](02-bevy-app-model.md)

이 장은 Rust 전체 강의가 아닙니다. 이 저장소의 예제를 Bevy 마법처럼 외우지 않고 읽기 위해 필요한 Rust만 다룹니다.

가장 중요한 습관은 타입을 먼저 읽는 것입니다. Bevy에서 시스템 시그니처는 그 시스템이 어떤 데이터를 읽는지, 무엇을 수정하는지, 그 데이터가 엔티티별 컴포넌트인지 전역 리소스인지 시스템 로컬 데이터인지 알려줍니다.

## 변수와 가변성

Rust 변수는 기본적으로 불변입니다.

```rust
let direction = Vec2::ZERO;
direction.x += 1.0; // compile error
```

바인딩 자체가 바뀌어야 하면 `mut`을 씁니다.

```rust
let mut direction = Vec2::ZERO;
direction.x += 1.0;
```

`examples/03_player_input.rs`에서 이 패턴을 볼 수 있습니다.

```rust
let mut direction = Vec2::ZERO;

if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
    direction.x -= 1.0;
}
```

`mut`은 바인딩에 붙습니다. `Vec2` 타입의 일부가 아닙니다. 같은 규칙은 시스템 파라미터에도 나타납니다.

```rust
mut players: Query<&mut Transform, With<Player>>
```

이 말은 지역 변수 `players`를 가변으로 순회할 수 있고, 쿼리가 매칭된 각 `Transform`에 대한 가변 접근을 준다는 뜻입니다.

## Struct 형태

Rust의 `struct`는 타입을 만듭니다. 예제에서는 세 가지 흔한 형태를 씁니다.

### Unit Struct

```rust
#[derive(Component)]
struct Player;
```

이 타입은 필드를 저장하지 않습니다. Bevy에서는 marker component로 유용합니다. 엔티티에 `Player`가 있거나 없거나 둘 중 하나입니다.

`examples/07_rpg_slice.rs`의 `Player`, `Enemy`, `Collectible`, `HealthBarFill`은 모두 marker component입니다.

### Tuple Struct

```rust
#[derive(Component)]
struct Velocity(Vec2);
```

tuple struct는 이름 없는 필드를 가집니다. 접근할 때는 번호 필드 문법을 씁니다.

```rust
velocity.0 = direction.normalize_or_zero() * PLAYER_SPEED;
```

원시 타입이나 엔진 타입에 도메인 의미를 붙이고 싶을 때 tuple struct가 좋습니다. `Vec2`는 그냥 벡터이지만, `Velocity(Vec2)`는 "이 엔티티의 이동 속도"를 뜻합니다.

### Named-Field Struct

```rust
#[derive(Component)]
struct Body {
    half_size: Vec2,
}
```

데이터에 여러 의미가 있거나 필드 이름이 코드를 더 명확하게 만들 때는 이름 있는 필드가 좋습니다. 최종 예제에서 `Body`는 `half_size`를 저장하고, 충돌 코드는 사각형의 반지름 크기를 비교합니다.

```rust
let allowed = a_body.half_size + b_body.half_size;
```

## Enum

`enum`은 값이 여러 variant 중 하나인 타입입니다. `examples/05_plugins_sets.rs`는 시스템 순서를 위해 enum을 씁니다.

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
}
```

`GameSet::Input`과 `GameSet::Movement`는 문자열이 아닙니다. Bevy가 시스템 스케줄링에 사용할 수 있는 타입 있는 라벨입니다.

최종 예제에서는 이 enum이 더 커집니다.

```rust
enum GameSet {
    Input,
    Ai,
    Movement,
    Collision,
    Display,
}
```

## Derive

`derive`는 Rust에게 trait 구현을 생성해 달라고 요청합니다.

```rust
#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PlayerSpeed(f32);

#[derive(Bundle)]
struct BodyBundle {
    body: Body,
    velocity: Velocity,
    transform: Transform,
}
```

타입은 여전히 우리가 만든 타입입니다. derive는 그 타입을 특정 Bevy 역할에서 쓸 수 있게 만듭니다.

- `Component`: 엔티티에 붙일 수 있습니다.
- `Resource`: 월드에 하나 저장할 수 있습니다.
- `Bundle`: spawn할 때 여러 컴포넌트로 펼쳐질 수 있습니다.
- `SystemSet`: 시스템 순서 라벨로 쓸 수 있습니다.

일부 derive는 Rust 표준 trait나 스케줄링에 필요한 공통 trait에서 옵니다. `GameSet`의 `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`는 Bevy가 set 라벨을 비교하고 저장할 수 있게 합니다.

## Impl 블록

`impl`은 타입에 함수를 붙입니다.

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

`Self`는 지금 구현 중인 타입입니다. 여기서는 `BodyBundle`을 뜻합니다.

예제들은 `new` 생성자를 사용해서 spawn 규칙을 한 곳에 모읍니다.

```rust
commands.spawn(PlayerBundle::new());
```

모든 spawn 지점에서 컴포넌트를 반복하는 것보다 이 줄이 검토하기 쉽습니다.

## Trait와 Trait Bound

trait는 행동 계약입니다. Bevy의 `Plugin` trait는 플러그인이 `build`를 제공해야 한다고 말합니다.

```rust
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
```

이렇게 읽으면 됩니다.

```text
GamePlugin satisfies Bevy's Plugin contract.
```

Bevy 코드에서는 trait bound가 붙은 generic도 자주 봅니다. helper 함수가 어떤 컴포넌트 타입이든 받게 하려면 bound를 씁니다.

```rust
fn spawn_marker<T: Component>(commands: &mut Commands, marker: T) {
    commands.spawn((marker, Transform::default()));
}
```

`T: Component`는 "`T`는 Bevy 컴포넌트이기만 하면 어떤 타입이어도 된다"는 뜻입니다. 그래서 이 함수는 서로 다른 marker component를 받을 수 있습니다.

```rust
spawn_marker(&mut commands, Player);
spawn_marker(&mut commands, Enemy);
```

bound가 없으면 Bevy는 `marker: T`가 유효한 컴포넌트 데이터인지 알 수 없습니다.

최종 예제는 명시적인 `PlayerBundle`, `EnemyBundle`, `CollectibleBundle` 타입을 사용합니다. 더 반복적이지만, 배우는 중에는 각 spawn 형태가 구체적인 이름을 가지므로 읽기 쉽습니다.

## Ownership, Move, Copy, Clone

Rust에서 대부분의 값은 소유자가 하나입니다. `Copy`가 아닌 값을 대입하면 move됩니다.

```rust
let a = String::from("player");
let b = a;
// a is no longer usable
```

작고 단순한 값은 보통 `Copy`를 구현하므로 대입할 때 비트를 복사합니다.

```rust
let a = 10;
let b = a;
println!("{a} {b}");
```

많은 Bevy 수학 타입은 값으로 복사하기 저렴합니다. 위치나 벡터를 복사하는 코드를 자주 보게 됩니다.

```rust
let player_position = player.translation.truncate();
```

타입이 `Copy`가 아니고 복제가 필요할 때는 그 타입이 `Clone`을 지원하고 실제 복사가 의도된 경우에만 `clone()`을 쓰세요. 소유권 오류를 조용히 없애기 위해 반사적으로 `clone()`을 쓰지 말고, 먼저 누가 값을 소유해야 하는지 물어보세요.

## Reference: `&`와 `&mut`

Rust는 값을 소유하는 것과 빌리는 것을 구분합니다.

```rust
T       // owned value
&T      // shared read-only reference
&mut T  // exclusive mutable reference
```

Bevy 시스템 파라미터도 이 구분을 명시합니다.

```rust
Query<&Transform>      // read Transform
Query<&mut Transform>  // mutate Transform
Res<Score>             // read a resource
ResMut<Score>          // mutate a resource
```

같은 데이터에 대한 가변 borrow는 동시에 하나만 존재할 수 있습니다. 이 Rust 규칙 때문에 Bevy는 query conflict를 신경 씁니다.

```rust
Query<&Transform, With<Player>>
Query<&mut Transform, With<Camera2d>>
```

두 쿼리 모두 `Transform`을 건드리고, 하나는 가변입니다. Bevy가 두 쿼리가 다른 엔티티 집합을 매칭한다고 증명할 수 없다면 `Without<Camera2d>` 같은 필터를 추가하거나 작업을 별도 시스템으로 나누어야 합니다.

## Option, Result, `let else`

Rust는 값이 없을 수 있으면 `Option<T>`, 작업이 실패할 수 있으면 `Result<T, E>`를 씁니다.

Bevy의 단일 엔티티 query helper는 흔히 `Result`를 반환합니다.

```rust
let Ok(player) = player.single() else {
    return;
};
```

이것이 `let else`입니다. 뜻은 다음과 같습니다.

```text
player.single()이 Ok(value)를 반환하면 value를 player에 바인딩한다.
그렇지 않으면 함수를 일찍 끝낸다.
```

매칭 엔티티가 없거나 여러 개일 때 그 프레임을 건너뛰고 싶으면 이 방식을 씁니다. 최종 예제는 대신 `Single<...>`을 사용합니다. 더 엄격하며, 정확히 하나의 매칭 엔티티가 있어야 한다는 것을 시스템 계약으로 둡니다.

`Option`은 Bevy 컴포넌트 필드에서도 나타납니다.

```rust
sprite.custom_size = Some(Vec2::new(160.0 * health_fraction, 14.0));
```

`Some(value)`는 크기가 명시적으로 설정되었다는 뜻입니다. `None`이면 custom size가 없다는 뜻입니다.

## Module, `pub`, `use`

Rust는 모든 파일을 자동으로 컴파일하지 않습니다. 부모에서 모듈을 선언해야 합니다.

```rust
mod player;
```

이 줄은 `src/player.rs`를 로드합니다.

다른 모듈이 필요로 하는 이름에만 `pub`을 씁니다.

```rust
pub struct PlayerPlugin;
```

구현 세부사항은 기본적으로 private으로 두세요. `spawn_player`가 플러그인 안에서만 필요하다면 public일 필요가 없습니다.

`use`는 이름을 현재 scope로 가져옵니다.

```rust
use bevy::prelude::*;
```

Bevy prelude는 이 튜토리얼 전체에서 쓰는 공통 타입을 내보냅니다. `App`, `Plugin`, `Commands`, `Component`, `Resource`, `Query`, `Res`, `Transform`, `Vec2`, `Vec3`, `Color`, `Sprite` 등이 여기에 포함됩니다.

## 컴파일러 오류 읽기

Rust 오류는 보통 정확하지만 밀도가 높습니다. 다음 순서로 읽으세요.

1. 마지막 오류가 아니라 첫 오류부터 봅니다.
2. 내 코드의 파일과 줄 번호를 찾습니다.
3. expected type과 found type을 읽습니다.
4. "mutable", "immutable", "moved", "borrowed" 같은 borrow 관련 단어를 찾습니다.
5. 오류 하나를 고친 뒤 다시 `cargo check`를 실행합니다.

이 예제들에서 흔한 초보 오류:

- 수정하는 바인딩에 `mut`이 빠짐: `let direction`은 `let mut direction`이어야 합니다.
- 쿼리는 `&mut Transform`을 요청했지만 loop 변수가 mutable이 아님: `for mut transform in &mut players`를 씁니다.
- 쿼리에 쓰는 컴포넌트 타입이 `Component`를 derive하지 않음.
- 리소스를 insert했지만 `Resource`를 derive하지 않음.
- 시스템 함수는 올바르게 작성했지만 `add_systems`로 등록하지 않음.

## 체크포인트

다음 장으로 넘어가기 전에 `examples/03_player_input.rs`를 열고 확인해 보세요.

- 어떤 타입이 컴포넌트인가?
- 어떤 타입이 리소스인가?
- 어떤 지역 변수가 mutable인가?
- 어떤 시스템 파라미터가 입력을 읽는가?
- 어떤 시스템 파라미터가 컴포넌트 데이터를 수정하는가?

타입 시그니처만 보고 답할 수 있다면 Bevy 앱 모델로 넘어갈 준비가 된 것입니다.
