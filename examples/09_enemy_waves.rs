use bevy::prelude::*;

const PLAYER_SPEED: f32 = 280.0;
const ENEMY_SPEED: f32 = 95.0;
const PLAYER_SIZE: Vec2 = Vec2::splat(42.0);
const ENEMY_SIZE: Vec2 = Vec2::splat(36.0);
const ARENA_HALF_SIZE: Vec2 = Vec2::new(520.0, 320.0);
const SPAWN_POINTS: [Vec2; 4] = [
    Vec2::new(-470.0, 260.0),
    Vec2::new(470.0, 260.0),
    Vec2::new(-470.0, -260.0),
    Vec2::new(470.0, -260.0),
];

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Wave,
    Ai,
    Movement,
    Ui,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct EnemyLifetime(Timer);

#[derive(Component)]
struct Body {
    half_size: Vec2,
}

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct WaveText;

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
            remaining_to_spawn: 4,
            spawn_index: 0,
            timer: Timer::from_seconds(0.35, TimerMode::Repeating),
        }
    }
}

#[derive(Bundle)]
struct BodyBundle {
    body: Body,
    velocity: Velocity,
    transform: Transform,
}

impl BodyBundle {
    fn new(position: Vec3, size: Vec2) -> Self {
        Self {
            body: Body {
                half_size: size / 2.0,
            },
            velocity: Velocity(Vec2::ZERO),
            transform: Transform::from_translation(position),
        }
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    body: BodyBundle,
    sprite: Sprite,
}

impl PlayerBundle {
    fn new(asset_server: &AssetServer) -> Self {
        Self {
            player: Player,
            body: BodyBundle::new(Vec3::new(0.0, 0.0, 2.0), PLAYER_SIZE),
            sprite: Sprite::from_image(asset_server.load("player.png")),
        }
    }
}

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    lifetime: EnemyLifetime,
    body: BodyBundle,
    sprite: Sprite,
}

impl EnemyBundle {
    fn new(position: Vec3, asset_server: &AssetServer) -> Self {
        Self {
            enemy: Enemy,
            lifetime: EnemyLifetime(Timer::from_seconds(2.5, TimerMode::Once)),
            body: BodyBundle::new(position, ENEMY_SIZE),
            sprite: Sprite::from_image(asset_server.load("enemy.png")),
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .init_resource::<WaveSpawner>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .configure_sets(
            Update,
            (
                GameSet::Input,
                GameSet::Wave,
                GameSet::Ai,
                GameSet::Movement,
                GameSet::Ui,
            )
                .chain(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, player_input.in_set(GameSet::Input))
        .add_systems(
            Update,
            (expire_enemies, spawn_enemy_waves)
                .chain()
                .in_set(GameSet::Wave),
        )
        .add_systems(Update, enemy_ai.in_set(GameSet::Ai))
        .add_systems(
            Update,
            (move_bodies, clamp_to_arena)
                .chain()
                .in_set(GameSet::Movement),
        )
        .add_systems(Update, update_wave_text.in_set(GameSet::Ui))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(PlayerBundle::new(&asset_server));
    spawn_arena_frame(&mut commands);

    commands.spawn((
        WaveText,
        Text::new("Wave 1 | waiting"),
        TextFont::from_font_size(24.0),
        TextColor(Color::srgb(0.92, 0.95, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(16),
            left: px(16),
            ..default()
        },
    ));
}

fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Velocity, With<Player>>,
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

    for mut velocity in &mut players {
        velocity.0 = direction.normalize_or_zero() * PLAYER_SPEED;
    }
}

fn spawn_enemy_waves(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut spawner: ResMut<WaveSpawner>,
    enemies: Query<(), With<Enemy>>,
) {
    if spawner.remaining_to_spawn == 0 && enemies.iter().count() == 0 {
        spawner.wave += 1;
        spawner.remaining_to_spawn = spawner.wave + 3;
        spawner.timer.reset();
        info!("starting wave {}", spawner.wave);
    }

    if spawner.remaining_to_spawn == 0 {
        return;
    }

    spawner.timer.tick(time.delta());

    if !spawner.timer.just_finished() {
        return;
    }

    let spawn = SPAWN_POINTS[spawner.spawn_index % SPAWN_POINTS.len()];
    spawner.spawn_index += 1;
    spawner.remaining_to_spawn -= 1;

    commands.spawn(EnemyBundle::new(spawn.extend(2.0), asset_server.as_ref()));
}

fn expire_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut enemies: Query<(Entity, &mut EnemyLifetime), With<Enemy>>,
) {
    for (entity, mut lifetime) in &mut enemies {
        lifetime.0.tick(time.delta());

        if lifetime.0.is_finished() {
            commands.entity(entity).despawn();
        }
    }
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

fn clamp_to_arena(mut bodies: Query<(&mut Transform, &Body), With<Body>>) {
    for (mut transform, body) in &mut bodies {
        let min = -ARENA_HALF_SIZE + body.half_size;
        let max = ARENA_HALF_SIZE - body.half_size;
        transform.translation.x = transform.translation.x.clamp(min.x, max.x);
        transform.translation.y = transform.translation.y.clamp(min.y, max.y);
    }
}

fn update_wave_text(
    spawner: Res<WaveSpawner>,
    enemies: Query<(), With<Enemy>>,
    mut text: Single<&mut Text, With<WaveText>>,
) {
    text.0 = format!(
        "Wave {} | alive: {} | queued: {}",
        spawner.wave,
        enemies.iter().count(),
        spawner.remaining_to_spawn
    );
}

fn spawn_arena_frame(commands: &mut Commands) {
    let wall_color = Color::srgb(0.25, 0.30, 0.38);
    let width = ARENA_HALF_SIZE.x * 2.0 + 10.0;
    let height = ARENA_HALF_SIZE.y * 2.0 + 10.0;

    for (position, size) in [
        (
            Vec3::new(0.0, ARENA_HALF_SIZE.y + 5.0, 1.0),
            Vec2::new(width, 10.0),
        ),
        (
            Vec3::new(0.0, -ARENA_HALF_SIZE.y - 5.0, 1.0),
            Vec2::new(width, 10.0),
        ),
        (
            Vec3::new(-ARENA_HALF_SIZE.x - 5.0, 0.0, 1.0),
            Vec2::new(10.0, height),
        ),
        (
            Vec3::new(ARENA_HALF_SIZE.x + 5.0, 0.0, 1.0),
            Vec2::new(10.0, height),
        ),
    ] {
        commands.spawn((
            Sprite::from_color(wall_color, size),
            Transform::from_translation(position),
        ));
    }
}
