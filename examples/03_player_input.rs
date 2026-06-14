use bevy::prelude::*;

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
        .add_systems(Update, move_player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Player,
        Sprite::from_color(Color::srgb(0.25, 0.70, 1.0), Vec2::splat(80.0)),
        Transform::from_translation(Vec3::ZERO),
    ));
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
