# 14. Handmade Map Geometry

<div align="center">

[Index](index.md) · [← Previous: Animation state](13-animation-state.md) · [Next: Game states →](15-game-states.md)

</div>

---

## Outcome

At the end of this chapter, the map is built from explicit geometry: visual floor tiles and solid wall rectangles with collision bodies.

![A handmade map uses visual floor tiles and solid wall bodies.](../../assets/screenshots/ch14-handmade-map.png)

## Run

```sh
cargo run --example 14_handmade_map_geometry
```

Move with WASD/arrows and push against the walls.

## Build Step 1: Separate Visual Floor From Solid Walls

The floor is visual only:

```rust
fn spawn_floor(commands: &mut Commands) {
    let tile_size = Vec2::splat(80.0);

    for x in -5..=5 {
        for y in -3..=3 {
            commands.spawn((
                Sprite::from_color(color, tile_size - Vec2::splat(2.0)),
                Transform::from_xyz(x as f32 * tile_size.x, y as f32 * tile_size.y, 0.0),
            ));
        }
    }
}
```

Floor tiles have `Sprite` and `Transform`, but no `Body` and no `Wall`.

Walls are gameplay geometry:

```rust
#[derive(Component)]
struct Wall;

#[derive(Bundle)]
struct WallBundle {
    wall: Wall,
    body: BodyBundle,
    sprite: Sprite,
}
```

They are visible and collidable.

## Build Step 2: Use Static Body Data

This chapter's `BodyBundle` stores `Body` and `Transform`:

```rust
#[derive(Bundle)]
struct BodyBundle {
    body: Body,
    transform: Transform,
}
```

The player adds `Velocity` separately because only moving things need velocity:

```rust
#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    body: BodyBundle,
    velocity: Velocity,
    sprite: Sprite,
}
```

That is a useful design distinction:

```text
Body       can collide
Velocity   can move this frame
Wall       solid map obstacle
Player     player-controlled body
```

## Build Step 3: Spawn Walls From Data

The wall list is explicit geometry:

```rust
for (position, size) in [
    (Vec3::new(0.0, 300.0, 2.0), Vec2::new(900.0, 40.0)),
    (Vec3::new(0.0, -300.0, 2.0), Vec2::new(900.0, 40.0)),
    (Vec3::new(-460.0, 0.0, 2.0), Vec2::new(40.0, 640.0)),
    (Vec3::new(460.0, 0.0, 2.0), Vec2::new(40.0, 640.0)),
    (Vec3::new(-130.0, 80.0, 2.0), Vec2::new(260.0, 36.0)),
] {
    commands.spawn(WallBundle::new(position, size));
}
```

This chapter expresses a small map directly as Rust geometry data.

## Build Step 4: Move First, Resolve Collision After

The system order is:

```rust
.configure_sets(
    Update,
    (GameSet::Input, GameSet::Movement, GameSet::Collision).chain(),
)
```

Movement first:

```rust
fn move_player(time: Res<Time>, mut players: Query<(&mut Transform, &Velocity), With<Player>>) {
    for (mut transform, velocity) in &mut players {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
    }
}
```

Collision then corrects illegal positions.

## Build Step 5: Resolve Wall Overlap

The resolver collects wall rectangles:

```rust
let mut walls = Vec::new();

for (transform, body, _, wall) in &mut bodies {
    if wall.is_some() {
        walls.push((transform.translation.truncate(), body.half_size));
    }
}
```

Then it pushes the player out along the shallowest overlap axis:

```rust
let delta = player_position - *wall_position;
let overlap = player_body.half_size + *wall_half_size - delta.abs();

if overlap.x <= 0.0 || overlap.y <= 0.0 {
    continue;
}

if overlap.x < overlap.y {
    player_transform.translation.x += overlap.x * delta.x.signum();
} else {
    player_transform.translation.y += overlap.y * delta.y.signum();
}
```

This is simple AABB collision response. It is enough for rectangular RPG walls.

## Rust Lens

The collision query uses optional components:

```rust
Query<(&mut Transform, &Body, Option<&Player>, Option<&Wall>)>
```

`Option<&Wall>` lets one query inspect both players and walls. `wall.is_some()` means this entity has the `Wall` component.

The map loops cast integers to floats:

```rust
x as f32 * tile_size.x
```

Loop variables from `-5..=5` are integers; Bevy positions are `f32`.

## Bevy Lens

Map rendering and map collision are related but not identical:

```text
floor tile    Sprite + Transform
wall          Wall + Body + Sprite + Transform
player        Player + Body + Velocity + Sprite + Transform
```

Do not make every visual tile collidable unless the gameplay needs it. The ECS data should say what systems need to process.

## Check

Run:

```sh
cargo run --example 14_handmade_map_geometry
```

Expected result:

- The floor is visible.
- The player can move across floor tiles.
- The player cannot pass through wall rectangles.
- Collision resolves after movement without stopping input.

## Change

Add a new wall entry:

```rust
(Vec3::new(0.0, 0.0, 2.0), Vec2::new(120.0, 36.0)),
```

Expected result: a new horizontal obstacle appears in the center and blocks the player.

---

<div align="center">

[← Previous: Animation state](13-animation-state.md) · [Index](index.md) · [Next: Game states →](15-game-states.md)

</div>
