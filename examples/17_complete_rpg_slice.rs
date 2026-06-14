use std::{fs, path::Path};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

const SAVE_PATH: &str = "target/tutorial-save/complete-progress.json";
const PLAYER_SPEED: f32 = 285.0;
const ENEMY_SPEED: f32 = 90.0;
const PLAYER_SIZE: Vec2 = Vec2::splat(40.0);
const ENEMY_SIZE: Vec2 = Vec2::splat(34.0);
const GEM_SIZE: Vec2 = Vec2::splat(26.0);
const HITBOX_SIZE: Vec2 = Vec2::new(58.0, 34.0);
const HITBOX_DISTANCE: f32 = 46.0;
const CAMERA_SMOOTHNESS: f32 = 8.0;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    GameOver,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Wave,
    Ai,
    Movement,
    Collision,
    Animation,
    Ui,
}

#[derive(Component)]
struct GameplayEntity;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Collectible;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Mover;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Body {
    half_size: Vec2,
}

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Facing(Vec2);

#[derive(Component)]
struct Health {
    current: i32,
    max: i32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum PlayerAnimState {
    Idle,
    Run,
    Attack,
}

#[derive(Component)]
struct PlayerAnimation {
    state: PlayerAnimState,
    frame_timer: Timer,
    attack_timer: Timer,
    run_frame: usize,
}

impl Default for PlayerAnimation {
    fn default() -> Self {
        Self {
            state: PlayerAnimState::Idle,
            frame_timer: Timer::from_seconds(0.13, TimerMode::Repeating),
            attack_timer: Timer::from_seconds(0.18, TimerMode::Once),
            run_frame: 1,
        }
    }
}

#[derive(Component)]
struct AttackHitbox {
    lifetime: Timer,
    damage: i32,
}

#[derive(Component)]
struct MenuUi;

#[derive(Component)]
struct PauseUi;

#[derive(Component)]
struct GameOverUi;

#[derive(Component)]
struct HudHealthText;

#[derive(Component)]
struct HudScoreText;

#[derive(Component)]
struct HudWaveText;

#[derive(Component)]
struct HudSaveText;

#[derive(Component)]
struct HudHealthFill;

#[derive(Resource, Clone)]
struct SpriteAssets {
    player_sheet: Handle<Image>,
    player_layout: Handle<TextureAtlasLayout>,
    enemy: Handle<Image>,
    gem: Handle<Image>,
    slash: Handle<Image>,
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
struct Progress {
    best_score: u32,
    unlocked_wave: u32,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            best_score: 0,
            unlocked_wave: 1,
        }
    }
}

#[derive(Resource)]
struct RunStats {
    score: u32,
    wave: u32,
}

impl Default for RunStats {
    fn default() -> Self {
        Self { score: 0, wave: 1 }
    }
}

#[derive(Resource)]
struct WaveSpawner {
    wave: u32,
    remaining_to_spawn: u32,
    spawn_index: usize,
    timer: Timer,
}

impl Default for WaveSpawner {
    fn default() -> Self {
        Self {
            wave: 1,
            remaining_to_spawn: 3,
            spawn_index: 0,
            timer: Timer::from_seconds(0.65, TimerMode::Repeating),
        }
    }
}

impl WaveSpawner {
    fn reset_to_wave(&mut self, wave: u32) {
        self.wave = wave.max(1);
        self.remaining_to_spawn = self.wave + 2;
        self.spawn_index = 0;
        self.timer.reset();
    }
}

#[derive(Resource)]
struct SaveStatus(String);

impl Default for SaveStatus {
    fn default() -> Self {
        Self(format!("Progress file: {SAVE_PATH}"))
    }
}

#[derive(Bundle)]
struct BodyBundle {
    mover: Mover,
    body: Body,
    velocity: Velocity,
    transform: Transform,
}

impl BodyBundle {
    fn new(position: Vec3, size: Vec2) -> Self {
        Self {
            mover: Mover,
            body: Body {
                half_size: size / 2.0,
            },
            velocity: Velocity(Vec2::ZERO),
            transform: Transform::from_translation(position),
        }
    }
}

#[derive(Bundle)]
struct StaticBodyBundle {
    body: Body,
    transform: Transform,
}

impl StaticBodyBundle {
    fn new(position: Vec3, size: Vec2) -> Self {
        Self {
            body: Body {
                half_size: size / 2.0,
            },
            transform: Transform::from_translation(position),
        }
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    gameplay: GameplayEntity,
    player: Player,
    body: BodyBundle,
    facing: Facing,
    health: Health,
    animation: PlayerAnimation,
    sprite: Sprite,
}

impl PlayerBundle {
    fn new(assets: &SpriteAssets) -> Self {
        Self {
            gameplay: GameplayEntity,
            player: Player,
            body: BodyBundle::new(Vec3::new(-260.0, -120.0, 3.0), PLAYER_SIZE),
            facing: Facing(Vec2::X),
            health: Health { current: 5, max: 5 },
            animation: PlayerAnimation::default(),
            sprite: Sprite {
                image: assets.player_sheet.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: assets.player_layout.clone(),
                    index: 0,
                }),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
struct EnemyBundle {
    gameplay: GameplayEntity,
    enemy: Enemy,
    body: BodyBundle,
    health: Health,
    sprite: Sprite,
}

impl EnemyBundle {
    fn new(position: Vec3, wave: u32, assets: &SpriteAssets) -> Self {
        Self {
            gameplay: GameplayEntity,
            enemy: Enemy,
            body: BodyBundle::new(position, ENEMY_SIZE),
            health: Health {
                current: 1 + (wave / 3) as i32,
                max: 1 + (wave / 3) as i32,
            },
            sprite: Sprite::from_image(assets.enemy.clone()),
        }
    }
}

#[derive(Bundle)]
struct CollectibleBundle {
    gameplay: GameplayEntity,
    collectible: Collectible,
    body: StaticBodyBundle,
    sprite: Sprite,
}

impl CollectibleBundle {
    fn new(position: Vec3, assets: &SpriteAssets) -> Self {
        Self {
            gameplay: GameplayEntity,
            collectible: Collectible,
            body: StaticBodyBundle::new(position, GEM_SIZE),
            sprite: Sprite::from_image(assets.gem.clone()),
        }
    }
}

#[derive(Bundle)]
struct WallBundle {
    gameplay: GameplayEntity,
    wall: Wall,
    body: StaticBodyBundle,
    sprite: Sprite,
}

impl WallBundle {
    fn new(position: Vec3, size: Vec2) -> Self {
        Self {
            gameplay: GameplayEntity,
            wall: Wall,
            body: StaticBodyBundle::new(position, size),
            sprite: Sprite::from_color(Color::srgb(0.28, 0.33, 0.42), size),
        }
    }
}

#[derive(Bundle)]
struct AttackHitboxBundle {
    gameplay: GameplayEntity,
    hitbox: AttackHitbox,
    body: Body,
    sprite: Sprite,
    transform: Transform,
}

impl AttackHitboxBundle {
    fn new(position: Vec3, angle: f32, assets: &SpriteAssets) -> Self {
        Self {
            gameplay: GameplayEntity,
            hitbox: AttackHitbox {
                lifetime: Timer::from_seconds(0.13, TimerMode::Once),
                damage: 1,
            },
            body: Body {
                half_size: HITBOX_SIZE / 2.0,
            },
            sprite: Sprite::from_image(assets.slash.clone()),
            transform: Transform {
                translation: position,
                rotation: Quat::from_rotation_z(angle),
                ..default()
            },
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .insert_resource(load_progress_from_disk())
        .init_resource::<RunStats>()
        .init_resource::<WaveSpawner>()
        .init_resource::<SaveStatus>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .configure_sets(
            Update,
            (
                GameSet::Input,
                GameSet::Wave,
                GameSet::Ai,
                GameSet::Movement,
                GameSet::Collision,
                GameSet::Animation,
                GameSet::Ui,
            )
                .chain(),
        )
        .add_systems(Startup, setup_camera_and_assets)
        .add_systems(OnEnter(GameState::Menu), spawn_menu)
        .add_systems(OnExit(GameState::Menu), cleanup_entities::<MenuUi>)
        .add_systems(OnEnter(GameState::Paused), spawn_pause_ui)
        .add_systems(OnExit(GameState::Paused), cleanup_entities::<PauseUi>)
        .add_systems(OnEnter(GameState::GameOver), enter_game_over)
        .add_systems(OnExit(GameState::GameOver), cleanup_entities::<GameOverUi>)
        .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)))
        .add_systems(Update, paused_input.run_if(in_state(GameState::Paused)))
        .add_systems(
            Update,
            game_over_input.run_if(in_state(GameState::GameOver)),
        )
        .add_systems(
            Update,
            (player_input, spawn_attack_hitbox)
                .chain()
                .in_set(GameSet::Input)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            spawn_enemy_waves
                .in_set(GameSet::Wave)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            enemy_ai
                .in_set(GameSet::Ai)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (move_bodies, resolve_wall_collisions, smooth_follow_camera)
                .chain()
                .in_set(GameSet::Movement)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (
                collect_gems,
                attack_hits_enemies,
                enemy_hits_player,
                expire_attack_hitboxes,
                end_game_if_dead,
            )
                .chain()
                .in_set(GameSet::Collision)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            animate_player
                .in_set(GameSet::Animation)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (
                save_load_hotkeys,
                update_health_ui,
                update_score_ui,
                update_wave_ui,
                update_save_ui,
            )
                .chain()
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}

fn setup_camera_and_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((Camera2d, MainCamera));

    let player_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(32),
        4,
        1,
        None,
        None,
    ));

    commands.insert_resource(SpriteAssets {
        player_sheet: asset_server.load("player_sheet.png"),
        player_layout,
        enemy: asset_server.load("enemy.png"),
        gem: asset_server.load("gem.png"),
        slash: asset_server.load("slash.png"),
    });
}

fn spawn_menu(mut commands: Commands, progress: Res<Progress>) {
    commands.spawn((
        MenuUi,
        Text::new(format!(
            "RUST + BEVY RPG\nEnter: start\nBest score: {}\nUnlocked wave: {}",
            progress.best_score, progress.unlocked_wave
        )),
        TextFont::from_font_size(36.0),
        TextColor(Color::srgb(0.92, 0.95, 1.0)),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: percent(32),
            left: percent(31),
            ..default()
        },
    ));
}

fn menu_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    assets: Res<SpriteAssets>,
    progress: Res<Progress>,
    mut stats: ResMut<RunStats>,
    mut spawner: ResMut<WaveSpawner>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        start_run(&mut commands, &assets, &progress, &mut stats, &mut spawner);
        next_state.set(GameState::Playing);
    }
}

fn spawn_pause_ui(mut commands: Commands) {
    commands.spawn((
        PauseUi,
        Text::new("PAUSED\nP: resume | Esc: menu"),
        TextFont::from_font_size(34.0),
        TextColor(Color::srgb(1.0, 0.90, 0.50)),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: percent(38),
            left: percent(34),
            ..default()
        },
    ));
}

fn paused_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    gameplay: Query<Entity, With<GameplayEntity>>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        next_state.set(GameState::Playing);
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        for entity in &gameplay {
            commands.entity(entity).despawn();
        }
        next_state.set(GameState::Menu);
    }
}

fn enter_game_over(
    mut commands: Commands,
    gameplay: Query<Entity, With<GameplayEntity>>,
    stats: Res<RunStats>,
    mut progress: ResMut<Progress>,
    mut save_status: ResMut<SaveStatus>,
) {
    for entity in &gameplay {
        commands.entity(entity).despawn();
    }

    progress.best_score = progress.best_score.max(stats.score);
    progress.unlocked_wave = progress.unlocked_wave.max(stats.wave);
    save_status.0 = match save_progress_to_disk(&progress) {
        Ok(()) => format!("Saved progress to {SAVE_PATH}"),
        Err(error) => format!("Save failed: {error}"),
    };

    commands.spawn((
        GameOverUi,
        Text::new(format!(
            "GAME OVER\nScore: {}\nWave: {}\nR: restart | Esc: menu\n{}",
            stats.score, stats.wave, save_status.0
        )),
        TextFont::from_font_size(30.0),
        TextColor(Color::srgb(1.0, 0.48, 0.48)),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: percent(32),
            left: percent(28),
            ..default()
        },
    ));
}

fn game_over_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    assets: Res<SpriteAssets>,
    progress: Res<Progress>,
    mut stats: ResMut<RunStats>,
    mut spawner: ResMut<WaveSpawner>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        start_run(&mut commands, &assets, &progress, &mut stats, &mut spawner);
        next_state.set(GameState::Playing);
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}

fn start_run(
    commands: &mut Commands,
    assets: &SpriteAssets,
    progress: &Progress,
    stats: &mut RunStats,
    spawner: &mut WaveSpawner,
) {
    *stats = RunStats::default();
    stats.wave = progress.unlocked_wave.max(1);
    spawner.reset_to_wave(stats.wave);

    commands.spawn(PlayerBundle::new(assets));
    spawn_map(commands);
    spawn_hud(commands);

    for position in [
        Vec3::new(-120.0, 150.0, 3.0),
        Vec3::new(250.0, -110.0, 3.0),
        Vec3::new(340.0, 190.0, 3.0),
    ] {
        commands.spawn(CollectibleBundle::new(position, assets));
    }
}

fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&mut Velocity, &mut Facing, &mut PlayerAnimation), With<Player>>,
) {
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

    let normalized = direction.normalize_or_zero();

    for (mut velocity, mut facing, mut animation) in &mut players {
        velocity.0 = normalized * PLAYER_SPEED;

        if normalized != Vec2::ZERO {
            facing.0 = normalized;
        }

        if animation.state != PlayerAnimState::Attack {
            animation.state = if normalized == Vec2::ZERO {
                PlayerAnimState::Idle
            } else {
                PlayerAnimState::Run
            };
        }
    }
}

fn spawn_attack_hitbox(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    assets: Res<SpriteAssets>,
    player: Single<(&Transform, &Facing, &mut PlayerAnimation), With<Player>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let (transform, facing, mut animation) = player.into_inner();
    animation.state = PlayerAnimState::Attack;
    animation.attack_timer.reset();

    let position = transform.translation + (facing.0 * HITBOX_DISTANCE).extend(1.0);
    let angle = facing.0.y.atan2(facing.0.x);

    commands.spawn(AttackHitboxBundle::new(position, angle, &assets));
}

fn spawn_enemy_waves(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<SpriteAssets>,
    mut spawner: ResMut<WaveSpawner>,
    mut stats: ResMut<RunStats>,
    enemies: Query<(), With<Enemy>>,
) {
    if spawner.remaining_to_spawn == 0 && enemies.iter().count() == 0 {
        spawner.wave += 1;
        spawner.remaining_to_spawn = spawner.wave + 2;
        spawner.timer.reset();
        stats.wave = spawner.wave;
    }

    if spawner.remaining_to_spawn == 0 {
        return;
    }

    spawner.timer.tick(time.delta());

    if !spawner.timer.just_finished() {
        return;
    }

    let points = [
        Vec2::new(-380.0, 250.0),
        Vec2::new(380.0, 250.0),
        Vec2::new(-380.0, -250.0),
        Vec2::new(380.0, -250.0),
    ];
    let spawn = points[spawner.spawn_index % points.len()];

    spawner.spawn_index += 1;
    spawner.remaining_to_spawn -= 1;

    commands.spawn(EnemyBundle::new(spawn.extend(3.0), spawner.wave, &assets));
}

fn enemy_ai(
    player: Single<&Transform, With<Player>>,
    mut enemies: Query<(&Transform, &mut Velocity), With<Enemy>>,
) {
    let player_position = player.translation.truncate();

    for (transform, mut velocity) in &mut enemies {
        let to_player = player_position - transform.translation.truncate();
        velocity.0 = to_player.normalize_or_zero() * ENEMY_SPEED;
    }
}

fn move_bodies(time: Res<Time>, mut bodies: Query<(&mut Transform, &Velocity), With<Body>>) {
    for (mut transform, velocity) in &mut bodies {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
    }
}

fn resolve_wall_collisions(
    mut bodies: Query<(&mut Transform, &Body, Option<&Mover>, Option<&Wall>)>,
) {
    let mut walls = Vec::new();

    for (transform, body, _, wall) in &mut bodies {
        if wall.is_some() {
            walls.push((transform.translation.truncate(), body.half_size));
        }
    }

    for (mut mover_transform, mover_body, mover, _) in &mut bodies {
        if mover.is_none() {
            continue;
        }

        for (wall_position, wall_half_size) in &walls {
            let mover_position = mover_transform.translation.truncate();
            let delta = mover_position - *wall_position;
            let overlap = mover_body.half_size + *wall_half_size - delta.abs();

            if overlap.x <= 0.0 || overlap.y <= 0.0 {
                continue;
            }

            if overlap.x < overlap.y {
                mover_transform.translation.x += overlap.x * delta.x.signum();
            } else {
                mover_transform.translation.y += overlap.y * delta.y.signum();
            }
        }
    }
}

fn smooth_follow_camera(
    time: Res<Time>,
    player: Single<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let target = Vec3::new(
        player.translation.x,
        player.translation.y,
        camera.translation.z,
    );
    let blend = 1.0 - (-CAMERA_SMOOTHNESS * time.delta_secs()).exp();
    camera.translation = camera.translation.lerp(target, blend);
}

fn collect_gems(
    mut commands: Commands,
    mut stats: ResMut<RunStats>,
    player: Single<(&Transform, &Body), With<Player>>,
    gems: Query<(Entity, &Transform, &Body), With<Collectible>>,
) {
    let (player_transform, player_body) = *player;

    for (entity, gem_transform, gem_body) in &gems {
        if overlaps(player_transform, player_body, gem_transform, gem_body) {
            commands.entity(entity).despawn();
            stats.score += 10;
        }
    }
}

fn attack_hits_enemies(
    mut commands: Commands,
    mut stats: ResMut<RunStats>,
    hitboxes: Query<(Entity, &Transform, &Body, &AttackHitbox)>,
    mut enemies: Query<(Entity, &Transform, &Body, &mut Health), With<Enemy>>,
) {
    let mut defeated_enemies = Vec::new();

    for (hitbox_entity, hitbox_transform, hitbox_body, hitbox) in &hitboxes {
        let mut hit_anything = false;

        for (enemy_entity, enemy_transform, enemy_body, mut health) in &mut enemies {
            if defeated_enemies.contains(&enemy_entity) {
                continue;
            }

            if overlaps(hitbox_transform, hitbox_body, enemy_transform, enemy_body) {
                health.current -= hitbox.damage;
                hit_anything = true;

                if health.current <= 0 {
                    commands.entity(enemy_entity).despawn();
                    defeated_enemies.push(enemy_entity);
                    stats.score += 25;
                }
            }
        }

        if hit_anything {
            commands.entity(hitbox_entity).despawn();
        }
    }
}

fn enemy_hits_player(
    time: Res<Time>,
    player: Single<(&Transform, &Body, &mut Health), With<Player>>,
    enemies: Query<(&Transform, &Body), With<Enemy>>,
    mut cooldown: Local<f32>,
) {
    *cooldown -= time.delta_secs();

    if *cooldown > 0.0 {
        return;
    }

    let (player_transform, player_body, mut health) = player.into_inner();

    for (enemy_transform, enemy_body) in &enemies {
        if overlaps(player_transform, player_body, enemy_transform, enemy_body) {
            health.current = (health.current - 1).max(0);
            *cooldown = 0.85;
            break;
        }
    }
}

fn expire_attack_hitboxes(
    mut commands: Commands,
    time: Res<Time>,
    mut hitboxes: Query<(Entity, &mut AttackHitbox)>,
) {
    for (entity, mut hitbox) in &mut hitboxes {
        hitbox.lifetime.tick(time.delta());

        if hitbox.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn end_game_if_dead(
    player: Single<&Health, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player.current <= 0 {
        next_state.set(GameState::GameOver);
    }
}

fn animate_player(
    time: Res<Time>,
    player: Single<(&mut Sprite, &Velocity, &mut PlayerAnimation), With<Player>>,
) {
    let (mut sprite, velocity, mut animation) = player.into_inner();
    let Some(atlas) = &mut sprite.texture_atlas else {
        return;
    };

    match animation.state {
        PlayerAnimState::Idle => atlas.index = 0,
        PlayerAnimState::Run => {
            animation.frame_timer.tick(time.delta());

            if animation.frame_timer.just_finished() {
                animation.run_frame = if animation.run_frame == 1 { 2 } else { 1 };
            }

            atlas.index = animation.run_frame;
        }
        PlayerAnimState::Attack => {
            atlas.index = 3;
            animation.attack_timer.tick(time.delta());

            if animation.attack_timer.is_finished() {
                animation.state = if velocity.0 == Vec2::ZERO {
                    PlayerAnimState::Idle
                } else {
                    PlayerAnimState::Run
                };
            }
        }
    }
}

fn save_load_hotkeys(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut progress: ResMut<Progress>,
    stats: Res<RunStats>,
    mut save_status: ResMut<SaveStatus>,
) {
    if keyboard.just_pressed(KeyCode::F5) {
        progress.best_score = progress.best_score.max(stats.score);
        progress.unlocked_wave = progress.unlocked_wave.max(stats.wave);

        save_status.0 = match save_progress_to_disk(&progress) {
            Ok(()) => format!("Saved progress to {SAVE_PATH}"),
            Err(error) => format!("Save failed: {error}"),
        };
    }

    if keyboard.just_pressed(KeyCode::F9) {
        *progress = load_progress_from_disk();
        save_status.0 = format!("Loaded progress from {SAVE_PATH}");
    }
}

fn update_health_ui(
    player: Single<&Health, With<Player>>,
    mut health_text: Single<&mut Text, With<HudHealthText>>,
    mut health_fill: Single<&mut Node, With<HudHealthFill>>,
) {
    let health = *player;
    let health_fraction = health.current as f32 / health.max as f32;

    health_text.0 = format!("Health: {}/{}", health.current, health.max);
    health_fill.width = px(180.0 * health_fraction);
}

fn update_score_ui(
    stats: Res<RunStats>,
    progress: Res<Progress>,
    mut score_text: Single<&mut Text, With<HudScoreText>>,
) {
    score_text.0 = format!("Score: {} | Best: {}", stats.score, progress.best_score);
}

fn update_wave_ui(
    stats: Res<RunStats>,
    spawner: Res<WaveSpawner>,
    mut wave_text: Single<&mut Text, With<HudWaveText>>,
) {
    wave_text.0 = format!(
        "Wave: {} | queued: {}",
        stats.wave, spawner.remaining_to_spawn
    );
}

fn update_save_ui(
    save_status: Res<SaveStatus>,
    mut save_text: Single<&mut Text, With<HudSaveText>>,
) {
    save_text.0 = format!("F5: save progress | {}", save_status.0);
}

fn overlaps(
    a_transform: &Transform,
    a_body: &Body,
    b_transform: &Transform,
    b_body: &Body,
) -> bool {
    let a = a_transform.translation.truncate();
    let b = b_transform.translation.truncate();
    let distance = (a - b).abs();
    let allowed = a_body.half_size + b_body.half_size;

    distance.x < allowed.x && distance.y < allowed.y
}

fn spawn_map(commands: &mut Commands) {
    let tile_size = Vec2::splat(80.0);

    for x in -5..=5 {
        for y in -3..=3 {
            let color = if (x + y) % 2 == 0 {
                Color::srgb(0.13, 0.16, 0.20)
            } else {
                Color::srgb(0.15, 0.18, 0.23)
            };

            commands.spawn((
                GameplayEntity,
                Sprite::from_color(color, tile_size - Vec2::splat(2.0)),
                Transform::from_xyz(x as f32 * tile_size.x, y as f32 * tile_size.y, 0.0),
            ));
        }
    }

    for (position, size) in [
        (Vec3::new(0.0, 300.0, 2.0), Vec2::new(900.0, 40.0)),
        (Vec3::new(0.0, -300.0, 2.0), Vec2::new(900.0, 40.0)),
        (Vec3::new(-460.0, 0.0, 2.0), Vec2::new(40.0, 640.0)),
        (Vec3::new(460.0, 0.0, 2.0), Vec2::new(40.0, 640.0)),
        (Vec3::new(-120.0, 70.0, 2.0), Vec2::new(240.0, 36.0)),
        (Vec3::new(210.0, -110.0, 2.0), Vec2::new(250.0, 36.0)),
        (Vec3::new(80.0, 145.0, 2.0), Vec2::new(36.0, 210.0)),
    ] {
        commands.spawn(WallBundle::new(position, size));
    }
}

fn spawn_hud(commands: &mut Commands) {
    commands.spawn((
        GameplayEntity,
        HudHealthText,
        Text::new("Health: 5/5"),
        TextFont::from_font_size(22.0),
        TextColor(Color::srgb(0.94, 0.97, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(16),
            left: px(16),
            ..default()
        },
    ));

    commands.spawn((
        GameplayEntity,
        Node {
            position_type: PositionType::Absolute,
            top: px(48),
            left: px(16),
            width: px(180),
            height: px(14),
            ..default()
        },
        BackgroundColor(Color::srgb(0.22, 0.24, 0.30)),
    ));

    commands.spawn((
        GameplayEntity,
        HudHealthFill,
        Node {
            position_type: PositionType::Absolute,
            top: px(48),
            left: px(16),
            width: px(180),
            height: px(14),
            ..default()
        },
        BackgroundColor(Color::srgb(0.22, 0.84, 0.40)),
    ));

    commands.spawn((
        GameplayEntity,
        HudScoreText,
        Text::new("Score: 0"),
        TextFont::from_font_size(22.0),
        TextColor(Color::srgb(1.0, 0.82, 0.30)),
        Node {
            position_type: PositionType::Absolute,
            top: px(70),
            left: px(16),
            ..default()
        },
    ));

    commands.spawn((
        GameplayEntity,
        HudWaveText,
        Text::new("Wave: 1"),
        TextFont::from_font_size(22.0),
        TextColor(Color::srgb(0.86, 0.92, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(96),
            left: px(16),
            ..default()
        },
    ));

    commands.spawn((
        GameplayEntity,
        HudSaveText,
        Text::new("F5: save progress"),
        TextFont::from_font_size(18.0),
        TextColor(Color::srgb(0.82, 0.86, 0.92)),
        Node {
            position_type: PositionType::Absolute,
            top: px(124),
            left: px(16),
            ..default()
        },
    ));
}

fn load_progress_from_disk() -> Progress {
    fs::read_to_string(SAVE_PATH)
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

fn save_progress_to_disk(progress: &Progress) -> Result<(), String> {
    let Some(parent) = Path::new(SAVE_PATH).parent() else {
        return Err("save path has no parent directory".to_string());
    };

    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let json = serde_json::to_string_pretty(progress).map_err(|error| error.to_string())?;
    fs::write(SAVE_PATH, json).map_err(|error| error.to_string())
}

fn cleanup_entities<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
