# 2. The Bevy App Model

[Index](index.md) | Previous: [Rust for Bevy](01-rust-for-bevy.md) | Next: [ECS fundamentals](03-ecs-fundamentals.md)

Run the first example:

```sh
cargo run --example 01_empty_app
```

You should see a Bevy window with a dark background. There is no gameplay yet, but this example already contains the core application shape.

## Walkthrough: `01_empty_app`

The example starts with Bevy's prelude:

```rust
use bevy::prelude::*;
```

Then `main` builds and runs an app:

```rust
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.11)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .run();
}
```

Read the chain from top to bottom:

```text
App::new()                 create the app builder
insert_resource(...)       store one global value in the world
add_plugins(DefaultPlugins) add Bevy's standard engine plugins
add_systems(Startup, ...)  register one startup system
run()                      enter the engine loop
```

`DefaultPlugins` adds the normal engine pieces: windowing, rendering, input, assets, logging, and related defaults. Without it, many familiar Bevy features are not present.

## `App` Is Configuration, Not Gameplay

`App` is where you register the data and behavior Bevy should run. The movement logic, AI, collision, and UI do not live inside `App`; they live in systems that `App` schedules.

This is the important split:

```text
App setup = register plugins, resources, systems, and schedules
Systems   = do work by reading and writing ECS data
```

## Startup And Update

Bevy systems are plain Rust functions registered into schedules.

```rust
.add_systems(Startup, setup_camera)
.add_systems(Update, move_bodies)
```

The schedule controls when a system runs:

```text
Startup = run once when the app starts
Update  = run every frame
```

The function name does not decide timing. A function named `setup` runs every frame if you register it in `Update`.

## Commands And Deferred World Changes

The startup system in `01_empty_app` is:

```rust
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
```

`Commands` queues changes to the ECS world. Spawning an entity changes world structure, so it goes through `Commands`.

Common `Commands` uses:

- `commands.spawn(...)`: create an entity with components.
- `commands.entity(entity).despawn()`: remove an entity.
- `commands.entity(entity).insert(...)`: add components.
- `commands.entity(entity).remove::<T>()`: remove a component type.

Commands are deferred. Inside a system, `commands.spawn(...)` does not immediately make the new entity visible to every query currently running in that same system. Bevy applies queued commands at schedule boundaries and other defined sync points. This keeps system execution safe and parallelizable.

Rule of thumb:

```text
Commands = change which entities/components exist
Query    = read or mutate component values that already exist
```

## Walkthrough: `02_spawn_sprite`

Run:

```sh
cargo run --example 02_spawn_sprite
```

The setup system creates two entities:

```rust
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(Color::srgb(0.25, 0.70, 1.0), Vec2::splat(80.0)),
        Transform::from_translation(Vec3::ZERO),
    ));
}
```

The camera entity has a `Camera2d` component. The square entity has a `Sprite` and a `Transform`.

This square is not an object instance in a class hierarchy. It is an entity with components:

```text
Entity
  Sprite
  Transform
```

`Sprite` controls what is drawn. `Transform` controls where it is drawn.

## Plugins Are Registration Units

A plugin groups app registration:

```rust
struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}
```

`build` is called when the plugin is added. It is not a per-frame update function.

Later examples use plugins to divide responsibilities:

```text
GamePlugin   = top-level game setup and ordering
BodyPlugin   = movement data and movement systems
PlayerPlugin = player spawn and input systems
```

## Checkpoint

Modify `examples/02_spawn_sprite.rs` in your own experiment and answer these questions:

- What happens if you remove `commands.spawn(Camera2d)`?
- What happens if you change `Vec2::splat(80.0)` to `Vec2::splat(30.0)`?
- What happens if you change `Transform::from_translation(Vec3::ZERO)` to `Transform::from_translation(Vec3::new(200.0, 0.0, 0.0))`?

Expected lesson: rendering is just ECS data. You make visible things by spawning entities with the right components.

## Common Mistakes

- Registering a system in `Startup` and expecting it to run every frame.
- Mutating an existing component through `Commands` when a `Query<&mut T>` would be clearer.
- Forgetting `DefaultPlugins`, then wondering why no window or rendering appears.
- Spawning a sprite without a camera and seeing an empty window.
