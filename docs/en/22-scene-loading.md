# 22. Scene Loading

<div align="center">

[Index](index.md) · [← Previous: Audio events](21-audio-events.md) · [Contribute →](https://github.com/smturtle2/bevy-tutorial)

</div>

---

## Outcome

At the end of this chapter, the game loads level data from JSON files. The scene file chooses the player start, walls, gems, and NPCs. The Bevy systems turn that data into entities.

![Scene loading swaps between two data-driven arenas.](../../assets/screenshots/ch22-scene-loading.png)

## Run

```sh
cargo run --example 22_scene_loading
```

Press 1 or 2 to load different scene files. Move with WASD or arrow keys.

## Build Step 1: Define What A Scene Means

This chapter uses a tutorial scene format, not Bevy's full reflected `DynamicScene` format. The scene is level data:

```rust
#[derive(Deserialize)]
struct SceneData {
    name: String,
    player_start: [f32; 2],
    walls: Vec<RectData>,
    gems: Vec<PointData>,
    npcs: Vec<NpcData>,
}
```

The data file looks like this:

```json
{
  "name": "Training Yard",
  "player_start": [-260.0, -120.0],
  "walls": [
    { "x": 0.0, "y": 260.0, "w": 760.0, "h": 34.0 }
  ],
  "gems": [
    { "x": -120.0, "y": 140.0 }
  ],
  "npcs": [
    { "name": "Mapper", "x": 190.0, "y": 120.0 }
  ]
}
```

The contract is simple: scene files describe where things start. Game systems still decide what those things do.

## Build Step 2: Mark Scene-Owned Entities

Every entity spawned from scene data receives a marker:

```rust
#[derive(Component)]
struct SceneEntity;
```

When loading a new scene, the old scene entities are removed:

```rust
for entity in &entities {
    commands.entity(entity).despawn();
}
```

This prevents old walls, gems, and NPCs from staying behind.

## Build Step 3: Read And Parse The Scene File

The loader reads from `assets/scenes/...`:

```rust
let fs_path = format!("assets/{asset_path}");
let text = match fs::read_to_string(&fs_path) {
    Ok(text) => text,
    Err(error) => return format!("Failed to read {asset_path}: {error}"),
};
let scene = match serde_json::from_str::<SceneData>(&text) {
    Ok(scene) => scene,
    Err(error) => return format!("Failed to parse {asset_path}: {error}"),
};
```

The example returns a status message instead of panicking. That message appears in the HUD, so loading failures are visible.

## Build Step 4: Spawn Entities From Data

The scene data is converted into normal ECS entities:

```rust
for wall in &scene.walls {
    let size = Vec2::new(wall.w, wall.h);
    commands.spawn((
        SceneEntity,
        Wall,
        Body { half_size: size / 2.0 },
        Sprite::from_color(Color::srgb(0.28, 0.33, 0.42), size),
        Transform::from_xyz(wall.x, wall.y, 2.0),
    ));
}
```

The same pattern spawns the player, gems, and NPCs. Loading a scene is not magic. It is data-driven spawning.

## Build Step 5: Keep Runtime Rules In Code

The scene file does not store collision functions, movement code, or UI systems. Those stay in Rust:

```rust
fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Option<Single<&mut Transform, With<Player>>>,
    walls: Query<(&Transform, &Body), (With<Wall>, Without<Player>)>,
)
```

The scene changes the data. The systems keep the behavior.

## Build Step 6: Switch Scenes With A Hotkey

The hotkey system chooses a path:

```rust
let next_path = if keyboard.just_pressed(KeyCode::Digit1) {
    Some("scenes/arena_a.json")
} else if keyboard.just_pressed(KeyCode::Digit2) {
    Some("scenes/arena_b.json")
} else {
    None
};
```

Then it clears old scene entities and loads the new file. This is the basic room transition contract.

## Rust Lens

Nested structs mirror nested data:

```rust
struct SceneData {
    walls: Vec<RectData>,
    gems: Vec<PointData>,
    npcs: Vec<NpcData>,
}
```

`Vec<T>` means the scene can contain any number of items. `serde_json::from_str::<SceneData>` asks serde to parse JSON into exactly that Rust type.

## Bevy Lens

Scene loading separates three kinds of data:

```text
persistent progress     saved player progress
scene data              walls, gems, NPC positions
runtime entities         spawned ECS objects
```

The marker component connects the scene data to cleanup. The gameplay systems do not care whether an entity came from code or a JSON file after it has the right components.

## Check

Run:

```sh
cargo run --example 22_scene_loading
```

Expected result:

- Scene 1 loads on startup.
- Pressing 2 loads a different layout.
- Previous walls, gems, and NPCs disappear before the new scene spawns.
- The player starts at the scene's configured start point.
- The HUD shows the loaded scene name and counts.

## Change

Open `assets/scenes/arena_a.json` and add a gem:

```json
{ "x": 40.0, "y": 210.0 }
```

Expected result: running the example again shows the extra gem without changing Rust code.

---

<div align="center">

[← Previous: Audio events](21-audio-events.md) · [Index](index.md) · [Contribute →](https://github.com/smturtle2/bevy-tutorial)

</div>
