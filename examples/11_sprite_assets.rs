use bevy::prelude::*;

const PLAYER_SPEED: f32 = 260.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct AssetLabel;

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
            transform: Transform::from_xyz(0.0, -60.0, 2.0),
        }
    }
}

#[derive(Bundle)]
struct DisplaySpriteBundle {
    sprite: Sprite,
    transform: Transform,
}

impl DisplaySpriteBundle {
    fn new(path: &'static str, position: Vec3, asset_server: &AssetServer) -> Self {
        Self {
            sprite: Sprite::from_image(asset_server.load(path)),
            transform: Transform::from_translation(position),
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, update_asset_label).chain())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(PlayerBundle::new(&asset_server));

    for (path, x) in [("enemy.png", -160.0), ("gem.png", 160.0)] {
        commands.spawn(DisplaySpriteBundle::new(
            path,
            Vec3::new(x, 100.0, 2.0),
            &asset_server,
        ));
    }

    commands.spawn((
        AssetLabel,
        Text::new("Loaded: player.png, enemy.png, gem.png"),
        TextFont::from_font_size(23.0),
        TextColor(Color::srgb(0.92, 0.95, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(16),
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

fn update_asset_label(
    player: Single<&Transform, With<Player>>,
    mut text: Single<&mut Text, With<AssetLabel>>,
) {
    text.0 = format!(
        "Loaded assets stay attached to entities | player: {:.0}, {:.0}",
        player.translation.x, player.translation.y
    );
}
