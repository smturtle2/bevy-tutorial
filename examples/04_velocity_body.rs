use bevy::prelude::*;
use bevy_tutorial::tutorial_capture::tutorial_capture_enabled;

#[derive(Component)]
struct Body;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Resource)]
struct BodySpeed(f32);

#[derive(Bundle)]
struct BodyBundle {
    body: Body,
    velocity: Velocity,
    transform: Transform,
}

impl BodyBundle {
    fn new(position: Vec3) -> Self {
        Self {
            body: Body,
            velocity: Velocity(Vec2::ZERO),
            transform: Transform::from_translation(position),
        }
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    body: BodyBundle,
    sprite: Sprite,
}

impl PlayerBundle {
    fn new() -> Self {
        Self {
            player: Player,
            body: BodyBundle::new(Vec3::ZERO),
            sprite: Sprite::from_color(Color::srgb(0.25, 0.70, 1.0), Vec2::splat(80.0)),
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.11)))
        .insert_resource(BodySpeed(220.0))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (handle_player_input, move_bodies, capture_velocity_body_pose).chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(PlayerBundle::new());

    if tutorial_capture_enabled() {
        for (position, color) in [
            (
                Vec3::new(-210.0, -130.0, -0.1),
                Color::srgba(0.25, 0.70, 1.0, 0.20),
            ),
            (
                Vec3::new(-70.0, -20.0, -0.1),
                Color::srgba(0.25, 0.70, 1.0, 0.32),
            ),
            (
                Vec3::new(80.0, 80.0, -0.1),
                Color::srgba(0.25, 0.70, 1.0, 0.45),
            ),
        ] {
            commands.spawn((
                Sprite::from_color(color, Vec2::splat(80.0)),
                Transform::from_translation(position),
            ));
        }

        commands.spawn((
            Text::new("Player input writes Velocity; Body movement writes Transform"),
            TextFont::from_font_size(25.0),
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

fn handle_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Velocity, With<Player>>,
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

    for mut velocity in &mut players {
        velocity.0 = direction.normalize_or_zero();
    }
}

fn move_bodies(
    time: Res<Time>,
    speed: Res<BodySpeed>,
    mut bodies: Query<(&mut Transform, &Velocity), With<Body>>,
) {
    let movement_scale = speed.0 * time.delta_secs();

    for (mut transform, velocity) in &mut bodies {
        transform.translation += (velocity.0 * movement_scale).extend(0.0);
    }
}

fn capture_velocity_body_pose(mut players: Query<(&mut Transform, &mut Velocity), With<Player>>) {
    if !tutorial_capture_enabled() {
        return;
    }

    for (mut transform, mut velocity) in &mut players {
        velocity.0 = Vec2::new(0.78, 0.62);
        transform.translation = Vec3::new(245.0, 155.0, 1.0);
    }
}
