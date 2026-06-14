use bevy::prelude::*;
use bevy_tutorial::tutorial_capture::{add_tutorial_screenshot, tutorial_capture_enabled};

const PLAYER_SPEED: f32 = 280.0;
const PROJECTILE_SPEED: f32 = 520.0;
const PROJECTILE_LIFETIME: f32 = 0.9;
const PLAYER_SIZE: Vec2 = Vec2::splat(38.0);
const ENEMY_SIZE: Vec2 = Vec2::splat(34.0);
const PROJECTILE_SIZE: Vec2 = Vec2::new(28.0, 12.0);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
    Collision,
    Ui,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Projectile {
    lifetime: Timer,
    damage: i32,
}

#[derive(Component)]
struct Body {
    half_size: Vec2,
}

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Facing(Vec2);

#[derive(Component)]
struct Health(i32);

#[derive(Component)]
struct StatusText;

#[derive(Resource, Default)]
struct ProjectileStats {
    fired: u32,
    hits: u32,
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
    facing: Facing,
    sprite: Sprite,
}

impl PlayerBundle {
    fn new() -> Self {
        Self {
            player: Player,
            body: BodyBundle::new(Vec3::new(-260.0, 0.0, 2.0), PLAYER_SIZE),
            facing: Facing(Vec2::X),
            sprite: Sprite::from_color(Color::srgb(0.25, 0.64, 1.0), PLAYER_SIZE),
        }
    }
}

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    body: BodyBundle,
    health: Health,
    sprite: Sprite,
}

impl EnemyBundle {
    fn new(position: Vec3) -> Self {
        Self {
            enemy: Enemy,
            body: BodyBundle::new(position, ENEMY_SIZE),
            health: Health(2),
            sprite: Sprite::from_color(Color::srgb(0.90, 0.24, 0.30), ENEMY_SIZE),
        }
    }
}

#[derive(Bundle)]
struct ProjectileBundle {
    projectile: Projectile,
    body: BodyBundle,
    sprite: Sprite,
}

impl ProjectileBundle {
    fn new(position: Vec3, direction: Vec2) -> Self {
        let angle = direction.y.atan2(direction.x);

        Self {
            projectile: Projectile {
                lifetime: Timer::from_seconds(PROJECTILE_LIFETIME, TimerMode::Once),
                damage: 1,
            },
            body: BodyBundle {
                body: Body {
                    half_size: PROJECTILE_SIZE / 2.0,
                },
                velocity: Velocity(direction * PROJECTILE_SPEED),
                transform: Transform {
                    translation: position,
                    rotation: Quat::from_rotation_z(angle),
                    ..default()
                },
            },
            sprite: Sprite::from_color(Color::srgb(1.0, 0.82, 0.28), PROJECTILE_SIZE),
        }
    }
}

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .init_resource::<ProjectileStats>()
        .add_plugins(DefaultPlugins)
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
            (player_input, fire_projectile)
                .chain()
                .in_set(GameSet::Input),
        )
        .add_systems(
            Update,
            (move_bodies, tick_projectile_lifetime)
                .chain()
                .in_set(GameSet::Movement),
        )
        .add_systems(Update, projectiles_hit_enemies.in_set(GameSet::Collision))
        .add_systems(Update, update_status_text.in_set(GameSet::Ui));

    add_tutorial_screenshot(&mut app, "assets/screenshots/ch18-projectiles.png", 20);
    app.run();
}

fn setup(mut commands: Commands, mut stats: ResMut<ProjectileStats>) {
    commands.spawn(Camera2d);
    commands.spawn(PlayerBundle::new());

    for position in [
        Vec3::new(120.0, -120.0, 2.0),
        Vec3::new(240.0, 30.0, 2.0),
        Vec3::new(360.0, 150.0, 2.0),
    ] {
        commands.spawn(EnemyBundle::new(position));
    }

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

    if tutorial_capture_enabled() {
        stats.fired = 1;
        commands.spawn((
            Projectile {
                lifetime: Timer::from_seconds(30.0, TimerMode::Once),
                damage: 1,
            },
            Body {
                half_size: PROJECTILE_SIZE / 2.0,
            },
            Velocity(Vec2::ZERO),
            Sprite::from_color(Color::srgb(1.0, 0.82, 0.28), PROJECTILE_SIZE),
            Transform::from_xyz(-60.0, 20.0, 3.0),
        ));
    }
}

fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Velocity, &mut Facing), With<Player>>,
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
    let (mut velocity, mut facing) = player.into_inner();
    velocity.0 = normalized * PLAYER_SPEED;

    if normalized != Vec2::ZERO {
        facing.0 = normalized;
    }
}

fn fire_projectile(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Single<(&Transform, &Facing), With<Player>>,
    mut stats: ResMut<ProjectileStats>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let (transform, facing) = *player;
    let start = transform.translation + (facing.0 * 34.0).extend(1.0);

    commands.spawn(ProjectileBundle::new(start, facing.0));
    stats.fired += 1;
}

fn move_bodies(time: Res<Time>, mut bodies: Query<(&mut Transform, &Velocity), With<Body>>) {
    for (mut transform, velocity) in &mut bodies {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
    }
}

fn tick_projectile_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(Entity, &mut Projectile)>,
) {
    for (entity, mut projectile) in &mut projectiles {
        projectile.lifetime.tick(time.delta());

        if projectile.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn projectiles_hit_enemies(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform, &Body, &Projectile)>,
    mut enemies: Query<(Entity, &Transform, &Body, &mut Health), With<Enemy>>,
    mut stats: ResMut<ProjectileStats>,
) {
    let mut defeated = Vec::new();

    for (projectile_entity, projectile_transform, projectile_body, projectile) in &projectiles {
        for (enemy_entity, enemy_transform, enemy_body, mut health) in &mut enemies {
            if defeated.contains(&enemy_entity) {
                continue;
            }

            if overlaps(
                projectile_transform,
                projectile_body,
                enemy_transform,
                enemy_body,
            ) {
                health.0 -= projectile.damage;
                commands.entity(projectile_entity).despawn();
                stats.hits += 1;

                if health.0 <= 0 {
                    commands.entity(enemy_entity).despawn();
                    defeated.push(enemy_entity);
                }

                break;
            }
        }
    }
}

fn update_status_text(
    stats: Res<ProjectileStats>,
    enemies: Query<(), With<Enemy>>,
    projectiles: Query<(), With<Projectile>>,
    mut text: Single<&mut Text, With<StatusText>>,
) {
    text.0 = format!(
        "WASD move | Space fire\nprojectiles: {} | fired: {} | hits: {} | enemies: {}",
        projectiles.iter().count(),
        stats.fired,
        stats.hits,
        enemies.iter().count()
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
