# 1. Rust For Bevy


<div align="center">

[Index](index.md) · [← Previous: Project setup](00-project-setup.md) · [Next: The Bevy app model →](02-bevy-app-model.md)

</div>

---

This is not a complete Rust course. It is the Rust you need to read the examples in this repository without treating Bevy code as magic.

The key habit is to read types first. In Bevy, a system signature tells you what data the system reads, what it mutates, and whether the data is per-entity component data, global resource data, or local system data.

## Chapter Contract

After this chapter, you should be able to:

- Read a system signature such as `fn move_bodies(time: Res<Time>, mut bodies: Query<...>)` from left to right.
- Explain the difference between `struct Player;`, `struct Velocity(Vec2);`, and `struct Body { ... }`.
- Distinguish binding mutability from data access rights in `mut players: Query<&mut Transform, With<Player>>`.
- Tell what `::` and `.` mean in `App::new()`, `Transform::from_translation(...)`, and `direction.normalize_or_zero()`.
- Understand why `Option`, `Result`, `let else`, and `match` appear often in Bevy code.
- Respond to ownership errors by deciding whether a value should be owned or borrowed instead of reflexively adding `clone()`.

Out of scope:

- unsafe Rust
- advanced lifetime design
- writing macros
- async Rust
- crate publishing and workspace design

Those topics are easier to learn after the game structure is larger.

## How To Read Rust Code

When Rust code looks dense, read Bevy examples in this order:

```text
1. Read the function name.
2. Read parameter names and types.
3. Check for & or &mut.
4. Check whether it receives Query, Res, ResMut, or Commands.
5. Read the body for what values it creates and what values it changes.
```

For example:

```rust
fn move_bodies(time: Res<Time>, mut bodies: Query<(&mut Transform, &Velocity), With<Body>>) {
    for (mut transform, velocity) in &mut bodies {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}
```

Read it as:

```text
move_bodies is a Rust function that may run every frame.
It reads the Time resource.
It finds entities with Body.
For each entity, it mutates Transform and reads Velocity.
It turns Velocity.0 from Vec2 into Vec3, multiplies by delta time, and adds it to position.
```

Rust makes "which data is accessed with which permissions" visible in types. Bevy uses that property directly for ECS scheduling.

## Type Annotations And Function Signatures

Rust writes variable, parameter, and return types in this form:

```rust
let speed: f32 = 220.0;

fn add_score(current: u32, amount: u32) -> u32 {
    current + amount
}
```

Rules:

```text
name: Type        variable or parameter type
fn name(...)      function definition
-> Type           return type
final expression  return value when there is no semicolon
```

The last line of that function is `current + amount`, with no semicolon. In Rust, the final expression of a block can be the return value.

If you add a semicolon, that expression becomes a statement and is not returned:

```rust
fn add_score(current: u32, amount: u32) -> u32 {
    current + amount;
    // compile error: expected u32, found ()
}
```

`()` is the unit type, meaning "no meaningful return value." Bevy system functions usually return nothing, so `-> ()` is omitted:

```rust
fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Velocity, With<Player>>,
) {
    // return type is ()
}
```

## Type Inference

Rust infers many types:

```rust
let mut direction = Vec2::ZERO;
```

Here `direction` is a `Vec2` because the right side is `Vec2::ZERO`.

Function parameter types are usually written explicitly. Bevy system parameters especially should be explicit because the type itself is the system contract:

```rust
fn player_input(keyboard: Res<ButtonInput<KeyCode>>) {}
```

That line says: "this system reads the keyboard input resource."

## `const` And Number Types

Use `const` for repeated fixed values:

```rust
const PLAYER_SPEED: f32 = 260.0;
const PLAYER_SIZE: Vec2 = Vec2::splat(42.0);
```

Rule:

```text
const NAME: Type = value;
```

A `const` is not a runtime setting. It is a compile-time fixed value. The examples use it for numbers that should be obvious while reading the tutorial, such as player size, enemy size, and attack range.

Bevy 2D examples use `f32` heavily because screen coordinates, speed, delta time, and color values are usually floating-point values:

```rust
let seconds: f32 = time.delta_secs();
let x = 120.0;
```

`120.0` is a floating-point literal. `120` is an integer literal. Bevy math types such as `Vec2`, `Vec3`, and `Transform` mostly operate on `f32`.

## `::`, `.`, Associated Functions, And Methods

Rust uses `::` and `.` for different things:

```rust
App::new()
Transform::from_translation(Vec3::ZERO)
Vec2::new(1.0, 0.0)
direction.normalize_or_zero()
```

Read them as:

```text
Type::name(...)   associated function or constant on a type
value.name(...)   method call on a value
Type::CONSTANT    constant on a type
```

`App::new()` creates a value from the type because there is no `App` value yet.

`Transform::from_translation(...)` creates a new `Transform`.

`direction.normalize_or_zero()` calls a method on the existing `direction` value.

This is clearer inside an `impl` block:

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

Calls:

```rust
let mut score = Score::new();
score.add(10);
```

`new` has no `self` parameter, so it is called as `Score::new()`. `add` takes `&mut self`, so it is called as `score.add(...)`.

## Variables And Mutability

Rust variables are immutable by default:

```rust
let direction = Vec2::ZERO;
direction.x += 1.0; // compile error
```

Use `mut` when the binding itself must change:

```rust
let mut direction = Vec2::ZERO;
direction.x += 1.0;
```

You see this in `examples/03_player_input.rs`:

```rust
let mut direction = Vec2::ZERO;

if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
    direction.x -= 1.0;
}
```

`mut` belongs to the binding. It is not part of the `Vec2` type. The same rule appears in system parameters:

```rust
mut players: Query<&mut Transform, With<Player>>
```

This means the local variable `players` can be iterated mutably, and the query gives mutable access to each matched `Transform`.

These look similar, but they are different:

```rust
let mut value = 10;
let reference = &mut value;
```

The first `mut` means the `value` binding can change. The second `&mut value` means `value` is borrowed exclusively.

Another example:

```rust
let a = 10;
let mut r = &a;
let b = 20;
r = &b;
```

This compiles. The binding `r` is mutable, so it can point to `a` and later to `b`. But `r` is still an `&i32`; it does not allow you to modify `a` or `b`.

If you want a mutable `i32` copied from a read-only reference, write:

```rust
let a = 10;
let r = &a;
let mut copied: i32 = *r;
copied += 1;
```

`i32` is `Copy`, so `*r` copies the value. Changing `copied` does not change `a`.

## Shadowing

Rust lets you declare the same name again with `let`. This is called shadowing:

```rust
let speed = "220";
let speed: f32 = speed.parse().unwrap_or(220.0);
```

The first `speed` is a string. The second `speed` is an `f32`. This is not mutating the same variable; it creates a new binding that hides the previous one.

The examples do not overuse shadowing, but patterns such as `let Ok(player) = ... else { ... };` also create new bindings.

## Struct Shapes

Rust `struct` creates a type. The examples use three common shapes.

### Unit Structs

```rust
#[derive(Component)]
struct Player;
```

This type stores no fields. In Bevy it is useful as a marker component: the entity either has `Player` or it does not.

`Player`, `Enemy`, `Collectible`, and `HealthBarFill` in `examples/07_rpg_slice.rs` are all marker components.

### Tuple Structs

```rust
#[derive(Component)]
struct Velocity(Vec2);
```

A tuple struct has unnamed fields. Access uses numeric field syntax:

```rust
velocity.0 = direction.normalize_or_zero() * PLAYER_SPEED;
```

Tuple structs are useful when a primitive or engine type needs domain meaning. `Vec2` is just a vector; `Velocity(Vec2)` means "this entity's movement velocity."

## Reading Tuples And Parentheses

Rust uses parentheses for several things. Bevy `spawn` calls are a common place to get confused:

```rust
commands.spawn((
    Player,
    Velocity(Vec2::ZERO),
    Transform::from_translation(Vec3::ZERO),
));
```

That code has two layers of parentheses:

```text
spawn(...)    function-call parentheses
(A, B, C)     tuple containing several components
```

You can spawn a single component with `commands.spawn(Player)`, but entities usually have several components, so the components are grouped into a tuple.

Queries use tuples too:

```rust
Query<(&mut Transform, &Velocity), With<Body>>
```

Here `(&mut Transform, &Velocity)` is a tuple type meaning "fetch these two components from the same entity."

### Named-Field Structs

```rust
#[derive(Component)]
struct Body {
    half_size: Vec2,
}
```

Named fields are best when the data has more than one meaning or when the field name improves the code. In the final example, `Body` stores `half_size` so collision code can compare rectangle extents:

```rust
let allowed = a_body.half_size + b_body.half_size;
```

Construct a named-field struct by naming each field:

```rust
let body = Body {
    half_size: Vec2::splat(16.0),
};
```

When a struct has several fields, this is easier to audit than a tuple struct. When there is one field and the type name carries the meaning, `Velocity(Vec2)` is simpler.

Struct choice:

```text
marker only                  -> unit struct
one value with domain meaning -> tuple struct
several fields               -> named-field struct
```

## `Default` And `..default()`

A Rust type can create a default value if it implements the `Default` trait:

```rust
let transform = Transform::default();
```

Many Bevy types implement `Default`. That lets you specify only the fields you care about and fill the rest with defaults:

```rust
Sprite {
    image: asset_server.load("player.png"),
    custom_size: Some(Vec2::splat(48.0)),
    ..default()
}
```

`..default()` means "use default values for the fields not listed above." This is very common in Bevy examples.

Rules:

```text
Type::default()  create a default value from the type name
default()        create a default value when the type can be inferred
..default()      fill missing fields in a struct literal
```

`..default()` must come last in the struct literal.

## Enums

An `enum` is a type whose value is one of several variants. `examples/05_plugins_sets.rs` uses one for system ordering:

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
}
```

`GameSet::Input` and `GameSet::Movement` are not strings. They are typed labels Bevy can use when scheduling systems.

The final example expands that enum:

```rust
enum GameSet {
    Input,
    Ai,
    Movement,
    Collision,
    Display,
}
```

Enums are also a good fit for game states:

```rust
enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}
```

Use `match` to run different code for each state:

```rust
match state {
    GameState::Menu => show_menu(),
    GameState::Playing => update_game(),
    GameState::Paused => show_pause(),
    GameState::GameOver => show_game_over(),
}
```

The useful part is that the compiler catches missing variants. If you later add `Loading` to `GameState`, a `match` that does not handle `Loading` becomes a compile error.

## Derive

`derive` asks Rust to generate an implementation of a trait.

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

The type is still your type. The derive makes it usable in a specific Bevy role:

- `Component`: can be attached to an entity.
- `Resource`: can be stored once in the world.
- `Bundle`: can be expanded into several components when spawning.
- `SystemSet`: can label systems for ordering.

Some derives come from Rust's standard traits or common scheduling requirements. `Debug`, `Clone`, `PartialEq`, `Eq`, and `Hash` on `GameSet` allow Bevy to compare and store set labels.

## Impl Blocks

`impl` attaches functions to a type:

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

`Self` means the type being implemented. Here it means `BodyBundle`.

The examples use `new` constructors to keep spawn rules in one place:

```rust
commands.spawn(PlayerBundle::new());
```

That line is easier to audit than repeating every component at every spawn site.

Functions inside an `impl` commonly fall into three forms:

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

Read them as:

```text
fn new() -> Self    creates a value. Call as WaveSpawner::new()
fn wave(&self)      reads the value. Call as spawner.wave()
fn reset(&mut self) mutates the value. Call as spawner.reset()
```

`self` is the value the method was called on. `&self` borrows it read-only, and `&mut self` borrows it mutably.

## Traits And Trait Bounds

A trait is a behavior contract. Bevy's `Plugin` trait says a plugin must provide `build`:

```rust
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
```

Read it as:

```text
GamePlugin satisfies Bevy's Plugin contract.
```

You will often see generics with trait bounds in Bevy code. A helper function can accept any component type by writing a bound:

```rust
fn spawn_marker<T: Component>(commands: &mut Commands, marker: T) {
    commands.spawn((marker, Transform::default()));
}
```

`T: Component` means "`T` can be any type, as long as it is a Bevy component." That lets the function accept different marker components:

```rust
spawn_marker(&mut commands, Player);
spawn_marker(&mut commands, Enemy);
```

Without the bound, Bevy could not know that `marker: T` is valid component data.

The final example uses explicit `PlayerBundle`, `EnemyBundle`, and `CollectibleBundle` types. That is more repetitive, but it is easier to read while learning because each spawn shape has a concrete name.

## Reading Generic Types

Bevy types often contain `<...>`:

```rust
Res<ButtonInput<KeyCode>>
Query<(&mut Transform, &Velocity), With<Player>>
Handle<Image>
Assets<TextureAtlasLayout>
```

`<...>` contains type parameters. Read from the outside in:

```text
Res<ButtonInput<KeyCode>>
= read the ButtonInput resource that stores KeyCode input state

Query<(&mut Transform, &Velocity), With<Player>>
= for entities with Player, mutate Transform and read Velocity

Handle<Image>
= a handle that refers to an Image asset
```

For `Query`, the first type parameter is the data to fetch and the second is the filter:

```rust
Query<Data, Filter>
```

If there is no filter, it can be omitted:

```rust
Query<&Transform>
```

Generic syntax can look heavy, but in Bevy it usually says which typed data a system reads or writes.

## Ownership, Move, Copy, And Clone

Rust has one owner for most values. Assigning a non-`Copy` value moves it:

```rust
let a = String::from("player");
let b = a;
// a is no longer usable
```

Small simple values often implement `Copy`, so assignment copies the bits:

```rust
let a = 10;
let b = a;
println!("{a} {b}");
```

Many Bevy math types are cheap value types. You will often see code that copies positions or vectors:

```rust
let player_position = player.translation.truncate();
```

When a type is not `Copy` and you need a duplicate, use `clone()` only when the type supports `Clone` and a real copy is intended. Do not use `clone()` as a reflex to silence ownership errors; first ask who should own the value.

In Bevy, ownership most often appears around asset handles and component values:

```rust
let image: Handle<Image> = asset_server.load("player.png");
commands.spawn(Sprite::from_image(image.clone()));
commands.spawn(Sprite::from_image(image));
```

Here `clone()` does not copy the whole image file. It creates another `Handle<Image>` reference. Several entities should be able to refer to the same asset, so this clone is intentional.

By contrast, repeatedly cloning large game state only to satisfy the compiler is usually a design smell. Ask who should own that value.

## References: `&` And `&mut`

Rust distinguishes owning a value from borrowing it:

```rust
T       // owned value
&T      // shared read-only reference
&mut T  // exclusive mutable reference
```

Bevy system parameters make this explicit:

```rust
Query<&Transform>      // read Transform
Query<&mut Transform>  // mutate Transform
Res<Score>             // read a resource
ResMut<Score>          // mutate a resource
```

Only one mutable borrow of the same data may exist at a time. That Rust rule is why Bevy cares about query conflicts:

```rust
Query<&Transform, With<Player>>
Query<&mut Transform, With<Camera2d>>
```

Both queries touch `Transform`, and one is mutable. If Bevy cannot prove the matched entities are different, you must add filters such as `Without<Camera2d>` or split the work into separate systems.

## Dereference: `*`

`*` appears when you need to access the value inside a reference or wrapper:

```rust
let mut cooldown = 1.0;
let cooldown_ref = &mut cooldown;
*cooldown_ref -= 0.1;
```

`cooldown_ref` is the reference. `*cooldown_ref` is the `f32` it points to.

The same idea appears with Bevy's `Local<T>` and `ResMut<T>`:

```rust
fn tick(time: Res<Time>, mut hit_cooldown: Local<f32>) {
    *hit_cooldown -= time.delta_secs();
}
```

`hit_cooldown` is a `Local<f32>` wrapper. Dereferencing lets you mutate the inner `f32`.

Many field accesses are handled automatically by Rust's deref coercion:

```rust
score.0 += 1;
```

Even if `score` is `ResMut<Score>`, Rust can often insert the needed dereference. When you assign to or do arithmetic on the inner value itself, you may see `*` explicitly.

## Lifetime Intuition

The early Bevy examples rarely require writing lifetime annotations. The rule is still important:

```text
A reference cannot outlive the original value.
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

`value` disappears when the inner block ends. If `r` could still point to it, it would be a dangling reference, so Rust rejects the code.

In Bevy systems, the world owns components and resources, and system parameters borrow them only for one system run. Do not try to store `&Transform` outside the system. If you need to remember something longer, store an `Entity` ID, a copied value, a resource, or a component.

## Control Flow: `if`, `for`, `return`

Rust `if` conditions do not need parentheses:

```rust
if direction.length_squared() > 0.0 {
    velocity.0 = direction.normalize() * speed.0;
}
```

`if` is also an expression:

```rust
let animation = if velocity.0.length_squared() > 0.0 {
    PlayerAnimState::Run
} else {
    PlayerAnimState::Idle
};
```

`for` loops over an iterator. Bevy queries can be iterated too:

```rust
for mut transform in &mut players {
    transform.translation.x += 1.0;
}
```

Read that line as:

```text
Iterate the players query mutably.
Borrow each matched Transform mutably.
Use transform as the loop variable for that borrowed value.
```

Use `return` to leave a function early:

```rust
let Ok(player) = players.single() else {
    return;
};
```

Game loops often need to skip a frame when there is no valid target, so early returns are common.

## Patterns And Destructuring

Rust can break a value apart by matching its shape:

```rust
for (mut transform, velocity) in &mut bodies {
    transform.translation += velocity.0.extend(0.0);
}
```

The query item type is `(&mut Transform, &Velocity)`, so the loop variable uses the same shape.

```text
(mut transform, velocity)
= first value is a mutable Transform reference
= second value is a Velocity reference
```

`let else` is also pattern matching:

```rust
let Ok(player) = players.single() else {
    return;
};
```

If `players.single()` returns `Ok(...)`, the inner value is bound to `player`. If it returns `Err(...)`, the `else` block runs.

## Option, Result, And `let else`

Rust uses `Option<T>` when a value may be absent and `Result<T, E>` when an operation may fail.

Bevy's single-entity query helpers commonly return `Result`:

```rust
let Ok(player) = player.single() else {
    return;
};
```

This is `let else`. It means:

```text
If player.single() returns Ok(value), bind value to player.
Otherwise, leave the function early.
```

Use this when zero or multiple matching entities should make the system skip the frame. The final example uses `Single<...>` instead, which is stricter: it expects exactly one matching entity as part of the system contract.

`Option` appears in Bevy component fields too:

```rust
sprite.custom_size = Some(Vec2::new(160.0 * health_fraction, 14.0));
```

`Some(value)` means the size is explicitly set. `None` would mean no custom size.

You can also unpack it with `match`:

```rust
match sprite.custom_size {
    Some(size) => println!("custom size: {size:?}"),
    None => println!("default size"),
}
```

`Result` matters more in the save/load example:

```rust
fn save_progress(progress: &Progress) -> Result<(), String> {
    let text = serde_json::to_string_pretty(progress).map_err(|err| err.to_string())?;
    std::fs::write(save_path(), text).map_err(|err| err.to_string())
}
```

Read it as:

```text
Result<(), String>
= success returns no meaningful value, failure returns a String error

?
= if the value is Ok, unwrap it and continue;
  if it is Err, return that Err from the current function
```

You can use `?` only when the current function also returns a failure-capable type such as `Result` or `Option`.

## Modules, `pub`, And `use`

Rust does not compile every file automatically. A module must be declared from its parent:

```rust
mod player;
```

That loads `src/player.rs`.

Use `pub` for names another module needs:

```rust
pub struct PlayerPlugin;
```

Keep implementation details private by default. If only the plugin needs `spawn_player`, it does not need to be public.

`use` brings names into scope:

```rust
use bevy::prelude::*;
```

The Bevy prelude exports the common types used throughout this tutorial: `App`, `Plugin`, `Commands`, `Component`, `Resource`, `Query`, `Res`, `Transform`, `Vec2`, `Vec3`, `Color`, `Sprite`, and more.

Think of file splitting like this:

```text
src/main.rs declares mod player;
-> src/player.rs becomes the player module

pub struct PlayerPlugin; inside player.rs
-> main.rs or another module can access player::PlayerPlugin

struct Player; inside player.rs
-> only the player module can access it
```

In Bevy projects, it is common to make plugin types public and keep internal components and systems private unless another module really needs them. A smaller public API makes later file restructuring easier.

## What Rust And Bevy Do Not Do For You

Bevy checks many ECS borrow conflicts, but Rust rules do not disappear:

```text
Rust guarantees:
- no dangling references
- no several mutable references to the same value at the same time
- no using a value as the wrong type

Bevy adds:
- world data access based on system parameter types
- entity set selection through Query filters
- deferred structural changes through Commands
```

So the right model is not "Bevy handles ownership so I can ignore Rust." The better model is: "Bevy provides an ECS API on top of Rust ownership, so writing the correct system signature is part of the ownership design."

## Reading Compiler Errors

Rust errors are usually precise, but they are dense. Read them in this order:

1. Start at the first error, not the last.
2. Find the file and line number in your code.
3. Read the expected type and the found type.
4. Look for borrow words: "mutable", "immutable", "moved", "borrowed".
5. Fix one error, then run `cargo check` again.

Common beginner errors in these examples:

- Missing `mut` on a binding you modify: `let direction` should be `let mut direction`.
- Query asks for `&mut Transform`, but the loop variable is not mutable: use `for mut transform in &mut players`.
- A component type is used in a query but does not derive `Component`.
- A resource is inserted without deriving `Resource`.
- A system function is written correctly but never registered with `add_systems`.
- Two `Query<&mut T>` parameters can match the same entity and Bevy reports a conflict.
- Trying to mutate through a read-only `&T`.
- Trying to use `Option<T>` or `Result<T, E>` as if it were the inner value.

## Checkpoint

Before moving on, open `examples/03_player_input.rs` and `examples/04_velocity_body.rs` and identify:

- Which types are components?
- Which type is a resource?
- Which local variables are mutable?
- Which system parameter reads input?
- Which system parameter mutates component data?
- Which calls use `::`, and which calls use `.`?
- Where does tuple struct `.0` access appear?
- How does each `for` loop item shape match its query item type?

If you can answer those from the type signatures, you are ready for Bevy's app model.

---

<div align="center">

[← Previous: Project setup](00-project-setup.md) · [Index](index.md) · [Next: The Bevy app model →](02-bevy-app-model.md)

</div>
