use std::time::Duration;

use bevy::prelude::*;
use bevy_tutorial::tutorial_capture::{add_tutorial_screenshot, tutorial_capture_enabled};

const PLAYER_SPEED: f32 = 260.0;
const PLAYER_SIZE: Vec2 = Vec2::splat(40.0);
const GEM_SIZE: Vec2 = Vec2::splat(28.0);

#[derive(Message, Debug, Clone, Copy)]
enum GameAudioEvent {
    Attack,
    Pickup,
    Hurt,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Gem;

#[derive(Component)]
struct Body {
    half_size: Vec2,
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
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_player,
                emit_attack_sound,
                collect_gems,
                emit_hurt_sound,
                play_audio_events,
                update_status_text,
            )
                .chain(),
        );

    add_tutorial_screenshot(&mut app, "assets/screenshots/ch21-audio-events.png", 20);
    app.run();
}

fn setup(mut commands: Commands, mut stats: ResMut<AudioStats>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player,
        Body {
            half_size: PLAYER_SIZE / 2.0,
        },
        Sprite::from_color(Color::srgb(0.25, 0.64, 1.0), PLAYER_SIZE),
        Transform::from_xyz(-260.0, -80.0, 2.0),
    ));

    for position in [
        Vec3::new(-60.0, 90.0, 2.0),
        Vec3::new(160.0, -70.0, 2.0),
        Vec3::new(300.0, 110.0, 2.0),
    ] {
        commands.spawn((
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
    }
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
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

    player.translation +=
        (direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs()).extend(0.0);
}

fn emit_attack_sound(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut audio_events: MessageWriter<GameAudioEvent>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        audio_events.write(GameAudioEvent::Attack);
    }
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

fn emit_hurt_sound(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut audio_events: MessageWriter<GameAudioEvent>,
) {
    if keyboard.just_pressed(KeyCode::KeyH) {
        audio_events.write(GameAudioEvent::Hurt);
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
    gems: Query<(), With<Gem>>,
    mut text: Single<&mut Text, With<StatusText>>,
) {
    text.0 = format!(
        "WASD move | Space attack sound | H hurt sound\npickup gems for pickup sound\nattack: {} | pickup: {} | hurt: {} | gems: {}",
        stats.attack_sounds,
        stats.pickup_sounds,
        stats.hurt_sounds,
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
