use bevy::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::splat(42.0);
const ENEMY_SIZE: Vec2 = Vec2::new(56.0, 56.0);
const COLLECTIBLE_SIZE: Vec2 = Vec2::splat(24.0);
const PLAYER_SPEED: f32 = 260.0;
const ENEMY_SPEED: f32 = 80.0;
const MAX_HEALTH: i32 = 5;
const ARENA_HALF_SIZE: Vec2 = Vec2::new(420.0, 260.0);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Ai,
    Movement,
    Collision,
    Display,
}

#[derive(Component)]
struct Body {
    half_size: Vec2,
}

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Collectible;

#[derive(Component)]
struct Health {
    current: i32,
    max: i32,
}

#[derive(Component)]
struct HealthBarFill;

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct ScoreText;

#[derive(Resource)]
struct Score(u32);

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
    health: Health,
}

impl PlayerBundle {
    fn new(position: Vec3) -> Self {
        Self {
            player: Player,
            body: BodyBundle::new(position, PLAYER_SIZE),
            sprite: Sprite::from_color(Color::srgb(0.25, 0.70, 1.0), PLAYER_SIZE),
            health: Health {
                current: MAX_HEALTH,
                max: MAX_HEALTH,
            },
        }
    }
}

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    body: BodyBundle,
    sprite: Sprite,
}

impl EnemyBundle {
    fn new(position: Vec3) -> Self {
        Self {
            enemy: Enemy,
            body: BodyBundle::new(position, ENEMY_SIZE),
            sprite: Sprite::from_color(Color::srgb(0.95, 0.22, 0.25), ENEMY_SIZE),
        }
    }
}

#[derive(Bundle)]
struct CollectibleBundle {
    collectible: Collectible,
    body: BodyBundle,
    sprite: Sprite,
}

impl CollectibleBundle {
    fn new(position: Vec3) -> Self {
        Self {
            collectible: Collectible,
            body: BodyBundle::new(position, COLLECTIBLE_SIZE),
            sprite: Sprite::from_color(Color::srgb(1.0, 0.82, 0.25), COLLECTIBLE_SIZE),
        }
    }
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.11)))
            .insert_resource(Score(0))
            .configure_sets(
                Update,
                (
                    GameSet::Input,
                    GameSet::Ai,
                    GameSet::Movement,
                    GameSet::Collision,
                    GameSet::Display,
                )
                    .chain(),
            )
            .add_systems(Startup, setup)
            .add_systems(Update, player_input.in_set(GameSet::Input))
            .add_systems(Update, enemy_ai.in_set(GameSet::Ai))
            .add_systems(
                Update,
                (move_bodies, clamp_to_arena)
                    .chain()
                    .in_set(GameSet::Movement),
            )
            .add_systems(Update, collect_items.in_set(GameSet::Collision))
            .add_systems(Update, enemy_hits_player.in_set(GameSet::Collision))
            .add_systems(
                Update,
                (update_health_bar, update_hud_text).in_set(GameSet::Display),
            );
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    spawn_arena_frame(&mut commands);

    commands.spawn(PlayerBundle::new(Vec3::new(-260.0, 0.0, 1.0)));

    for position in [
        Vec3::new(180.0, 140.0, 1.0),
        Vec3::new(260.0, -80.0, 1.0),
        Vec3::new(-60.0, -170.0, 1.0),
    ] {
        commands.spawn(EnemyBundle::new(position));
    }

    for position in [
        Vec3::new(-80.0, 130.0, 1.0),
        Vec3::new(80.0, -40.0, 1.0),
        Vec3::new(330.0, 190.0, 1.0),
        Vec3::new(-340.0, -180.0, 1.0),
    ] {
        commands.spawn(CollectibleBundle::new(position));
    }

    commands.spawn((
        Sprite::from_color(Color::srgba(0.12, 0.13, 0.16, 0.85), Vec2::new(170.0, 22.0)),
        Transform::from_translation(Vec3::new(-315.0, 290.0, 2.0)),
    ));

    commands.spawn((
        HealthBarFill,
        Sprite::from_color(Color::srgb(0.20, 0.85, 0.35), Vec2::new(160.0, 14.0)),
        Transform::from_translation(Vec3::new(-315.0, 290.0, 3.0)),
    ));

    commands.spawn((
        HealthText,
        Text2d::new(format!("Health: {MAX_HEALTH}/{MAX_HEALTH}")),
        TextFont::from_font_size(22.0),
        TextColor(Color::srgb(0.86, 0.91, 0.98)),
        Transform::from_translation(Vec3::new(-260.0, 258.0, 4.0)),
    ));

    commands.spawn((
        ScoreText,
        Text2d::new("Score: 0"),
        TextFont::from_font_size(28.0),
        TextColor(Color::srgb(1.0, 0.82, 0.25)),
        Transform::from_translation(Vec3::new(330.0, 290.0, 4.0)),
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

fn collect_items(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player: Single<(&Transform, &Body), With<Player>>,
    collectibles: Query<(Entity, &Transform, &Body), With<Collectible>>,
) {
    let (player_transform, player_body) = *player;

    for (entity, transform, body) in &collectibles {
        if overlaps(player_transform, player_body, transform, body) {
            commands.entity(entity).despawn();
            score.0 += 1;
            info!("score: {}", score.0);
        }
    }
}

fn enemy_hits_player(
    time: Res<Time>,
    player: Single<(&Transform, &Body, &mut Health), With<Player>>,
    enemies: Query<(&Transform, &Body), With<Enemy>>,
    mut hit_cooldown: Local<f32>,
) {
    *hit_cooldown -= time.delta_secs();

    if *hit_cooldown > 0.0 {
        return;
    }

    let (player_transform, player_body, mut health) = player.into_inner();

    for (enemy_transform, enemy_body) in &enemies {
        if overlaps(player_transform, player_body, enemy_transform, enemy_body) {
            health.current = (health.current - 1).max(0);
            *hit_cooldown = 1.0;
            info!("health: {}", health.current);
            break;
        }
    }
}

fn update_health_bar(
    player: Single<&Health, With<Player>>,
    mut bars: Query<(&mut Sprite, &mut Transform), With<HealthBarFill>>,
) {
    let health = *player;
    let health_fraction = health.current as f32 / health.max as f32;

    for (mut sprite, mut transform) in &mut bars {
        sprite.custom_size = Some(Vec2::new(160.0 * health_fraction, 14.0));
        transform.translation.x = -315.0 - (160.0 * (1.0 - health_fraction) / 2.0);
    }
}

fn update_hud_text(
    score: Res<Score>,
    player: Single<&Health, With<Player>>,
    mut health_text: Single<&mut Text2d, (With<HealthText>, Without<ScoreText>)>,
    mut score_text: Single<&mut Text2d, (With<ScoreText>, Without<HealthText>)>,
) {
    let health = *player;
    health_text.0 = format!("Health: {}/{}", health.current, health.max);
    score_text.0 = format!("Score: {}", score.0);
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

fn spawn_arena_frame(commands: &mut Commands) {
    let color = Color::srgb(0.26, 0.32, 0.40);
    let z = 0.5;
    let thickness = 8.0;
    let width = ARENA_HALF_SIZE.x * 2.0 + thickness;
    let height = ARENA_HALF_SIZE.y * 2.0 + thickness;

    for (position, size) in [
        (
            Vec3::new(0.0, ARENA_HALF_SIZE.y + thickness / 2.0, z),
            Vec2::new(width, thickness),
        ),
        (
            Vec3::new(0.0, -ARENA_HALF_SIZE.y - thickness / 2.0, z),
            Vec2::new(width, thickness),
        ),
        (
            Vec3::new(-ARENA_HALF_SIZE.x - thickness / 2.0, 0.0, z),
            Vec2::new(thickness, height),
        ),
        (
            Vec3::new(ARENA_HALF_SIZE.x + thickness / 2.0, 0.0, z),
            Vec2::new(thickness, height),
        ),
    ] {
        commands.spawn((
            Sprite::from_color(color, size),
            Transform::from_translation(position),
        ));
    }
}
