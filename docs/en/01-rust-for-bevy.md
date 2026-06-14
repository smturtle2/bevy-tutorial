# 1. Rust For Bevy

[Index](index.md) | Previous: [Project setup](00-project-setup.md) | Next: [The Bevy app model](02-bevy-app-model.md)

This is not a complete Rust course. It is the Rust you need to read the examples in this repository without treating Bevy code as magic.

The key habit is to read types first. In Bevy, a system signature tells you what data the system reads, what it mutates, and whether the data is per-entity component data, global resource data, or local system data.

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

## Checkpoint

Before moving on, open `examples/03_player_input.rs` and identify:

- Which types are components?
- Which type is a resource?
- Which local variables are mutable?
- Which system parameter reads input?
- Which system parameter mutates component data?

If you can answer those from the type signatures, you are ready for Bevy's app model.
