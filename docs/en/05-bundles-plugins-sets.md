# 5. Bundles, Plugins, And Sets


<div align="center">

[Index](index.md) · [← Previous: Input and movement](04-input-and-movement.md) · [Next: Assets, camera, and UI →](06-assets-camera-ui.md)

</div>

---

Run:

```sh
cargo run --example 05_plugins_sets
```

This example behaves like `04_velocity_body`, but the code is reorganized around spawn bundles, plugins, and named system sets. The behavior is intentionally familiar so you can focus on structure.

## Why Bundles

Tuple spawning is fine for tiny examples:

```rust
commands.spawn((Player, Velocity(Vec2::ZERO), Transform::default()));
```

As examples grow, repeated spawn tuples become hard to audit. This project uses a stricter rule:

```text
Domain entities are spawned through Bundle + impl new().
```

That keeps spawn rules in one place.

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

`BodyBundle` is not a runtime superclass. It is a spawn-time component package.

After spawn, the entity simply has:

```text
Body
Velocity
Transform
```

That means queries never ask for `BodyBundle`. They ask for the components that came out of it:

```rust
Query<(&mut Transform, &Velocity), With<Body>>
```

## Nested Bundles

```rust
#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    body: BodyBundle,
    sprite: Sprite,
}
```

Nested bundles flatten into components on the entity:

```text
Player
Body
Velocity
Transform
Sprite
```

Flattening is why `move_bodies` can move the player. The player entity does not contain a bundle object at runtime; it contains `Body`, `Velocity`, and `Transform`.

## Plugins

Plugins are registration boundaries:

```rust
struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}
```

The plugin does not run every frame. It registers things with the app.

`examples/05_plugins_sets.rs` has three plugins:

```text
GamePlugin   inserts clear color, configures order, adds feature plugins
BodyPlugin   inserts BodySpeed and registers move_bodies
PlayerPlugin registers player spawn and player input
```

`main` stays small:

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
```

This is the main reason plugins matter: they keep feature registration out of `main` as the project grows.

## SystemSet

When systems live in different plugins, `.chain()` on one tuple is not enough. Use sets.

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
}
```

Configure order:

```rust
app.configure_sets(Update, (GameSet::Input, GameSet::Movement).chain());
```

Put systems in the correct set:

```rust
handle_player_input.in_set(GameSet::Input)
move_bodies.in_set(GameSet::Movement)
```

Mental model:

```text
Update
  Input
    handle_player_input
  Movement
    move_bodies
```

Use `SystemSet` when order matters across plugins.

## Exercise

Try these changes in a local experiment:

1. Move `move_bodies` into `GameSet::Input` and think about what ordering contract you just broke.
2. Add a second system to `GameSet::Movement` that prints body positions.
3. Remove `.add_plugins(PlayerPlugin)` from `GamePlugin` and confirm the app still opens but no player spawns.

## Common Mistakes

- Querying for a bundle type after spawn. Query the flattened components instead.
- Putting every system in one plugin even after feature boundaries are clear.
- Assuming plugin add order is enough for frame ordering. Use sets when update order matters.
- Making bundle fields `pub` without a module boundary that needs them.

---

<div align="center">

[← Previous: Input and movement](04-input-and-movement.md) · [Index](index.md) · [Next: Assets, camera, and UI →](06-assets-camera-ui.md)

</div>
