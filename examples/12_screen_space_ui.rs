use bevy::prelude::*;

const PLAYER_SPEED: f32 = 260.0;
const MAX_HEALTH: i32 = 5;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct HealthBarFill;

#[derive(Component)]
struct Health {
    current: i32,
    max: i32,
}

#[derive(Resource, Default)]
struct Score(u32);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .init_resource::<Score>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_player,
                follow_player_with_camera,
                debug_change_stats,
                update_screen_space_ui,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, MainCamera));

    commands.spawn((
        Player,
        Health {
            current: MAX_HEALTH,
            max: MAX_HEALTH,
        },
        Sprite::from_image(asset_server.load("player.png")),
        Transform::from_xyz(0.0, 0.0, 2.0),
    ));

    commands.spawn((
        Sprite::from_color(Color::srgb(0.12, 0.14, 0.18), Vec2::new(1300.0, 900.0)),
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));

    commands.spawn((
        Text::new("H/J health | Space score | UI is fixed to the screen"),
        TextFont::from_font_size(20.0),
        TextColor(Color::srgb(0.86, 0.90, 0.97)),
        Node {
            position_type: PositionType::Absolute,
            top: px(14),
            left: px(16),
            ..default()
        },
    ));

    commands.spawn((
        HealthText,
        Text::new("Health: 5/5"),
        TextFont::from_font_size(24.0),
        TextColor(Color::srgb(0.94, 0.97, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(48),
            left: px(16),
            ..default()
        },
    ));

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(82),
            left: px(16),
            width: px(200),
            height: px(16),
            ..default()
        },
        BackgroundColor(Color::srgb(0.22, 0.24, 0.30)),
    ));

    commands.spawn((
        HealthBarFill,
        Node {
            position_type: PositionType::Absolute,
            top: px(82),
            left: px(16),
            width: px(200),
            height: px(16),
            ..default()
        },
        BackgroundColor(Color::srgb(0.22, 0.84, 0.40)),
    ));

    commands.spawn((
        ScoreText,
        Text::new("Score: 0"),
        TextFont::from_font_size(24.0),
        TextColor(Color::srgb(1.0, 0.82, 0.30)),
        Node {
            position_type: PositionType::Absolute,
            top: px(108),
            left: px(16),
            ..default()
        },
    ));
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

fn follow_player_with_camera(
    player: Single<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}

fn debug_change_stats(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<Score>,
    mut player: Single<&mut Health, With<Player>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        score.0 += 1;
    }

    if keyboard.just_pressed(KeyCode::KeyH) {
        player.current = (player.current - 1).max(0);
    }

    if keyboard.just_pressed(KeyCode::KeyJ) {
        player.current = (player.current + 1).min(player.max);
    }
}

fn update_screen_space_ui(
    score: Res<Score>,
    player: Single<&Health, With<Player>>,
    mut health_text: Single<&mut Text, (With<HealthText>, Without<ScoreText>)>,
    mut score_text: Single<&mut Text, (With<ScoreText>, Without<HealthText>)>,
    mut health_bar: Single<&mut Node, With<HealthBarFill>>,
) {
    let health = *player;
    let health_fraction = health.current as f32 / health.max as f32;

    health_text.0 = format!("Health: {}/{}", health.current, health.max);
    score_text.0 = format!("Score: {}", score.0);
    health_bar.width = px(200.0 * health_fraction);
}
