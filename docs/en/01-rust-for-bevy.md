# 1. Rust for Bevy


<div align="center">

[Index](index.md) · [← Previous: Project setup](00-project-setup.md) · [Next: The Bevy app model →](02-bevy-app-model.md)

</div>

---

Rust is the language layer of every Bevy system you will write. Bevy uses Rust's model directly: components are Rust types, resources are Rust types, systems are Rust functions, and queries describe Rust borrow permissions.

This chapter builds the Rust foundation needed to read and change the tutorial examples with confidence.

## Chapter Contract

After this chapter, you should be able to:

- Read a Rust function signature and identify parameter types, return types, and mutable bindings.
- Explain what `let`, `mut`, `const`, `fn`, `struct`, `enum`, `impl`, `use`, and `pub` do.
- Distinguish owned values, shared references `&T`, and mutable references `&mut T`.
- Explain the difference between `struct Player;`, `struct Velocity(Vec2);`, and `struct Body { half_size: Vec2 }`.
- Read `App::new()`, `Transform::from_translation(...)`, `direction.normalize_or_zero()`, and `velocity.0`.
- Understand why Bevy code contains `derive`, generic types such as `Query<...>`, and wrappers such as `Res<T>`.
- Handle `Option`, `Result`, `match`, `let else`, and `?` when a value may be absent or an operation may fail.
- Read a Bevy system signature as a data-access contract.

## The Rust Model

Rust code is built from a few ideas:

```text
values       data at runtime
types        names for the shape and rules of data
bindings     local names created with let
functions    reusable blocks of behavior
ownership    one clear owner for most values
borrowing    temporary access through & or &mut
traits       behavior contracts implemented by types
modules      file and namespace organization
```

Bevy maps those ideas directly onto game code:

```text
component    Rust type attached to an entity
resource     Rust type stored once in the world
system       Rust function run by Bevy
plugin       Rust type that registers systems and resources
query        typed request for component access
```

When Bevy code looks dense, start by reading the Rust types. The type signature usually tells you what the code is allowed to touch.

## A Small Bevy File As Rust

`examples/01_empty_app.rs` is already real Rust:

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

Read it as Rust first:

```text
use bevy::prelude::*;
    Bring common Bevy names into scope.

fn main() { ... }
    Program entry point.

App::new()
    Call an associated function on the App type to create an App value.

.insert_resource(...)
.add_plugins(...)
.add_systems(...)
.run()
    Call methods on the App value, one after another.

fn setup_camera(mut commands: Commands)
    Define a function. Bevy can run it as a system.

commands.spawn(Camera2d);
    Use the Commands value to request a new entity.
```

The chain in `main` is standard Rust method-call syntax. Each method returns a value that allows the next method to be called.

## Bindings, Values, And Types

Rust creates local names with `let`:

```rust
let score = 0;
let speed: f32 = 280.0;
let name = "player";
```

Rules:

```text
let name = value;        create a binding
let name: Type = value;  create a binding with an explicit type
```

Rust often infers the type from the right side:

```rust
let mut direction = Vec2::ZERO;
```

`direction` is a `Vec2` because `Vec2::ZERO` is a `Vec2`.

Function parameters are different. Their types are normally written explicitly:

```rust
fn add_score(current: u32, amount: u32) -> u32 {
    current + amount
}
```

Rules:

```text
current: u32     parameter named current with type u32
amount: u32      parameter named amount with type u32
-> u32           function returns a u32
```

Bevy system parameters should be explicit because the type is the system's contract:

```rust
fn player_input(keyboard: Res<ButtonInput<KeyCode>>) {}
```

That signature says: "this system reads the keyboard input resource."

## Mutability

Rust bindings are immutable by default:

```rust
let direction = Vec2::ZERO;
direction.x += 1.0; // compile error
```

Use `mut` when the binding must change:

```rust
let mut direction = Vec2::ZERO;
direction.x += 1.0;
```

The input example uses this because it builds a direction from key presses:

```rust
let mut direction = Vec2::ZERO;

if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
    direction.x -= 1.0;
}
```

`mut` is attached to a binding. The type stays `Vec2`.

These two lines mean different things:

```rust
let mut value = 10;
let reference = &mut value;
```

The first line says the local binding `value` can change. The second line creates an exclusive mutable borrow of `value`.

You can also make a reference binding itself mutable:

```rust
let a = 10;
let b = 20;
let mut r = &a;
r = &b;
```

`r` can be reassigned to point at `b`, but its type is still `&i32`, which is read-only access to an `i32`.

If you want a mutable integer copied from a read-only reference:

```rust
let a = 10;
let r = &a;
let mut copied: i32 = *r;
copied += 1;
```

`i32` is `Copy`, so `*r` copies the value. `copied` then changes independently from `a`.

## Numbers And Constants

The tutorial examples use these numeric types often:

```text
i32, u32    integers
f32         32-bit floating-point number
usize       size or index type
```

Bevy 2D math uses `f32` heavily because positions, speeds, time deltas, and sizes are usually floating-point values:

```rust
let seconds: f32 = time.delta_secs();
let x = 120.0;
let lives: u32 = 3;
```

`120.0` is a floating-point literal. `120` is an integer literal.

Use `const` for fixed values that should be visible while reading the code:

```rust
const PLAYER_SPEED: f32 = 280.0;
const PLAYER_SIZE: Vec2 = Vec2::splat(80.0);
```

Rule:

```text
const NAME: Type = value;
```

A `const` is a compile-time fixed value. Runtime settings belong in resources.

## Expressions, Statements, And Semicolons

Rust has expressions that produce values and statements that perform actions.

The final expression in a block can be the return value:

```rust
fn add_score(current: u32, amount: u32) -> u32 {
    current + amount
}
```

There is no semicolon after `current + amount`, so it is returned.

With a semicolon, the expression becomes a statement:

```rust
fn add_score(current: u32, amount: u32) -> u32 {
    current + amount;
    // compile error: expected u32, found ()
}
```

`()` is Rust's unit type. It means "no meaningful value."

Most Bevy systems return unit:

```rust
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
```

The return type is omitted because it is `()`.

`if` can also produce a value:

```rust
let animation = if direction.length_squared() > 0.0 {
    PlayerAnimState::Run
} else {
    PlayerAnimState::Idle
};
```

Both branches must produce compatible types.

## `::`, `.`, Associated Functions, And Methods

Rust uses `::` for names that live under a type or module:

```rust
App::new()
Transform::from_translation(Vec3::ZERO)
Vec2::new(1.0, 0.0)
Vec2::ZERO
```

Rules:

```text
Type::function(...)  associated function on a type
Type::CONSTANT       associated constant on a type
module::name         name inside a module
```

Rust uses `.` for methods or fields on an existing value:

```rust
direction.normalize_or_zero()
transform.translation
velocity.0
```

Rules:

```text
value.method(...)    call a method on a value
value.field          access a named field
value.0              access tuple field number 0
```

`App::new()` creates an `App` because there is no `App` value yet. `.add_plugins(...)` is called after an `App` value exists.

## Structs

`struct` creates a new type. Bevy code uses structs constantly because components and resources are Rust types.

### Unit Structs

```rust
#[derive(Component)]
struct Player;
```

This struct has no fields. It is useful as a marker component: the presence of `Player` marks the entity as the player.

Typical markers:

```rust
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;
```

### Tuple Structs

```rust
#[derive(Resource)]
struct PlayerSpeed(f32);

#[derive(Component)]
struct Velocity(Vec2);
```

A tuple struct has unnamed fields. Access uses numeric field syntax:

```rust
let speed = player_speed.0;
velocity.0 = direction.normalize_or_zero() * speed;
```

Tuple structs are useful when one value needs domain meaning. `Vec2` is a vector. `Velocity(Vec2)` means "this entity's movement velocity."

### Named-Field Structs

```rust
#[derive(Component)]
struct Body {
    half_size: Vec2,
}
```

Named fields are better when field names make the code clearer:

```rust
let body = Body {
    half_size: Vec2::splat(16.0),
};

let allowed = player_body.half_size + enemy_body.half_size;
```

Choice rule:

```text
marker only                   -> unit struct
one value with domain meaning -> tuple struct
several named facts           -> named-field struct
```

## Tuples And Spawn Calls

Rust tuples group several values:

```rust
let pair = (10, 20);
let x = pair.0;
let y = pair.1;
```

Bevy uses tuples when spawning several components at once:

```rust
commands.spawn((
    Player,
    Sprite::from_color(Color::srgb(0.25, 0.70, 1.0), Vec2::splat(80.0)),
    Transform::from_translation(Vec3::ZERO),
));
```

There are two layers of parentheses:

```text
spawn(...)    function-call parentheses
(A, B, C)     tuple containing several components
```

Queries use tuples too:

```rust
Query<(&mut Transform, &Velocity), With<Body>>
```

`(&mut Transform, &Velocity)` means "fetch both components from the same entity."

## Derive And Traits

A trait is a behavior contract. `derive` asks Rust to generate a trait implementation:

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

Read those derives as role declarations:

```text
Component  this type can be attached to an entity
Resource   this type can be stored once in the world
Bundle     this type can expand into several components during spawn
```

Some derives are ordinary Rust traits:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
}
```

Bevy needs those traits to store, compare, and debug system-set labels.

You can also implement a trait manually. A Bevy plugin implements `Plugin`:

```rust
struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
```

Read it as:

```text
GamePlugin satisfies Bevy's Plugin contract by providing build.
```

## Impl Blocks And `Self`

`impl` attaches functions to a type:

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

`Self` means the type currently being implemented. Here it means `Score`.

Calls:

```rust
let mut score = Score::new();
score.add(10);
```

Rules:

```text
fn new() -> Self       no self parameter, call with Score::new()
fn value(&self)        reads an existing value, call with score.value()
fn add(&mut self, ...) mutates an existing value, call with score.add(...)
```

Bevy examples use constructors to keep spawn rules in one place:

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

Then spawning reads as a single intent:

```rust
commands.spawn(PlayerBundle::new(Vec3::ZERO));
```

## Enums And Match

An `enum` is a type whose value is one of several variants:

```rust
enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}
```

Variants are namespaced under the enum:

```rust
GameState::Menu
GameState::Playing
```

Use `match` to handle variants:

```rust
match state {
    GameState::Menu => show_menu(),
    GameState::Playing => update_game(),
    GameState::Paused => show_pause(),
    GameState::GameOver => show_game_over(),
}
```

The compiler checks that every variant is handled. If you add `Loading` later, Rust points at the `match` expressions that need an update.

Bevy also uses enums for typed system labels:

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
}
```

`GameSet::Input` is a typed label.

## Generics

Generics let a type or function mention another type as a parameter:

```rust
Vec<T>
Option<T>
Result<T, E>
Handle<Image>
Res<ButtonInput<KeyCode>>
Query<&mut Transform, With<Player>>
```

Read from the outside in:

```text
Handle<Image>
    a handle to an Image asset

Res<ButtonInput<KeyCode>>
    read access to the ButtonInput resource that tracks KeyCode input

Query<&mut Transform, With<Player>>
    for entities with Player, fetch mutable Transform access
```

`Query` has this shape:

```rust
Query<Data, Filter>
```

Examples:

```rust
Query<&Transform>
Query<&mut Transform, With<Player>>
Query<(&mut Transform, &Velocity), With<Body>>
```

The first type parameter says what component data to fetch. The second type parameter, when present, filters which entities match.

Functions can be generic too:

```rust
fn spawn_marker<T: Component>(commands: &mut Commands, marker: T) {
    commands.spawn((marker, Transform::default()));
}
```

`T: Component` means "`T` can be any type that implements Bevy's `Component` trait."

## Ownership, Move, Copy, And Clone

Rust gives most values one owner.

Assigning a non-`Copy` value moves ownership:

```rust
let a = String::from("player");
let b = a;
// a is no longer usable
```

Small simple values often implement `Copy`, so assignment copies them:

```rust
let a = 10;
let b = a;
println!("{a} {b}");
```

Many Bevy math types are cheap value types. You will often see code copy vectors:

```rust
let player_position = player_transform.translation.truncate();
```

Use `clone()` when a real duplicate is intended:

```rust
let image: Handle<Image> = asset_server.load("player.png");
commands.spawn(Sprite::from_image(image.clone()));
commands.spawn(Sprite::from_image(image));
```

This clone creates another `Handle<Image>` that points to the same asset. The image data stays shared through the asset system.

Weak clone habit:

```text
The compiler says "moved", so add clone everywhere.
```

Better ownership question:

```text
Who should own this value?
Should this function borrow it instead?
Is this type cheap and intended to be copied?
```

## References And Borrowing

References borrow a value without taking ownership:

```rust
T       owned value
&T      shared read-only reference
&mut T  exclusive mutable reference
```

Examples:

```rust
fn print_score(score: &Score) {
    println!("{}", score.0);
}

fn add_score(score: &mut Score, amount: u32) {
    score.0 += amount;
}
```

Rules:

```text
Many &T references can exist at the same time.
Only one &mut T reference to the same value can exist at a time.
&mut T excludes all other references to that same value while it is active.
```

Bevy system parameters use the same idea:

```rust
Query<&Transform>      // read Transform
Query<&mut Transform>  // mutate Transform
Res<Score>             // read Score resource
ResMut<Score>          // mutate Score resource
```

This is why Bevy can schedule many systems in parallel when their data access is compatible.

## Dereference And Lifetime Intuition

`*` accesses the value behind a reference or wrapper:

```rust
let mut cooldown = 1.0;
let cooldown_ref = &mut cooldown;
*cooldown_ref -= 0.1;
```

The same idea appears with `Local<T>`:

```rust
fn tick(time: Res<Time>, mut hit_cooldown: Local<f32>) {
    *hit_cooldown -= time.delta_secs();
}
```

The early examples rarely require writing lifetime annotations. The rule still matters:

```text
A reference must live no longer than the value it points to.
```

Invalid code:

```rust
let r;
{
    let value = 10;
    r = &value;
}
println!("{r}");
```

`value` disappears when the inner block ends. Rust rejects `r` because it would point to gone data.

In Bevy systems, the world owns components and resources. A system borrows them only for one run. If you need to remember something after the system finishes, store an `Entity`, a copied value, a component, or a resource.

## Lists, Arrays, And Iteration

Rust arrays have fixed length:

```rust
let spawn_points = [
    Vec3::new(-100.0, 0.0, 0.0),
    Vec3::new(100.0, 0.0, 0.0),
];
```

`Vec<T>` is a growable list:

```rust
let mut enemies: Vec<Entity> = Vec::new();
enemies.push(enemy_entity);
```

`for` loops over an iterator:

```rust
for point in spawn_points {
    commands.spawn(Transform::from_translation(point));
}
```

Bevy queries also behave like iterators:

```rust
for mut transform in &mut players {
    transform.translation.x += 1.0;
}
```

Read it as:

```text
Iterate the players query mutably.
For each matched entity, borrow its Transform mutably.
Call that borrowed value transform inside the loop.
```

When a query fetches several components, destructure the tuple:

```rust
for (mut transform, velocity) in &mut bodies {
    transform.translation += velocity.0.extend(0.0);
}
```

The loop variable shape matches the query data shape:

```text
Query<(&mut Transform, &Velocity), ...>
for   (mut transform, velocity) in ...
```

## Option, Result, `match`, And `let else`

Rust uses explicit types for absence and failure:

```text
Option<T>     Some(T) or None
Result<T, E>  Ok(T) or Err(E)
```

Use `Option` when a value may be missing:

```rust
let target: Option<Entity> = None;
```

Use `Result` when an operation may fail:

```rust
let parsed: Result<f32, _> = "280.0".parse();
```

Handle the cases with `match`:

```rust
match parsed {
    Ok(speed) => println!("speed: {speed}"),
    Err(error) => println!("invalid speed: {error}"),
}
```

Bevy query helpers often return `Result`:

```rust
let Ok(player) = players.single() else {
    return;
};
```

This is `let else`:

```text
If players.single() returns Ok(value), bind value to player.
Otherwise, run the else block and return from the system.
```

It is common in games because a system can safely skip a frame when the expected entity is unavailable.

The `?` operator forwards errors:

```rust
fn save_progress(progress: &Progress) -> Result<(), String> {
    let text = serde_json::to_string_pretty(progress).map_err(|err| err.to_string())?;
    std::fs::write(save_path(), text).map_err(|err| err.to_string())
}
```

Read `?` as:

```text
If Ok(value), unwrap value and continue.
If Err(error), return that error from the current function.
```

You can use `?` only inside a function that returns a compatible failure type such as `Result` or `Option`.

## `Default` And `..default()`

Many Rust and Bevy types can create a default value:

```rust
let transform = Transform::default();
```

Bevy examples often specify the important fields and fill the rest with defaults:

```rust
Sprite {
    image: asset_server.load("player.png"),
    custom_size: Some(Vec2::splat(48.0)),
    ..default()
}
```

Rules:

```text
Type::default()  create a default value from the type name
default()        create a default value when the type can be inferred
..default()      fill missing fields in a struct literal
```

`..default()` must come last in the struct literal.

## Modules, `use`, And `pub`

Rust organizes code with modules. A module declaration brings a file into the program.

Declare a module from its parent:

```rust
mod player;
```

That usually loads `src/player.rs`.

Use `pub` only for names other modules need:

```rust
pub struct PlayerPlugin;
```

Keep internal systems and components private until another module needs them.

`use` brings names into the current scope:

```rust
use bevy::prelude::*;
```

The Bevy prelude exports common names used throughout the tutorial: `App`, `Plugin`, `Commands`, `Component`, `Resource`, `Query`, `Res`, `Transform`, `Vec2`, `Vec3`, `Color`, `Sprite`, and more.

File-splitting rule:

```text
src/main.rs declares mod player;
-> src/player.rs becomes the player module

player.rs contains pub struct PlayerPlugin;
-> main.rs can access player::PlayerPlugin

player.rs contains struct Player;
-> Player stays private to the player module
```

In Bevy projects, plugin types are often public, while internal components and systems stay private.

## Reading A System Signature

Now combine the Rust pieces:

```rust
fn move_bodies(time: Res<Time>, mut bodies: Query<(&mut Transform, &Velocity), With<Body>>) {
    for (mut transform, velocity) in &mut bodies {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}
```

Read it left to right:

```text
fn move_bodies(...)
    Define a Rust function named move_bodies.

time: Res<Time>
    Read the Time resource.

mut bodies: Query<...>
    The local query binding is mutable because iteration needs mutable access.

(&mut Transform, &Velocity)
    For each matched entity, mutate Transform and read Velocity.

With<Body>
    Only match entities that have Body.

for (mut transform, velocity) in &mut bodies
    Iterate the query and destructure each item.

velocity.0
    Read the Vec2 inside the Velocity tuple struct.

time.delta_secs()
    Call a method on the Time resource.
```

That is the core Rust-Bevy bridge: the system signature is both Rust code and Bevy's data-access contract.

## Reading Compiler Errors

Rust compiler errors are dense, but they usually point at the real contract violation. Use this order:

1. Start with the first error.
2. Find the file and line in your code.
3. Read "expected" and "found" types.
4. Look for ownership words: `moved`, `borrowed`, `mutable`, `immutable`, `does not live long enough`.
5. Fix one error and run `cargo check` again.

Common errors in this tutorial:

- You modify a binding that is not `mut`.
- A query requests `&mut Transform`, but the loop variable is not written as `mut transform`.
- A component type is used without `#[derive(Component)]`.
- A resource type is inserted without `#[derive(Resource)]`.
- A system function exists but is not registered with `add_systems`.
- Two queries may mutably access the same component on the same entity.
- You try to mutate through `&T` instead of `&mut T`.
- You use `Option<T>` or `Result<T, E>` as if it were the inner `T`.
- You move a value and then try to use the old binding again.

## Checkpoint

Open `examples/03_player_input.rs` and answer these from the code:

- Which names are types and which names are values?
- Which type is a component?
- Which type is a resource?
- Which local bindings are mutable?
- Which system parameter reads keyboard input?
- Which system parameter mutates component data?
- Which calls use `::`, and which calls use `.`?
- Where does tuple-struct `.0` access appear?
- Why does the loop say `for mut transform in &mut players`?

Then open `examples/04_velocity_body.rs` and answer:

- Why is `Velocity(Vec2)` a tuple struct instead of a plain `Vec2`?
- Which system writes `Velocity`?
- Which system reads `Velocity` and writes `Transform`?
- How does `Query<(&mut Transform, &Velocity), With<Body>>` shape the loop variable?

If you can answer those from the signatures and type definitions, the next chapter's App and system model will feel much less mysterious.

---

<div align="center">

[← Previous: Project setup](00-project-setup.md) · [Index](index.md) · [Next: The Bevy app model →](02-bevy-app-model.md)

</div>
