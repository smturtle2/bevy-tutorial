use bevy::prelude::*;

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
        .add_systems(Update, (handle_player_input, move_bodies).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(PlayerBundle::new());
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
