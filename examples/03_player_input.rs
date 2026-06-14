use bevy::prelude::*;
use bevy_tutorial::tutorial_capture::tutorial_capture_enabled;

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PlayerSpeed(f32);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.11)))
        .insert_resource(PlayerSpeed(280.0))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, capture_player_pose).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Player,
        Sprite::from_color(Color::srgb(0.25, 0.70, 1.0), Vec2::splat(80.0)),
        Transform::from_translation(Vec3::ZERO),
    ));

    if tutorial_capture_enabled() {
        for (index, position) in [
            Vec3::new(-180.0, -100.0, -0.1),
            Vec3::new(-40.0, 0.0, -0.1),
            Vec3::new(110.0, 80.0, -0.1),
        ]
        .into_iter()
        .enumerate()
        {
            commands.spawn((
                Sprite::from_color(
                    Color::srgba(0.25, 0.70, 1.0, 0.20 + index as f32 * 0.12),
                    Vec2::splat(80.0),
                ),
                Transform::from_translation(position),
            ));
        }

        commands.spawn((
            Text::new("WASD / arrows -> Transform changes each frame"),
            TextFont::from_font_size(26.0),
            TextColor(Color::srgb(0.92, 0.95, 1.0)),
            Node {
                position_type: PositionType::Absolute,
                top: px(18),
                left: px(22),
                ..default()
            },
        ));
    }
}

fn move_player(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    speed: Res<PlayerSpeed>,
    mut players: Query<&mut Transform, With<Player>>,
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

    let movement = direction.normalize_or_zero() * speed.0 * time.delta_secs();

    for mut transform in &mut players {
        transform.translation += movement.extend(0.0);
    }
}

fn capture_player_pose(mut players: Query<&mut Transform, With<Player>>) {
    if !tutorial_capture_enabled() {
        return;
    }

    for mut transform in &mut players {
        transform.translation = Vec3::new(260.0, 150.0, 1.0);
    }
}
