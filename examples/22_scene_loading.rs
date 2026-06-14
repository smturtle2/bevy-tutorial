use std::fs;

use bevy::prelude::*;
use bevy_tutorial::tutorial_capture::{add_tutorial_screenshot, tutorial_capture_enabled};
use serde::Deserialize;

const PLAYER_SIZE: Vec2 = Vec2::splat(40.0);
const ITEM_SIZE: Vec2 = Vec2::splat(26.0);
const NPC_SIZE: Vec2 = Vec2::splat(38.0);
const PLAYER_SPEED: f32 = 260.0;
const INTERACT_DISTANCE: f32 = 82.0;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Playing,
    Dialogue,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
    Collision,
    Ui,
}

#[derive(Component)]
struct GameplayEntity;

#[derive(Component)]
struct SceneEntity;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct InventoryItem {
    kind: ItemKind,
}

#[derive(Component)]
struct Npc {
    name: String,
    lines: Vec<String>,
}

#[derive(Component)]
struct Body {
    half_size: Vec2,
}

#[derive(Component)]
struct StatusText;

#[derive(Component)]
struct DialogueText;

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

#[derive(Resource, Default)]
struct Inventory {
    gems: u32,
    keys: u32,
    potions: u32,
}

#[derive(Resource, Default)]
struct RunStats {
    score: u32,
}

#[derive(Resource, Default)]
struct DialogueState {
    active_npc: Option<Entity>,
    line_index: usize,
}

#[derive(Component, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ItemKind {
    Gem,
    Key,
    Potion,
}

impl ItemKind {
    fn color(self) -> Color {
        match self {
            ItemKind::Gem => Color::srgb(0.18, 0.88, 0.76),
            ItemKind::Key => Color::srgb(1.0, 0.82, 0.20),
            ItemKind::Potion => Color::srgb(0.95, 0.24, 0.42),
        }
    }

    fn score_value(self) -> u32 {
        match self {
            ItemKind::Gem => 10,
            ItemKind::Key => 50,
            ItemKind::Potion => 0,
        }
    }
}

impl Inventory {
    fn add(&mut self, kind: ItemKind) {
        match kind {
            ItemKind::Gem => self.gems += 1,
            ItemKind::Key => self.keys += 1,
            ItemKind::Potion => self.potions += 1,
        }
    }
}

#[derive(Deserialize)]
struct SceneData {
    name: String,
    player_start: [f32; 2],
    walls: Vec<RectData>,
    items: Vec<ItemData>,
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
struct ItemData {
    kind: ItemKind,
    x: f32,
    y: f32,
}

#[derive(Deserialize)]
struct NpcData {
    name: String,
    x: f32,
    y: f32,
    lines: Vec<String>,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .init_resource::<CurrentScene>()
        .init_resource::<Inventory>()
        .init_resource::<RunStats>()
        .init_resource::<DialogueState>()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .configure_sets(
            Update,
            (
                GameSet::Input,
                GameSet::Movement,
                GameSet::Collision,
                GameSet::Ui,
            )
                .chain(),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (scene_hotkeys, dialogue_input)
                .chain()
                .in_set(GameSet::Input),
        )
        .add_systems(
            Update,
            move_player
                .in_set(GameSet::Movement)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            collect_items
                .in_set(GameSet::Collision)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (
                start_capture_dialogue,
                update_status_text,
                update_dialogue_text,
            )
                .chain()
                .in_set(GameSet::Ui),
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
            padding: UiRect::all(px(8)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.06, 0.08, 0.86)),
    ));

    commands.spawn((
        DialogueText,
        Text::new(""),
        TextFont::from_font_size(24.0),
        TextColor(Color::srgb(1.0, 0.92, 0.62)),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(28),
            left: px(32),
            right: px(32),
            padding: UiRect::all(px(14)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.06, 0.07, 0.10, 0.88)),
    ));

    let path = current.path;
    current.message = load_scene(&mut commands, path);
}

fn scene_hotkeys(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    entities: Query<Entity, With<SceneEntity>>,
    mut current: ResMut<CurrentScene>,
    mut inventory: ResMut<Inventory>,
    mut stats: ResMut<RunStats>,
    mut dialogue: ResMut<DialogueState>,
    mut next_state: ResMut<NextState<GameState>>,
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

    *inventory = Inventory::default();
    *stats = RunStats::default();
    *dialogue = DialogueState::default();
    next_state.set(GameState::Playing);
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
        GameplayEntity,
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
            GameplayEntity,
            SceneEntity,
            Wall,
            Body {
                half_size: size / 2.0,
            },
            Sprite::from_color(Color::srgb(0.28, 0.33, 0.42), size),
            Transform::from_xyz(wall.x, wall.y, 2.0),
        ));
    }

    for item in &scene.items {
        commands.spawn((
            GameplayEntity,
            SceneEntity,
            InventoryItem { kind: item.kind },
            Body {
                half_size: ITEM_SIZE / 2.0,
            },
            Sprite::from_color(item.kind.color(), ITEM_SIZE),
            Transform::from_xyz(item.x, item.y, 3.0),
        ));
    }

    for npc in &scene.npcs {
        commands.spawn((
            GameplayEntity,
            SceneEntity,
            Npc {
                name: npc.name.clone(),
                lines: npc.lines.clone(),
            },
            Body {
                half_size: NPC_SIZE / 2.0,
            },
            Sprite::from_color(Color::srgb(0.95, 0.68, 0.30), NPC_SIZE),
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

fn dialogue_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Option<Single<&Transform, With<Player>>>,
    npcs: Query<(Entity, &Transform, &Npc)>,
    mut dialogue: ResMut<DialogueState>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        dialogue.active_npc = None;
        dialogue.line_index = 0;
        next_state.set(GameState::Playing);
        return;
    }

    if *state.get() == GameState::Dialogue {
        if keyboard.just_pressed(KeyCode::Space) {
            let Some(active_npc) = dialogue.active_npc else {
                next_state.set(GameState::Playing);
                return;
            };

            let Ok((_, _, npc)) = npcs.get(active_npc) else {
                dialogue.active_npc = None;
                dialogue.line_index = 0;
                next_state.set(GameState::Playing);
                return;
            };

            dialogue.line_index += 1;

            if dialogue.line_index >= npc.lines.len() {
                dialogue.active_npc = None;
                dialogue.line_index = 0;
                next_state.set(GameState::Playing);
            }
        }

        return;
    }

    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    let Some(player) = player else {
        return;
    };

    let nearest = npcs.iter().find(|(_, transform, _)| {
        player
            .translation
            .truncate()
            .distance(transform.translation.truncate())
            <= INTERACT_DISTANCE
    });

    if let Some((entity, _, _)) = nearest {
        dialogue.active_npc = Some(entity);
        dialogue.line_index = 0;
        next_state.set(GameState::Dialogue);
    }
}

fn collect_items(
    mut commands: Commands,
    player: Option<Single<(&Transform, &Body), With<Player>>>,
    items: Query<(Entity, &Transform, &Body, &InventoryItem)>,
    mut inventory: ResMut<Inventory>,
    mut stats: ResMut<RunStats>,
) {
    let Some(player) = player else {
        return;
    };
    let (player_transform, player_body) = *player;

    for (entity, item_transform, item_body, item) in &items {
        if overlaps(player_transform, player_body, item_transform, item_body) {
            inventory.add(item.kind);
            stats.score += item.kind.score_value();
            commands.entity(entity).despawn();
        }
    }
}

fn start_capture_dialogue(
    mut done: Local<bool>,
    npcs: Query<Entity, With<Npc>>,
    mut dialogue: ResMut<DialogueState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *done || !tutorial_capture_enabled() {
        return;
    }

    let Some(entity) = npcs.iter().next() else {
        return;
    };

    *done = true;
    dialogue.active_npc = Some(entity);
    dialogue.line_index = 0;
    next_state.set(GameState::Dialogue);
}

fn update_status_text(
    current: Res<CurrentScene>,
    inventory: Res<Inventory>,
    stats: Res<RunStats>,
    state: Res<State<GameState>>,
    player: Option<Single<&Transform, With<Player>>>,
    items: Query<(), With<InventoryItem>>,
    npcs: Query<&Npc>,
    npc_positions: Query<(&Transform, &Npc)>,
    walls: Query<(), With<Wall>>,
    mut text: Single<&mut Text, With<StatusText>>,
) {
    let prompt = if *state.get() == GameState::Dialogue {
        "Dialogue | Space next | Esc close".to_string()
    } else if let Some(player) = player {
        let nearby = npc_positions.iter().find(|(transform, _)| {
            player
                .translation
                .truncate()
                .distance(transform.translation.truncate())
                <= INTERACT_DISTANCE
        });

        match nearby {
            Some((_, npc)) => format!("Playing | E talk to {}", npc.name),
            None => "Playing | WASD move | 1/2 load scene".to_string(),
        }
    } else {
        "Loading scene".to_string()
    };

    let npc_summary = npcs
        .iter()
        .map(|npc| {
            let first_line = npc.lines.first().map(String::as_str).unwrap_or("...");
            format!("{}: {}", npc.name, first_line)
        })
        .collect::<Vec<_>>()
        .join(" | ");

    text.0 = format!(
        "{prompt}\n{}\nscore: {} | gems: {} | keys: {} | potions: {} | items left: {}\nwalls: {} | npcs: {}",
        current.message,
        stats.score,
        inventory.gems,
        inventory.keys,
        inventory.potions,
        items.iter().count(),
        walls.iter().count(),
        if npc_summary.is_empty() {
            "none".to_string()
        } else {
            npc_summary
        }
    );
}

fn update_dialogue_text(
    dialogue: Res<DialogueState>,
    npcs: Query<&Npc>,
    mut text: Single<&mut Text, With<DialogueText>>,
) {
    let Some(entity) = dialogue.active_npc else {
        text.0.clear();
        return;
    };

    let Ok(npc) = npcs.get(entity) else {
        text.0.clear();
        return;
    };

    let line = npc
        .lines
        .get(dialogue.line_index)
        .map(String::as_str)
        .unwrap_or("");
    text.0 = format!("{}:\n{}\n\nSpace: continue | Esc: close", npc.name, line);
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
