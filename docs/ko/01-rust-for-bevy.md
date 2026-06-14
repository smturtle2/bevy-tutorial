# 1. Bevy를 위한 Rust


<div align="center">

[목차](index.md) · [← 이전: 프로젝트 설정](00-project-setup.md) · [다음: Bevy 앱 모델 →](02-bevy-app-model.md)

</div>

---

Rust는 앞으로 작성할 모든 Bevy 시스템의 언어 계층입니다. Bevy는 Rust의 모델을 직접 활용합니다. 컴포넌트는 Rust 타입이고, 리소스도 Rust 타입이고, 시스템은 Rust 함수이며, 쿼리는 Rust의 borrow 권한을 타입으로 표현합니다.

이 장에서는 튜토리얼 예제를 직접 읽고 고칠 수 있도록 Rust 기본기를 세웁니다.

## 이 장의 계약

이 장을 끝내면 다음을 할 수 있어야 합니다.

- Rust 함수 시그니처에서 파라미터 타입, 반환 타입, mutable 바인딩을 읽습니다.
- `let`, `mut`, `const`, `fn`, `struct`, `enum`, `impl`, `use`, `pub`의 역할을 설명합니다.
- 소유한 값, 공유 참조 `&T`, 가변 참조 `&mut T`를 구분합니다.
- `struct Player;`, `struct Velocity(Vec2);`, `struct Body { half_size: Vec2 }`의 차이를 설명합니다.
- `App::new()`, `Transform::from_translation(...)`, `direction.normalize_or_zero()`, `velocity.0`을 읽습니다.
- Bevy 코드에 `derive`, `Query<...>` 같은 generic 타입, `Res<T>` 같은 wrapper가 왜 나오는지 이해합니다.
- 값이 없거나 작업이 실패할 수 있을 때 `Option`, `Result`, `match`, `let else`, `?`를 처리합니다.
- Bevy 시스템 시그니처를 데이터 접근 계약으로 읽습니다.

## Rust의 기본 모델

Rust 코드는 몇 가지 생각으로 구성됩니다.

```text
value        런타임에 존재하는 데이터
type         데이터의 모양과 규칙에 붙인 이름
binding      let으로 만든 지역 이름
function     재사용 가능한 동작 블록
ownership    대부분의 값에는 명확한 소유자가 하나 있음
borrowing    & 또는 &mut으로 잠깐 접근을 빌림
trait        타입이 만족하는 행동 계약
module       파일과 namespace 구성
```

Bevy는 이 생각들을 게임 코드에 그대로 연결합니다.

```text
component    엔티티에 붙는 Rust 타입
resource     월드에 하나 저장되는 Rust 타입
system       Bevy가 실행하는 Rust 함수
plugin       시스템과 리소스를 등록하는 Rust 타입
query        컴포넌트 접근을 타입으로 요청하는 도구
```

Bevy 코드가 빽빽해 보이면 먼저 Rust 타입을 읽습니다. 타입 시그니처가 그 코드가 건드릴 수 있는 데이터를 거의 다 말해 줍니다.

## 작은 Bevy 파일을 Rust로 읽기

`examples/01_empty_app.rs`는 이미 실제 Rust 코드입니다.

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

Rust로 먼저 읽으면 다음과 같습니다.

```text
use bevy::prelude::*;
    Bevy에서 자주 쓰는 이름을 현재 파일로 가져온다.

fn main() { ... }
    프로그램 시작점이다.

App::new()
    App 타입에 붙은 연관 함수를 호출해서 App 값을 만든다.

.insert_resource(...)
.add_plugins(...)
.add_systems(...)
.run()
    App 값에 대해 메서드를 연속으로 호출한다.

fn setup_camera(mut commands: Commands)
    함수를 정의한다. Bevy는 이 함수를 시스템으로 실행할 수 있다.

commands.spawn(Camera2d);
    Commands 값을 사용해서 새 엔티티 생성을 요청한다.
```

`main`의 체이닝은 표준 Rust 메서드 호출 문법입니다. 각 메서드가 다음 메서드를 호출할 수 있는 값을 돌려주기 때문에 이어서 쓸 수 있습니다.

## 바인딩, 값, 타입

Rust는 `let`으로 지역 이름을 만듭니다.

```rust
let score = 0;
let speed: f32 = 280.0;
let name = "player";
```

규칙:

```text
let name = value;        바인딩 생성
let name: Type = value;  타입을 명시해서 바인딩 생성
```

Rust는 오른쪽 값을 보고 타입을 추론할 때가 많습니다.

```rust
let mut direction = Vec2::ZERO;
```

`direction`은 `Vec2`입니다. 오른쪽의 `Vec2::ZERO`가 `Vec2` 값이기 때문입니다.

함수 파라미터는 다릅니다. 보통 타입을 직접 씁니다.

```rust
fn add_score(current: u32, amount: u32) -> u32 {
    current + amount
}
```

규칙:

```text
current: u32     current라는 파라미터의 타입은 u32
amount: u32      amount라는 파라미터의 타입은 u32
-> u32           함수가 u32를 반환
```

Bevy 시스템 파라미터는 특히 명시해야 합니다. 타입 자체가 시스템 계약이기 때문입니다.

```rust
fn player_input(keyboard: Res<ButtonInput<KeyCode>>) {}
```

이 시그니처는 "이 시스템은 키보드 입력 리소스를 읽는다"는 뜻입니다.

## 가변성

Rust 바인딩은 기본적으로 불변입니다.

```rust
let direction = Vec2::ZERO;
direction.x += 1.0; // compile error
```

바인딩이 바뀌어야 하면 `mut`을 붙입니다.

```rust
let mut direction = Vec2::ZERO;
direction.x += 1.0;
```

입력 예제는 키 입력으로 방향을 만들어야 하므로 이 패턴을 씁니다.

```rust
let mut direction = Vec2::ZERO;

if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
    direction.x -= 1.0;
}
```

`mut`은 바인딩에 붙습니다. 타입은 그대로 `Vec2`입니다.

다음 두 줄은 의미가 다릅니다.

```rust
let mut value = 10;
let reference = &mut value;
```

첫 줄은 지역 바인딩 `value`를 바꿀 수 있다는 뜻입니다. 둘째 줄은 `value`를 독점적으로 가변 borrow한다는 뜻입니다.

참조 바인딩 자체를 mutable하게 만들 수도 있습니다.

```rust
let a = 10;
let b = 20;
let mut r = &a;
r = &b;
```

`r`은 나중에 `b`를 가리키도록 다시 대입될 수 있습니다. 하지만 타입은 여전히 `&i32`이고, 이는 `i32`에 대한 읽기 전용 접근입니다.

읽기 전용 참조에서 값을 복사해 수정 가능한 정수를 만들고 싶으면 이렇게 합니다.

```rust
let a = 10;
let r = &a;
let mut copied: i32 = *r;
copied += 1;
```

`i32`는 `Copy` 타입이므로 `*r`로 값을 복사할 수 있습니다. 그 뒤 `copied`는 `a`와 독립적으로 바뀝니다.

## 숫자와 상수

튜토리얼 예제에서는 이 숫자 타입을 자주 씁니다.

```text
i32, u32    정수
f32         32-bit 실수
usize       크기나 인덱스 타입
```

Bevy 2D 수학에서는 `f32`가 많이 나옵니다. 위치, 속도, 시간 delta, 크기가 보통 실수이기 때문입니다.

```rust
let seconds: f32 = time.delta_secs();
let x = 120.0;
let lives: u32 = 3;
```

`120.0`은 실수 리터럴입니다. `120`은 정수 리터럴입니다.

코드를 읽을 때 바로 보여야 하는 고정값은 `const`로 둡니다.

```rust
const PLAYER_SPEED: f32 = 280.0;
const PLAYER_SIZE: Vec2 = Vec2::splat(80.0);
```

규칙:

```text
const NAME: Type = value;
```

`const`는 컴파일 시점의 고정값입니다. 런타임 설정값은 리소스에 둡니다.

## 표현식, statement, 세미콜론

Rust에는 값을 만드는 expression과 동작을 수행하는 statement가 있습니다.

블록의 마지막 expression은 반환값이 될 수 있습니다.

```rust
fn add_score(current: u32, amount: u32) -> u32 {
    current + amount
}
```

`current + amount` 뒤에 세미콜론이 없으므로 이 값이 반환됩니다.

세미콜론을 붙이면 expression이 statement가 됩니다.

```rust
fn add_score(current: u32, amount: u32) -> u32 {
    current + amount;
    // compile error: expected u32, found ()
}
```

`()`는 Rust의 unit 타입입니다. "의미 있는 값 없음"이라는 뜻입니다.

대부분의 Bevy 시스템은 unit을 반환합니다.

```rust
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
```

반환 타입이 `()`라서 생략되어 있습니다.

`if`도 값을 만들 수 있습니다.

```rust
let animation = if direction.length_squared() > 0.0 {
    PlayerAnimState::Run
} else {
    PlayerAnimState::Idle
};
```

두 branch는 서로 호환되는 타입을 만들어야 합니다.

## `::`, `.`, 연관 함수, 메서드

Rust는 타입이나 모듈 아래에 있는 이름에 `::`를 씁니다.

```rust
App::new()
Transform::from_translation(Vec3::ZERO)
Vec2::new(1.0, 0.0)
Vec2::ZERO
```

규칙:

```text
Type::function(...)  타입에 붙은 연관 함수
Type::CONSTANT       타입에 붙은 연관 상수
module::name         모듈 안의 이름
```

이미 존재하는 값의 메서드나 필드에는 `.`을 씁니다.

```rust
direction.normalize_or_zero()
transform.translation
velocity.0
```

규칙:

```text
value.method(...)    값에 대해 메서드 호출
value.field          이름 있는 필드 접근
value.0              튜플 필드 0번 접근
```

`App::new()`는 아직 `App` 값이 없으므로 `App` 타입에서 새 값을 만듭니다. `.add_plugins(...)`는 `App` 값이 생긴 뒤 그 값에 대해 호출됩니다.

## Struct

`struct`는 새 타입을 만듭니다. Bevy 코드에서는 컴포넌트와 리소스가 Rust 타입이기 때문에 struct를 계속 보게 됩니다.

### Unit Struct

```rust
#[derive(Component)]
struct Player;
```

이 struct에는 필드가 없습니다. Bevy에서는 marker component로 유용합니다. 엔티티에 `Player`가 붙어 있거나 없거나 둘 중 하나입니다.

흔한 marker:

```rust
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;
```

### Tuple Struct

```rust
#[derive(Resource)]
struct PlayerSpeed(f32);

#[derive(Component)]
struct Velocity(Vec2);
```

tuple struct는 이름 없는 필드를 가집니다. 접근할 때는 숫자 필드 문법을 씁니다.

```rust
let speed = player_speed.0;
velocity.0 = direction.normalize_or_zero() * speed;
```

값 하나에 도메인 의미를 붙이고 싶을 때 tuple struct가 좋습니다. `Vec2`는 벡터입니다. `Velocity(Vec2)`는 "이 엔티티의 이동 속도"입니다.

### 이름 있는 필드 Struct

```rust
#[derive(Component)]
struct Body {
    half_size: Vec2,
}
```

필드 이름이 코드를 더 명확하게 만들면 이름 있는 필드가 낫습니다.

```rust
let body = Body {
    half_size: Vec2::splat(16.0),
};

let allowed = player_body.half_size + enemy_body.half_size;
```

선택 규칙:

```text
marker만 필요함             -> unit struct
값 하나에 도메인 의미를 붙임 -> tuple struct
이름 있는 여러 사실이 있음   -> 이름 있는 필드 struct
```

## 튜플과 Spawn 호출

Rust tuple은 여러 값을 묶습니다.

```rust
let pair = (10, 20);
let x = pair.0;
let y = pair.1;
```

Bevy는 여러 컴포넌트를 한 번에 spawn할 때 tuple을 씁니다.

```rust
commands.spawn((
    Player,
    Sprite::from_color(Color::srgb(0.25, 0.70, 1.0), Vec2::splat(80.0)),
    Transform::from_translation(Vec3::ZERO),
));
```

괄호가 두 겹입니다.

```text
spawn(...)    함수 호출 괄호
(A, B, C)     여러 컴포넌트를 담은 tuple
```

Query에서도 tuple이 나옵니다.

```rust
Query<(&mut Transform, &Velocity), With<Body>>
```

`(&mut Transform, &Velocity)`는 "같은 엔티티에서 두 컴포넌트를 함께 가져온다"는 뜻입니다.

## Derive와 Trait

trait는 행동 계약입니다. `derive`는 Rust에게 trait 구현을 생성해 달라고 요청합니다.

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

이 derive들은 역할 선언처럼 읽으면 됩니다.

```text
Component  이 타입은 엔티티에 붙을 수 있음
Resource   이 타입은 월드에 하나 저장될 수 있음
Bundle     이 타입은 spawn할 때 여러 컴포넌트로 펼쳐질 수 있음
```

일부 derive는 일반 Rust trait입니다.

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
}
```

Bevy는 system set label을 저장하고 비교하고 디버그하기 위해 이런 trait가 필요합니다.

trait를 직접 구현할 수도 있습니다. Bevy plugin은 `Plugin` trait를 구현합니다.

```rust
struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
```

이렇게 읽습니다.

```text
GamePlugin은 build를 제공해서 Bevy의 Plugin 계약을 만족한다.
```

## Impl 블록과 `Self`

`impl`은 타입에 함수를 붙입니다.

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

`Self`는 지금 구현 중인 타입입니다. 여기서는 `Score`를 뜻합니다.

호출:

```rust
let mut score = Score::new();
score.add(10);
```

규칙:

```text
fn new() -> Self       self 파라미터 없음, Score::new()로 호출
fn value(&self)        기존 값을 읽음, score.value()로 호출
fn add(&mut self, ...) 기존 값을 수정함, score.add(...)로 호출
```

Bevy 예제에서는 생성자를 사용해서 spawn 규칙을 한 곳에 모읍니다.

```rust
impl PlayerBundle {
    fn new(position: Vec3) -> Self {
        Self {
            player: Player,
            velocity: Velocity(Vec2::ZERO),
            transform: Transform::from_translation(position),
        }
    }
}
```

그러면 spawn이 하나의 의도로 읽힙니다.

```rust
commands.spawn(PlayerBundle::new(Vec3::ZERO));
```

## Enum과 Match

`enum`은 값이 여러 variant 중 하나인 타입입니다.

```rust
enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}
```

variant는 enum 아래에 이름이 붙습니다.

```rust
GameState::Menu
GameState::Playing
```

variant별 처리는 `match`로 합니다.

```rust
match state {
    GameState::Menu => show_menu(),
    GameState::Playing => update_game(),
    GameState::Paused => show_pause(),
    GameState::GameOver => show_game_over(),
}
```

컴파일러는 모든 variant가 처리됐는지 확인합니다. 나중에 `Loading`을 추가하면 Rust가 수정해야 할 `match`를 알려줍니다.

Bevy는 타입 있는 system label에도 enum을 씁니다.

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
}
```

`GameSet::Input`은 타입 있는 label입니다.

## Generic

generic은 타입이나 함수가 다른 타입을 파라미터로 받게 합니다.

```rust
Vec<T>
Option<T>
Result<T, E>
Handle<Image>
Res<ButtonInput<KeyCode>>
Query<&mut Transform, With<Player>>
```

바깥에서 안쪽으로 읽습니다.

```text
Handle<Image>
    Image 에셋을 가리키는 handle

Res<ButtonInput<KeyCode>>
    KeyCode 입력 상태를 추적하는 ButtonInput 리소스에 대한 읽기 접근

Query<&mut Transform, With<Player>>
    Player가 있는 엔티티에서 Transform을 가변으로 가져옴
```

`Query`는 이 모양입니다.

```rust
Query<Data, Filter>
```

예:

```rust
Query<&Transform>
Query<&mut Transform, With<Player>>
Query<(&mut Transform, &Velocity), With<Body>>
```

첫 번째 타입 파라미터는 가져올 컴포넌트 데이터입니다. 두 번째 타입 파라미터가 있으면 어떤 엔티티를 매칭할지 필터링합니다.

함수도 generic일 수 있습니다.

```rust
fn spawn_marker<T: Component>(commands: &mut Commands, marker: T) {
    commands.spawn((marker, Transform::default()));
}
```

`T: Component`는 "`T`는 Bevy의 `Component` trait를 구현한 어떤 타입이어도 된다"는 뜻입니다.

## Ownership, Move, Copy, Clone

Rust에서 대부분의 값은 소유자가 하나입니다.

`Copy`가 아닌 값을 대입하면 ownership이 move됩니다.

```rust
let a = String::from("player");
let b = a;
// a is no longer usable
```

작고 단순한 값은 보통 `Copy`를 구현하므로 대입할 때 복사됩니다.

```rust
let a = 10;
let b = a;
println!("{a} {b}");
```

많은 Bevy 수학 타입은 값으로 복사하기 저렴합니다. 벡터를 복사하는 코드를 자주 보게 됩니다.

```rust
let player_position = player_transform.translation.truncate();
```

실제 복제가 의도된 경우에 `clone()`을 씁니다.

```rust
let image: Handle<Image> = asset_server.load("player.png");
commands.spawn(Sprite::from_image(image.clone()));
commands.spawn(Sprite::from_image(image));
```

여기서 clone은 같은 에셋을 가리키는 `Handle<Image>`를 하나 더 만듭니다. 이미지 데이터는 에셋 시스템을 통해 공유됩니다.

약한 clone 습관:

```text
컴파일러가 moved라고 하니까 clone을 먼저 붙인다.
```

더 나은 ownership 질문:

```text
이 값은 누가 소유해야 하는가?
이 함수는 값을 소유하지 말고 borrow해야 하는가?
이 타입은 싸게 복사하도록 설계된 타입인가?
```

## 참조와 Borrowing

참조는 ownership을 가져오지 않고 값을 빌립니다.

```rust
T       소유한 값
&T      공유 읽기 전용 참조
&mut T  독점 가변 참조
```

예:

```rust
fn print_score(score: &Score) {
    println!("{}", score.0);
}

fn add_score(score: &mut Score, amount: u32) {
    score.0 += amount;
}
```

규칙:

```text
&T 참조는 동시에 여러 개 존재할 수 있다.
같은 값에 대한 &mut T 참조는 동시에 하나만 존재할 수 있다.
&mut T가 활성화되어 있는 동안 같은 값에 대한 다른 참조는 같이 존재할 수 없다.
```

Bevy 시스템 파라미터도 같은 생각을 씁니다.

```rust
Query<&Transform>      // Transform 읽기
Query<&mut Transform>  // Transform 수정
Res<Score>             // Score 리소스 읽기
ResMut<Score>          // Score 리소스 수정
```

그래서 Bevy는 데이터 접근이 호환되는 여러 시스템을 병렬로 실행할 수 있습니다.

## 역참조와 Lifetime 감각

`*`는 참조나 wrapper 뒤의 실제 값에 접근합니다.

```rust
let mut cooldown = 1.0;
let cooldown_ref = &mut cooldown;
*cooldown_ref -= 0.1;
```

`Local<T>`에서도 같은 생각이 나옵니다.

```rust
fn tick(time: Res<Time>, mut hit_cooldown: Local<f32>) {
    *hit_cooldown -= time.delta_secs();
}
```

초반 예제에서는 lifetime 표기를 직접 쓸 일이 거의 없습니다. 그래도 규칙은 중요합니다.

```text
참조는 자신이 가리키는 값보다 오래 살 수 없다.
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

`value`는 안쪽 블록이 끝나면 사라집니다. Rust는 사라진 데이터를 가리킬 수 있는 `r`을 거부합니다.

Bevy 시스템에서는 월드가 컴포넌트와 리소스를 소유합니다. 시스템은 한 번 실행되는 동안만 그것들을 빌립니다. 시스템이 끝난 뒤에도 기억해야 하는 것은 `Entity`, 복사한 값, 컴포넌트, 리소스로 저장합니다.

## 리스트, 배열, 반복

Rust 배열은 길이가 고정되어 있습니다.

```rust
let spawn_points = [
    Vec3::new(-100.0, 0.0, 0.0),
    Vec3::new(100.0, 0.0, 0.0),
];
```

`Vec<T>`는 늘어날 수 있는 리스트입니다.

```rust
let mut enemies: Vec<Entity> = Vec::new();
enemies.push(enemy_entity);
```

`for`는 iterator를 순회합니다.

```rust
for point in spawn_points {
    commands.spawn(Transform::from_translation(point));
}
```

Bevy query도 iterator처럼 순회합니다.

```rust
for mut transform in &mut players {
    transform.translation.x += 1.0;
}
```

이렇게 읽습니다.

```text
players query를 mutable하게 순회한다.
매칭된 각 엔티티의 Transform을 mutable하게 빌린다.
그 빌린 값을 loop 안에서 transform이라고 부른다.
```

쿼리가 여러 컴포넌트를 가져오면 tuple을 구조 분해합니다.

```rust
for (mut transform, velocity) in &mut bodies {
    transform.translation += velocity.0.extend(0.0);
}
```

loop 변수 모양은 query data 모양과 맞습니다.

```text
Query<(&mut Transform, &Velocity), ...>
for   (mut transform, velocity) in ...
```

## Option, Result, `match`, `let else`

Rust는 값 없음과 실패를 타입으로 명시합니다.

```text
Option<T>     Some(T) 또는 None
Result<T, E>  Ok(T) 또는 Err(E)
```

값이 없을 수 있으면 `Option`을 씁니다.

```rust
let target: Option<Entity> = None;
```

작업이 실패할 수 있으면 `Result`를 씁니다.

```rust
let parsed: Result<f32, _> = "280.0".parse();
```

각 경우는 `match`로 처리합니다.

```rust
match parsed {
    Ok(speed) => println!("speed: {speed}"),
    Err(error) => println!("invalid speed: {error}"),
}
```

Bevy query helper는 `Result`를 반환하는 경우가 많습니다.

```rust
let Ok(player) = players.single() else {
    return;
};
```

이것이 `let else`입니다.

```text
players.single()이 Ok(value)를 반환하면 value를 player에 바인딩한다.
그렇지 않으면 else 블록을 실행하고 시스템에서 return한다.
```

게임에서는 기대한 엔티티가 없을 때 그 프레임을 건너뛰는 일이 많으므로 자주 쓰입니다.

`?` 연산자는 오류를 전달합니다.

```rust
fn save_progress(progress: &Progress) -> Result<(), String> {
    let text = serde_json::to_string_pretty(progress).map_err(|err| err.to_string())?;
    std::fs::write(save_path(), text).map_err(|err| err.to_string())
}
```

`?`는 이렇게 읽습니다.

```text
Ok(value)이면 value를 꺼내 계속 진행한다.
Err(error)이면 현재 함수에서 그 error를 반환한다.
```

`?`는 현재 함수도 `Result`나 `Option`처럼 실패를 표현할 수 있는 타입을 반환할 때만 쓸 수 있습니다.

## `Default`와 `..default()`

많은 Rust/Bevy 타입은 기본값을 만들 수 있습니다.

```rust
let transform = Transform::default();
```

Bevy 예제에서는 중요한 필드만 지정하고 나머지는 기본값으로 채우는 코드가 자주 나옵니다.

```rust
Sprite {
    image: asset_server.load("player.png"),
    custom_size: Some(Vec2::splat(48.0)),
    ..default()
}
```

규칙:

```text
Type::default()  타입 이름으로 기본값 생성
default()        타입을 문맥으로 추론할 수 있을 때 기본값 생성
..default()      struct literal에서 빠진 필드를 기본값으로 채움
```

`..default()`는 struct literal의 마지막에 와야 합니다.

## Module, `use`, `pub`

Rust는 module로 코드를 나눕니다. 모듈 선언이 파일을 프로그램으로 가져옵니다.

부모에서 module을 선언합니다.

```rust
mod player;
```

보통 이 줄은 `src/player.rs`를 로드합니다.

다른 module이 필요로 하는 이름에만 `pub`을 붙입니다.

```rust
pub struct PlayerPlugin;
```

내부 system과 component는 다른 module이 필요로 하기 전까지 private으로 둡니다.

`use`는 이름을 현재 scope로 가져옵니다.

```rust
use bevy::prelude::*;
```

Bevy prelude는 튜토리얼 전체에서 쓰는 공통 이름을 내보냅니다. `App`, `Plugin`, `Commands`, `Component`, `Resource`, `Query`, `Res`, `Transform`, `Vec2`, `Vec3`, `Color`, `Sprite` 등이 여기에 포함됩니다.

파일 분리 규칙:

```text
src/main.rs에서 mod player; 선언
-> src/player.rs가 player module이 됨

player.rs 안의 pub struct PlayerPlugin;
-> main.rs에서 player::PlayerPlugin으로 접근 가능

player.rs 안의 struct Player;
-> Player는 player module 내부에만 보임
```

Bevy 프로젝트에서는 plugin 타입은 public인 경우가 많고, 내부 component와 system은 private으로 유지하는 경우가 많습니다.

## 시스템 시그니처 읽기

이제 Rust 조각을 합쳐 봅니다.

```rust
fn move_bodies(time: Res<Time>, mut bodies: Query<(&mut Transform, &Velocity), With<Body>>) {
    for (mut transform, velocity) in &mut bodies {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}
```

왼쪽에서 오른쪽으로 읽습니다.

```text
fn move_bodies(...)
    move_bodies라는 Rust 함수를 정의한다.

time: Res<Time>
    Time 리소스를 읽는다.

mut bodies: Query<...>
    query binding은 mutable이다. mutable 접근을 순회해야 하기 때문이다.

(&mut Transform, &Velocity)
    매칭된 각 엔티티에서 Transform은 수정하고 Velocity는 읽는다.

With<Body>
    Body가 붙은 엔티티만 매칭한다.

for (mut transform, velocity) in &mut bodies
    query를 순회하고 각 item을 구조 분해한다.

velocity.0
    Velocity tuple struct 안의 Vec2를 읽는다.

time.delta_secs()
    Time 리소스 값에 대해 메서드를 호출한다.
```

이것이 Rust와 Bevy가 만나는 핵심입니다. 시스템 시그니처는 Rust 코드이면서 동시에 Bevy의 데이터 접근 계약입니다.

## 컴파일러 오류 읽기

Rust 컴파일러 오류는 밀도가 높지만, 보통 실제 계약 위반을 가리킵니다. 이 순서로 읽습니다.

1. 첫 번째 오류부터 봅니다.
2. 내 코드의 파일과 줄을 찾습니다.
3. `expected` 타입과 `found` 타입을 읽습니다.
4. `moved`, `borrowed`, `mutable`, `immutable`, `does not live long enough` 같은 ownership 단어를 찾습니다.
5. 오류 하나를 고치고 다시 `cargo check`를 실행합니다.

이 튜토리얼에서 자주 나는 오류:

- 수정하는 바인딩에 `mut`이 없습니다.
- 쿼리는 `&mut Transform`을 요청했는데 loop 변수가 `mut transform`이 아닙니다.
- 컴포넌트 타입에 `#[derive(Component)]`가 없습니다.
- 리소스 타입에 `#[derive(Resource)]`가 없습니다.
- 시스템 함수는 만들었지만 `add_systems`에 등록하지 않았습니다.
- 두 쿼리가 같은 엔티티의 같은 컴포넌트를 동시에 가변 접근할 수 있습니다.
- `&T`로 받은 읽기 전용 참조를 수정하려고 합니다.
- `Option<T>`나 `Result<T, E>`를 안쪽의 `T`처럼 바로 쓰려고 합니다.
- 값을 move한 뒤 이전 바인딩을 다시 쓰려고 합니다.

## 체크포인트

`examples/03_player_input.rs`를 열고 코드에서 답해 보세요.

- 어떤 이름이 타입이고 어떤 이름이 값인가?
- 어떤 타입이 컴포넌트인가?
- 어떤 타입이 리소스인가?
- 어떤 지역 바인딩이 mutable인가?
- 어떤 시스템 파라미터가 키보드 입력을 읽는가?
- 어떤 시스템 파라미터가 컴포넌트 데이터를 수정하는가?
- `::`로 호출되는 것은 무엇이고, `.`으로 호출되는 것은 무엇인가?
- tuple struct의 `.0` 접근은 어디에 나오는가?
- 왜 loop가 `for mut transform in &mut players`라고 쓰였는가?

그다음 `examples/04_velocity_body.rs`를 열고 답해 보세요.

- 왜 `Velocity(Vec2)`는 그냥 `Vec2`가 아니라 tuple struct인가?
- 어떤 시스템이 `Velocity`를 쓰는가?
- 어떤 시스템이 `Velocity`를 읽고 `Transform`을 쓰는가?
- `Query<(&mut Transform, &Velocity), With<Body>>`는 loop 변수 모양을 어떻게 결정하는가?

이 질문에 타입 정의와 시그니처만 보고 답할 수 있으면 다음 장의 App과 시스템 모델이 훨씬 덜 낯설어집니다.

---

<div align="center">

[← 이전: 프로젝트 설정](00-project-setup.md) · [목차](index.md) · [다음: Bevy 앱 모델 →](02-bevy-app-model.md)

</div>
