use bevy::prelude::*;

const PLAYER_SPEED: f32 = 300.0;
const CAMERA_SMOOTHNESS: f32 = 9.0;
const MAP_HALF_SIZE: Vec2 = Vec2::new(900.0, 600.0);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct CameraFollow {
    target: Entity,
    offset: Vec3,
    smoothness: f32,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    sprite: Sprite,
    transform: Transform,
}

impl PlayerBundle {
    fn new(asset_server: &AssetServer) -> Self {
        Self {
            player: Player,
            sprite: Sprite::from_image(asset_server.load("player.png")),
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, smooth_follow_camera).chain())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player = commands.spawn(PlayerBundle::new(&asset_server)).id();

    commands.spawn((
        Camera2d,
        Transform::from_xyz(-420.0, 260.0, 0.0),
        CameraFollow {
            target: player,
            offset: Vec3::new(0.0, 0.0, 0.0),
            smoothness: CAMERA_SMOOTHNESS,
        },
    ));

    spawn_map_reference(&mut commands);

    commands.spawn((
        Text::new("Move with WASD / arrows. The camera eases toward the player."),
        TextFont::from_font_size(20.0),
        TextColor(Color::srgb(0.88, 0.92, 0.98)),
        Node {
            position_type: PositionType::Absolute,
            top: px(14),
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
    player.translation.x = player
        .translation
        .x
        .clamp(-MAP_HALF_SIZE.x, MAP_HALF_SIZE.x);
    player.translation.y = player
        .translation
        .y
        .clamp(-MAP_HALF_SIZE.y, MAP_HALF_SIZE.y);
}

fn smooth_follow_camera(
    time: Res<Time>,
    targets: Query<&Transform, Without<Camera2d>>,
    mut cameras: Query<(&CameraFollow, &mut Transform), With<Camera2d>>,
) {
    for (follow, mut camera_transform) in &mut cameras {
        let Ok(target_transform) = targets.get(follow.target) else {
            continue;
        };

        let target = Vec3::new(
            target_transform.translation.x,
            target_transform.translation.y,
            camera_transform.translation.z,
        ) + follow.offset;
        let blend = 1.0 - (-follow.smoothness * time.delta_secs()).exp();

        camera_transform.translation = camera_transform.translation.lerp(target, blend);
    }
}

fn spawn_map_reference(commands: &mut Commands) {
    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.12, 0.14, 0.18),
            MAP_HALF_SIZE * 2.0 + Vec2::splat(80.0),
        ),
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));

    for x in (-900..=900).step_by(180) {
        commands.spawn((
            Sprite::from_color(Color::srgb(0.18, 0.21, 0.27), Vec2::new(3.0, 1280.0)),
            Transform::from_xyz(x as f32, 0.0, 0.0),
        ));
    }

    for y in (-540..=540).step_by(180) {
        commands.spawn((
            Sprite::from_color(Color::srgb(0.18, 0.21, 0.27), Vec2::new(1880.0, 3.0)),
            Transform::from_xyz(0.0, y as f32, 0.0),
        ));
    }
}
