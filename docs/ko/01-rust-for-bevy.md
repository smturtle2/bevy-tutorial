# 1. Bevy를 위한 Rust


<div align="center">

[목차](index.md) · [← 이전: 프로젝트 설정](00-project-setup.md) · [다음: Bevy 앱 모델 →](02-bevy-app-model.md)

</div>

---

이 장은 Rust 전체 강의가 아닙니다. 이 저장소의 예제를 Bevy 마법처럼 외우지 않고 읽기 위해 필요한 Rust만 다룹니다.

가장 중요한 습관은 타입을 먼저 읽는 것입니다. Bevy에서 시스템 시그니처는 그 시스템이 어떤 데이터를 읽는지, 무엇을 수정하는지, 그 데이터가 엔티티별 컴포넌트인지 전역 리소스인지 시스템 로컬 데이터인지 알려줍니다.

## 이 장의 계약

이 장을 끝내면 다음을 할 수 있어야 합니다.

- `fn move_bodies(time: Res<Time>, mut bodies: Query<...>)` 같은 시스템 시그니처를 왼쪽에서 오른쪽으로 읽습니다.
- `struct Player;`, `struct Velocity(Vec2);`, `struct Body { ... }`의 차이를 설명합니다.
- `mut players: Query<&mut Transform, With<Player>>`에서 어느 부분이 바인딩의 가변성이고 어느 부분이 데이터 접근 권한인지 구분합니다.
- `App::new()`, `Transform::from_translation(...)`, `direction.normalize_or_zero()`에서 `::`와 `.`의 역할을 구분합니다.
- `Option`, `Result`, `let else`, `match`가 왜 Bevy 코드에 자주 나오는지 이해합니다.
- 소유권 오류가 나왔을 때 무작정 `clone()`을 붙이지 않고, 값 소유와 참조 중 어느 쪽이 필요한지 판단합니다.

다루지 않는 것:

- unsafe Rust
- 고급 lifetime 설계
- macro 작성
- async Rust
- crate 배포와 workspace 설계

이 주제들은 실제 게임 구조가 더 커진 뒤에 배우는 편이 낫습니다.

## Rust 코드 읽는 순서

Rust 코드를 처음 보면 기호가 많아 보입니다. Bevy 예제에서는 다음 순서로 읽으면 됩니다.

```text
1. 함수 이름을 본다.
2. 파라미터 이름과 타입을 본다.
3. & 또는 &mut가 있는지 본다.
4. Query, Res, ResMut, Commands 중 무엇을 받는지 본다.
5. 본문에서 어떤 값을 만들고 어떤 값을 바꾸는지 본다.
```

예를 들어 다음 시스템은:

```rust
fn move_bodies(time: Res<Time>, mut bodies: Query<(&mut Transform, &Velocity), With<Body>>) {
    for (mut transform, velocity) in &mut bodies {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}
```

이렇게 읽습니다.

```text
move_bodies는 매 프레임 호출될 수 있는 Rust 함수다.
Time 리소스를 읽는다.
Body가 붙은 엔티티들을 찾는다.
각 엔티티에서 Transform은 수정하고 Velocity는 읽는다.
Velocity.0의 Vec2를 Vec3로 바꾼 뒤 delta time을 곱해 위치에 더한다.
```

Rust는 "어떤 데이터에 어떤 권한으로 접근하는지"를 타입에 드러내는 언어입니다. Bevy는 이 성질을 ECS 스케줄링에 그대로 사용합니다.

## 타입 표기와 함수 시그니처

Rust에서 변수, 파라미터, 반환값의 타입 표기는 다음 형태입니다.

```rust
let speed: f32 = 220.0;

fn add_score(current: u32, amount: u32) -> u32 {
    current + amount
}
```

규칙:

```text
name: Type        변수나 파라미터의 타입
fn name(...)      함수 정의
-> Type           반환 타입
마지막 표현식       세미콜론이 없으면 반환값
```

위 함수의 마지막 줄은 `current + amount`이고 세미콜론이 없습니다. Rust에서는 블록의 마지막 표현식이 반환값이 될 수 있습니다.

세미콜론을 붙이면 그 표현식은 statement가 되고 값으로 반환되지 않습니다.

```rust
fn add_score(current: u32, amount: u32) -> u32 {
    current + amount;
    // compile error: expected u32, found ()
}
```

`()`는 "의미 있는 반환값 없음"을 뜻하는 unit 타입입니다. Bevy 시스템 함수는 보통 반환값이 없으므로 `-> ()`를 생략합니다.

```rust
fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Velocity, With<Player>>,
) {
    // return type is ()
}
```

## 타입 추론

Rust는 많은 타입을 추론합니다.

```rust
let mut direction = Vec2::ZERO;
```

여기서 `direction`의 타입은 `Vec2`입니다. 오른쪽 값이 `Vec2::ZERO`이기 때문입니다.

하지만 함수 파라미터 타입은 보통 직접 써야 합니다. 특히 Bevy 시스템 파라미터는 타입 자체가 시스템 계약이므로 명시하는 편이 맞습니다.

```rust
fn player_input(keyboard: Res<ButtonInput<KeyCode>>) {}
```

이 한 줄은 "이 시스템은 키보드 입력 리소스를 읽는다"는 계약입니다.

## `const`와 숫자 타입

반복해서 쓰는 고정값은 `const`로 둡니다.

```rust
const PLAYER_SPEED: f32 = 260.0;
const PLAYER_SIZE: Vec2 = Vec2::splat(42.0);
```

규칙:

```text
const NAME: Type = value;
```

`const`는 런타임에 바꾸는 설정값이 아니라 컴파일 시점의 고정값입니다. 예제에서 플레이어 크기, 적 크기, 공격 거리처럼 튜토리얼을 읽는 동안 명확해야 하는 숫자에 씁니다.

Bevy 2D 예제는 `f32`를 많이 씁니다. 화면 좌표, 속도, 시간 delta, 색상 값이 보통 실수이기 때문입니다.

```rust
let seconds: f32 = time.delta_secs();
let x = 120.0;
```

`120.0`은 실수 리터럴입니다. `120`은 정수 리터럴입니다. `Vec2`, `Vec3`, `Transform` 같은 Bevy 수학 타입은 대부분 `f32` 기반으로 움직인다고 생각하면 됩니다.

## `::`, `.`, 연관 함수와 메서드

Rust에서 `::`와 `.`는 다릅니다.

```rust
App::new()
Transform::from_translation(Vec3::ZERO)
Vec2::new(1.0, 0.0)
direction.normalize_or_zero()
```

읽는 규칙:

```text
Type::name(...)   타입에 붙은 연관 함수 또는 상수
value.name(...)   값에 대해 호출하는 메서드
Type::CONSTANT    타입에 붙은 상수
```

`App::new()`는 아직 `App` 값이 없으므로 타입에서 새 값을 만듭니다.

`Transform::from_translation(...)`도 새 `Transform` 값을 만듭니다.

`direction.normalize_or_zero()`는 이미 존재하는 `direction` 값에 대해 메서드를 호출합니다.

이 차이는 `impl` 블록에서 더 분명해집니다.

```rust
struct Score(u32);

impl Score {
    fn new() -> Self {
        Self(0)
    }

    fn add(&mut self, amount: u32) {
        self.0 += amount;
    }
}
```

호출:

```rust
let mut score = Score::new();
score.add(10);
```

`new`는 `self` 파라미터가 없으므로 `Score::new()`로 부릅니다. `add`는 `&mut self`를 받으므로 `score.add(...)`로 부릅니다.

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

비슷해 보이지만 다음 둘은 다릅니다.

```rust
let mut value = 10;
let 참조 = &mut value;
```

첫 줄의 `mut`은 `value` 바인딩을 바꿀 수 있다는 뜻입니다. 둘째 줄의 `&mut value`는 `value`를 독점적으로 빌린다는 뜻입니다.

또 다른 예를 보세요.

```rust
let a = 10;
let mut r = &a;
let b = 20;
r = &b;
```

이 코드는 됩니다. `r`이라는 바인딩이 mutable이라서 `r`이 가리키는 대상을 `a`에서 `b`로 바꿀 수 있습니다. 하지만 `r`의 타입은 여전히 `&i32`입니다. 참조를 통해 `a`나 `b` 값을 수정할 수 있다는 뜻이 아닙니다.

읽기 전용 참조에서 값을 복사해 수정 가능한 `i32`를 만들고 싶으면 이렇게 합니다.

```rust
let a = 10;
let r = &a;
let mut copied: i32 = *r;
copied += 1;
```

`i32`는 `Copy` 타입이므로 `*r`로 값을 복사할 수 있습니다. `copied`를 바꿔도 원래 `a`는 바뀌지 않습니다.

## Shadowing: 이름 가리기

Rust에서는 같은 이름을 다시 `let`으로 선언할 수 있습니다. 이것을 shadowing이라고 합니다.

```rust
let speed = "220";
let speed: f32 = speed.parse().unwrap_or(220.0);
```

첫 번째 `speed`는 문자열이고, 두 번째 `speed`는 `f32`입니다. 같은 변수를 수정한 것이 아니라 새 바인딩이 이전 이름을 가린 것입니다.

Bevy 예제에서는 과하게 쓰지 않지만, `let Ok(player) = ... else { ... };` 같은 패턴에서도 새 이름 바인딩이 만들어집니다.

## Struct 형태

Rust의 `struct`는 타입을 만듭니다. 예제에서는 세 가지 흔한 형태를 씁니다.

### Unit Struct: 필드 없는 타입

```rust
#[derive(Component)]
struct Player;
```

이 타입은 필드를 저장하지 않습니다. Bevy에서는 마커 컴포넌트(marker component)로 유용합니다. 엔티티에 `Player`가 있거나 없거나 둘 중 하나입니다.

`examples/07_rpg_slice.rs`의 `Player`, `Enemy`, `Collectible`, `HealthBarFill`은 모두 마커 컴포넌트입니다.

### 튜플 Struct: 이름 없는 필드

```rust
#[derive(Component)]
struct Velocity(Vec2);
```

튜플 struct는 이름 없는 필드를 가집니다. 접근할 때는 번호 필드 문법을 씁니다.

```rust
velocity.0 = direction.normalize_or_zero() * PLAYER_SPEED;
```

원시 타입이나 엔진 타입에 도메인 의미를 붙이고 싶을 때 튜플 struct가 좋습니다. `Vec2`는 그냥 벡터이지만, `Velocity(Vec2)`는 "이 엔티티의 이동 속도"를 뜻합니다.

## 튜플과 괄호 읽기

Rust에서 괄호는 여러 의미로 쓰입니다. Bevy의 `spawn`에서 특히 헷갈리기 쉽습니다.

```rust
commands.spawn((
    Player,
    Velocity(Vec2::ZERO),
    Transform::from_translation(Vec3::ZERO),
));
```

이 코드는 괄호가 두 겹입니다.

```text
spawn(...)    함수 호출 괄호
(A, B, C)     컴포넌트 여러 개를 묶은 튜플
```

`commands.spawn(Player)`처럼 하나만 넘길 수도 있지만, 보통 엔티티는 여러 컴포넌트를 함께 가지므로 튜플로 묶습니다.

Query에서도 튜플이 나옵니다.

```rust
Query<(&mut Transform, &Velocity), With<Body>>
```

여기서 `(&mut Transform, &Velocity)`는 "한 엔티티에서 두 컴포넌트를 함께 가져온다"는 튜플 타입입니다.

### 이름 있는 필드 Struct

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

이름 있는 struct 값을 만들 때는 필드 이름을 씁니다.

```rust
let body = Body {
    half_size: Vec2::splat(16.0),
};
```

필드가 많아지면 이 형태가 튜플 struct보다 검토하기 쉽습니다. 반대로 `Velocity(Vec2)`처럼 필드가 하나이고 의미가 타입 이름에 충분히 담기면 튜플 struct가 더 간단합니다.

Struct 선택 기준:

```text
marker만 필요하다          -> unit struct
값 하나에 도메인 이름을 준다 -> 튜플 struct
여러 필드가 있다            -> 이름 있는 필드 struct
```

## `Default`와 `..default()`

Rust 타입은 `Default` trait를 구현하면 기본값을 만들 수 있습니다.

```rust
let transform = Transform::default();
```

Bevy 타입 다수는 `Default`를 구현합니다. 그래서 필요한 필드만 지정하고 나머지는 기본값으로 채울 수 있습니다.

```rust
Sprite {
    image: asset_server.load("player.png"),
    custom_size: Some(Vec2::splat(48.0)),
    ..default()
}
```

`..default()`는 "위에서 지정하지 않은 나머지 필드는 기본값을 사용한다"는 뜻입니다. 이것은 Bevy 예제에서 매우 흔합니다.

주의할 점:

```text
Type::default()  타입 이름으로 기본값 생성
default()        타입을 문맥으로 추론할 수 있을 때 기본값 생성
..default()      struct literal에서 빠진 필드를 기본값으로 채움
```

`..default()`는 반드시 struct literal의 마지막에 옵니다.

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

Enum은 게임 상태에도 잘 맞습니다.

```rust
enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}
```

상태별로 다른 코드를 실행할 때는 `match`를 씁니다.

```rust
match state {
    GameState::Menu => show_menu(),
    GameState::Playing => update_game(),
    GameState::Paused => show_pause(),
    GameState::GameOver => show_game_over(),
}
```

`match`의 장점은 빠뜨린 variant를 컴파일러가 잡아준다는 것입니다. `GameState`에 `Loading`을 추가하면 그 상태를 처리하지 않은 `match`가 오류를 냅니다.

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

`impl` 안의 함수는 세 가지 형태로 자주 나뉩니다.

```rust
impl WaveSpawner {
    fn new() -> Self {
        Self { wave: 1 }
    }

    fn wave(&self) -> u32 {
        self.wave
    }

    fn reset(&mut self) {
        self.wave = 1;
    }
}
```

읽는 규칙:

```text
fn new() -> Self    값을 새로 만든다. 호출은 WaveSpawner::new()
fn wave(&self)      값을 읽는다. 호출은 spawner.wave()
fn reset(&mut self) 값을 수정한다. 호출은 spawner.reset()
```

`self`는 메서드를 호출한 값입니다. `&self`는 읽기 전용으로 빌리고, `&mut self`는 수정 가능하게 빌립니다.

## trait와 trait bound

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

## Generic 타입 읽기

Bevy 타입에는 `<...>`가 자주 나옵니다.

```rust
Res<ButtonInput<KeyCode>>
Query<(&mut Transform, &Velocity), With<Player>>
Handle<Image>
Assets<TextureAtlasLayout>
```

`<...>`는 타입 파라미터입니다. 읽을 때는 바깥에서 안쪽으로 읽습니다.

```text
Res<ButtonInput<KeyCode>>
= KeyCode 입력 상태를 담은 ButtonInput 리소스를 읽는다.

Query<(&mut Transform, &Velocity), With<Player>>
= Player가 있는 엔티티에서 Transform은 수정하고 Velocity는 읽는다.

Handle<Image>
= Image 에셋을 가리키는 핸들이다.
```

여기서 `Query`의 첫 번째 타입 파라미터는 가져올 데이터이고, 두 번째 타입 파라미터는 필터입니다.

```rust
Query<Data, Filter>
```

필터가 없으면 생략할 수 있습니다.

```rust
Query<&Transform>
```

이런 generic 문법은 어렵게 느껴져도, Bevy에서는 대부분 "무슨 타입의 데이터를 읽고 쓰는가"를 표현하기 위해 쓰입니다.

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

Bevy에서 소유권을 가장 자주 만나는 곳은 에셋 핸들과 컴포넌트 값입니다.

```rust
let image: Handle<Image> = asset_server.load("player.png");
commands.spawn(Sprite::from_image(image.clone()));
commands.spawn(Sprite::from_image(image));
```

여기서 `clone()`은 이미지 파일 전체를 복사한다는 뜻이 아닙니다. `Handle<Image>` 참조 값을 하나 더 만든다는 뜻입니다. 여러 엔티티가 같은 에셋을 가리켜야 하므로 이 경우의 `clone()`은 의도된 복제입니다.

반대로 단순히 컴파일러 오류를 없애기 위해 큰 게임 상태를 계속 `clone()`하는 것은 좋지 않습니다. 그 값이 누가 소유해야 하는지 다시 설계해야 합니다.

## 참조: `&`와 `&mut`

Rust는 값을 소유하는 것과 빌리는 것을 구분합니다.

```rust
T       // 소유한 값
&T      // 공유 읽기 전용 참조
&mut T  // 독점 가변 참조
```

Bevy 시스템 파라미터도 이 구분을 명시합니다.

```rust
Query<&Transform>      // Transform을 읽음
Query<&mut Transform>  // Transform을 수정함
Res<Score>             // 리소스를 읽음
ResMut<Score>          // 리소스를 수정함
```

같은 데이터에 대한 가변 borrow는 동시에 하나만 존재할 수 있습니다. 이 Rust 규칙 때문에 Bevy는 query conflict를 신경 씁니다.

```rust
Query<&Transform, With<Player>>
Query<&mut Transform, With<Camera2d>>
```

두 쿼리 모두 `Transform`을 건드리고, 하나는 가변입니다. Bevy가 두 쿼리가 다른 엔티티 집합을 매칭한다고 증명할 수 없다면 `Without<Camera2d>` 같은 필터를 추가하거나 작업을 별도 시스템으로 나누어야 합니다.

## 역참조: `*`

`*`는 참조나 wrapper 안쪽 값을 꺼내 접근할 때 보입니다.

```rust
let mut cooldown = 1.0;
let cooldown_ref = &mut cooldown;
*cooldown_ref -= 0.1;
```

`cooldown_ref` 자체는 참조입니다. `*cooldown_ref`는 그 참조가 가리키는 `f32`입니다.

Bevy의 `Local<T>`나 `ResMut<T>`에서도 같은 감각이 나옵니다.

```rust
fn tick(time: Res<Time>, mut hit_cooldown: Local<f32>) {
    *hit_cooldown -= time.delta_secs();
}
```

`hit_cooldown`은 `Local<f32>` wrapper입니다. 내부 `f32`를 바꾸려면 역참조해서 접근합니다.

대부분의 field 접근은 Rust의 deref coercion 때문에 자동으로 됩니다.

```rust
score.0 += 1;
```

`score`가 `ResMut<Score>`여도 Rust가 필요한 역참조를 어느 정도 자동 처리합니다. 하지만 숫자 자체를 대입하거나 산술할 때는 `*`가 직접 보일 수 있습니다.

## lifetime 감각

Bevy 초반 예제에서는 lifetime 표기를 직접 쓸 일이 거의 없습니다. 그래도 규칙은 알아야 합니다.

```text
참조는 원래 값보다 오래 살 수 없다.
```

잘못된 코드:

```rust
let r;
{
    let value = 10;
    r = &value;
}
println!("{r}");
```

`value`는 안쪽 블록이 끝나면 사라집니다. `r`이 그 뒤에도 `value`를 가리키면 dangling 참조가 되므로 Rust가 막습니다.

Bevy 시스템에서는 보통 월드가 컴포넌트와 리소스를 소유하고, 시스템 파라미터는 한 번의 시스템 실행 동안만 그것들을 빌립니다. 그래서 시스템 밖에 `&Transform` 같은 참조를 저장하려 하지 마세요. 오래 기억해야 하는 대상은 `Entity` ID, 복사 가능한 값, 리소스, 컴포넌트로 저장합니다.

## 제어 흐름: `if`, `for`, `return`

Rust의 `if`는 조건에 괄호가 필요 없습니다.

```rust
if direction.length_squared() > 0.0 {
    velocity.0 = direction.normalize() * speed.0;
}
```

`if`는 표현식이기도 합니다.

```rust
let animation = if velocity.0.length_squared() > 0.0 {
    PlayerAnimState::Run
} else {
    PlayerAnimState::Idle
};
```

`for`는 iterator를 순회합니다. Bevy query도 iterator처럼 순회할 수 있습니다.

```rust
for mut transform in &mut players {
    transform.translation.x += 1.0;
}
```

이 줄의 의미:

```text
players query를 mutable하게 순회한다.
각 엔티티의 Transform을 mutable하게 빌린다.
그 빌린 값을 transform이라는 loop 변수로 다룬다.
```

함수를 일찍 끝내려면 `return`을 씁니다.

```rust
let Ok(player) = players.single() else {
    return;
};
```

게임 루프에서는 "이번 프레임에 처리할 대상이 없으면 조용히 건너뛰기"가 자주 필요하므로 `return`이 흔합니다.

## 패턴과 구조 분해

Rust는 값을 형태에 맞춰 분해할 수 있습니다.

```rust
for (mut transform, velocity) in &mut bodies {
    transform.translation += velocity.0.extend(0.0);
}
```

`bodies` 쿼리의 item 타입이 `(&mut Transform, &Velocity)`이므로 loop 변수도 같은 모양으로 받습니다.

```text
(mut transform, velocity)
= 첫 번째 값은 mutable Transform 참조
= 두 번째 값은 Velocity 참조
```

`let else`도 pattern matching입니다.

```rust
let Ok(player) = players.single() else {
    return;
};
```

`players.single()`의 결과가 `Ok(...)` 모양이면 안쪽 값을 꺼내 `player`에 바인딩합니다. `Err(...)`이면 `else` 블록으로 갑니다.

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

`match`로 직접 풀 수도 있습니다.

```rust
match sprite.custom_size {
    Some(size) => println!("custom size: {size:?}"),
    None => println!("default size"),
}
```

저장/불러오기 예제에서는 `Result`가 더 중요합니다.

```rust
fn save_progress(progress: &Progress) -> Result<(), String> {
    let text = serde_json::to_string_pretty(progress).map_err(|err| err.to_string())?;
    std::fs::write(save_path(), text).map_err(|err| err.to_string())
}
```

읽는 규칙:

```text
Result<(), String>
= 성공하면 의미 있는 값은 없고, 실패하면 String 오류를 돌려준다.

?
= Ok 값이면 안쪽 값을 꺼내 계속 진행하고,
  Err 값이면 현재 함수에서 즉시 Err를 반환한다.
```

`?`를 쓰려면 현재 함수도 `Result`나 `Option`처럼 실패를 표현할 수 있는 타입을 반환해야 합니다.

## 모듈, `pub`, `use`

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

파일 분리 규칙은 다음처럼 생각하면 됩니다.

```text
src/main.rs에서 mod player; 선언
-> src/player.rs 파일이 player 모듈이 됨

player.rs 안의 pub struct PlayerPlugin;
-> main.rs나 다른 모듈에서 player::PlayerPlugin으로 접근 가능

player.rs 안의 struct Player;
-> player 모듈 내부에서만 접근 가능
```

Bevy 프로젝트에서는 보통 plugin 타입만 `pub`으로 열고, 내부 컴포넌트와 시스템 함수는 필요한 경우에만 공개합니다. 공개 API를 줄이면 나중에 파일 구조를 바꾸기 쉽습니다.

## Rust가 Bevy에서 대신 해주지 않는 것

Bevy가 ECS borrow conflict를 많이 검사해 주지만, Rust 규칙이 사라지는 것은 아닙니다.

```text
Rust가 보장하는 것:
- dangling 참조 없음
- 동시에 같은 값에 여러 mutable 참조 없음
- 타입이 맞지 않는 값 사용 불가

Bevy가 얹는 것:
- 시스템 파라미터 타입을 보고 월드 데이터 접근을 관리
- Query 필터로 엔티티 집합을 선택
- Commands로 구조 변경을 안전한 시점에 적용
```

따라서 "Bevy가 알아서 해주니까 소유권을 몰라도 된다"가 아닙니다. 더 정확한 말은 "Bevy가 Rust의 소유권 규칙 위에 ECS API를 제공하므로, 시스템 안에서는 타입 시그니처를 올바르게 쓰는 것이 곧 소유권 설계다"입니다.

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
- `Query<&mut T>`를 두 개 받아서 Bevy가 같은 엔티티를 동시에 수정할 수 있다고 판단함.
- 읽기 전용 참조 `&T`로 값을 수정하려고 함.
- `Option<T>`나 `Result<T, E>`를 풀지 않고 안쪽 값처럼 사용하려고 함.

## 체크포인트

다음 장으로 넘어가기 전에 `examples/03_player_input.rs`와 `examples/04_velocity_body.rs`를 열고 확인해 보세요.

- 어떤 타입이 컴포넌트인가?
- 어떤 타입이 리소스인가?
- 어떤 지역 변수가 mutable인가?
- 어떤 시스템 파라미터가 입력을 읽는가?
- 어떤 시스템 파라미터가 컴포넌트 데이터를 수정하는가?
- `::`로 호출되는 함수와 `.`으로 호출되는 메서드는 각각 무엇인가?
- 튜플 struct의 `.0` 접근은 어디에 나오는가?
- `for` loop의 아이템 모양은 query item 타입과 어떻게 대응되는가?

타입 시그니처만 보고 답할 수 있다면 Bevy 앱 모델로 넘어갈 준비가 된 것입니다.

---

<div align="center">

[← 이전: 프로젝트 설정](00-project-setup.md) · [목차](index.md) · [다음: Bevy 앱 모델 →](02-bevy-app-model.md)

</div>
