use std::fs;

use bevy::prelude::*;
use bevy_tutorial::tutorial_capture::{add_tutorial_screenshot, tutorial_capture_enabled};
use serde::Deserialize;

mod tutorial_visuals;
use tutorial_visuals::{
    TutorialSprites, gem_sprite, npc_sprite, player_sprite, spawn_camera, spawn_dialogue_panel,
    spawn_floor, spawn_world_label,
};

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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut current: ResMut<CurrentScene>,
) {
    spawn_camera(&mut commands);

    let assets = TutorialSprites::load(&asset_server, &mut texture_atlas_layouts);
    commands.insert_resource(assets.clone());

    spawn_floor(&mut commands, -5..=5, -3..=3);
    commands.spawn((
        StatusText,
        Text::new(""),
        TextFont::from_font_size(19.0),
        TextColor(Color::srgb(0.92, 0.95, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(18),
            left: px(18),
            width: px(390),
            padding: UiRect::axes(px(14), px(10)),
            border: UiRect::all(px(1)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.045, 0.055, 0.075, 0.90)),
        BorderColor::all(Color::srgba(0.36, 0.45, 0.58, 0.75)),
    ));

    spawn_dialogue_panel(&mut commands, DialogueText);

    let path = current.path;
    current.message = load_scene(&mut commands, path, &assets);
}

fn scene_hotkeys(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    assets: Res<TutorialSprites>,
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
    current.message = load_scene(&mut commands, path, &assets);
}

fn load_scene(commands: &mut Commands, asset_path: &str, assets: &TutorialSprites) -> String {
    let fs_path = format!("assets/{asset_path}");
    let text = match fs::read_to_string(&fs_path) {
        Ok(text) => text,
        Err(error) => return format!("Failed to read {asset_path}: {error}"),
    };

    let scene = match serde_json::from_str::<SceneData>(&text) {
        Ok(scene) => scene,
        Err(error) => return format!("Failed to parse {asset_path}: {error}"),
    };

    spawn_scene(commands, &scene, assets);
    format!("Scene: {}", scene.name)
}

fn spawn_scene(commands: &mut Commands, scene: &SceneData, assets: &TutorialSprites) {
    commands.spawn((
        GameplayEntity,
        SceneEntity,
        Player,
        Body {
            half_size: PLAYER_SIZE / 2.0,
        },
        player_sprite(assets),
        Transform::from_xyz(scene.player_start[0], scene.player_start[1], 3.0),
    ));
    spawn_world_label(
        commands,
        "player_start",
        Vec3::new(scene.player_start[0], scene.player_start[1] - 46.0, 4.0),
    );

    for wall in &scene.walls {
        let size = Vec2::new(wall.w, wall.h);
        commands.spawn((
            GameplayEntity,
            SceneEntity,
            Wall,
            Body {
                half_size: size / 2.0,
            },
            Sprite::from_color(Color::srgb(0.30, 0.37, 0.48), size),
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
            item_sprite(item.kind, assets),
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
            npc_sprite(assets),
            Transform::from_xyz(npc.x, npc.y, 3.0),
        ));
        spawn_world_label(commands, &npc.name, Vec3::new(npc.x, npc.y + 40.0, 4.0));
    }
}

fn item_sprite(kind: ItemKind, assets: &TutorialSprites) -> Sprite {
    match kind {
        ItemKind::Gem => gem_sprite(assets),
        ItemKind::Key => Sprite::from_color(kind.color(), Vec2::new(38.0, 16.0)),
        ItemKind::Potion => Sprite::from_color(kind.color(), Vec2::new(24.0, 38.0)),
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
    mut player: Option<Single<&mut Transform, With<Player>>>,
    npcs: Query<(Entity, &Transform), (With<Npc>, Without<Player>)>,
    mut dialogue: ResMut<DialogueState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *done || !tutorial_capture_enabled() {
        return;
    }

    let Some((entity, npc_transform)) = npcs.iter().next() else {
        return;
    };

    *done = true;
    if let Some(player) = player.as_mut() {
        player.translation = npc_transform.translation + Vec3::new(-54.0, -34.0, 0.0);
    }
    dialogue.active_npc = Some(entity);
    dialogue.line_index = 0;
    next_state.set(GameState::Dialogue);
}

fn update_status_text(
    current: Res<CurrentScene>,
    state: Res<State<GameState>>,
    player: Option<Single<&Transform, With<Player>>>,
    items: Query<(), With<InventoryItem>>,
    npcs: Query<&Npc>,
    npc_positions: Query<(&Transform, &Npc), (With<Npc>, Without<Player>)>,
    walls: Query<(), With<Wall>>,
    mut text: Single<&mut Text, With<StatusText>>,
) {
    let prompt = if *state.get() == GameState::Dialogue {
        "Dialogue | Space next | Esc".to_string()
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
            None => "Playing | WASD | 1/2 scene".to_string(),
        }
    } else {
        "Loading scene".to_string()
    };

    text.0 = format!(
        "{prompt}\n{}\nJSON -> {} walls | {} items | {} NPCs",
        current.message,
        walls.iter().count(),
        items.iter().count(),
        npcs.iter().count()
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
