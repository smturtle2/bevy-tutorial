# 5. 번들, 플러그인, 세트

[목차](index.md) | 이전: [입력과 이동](04-input-and-movement.md) | 다음: [에셋, 카메라, UI](06-assets-camera-ui.md)

실행합니다.

```sh
cargo run --example 05_plugins_sets
```

이 예제는 `04_velocity_body`처럼 동작하지만, 코드는 spawn bundle, plugin, 이름 있는 system set 중심으로 재구성되어 있습니다. 동작은 일부러 익숙하게 유지해서 구조에 집중할 수 있게 합니다.

## Bundle을 쓰는 이유

작은 예제에서는 tuple spawning도 괜찮습니다.

```rust
commands.spawn((Player, Velocity(Vec2::ZERO), Transform::default()));
```

예제가 커지면 반복되는 spawn tuple은 검토하기 어려워집니다. 이 프로젝트는 더 엄격한 규칙을 씁니다.

```text
Domain entities are spawned through Bundle + impl new().
```

spawn 규칙을 한 곳에 모으기 위해서입니다.

## BodyBundle

```rust
#[derive(Bundle)]
struct BodyBundle {
    body: Body,
    velocity: Velocity,
    transform: Transform,
}

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

`BodyBundle`은 런타임 superclass가 아닙니다. spawn 시점의 컴포넌트 패키지입니다.

spawn 후 엔티티는 단순히 다음 컴포넌트를 가집니다.

```text
Body
Velocity
Transform
```

따라서 쿼리는 절대 `BodyBundle`을 요청하지 않습니다. 그 안에서 나온 컴포넌트를 요청합니다.

```rust
Query<(&mut Transform, &Velocity), With<Body>>
```

## 중첩 Bundle

```rust
#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    body: BodyBundle,
    sprite: Sprite,
}
```

중첩 bundle은 엔티티의 컴포넌트로 평평하게 펼쳐집니다.

```text
Player
Body
Velocity
Transform
Sprite
```

이 flattening 덕분에 `move_bodies`가 플레이어를 움직일 수 있습니다. 플레이어 엔티티는 런타임에 bundle 객체를 담고 있지 않습니다. `Body`, `Velocity`, `Transform`을 담고 있습니다.

## Plugin

플러그인은 등록 경계입니다.

```rust
struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}
```

플러그인은 매 프레임 실행되지 않습니다. 앱에 필요한 것들을 등록합니다.

`examples/05_plugins_sets.rs`에는 세 플러그인이 있습니다.

```text
GamePlugin   inserts clear color, configures order, adds feature plugins
BodyPlugin   inserts BodySpeed and registers move_bodies
PlayerPlugin registers player spawn and player input
```

`main`은 작게 유지됩니다.

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
```

플러그인이 중요한 주된 이유가 이것입니다. 프로젝트가 커져도 feature registration을 `main` 밖에 둘 수 있습니다.

## SystemSet

시스템이 서로 다른 플러그인에 있으면 한 tuple에 `.chain()`을 붙이는 것만으로는 충분하지 않습니다. set을 씁니다.

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
}
```

순서를 설정합니다.

```rust
app.configure_sets(Update, (GameSet::Input, GameSet::Movement).chain());
```

시스템을 올바른 set에 넣습니다.

```rust
handle_player_input.in_set(GameSet::Input)
move_bodies.in_set(GameSet::Movement)
```

정신 모델:

```text
Update
  Input
    handle_player_input
  Movement
    move_bodies
```

플러그인을 가로질러 순서가 중요할 때 `SystemSet`을 쓰세요.

## 연습

로컬 실험에서 다음을 시도하세요.

1. `move_bodies`를 `GameSet::Input`으로 옮기고 어떤 ordering contract를 깨뜨렸는지 생각해 보세요.
2. body 위치를 출력하는 두 번째 시스템을 `GameSet::Movement`에 추가하세요.
3. `GamePlugin`에서 `.add_plugins(PlayerPlugin)`을 제거하고 앱은 열리지만 플레이어가 spawn되지 않는지 확인하세요.

## 흔한 실수

- spawn 후 bundle 타입을 query함. 펼쳐진 컴포넌트를 query하세요.
- feature boundary가 분명해진 뒤에도 모든 시스템을 한 플러그인에 넣음.
- plugin add order만으로 프레임 순서가 충분하다고 가정함. update order가 중요하면 set을 쓰세요.
- 필요한 module boundary가 없는데 bundle 필드를 `pub`으로 만듦.
