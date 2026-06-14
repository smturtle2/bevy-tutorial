use bevy::prelude::*;
use bevy_tutorial::tutorial_capture::tutorial_capture_enabled;

const PLAYER_SPEED: f32 = 280.0;
const PLAYER_SIZE: Vec2 = Vec2::splat(42.0);
const ENEMY_SIZE: Vec2 = Vec2::splat(36.0);
const HITBOX_SIZE: Vec2 = Vec2::new(58.0, 34.0);
const HITBOX_DISTANCE: f32 = 48.0;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
    Combat,
    Ui,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

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
}

#[derive(Component)]
struct AttackHitbox {
    lifetime: Timer,
    damage: i32,
}

#[derive(Component)]
struct StatusText;

#[derive(Resource, Default)]
struct HitCount(u32);

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
    fn new(asset_server: &AssetServer) -> Self {
        Self {
            player: Player,
            body: BodyBundle::new(Vec3::new(0.0, 0.0, 2.0), PLAYER_SIZE),
            facing: Facing(Vec2::X),
            sprite: Sprite::from_image(asset_server.load("player.png")),
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
    fn new(position: Vec3, asset_server: &AssetServer) -> Self {
        Self {
            enemy: Enemy,
            body: BodyBundle::new(position, ENEMY_SIZE),
            health: Health { current: 2 },
            sprite: Sprite::from_image(asset_server.load("enemy.png")),
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .init_resource::<HitCount>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .configure_sets(
            Update,
            (
                GameSet::Input,
                GameSet::Movement,
                GameSet::Combat,
                GameSet::Ui,
            )
                .chain(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, player_input.in_set(GameSet::Input))
        .add_systems(Update, setup_capture_attack_scene.in_set(GameSet::Input))
        .add_systems(Update, spawn_attack_hitbox.in_set(GameSet::Input))
        .add_systems(Update, move_bodies.in_set(GameSet::Movement))
        .add_systems(
            Update,
            (attack_hits_enemies, expire_attack_hitboxes)
                .chain()
                .in_set(GameSet::Combat),
        )
        .add_systems(Update, update_status_text.in_set(GameSet::Ui))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(PlayerBundle::new(&asset_server));

    for position in [
        Vec3::new(170.0, 90.0, 2.0),
        Vec3::new(230.0, -100.0, 2.0),
        Vec3::new(-180.0, -70.0, 2.0),
    ] {
        commands.spawn(EnemyBundle::new(position, &asset_server));
    }

    commands.spawn((
        StatusText,
        Text::new("Space: attack | Hits: 0"),
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

fn setup_capture_attack_scene(
    mut done: Local<bool>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Single<(&mut Transform, &mut Facing), With<Player>>,
    mut enemies: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    if *done || !tutorial_capture_enabled() {
        return;
    }
    *done = true;

    let (mut player_transform, mut facing) = player.into_inner();
    player_transform.translation = Vec3::new(-165.0, 0.0, 2.0);
    facing.0 = Vec2::X;

    for (index, mut enemy_transform) in enemies.iter_mut().enumerate() {
        enemy_transform.translation = match index {
            0 => Vec3::new(-28.0, 0.0, 2.0),
            1 => Vec3::new(150.0, 92.0, 2.0),
            _ => Vec3::new(150.0, -92.0, 2.0),
        };
    }

    let position = player_transform.translation + (facing.0 * HITBOX_DISTANCE).extend(1.0);
    commands.spawn((
        Sprite::from_color(Color::srgba(1.0, 0.82, 0.25, 0.34), HITBOX_SIZE),
        Transform::from_xyz(position.x, position.y, 1.4),
    ));
    commands.spawn((
        AttackHitbox {
            lifetime: Timer::from_seconds(30.0, TimerMode::Once),
            damage: 0,
        },
        Body {
            half_size: HITBOX_SIZE / 2.0,
        },
        Sprite::from_image(asset_server.load("slash.png")),
        Transform::from_xyz(position.x, position.y, 3.0),
    ));
    commands.spawn((
        Text::new("Capture: slash sprite + AttackHitbox Body in front of the player"),
        TextFont::from_font_size(22.0),
        TextColor(Color::srgb(0.92, 0.95, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(50),
            left: px(16),
            ..default()
        },
    ));
}

fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&mut Velocity, &mut Facing), With<Player>>,
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

    for (mut velocity, mut facing) in &mut players {
        let normalized = direction.normalize_or_zero();
        velocity.0 = normalized * PLAYER_SPEED;

        if normalized != Vec2::ZERO {
            facing.0 = normalized;
        }
    }
}

fn spawn_attack_hitbox(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    player: Single<(&Transform, &Facing), With<Player>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let (player_transform, facing) = *player;
    let position = player_transform.translation + (facing.0 * HITBOX_DISTANCE).extend(1.0);
    let angle = facing.0.y.atan2(facing.0.x);

    commands.spawn((
        AttackHitbox {
            lifetime: Timer::from_seconds(0.14, TimerMode::Once),
            damage: 1,
        },
        Body {
            half_size: HITBOX_SIZE / 2.0,
        },
        Sprite::from_image(asset_server.load("slash.png")),
        Transform {
            translation: position,
            rotation: Quat::from_rotation_z(angle),
            ..default()
        },
    ));
}

fn move_bodies(time: Res<Time>, mut bodies: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut bodies {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
    }
}

fn attack_hits_enemies(
    mut commands: Commands,
    mut hit_count: ResMut<HitCount>,
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
                hit_count.0 += 1;
                hit_anything = true;

                if health.current <= 0 {
                    commands.entity(enemy_entity).despawn();
                    defeated_enemies.push(enemy_entity);
                }
            }
        }

        if hit_anything {
            commands.entity(hitbox_entity).despawn();
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

fn update_status_text(hit_count: Res<HitCount>, mut text: Single<&mut Text, With<StatusText>>) {
    text.0 = format!("Space: attack | Hits: {}", hit_count.0);
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
