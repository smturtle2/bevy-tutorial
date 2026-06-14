use bevy::prelude::*;

const PLAYER_SPEED: f32 = 240.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct HudText;

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
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.11)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_player,
                follow_player,
                update_hud_text,
                position_hud_text,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(PlayerBundle::new(&asset_server));

    commands.spawn((
        Sprite::from_color(Color::srgb(0.18, 0.22, 0.28), Vec2::new(900.0, 540.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        HudText,
        Text2d::new("Position: 0, 0"),
        TextFont::from_font_size(24.0),
        TextColor(Color::srgb(0.86, 0.91, 0.98)),
        Transform::from_xyz(0.0, 230.0, 2.0),
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

fn follow_player(
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}

fn update_hud_text(
    player: Single<&Transform, With<Player>>,
    mut hud: Single<&mut Text2d, With<HudText>>,
) {
    hud.0 = format!(
        "Position: {:.0}, {:.0}",
        player.translation.x, player.translation.y
    );
}

fn position_hud_text(
    player: Single<&Transform, (With<Player>, Without<HudText>)>,
    mut hud: Single<&mut Transform, (With<HudText>, Without<Player>)>,
) {
    hud.translation.x = player.translation.x;
    hud.translation.y = player.translation.y + 230.0;
}
