use std::fs;

use bevy::prelude::*;
use bevy_tutorial::tutorial_capture::add_tutorial_screenshot;
use serde::Deserialize;

const PLAYER_SIZE: Vec2 = Vec2::splat(40.0);
const GEM_SIZE: Vec2 = Vec2::splat(26.0);
const PLAYER_SPEED: f32 = 260.0;

#[derive(Component)]
struct SceneEntity;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Gem;

#[derive(Component)]
struct Npc {
    name: String,
}

#[derive(Component)]
struct Body {
    half_size: Vec2,
}

#[derive(Component)]
struct StatusText;

#[derive(Resource)]
struct CurrentScene {
    path: &'static str,
    message: String,
}

impl Default for CurrentScene {
    fn default() -> Self {
        Self {
            path: "scenes/arena_a.json",
            message: "Scene file: scenes/arena_a.json".to_string(),
        }
    }
}

#[derive(Deserialize)]
struct SceneData {
    name: String,
    player_start: [f32; 2],
    walls: Vec<RectData>,
    gems: Vec<PointData>,
    npcs: Vec<NpcData>,
}

#[derive(Deserialize)]
struct RectData {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(Deserialize)]
struct PointData {
    x: f32,
    y: f32,
}

#[derive(Deserialize)]
struct NpcData {
    name: String,
    x: f32,
    y: f32,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .init_resource::<CurrentScene>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (scene_hotkeys, move_player, update_status_text).chain(),
        );

    add_tutorial_screenshot(&mut app, "assets/screenshots/ch22-scene-loading.png", 20);
    app.run();
}

fn setup(mut commands: Commands, mut current: ResMut<CurrentScene>) {
    commands.spawn(Camera2d);
    commands.spawn((
        StatusText,
        Text::new(""),
        TextFont::from_font_size(22.0),
        TextColor(Color::srgb(0.92, 0.95, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(18),
            left: px(18),
            ..default()
        },
    ));

    let path = current.path;
    current.message = load_scene(&mut commands, path);
}

fn scene_hotkeys(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    entities: Query<Entity, With<SceneEntity>>,
    mut current: ResMut<CurrentScene>,
) {
    let next_path = if keyboard.just_pressed(KeyCode::Digit1) {
        Some("scenes/arena_a.json")
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        Some("scenes/arena_b.json")
    } else {
        None
    };

    let Some(path) = next_path else {
        return;
    };

    for entity in &entities {
        commands.entity(entity).despawn();
    }

    current.path = path;
    current.message = load_scene(&mut commands, path);
}

fn load_scene(commands: &mut Commands, asset_path: &str) -> String {
    let fs_path = format!("assets/{asset_path}");
    let text = match fs::read_to_string(&fs_path) {
        Ok(text) => text,
        Err(error) => return format!("Failed to read {asset_path}: {error}"),
    };

    let scene = match serde_json::from_str::<SceneData>(&text) {
        Ok(scene) => scene,
        Err(error) => return format!("Failed to parse {asset_path}: {error}"),
    };

    spawn_scene(commands, &scene);
    format!("Loaded {} from {}", scene.name, asset_path)
}

fn spawn_scene(commands: &mut Commands, scene: &SceneData) {
    commands.spawn((
        SceneEntity,
        Player,
        Body {
            half_size: PLAYER_SIZE / 2.0,
        },
        Sprite::from_color(Color::srgb(0.25, 0.64, 1.0), PLAYER_SIZE),
        Transform::from_xyz(scene.player_start[0], scene.player_start[1], 3.0),
    ));

    for wall in &scene.walls {
        let size = Vec2::new(wall.w, wall.h);
        commands.spawn((
            SceneEntity,
            Wall,
            Body {
                half_size: size / 2.0,
            },
            Sprite::from_color(Color::srgb(0.28, 0.33, 0.42), size),
            Transform::from_xyz(wall.x, wall.y, 2.0),
        ));
    }

    for gem in &scene.gems {
        commands.spawn((
            SceneEntity,
            Gem,
            Body {
                half_size: GEM_SIZE / 2.0,
            },
            Sprite::from_color(Color::srgb(0.18, 0.88, 0.76), GEM_SIZE),
            Transform::from_xyz(gem.x, gem.y, 3.0),
        ));
    }

    for npc in &scene.npcs {
        let size = Vec2::splat(38.0);
        commands.spawn((
            SceneEntity,
            Npc {
                name: npc.name.clone(),
            },
            Body {
                half_size: size / 2.0,
            },
            Sprite::from_color(Color::srgb(0.95, 0.68, 0.30), size),
            Transform::from_xyz(npc.x, npc.y, 3.0),
        ));
    }
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Option<Single<&mut Transform, With<Player>>>,
    walls: Query<(&Transform, &Body), (With<Wall>, Without<Player>)>,
) {
    let Some(player) = player.as_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }

    let previous = player.translation;
    player.translation +=
        (direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs()).extend(0.0);

    let player_body = Body {
        half_size: PLAYER_SIZE / 2.0,
    };

    for (wall_transform, wall_body) in &walls {
        if overlaps(&player, &player_body, wall_transform, wall_body) {
            player.translation = previous;
            break;
        }
    }
}

fn update_status_text(
    current: Res<CurrentScene>,
    gems: Query<(), With<Gem>>,
    npcs: Query<&Npc>,
    walls: Query<(), With<Wall>>,
    mut text: Single<&mut Text, With<StatusText>>,
) {
    let npc_names = npcs
        .iter()
        .map(|npc| npc.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    text.0 = format!(
        "1/2 load scene | WASD move\n{}\nwalls: {} | gems: {} | npcs: {}",
        current.message,
        walls.iter().count(),
        gems.iter().count(),
        if npc_names.is_empty() {
            "none".to_string()
        } else {
            npc_names
        }
    );
}

fn overlaps(
    a_transform: &Transform,
    a_body: &Body,
    b_transform: &Transform,
    b_body: &Body,
) -> bool {
    let distance = (a_transform.translation - b_transform.translation)
        .truncate()
        .abs();
    let allowed = a_body.half_size + b_body.half_size;

    distance.x < allowed.x && distance.y < allowed.y
}
