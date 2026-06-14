use std::time::Duration;

use bevy::prelude::*;
use bevy_tutorial::tutorial_capture::{add_tutorial_screenshot, tutorial_capture_enabled};

const PLAYER_SPEED: f32 = 260.0;
const PLAYER_SIZE: Vec2 = Vec2::splat(40.0);
const ENEMY_SIZE: Vec2 = Vec2::splat(34.0);
const GEM_SIZE: Vec2 = Vec2::splat(28.0);
const HITBOX_SIZE: Vec2 = Vec2::new(58.0, 34.0);
const HITBOX_DISTANCE: f32 = 44.0;

#[derive(Message, Debug, Clone, Copy)]
enum GameAudioEvent {
    Attack,
    Pickup,
    Hurt,
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Playing,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Collision,
    Ui,
}

#[derive(Component)]
struct GameplayEntity;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Gem;

#[derive(Component)]
struct AttackHitbox {
    lifetime: Timer,
    damage: i32,
}

#[derive(Component)]
struct Body {
    half_size: Vec2,
}

#[derive(Component)]
struct Facing(Vec2);

#[derive(Component)]
struct Health {
    current: i32,
    max: i32,
}

#[derive(Component)]
struct StatusText;

#[derive(Resource, Default)]
struct AudioStats {
    attack_sounds: u32,
    pickup_sounds: u32,
    hurt_sounds: u32,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .init_resource::<AudioStats>()
        .add_message::<GameAudioEvent>()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .configure_sets(
            Update,
            (GameSet::Input, GameSet::Collision, GameSet::Ui).chain(),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (move_player, spawn_attack_hitbox)
                .chain()
                .in_set(GameSet::Input)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (collect_gems, attack_hits_enemies, enemy_hits_player)
                .chain()
                .in_set(GameSet::Collision)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (
                expire_attack_hitboxes,
                play_audio_events,
                update_status_text,
            )
                .chain()
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::Playing)),
        );

    add_tutorial_screenshot(&mut app, "assets/screenshots/ch21-audio-events.png", 20);
    app.run();
}

fn setup(mut commands: Commands, mut stats: ResMut<AudioStats>) {
    commands.spawn(Camera2d);
    commands.spawn((
        GameplayEntity,
        Player,
        Facing(Vec2::X),
        Body {
            half_size: PLAYER_SIZE / 2.0,
        },
        Health { current: 5, max: 5 },
        Sprite::from_color(Color::srgb(0.25, 0.64, 1.0), PLAYER_SIZE),
        Transform::from_xyz(-260.0, -80.0, 2.0),
    ));

    commands.spawn((
        GameplayEntity,
        Enemy,
        Body {
            half_size: ENEMY_SIZE / 2.0,
        },
        Health { current: 2, max: 2 },
        Sprite::from_color(Color::srgb(0.90, 0.24, 0.30), ENEMY_SIZE),
        Transform::from_xyz(120.0, -60.0, 2.0),
    ));

    for position in [
        Vec3::new(-60.0, 90.0, 2.0),
        Vec3::new(160.0, -70.0, 2.0),
        Vec3::new(300.0, 110.0, 2.0),
    ] {
        commands.spawn((
            GameplayEntity,
            Gem,
            Body {
                half_size: GEM_SIZE / 2.0,
            },
            Sprite::from_color(Color::srgb(0.18, 0.88, 0.76), GEM_SIZE),
            Transform::from_translation(position),
        ));
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
        stats.attack_sounds = 1;
        stats.pickup_sounds = 2;
        stats.hurt_sounds = 1;
        commands.spawn((
            GameplayEntity,
            AttackHitbox {
                lifetime: Timer::from_seconds(30.0, TimerMode::Once),
                damage: 0,
            },
            Body {
                half_size: HITBOX_SIZE / 2.0,
            },
            Sprite::from_color(Color::srgb(1.0, 0.46, 0.28), HITBOX_SIZE),
            Transform::from_xyz(-110.0, -80.0, 3.0),
        ));
    }
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Single<(&mut Transform, &mut Facing), With<Player>>,
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
    let (mut transform, mut facing) = player.into_inner();
    transform.translation += (normalized * PLAYER_SPEED * time.delta_secs()).extend(0.0);

    if normalized != Vec2::ZERO {
        facing.0 = normalized;
    }
}

fn spawn_attack_hitbox(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Single<(&Transform, &Facing), With<Player>>,
    mut audio_events: MessageWriter<GameAudioEvent>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let (transform, facing) = *player;
    let position = transform.translation + (facing.0 * HITBOX_DISTANCE).extend(1.0);

    commands.spawn((
        GameplayEntity,
        AttackHitbox {
            lifetime: Timer::from_seconds(0.14, TimerMode::Once),
            damage: 1,
        },
        Body {
            half_size: HITBOX_SIZE / 2.0,
        },
        Sprite::from_color(Color::srgb(1.0, 0.46, 0.28), HITBOX_SIZE),
        Transform {
            translation: position,
            rotation: Quat::from_rotation_z(facing.0.y.atan2(facing.0.x)),
            ..default()
        },
    ));
    audio_events.write(GameAudioEvent::Attack);
}

fn collect_gems(
    mut commands: Commands,
    player: Single<(&Transform, &Body), With<Player>>,
    gems: Query<(Entity, &Transform, &Body), With<Gem>>,
    mut audio_events: MessageWriter<GameAudioEvent>,
) {
    let (player_transform, player_body) = *player;

    for (entity, gem_transform, gem_body) in &gems {
        if overlaps(player_transform, player_body, gem_transform, gem_body) {
            commands.entity(entity).despawn();
            audio_events.write(GameAudioEvent::Pickup);
        }
    }
}

fn attack_hits_enemies(
    mut commands: Commands,
    hitboxes: Query<(Entity, &Transform, &Body, &AttackHitbox)>,
    mut enemies: Query<(Entity, &Transform, &Body, &mut Health), With<Enemy>>,
) {
    for (hitbox_entity, hitbox_transform, hitbox_body, hitbox) in &hitboxes {
        for (enemy_entity, enemy_transform, enemy_body, mut health) in &mut enemies {
            if overlaps(hitbox_transform, hitbox_body, enemy_transform, enemy_body) {
                health.current -= hitbox.damage;
                commands.entity(hitbox_entity).despawn();

                if health.current <= 0 {
                    commands.entity(enemy_entity).despawn();
                }

                break;
            }
        }
    }
}

fn enemy_hits_player(
    time: Res<Time>,
    enemies: Query<(&Transform, &Body), With<Enemy>>,
    player: Single<(&Transform, &Body, &mut Health), With<Player>>,
    mut cooldown: Local<f32>,
    mut audio_events: MessageWriter<GameAudioEvent>,
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
            audio_events.write(GameAudioEvent::Hurt);
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

fn play_audio_events(
    mut commands: Commands,
    mut pitch_assets: ResMut<Assets<Pitch>>,
    mut events: MessageReader<GameAudioEvent>,
    mut stats: ResMut<AudioStats>,
) {
    for event in events.read() {
        let frequency = match event {
            GameAudioEvent::Attack => {
                stats.attack_sounds += 1;
                360.0
            }
            GameAudioEvent::Pickup => {
                stats.pickup_sounds += 1;
                720.0
            }
            GameAudioEvent::Hurt => {
                stats.hurt_sounds += 1;
                180.0
            }
        };

        commands.spawn((
            AudioPlayer(pitch_assets.add(Pitch::new(frequency, Duration::from_millis(120)))),
            PlaybackSettings::DESPAWN,
        ));
    }
}

fn update_status_text(
    stats: Res<AudioStats>,
    player: Single<&Health, With<Player>>,
    enemies: Query<&Health, With<Enemy>>,
    gems: Query<(), With<Gem>>,
    mut text: Single<&mut Text, With<StatusText>>,
) {
    let player_health = player.into_inner();
    let enemy_health = enemies
        .iter()
        .map(|health| format!("{}/{}", health.current, health.max))
        .collect::<Vec<_>>()
        .join(", ");

    text.0 = format!(
        "WASD move | Space attack | touch gems/enemy\nattack sounds: {} | pickup sounds: {} | hurt sounds: {}\nplayer: {}/{} | enemy: {} | gems: {}",
        stats.attack_sounds,
        stats.pickup_sounds,
        stats.hurt_sounds,
        player_health.current,
        player_health.max,
        if enemy_health.is_empty() {
            "cleared".to_string()
        } else {
            enemy_health
        },
        gems.iter().count()
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
