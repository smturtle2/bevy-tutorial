use bevy::prelude::*;
use bevy_tutorial::tutorial_capture::tutorial_capture_enabled;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
}

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

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.11)))
            .configure_sets(Update, (GameSet::Input, GameSet::Movement).chain())
            .add_plugins(BodyPlugin)
            .add_plugins(PlayerPlugin)
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                capture_plugin_scene
                    .in_set(GameSet::Movement)
                    .after(move_bodies),
            );
    }
}

struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BodySpeed(220.0))
            .add_systems(Update, move_bodies.in_set(GameSet::Movement));
    }
}

struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, handle_player_input.in_set(GameSet::Input));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);

    if tutorial_capture_enabled() {
        commands.spawn((
            Text::new(
                "GamePlugin registers BodyPlugin + PlayerPlugin\nSystem order: Input -> Movement",
            ),
            TextFont::from_font_size(26.0),
            TextColor(Color::srgb(0.92, 0.95, 1.0)),
            Node {
                position_type: PositionType::Absolute,
                top: px(18),
                left: px(22),
                ..default()
            },
        ));

        for (label, y, color) in [
            ("PlayerPlugin", 120.0, Color::srgb(0.25, 0.70, 1.0)),
            ("BodyPlugin", 0.0, Color::srgb(0.22, 0.84, 0.40)),
            ("GameSet::Movement", -120.0, Color::srgb(1.0, 0.82, 0.25)),
        ] {
            commands.spawn((
                Sprite::from_color(color, Vec2::new(260.0, 58.0)),
                Transform::from_xyz(-210.0, y, 0.0),
            ));
            commands.spawn((
                Text2d::new(label),
                TextFont::from_font_size(24.0),
                TextColor(Color::srgb(0.06, 0.07, 0.09)),
                Transform::from_xyz(-210.0, y - 8.0, 1.0),
            ));
        }
    }
}

fn spawn_player(mut commands: Commands) {
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

fn capture_plugin_scene(mut players: Query<(&mut Transform, &mut Velocity), With<Player>>) {
    if !tutorial_capture_enabled() {
        return;
    }

    for (mut transform, mut velocity) in &mut players {
        velocity.0 = Vec2::new(0.86, 0.28);
        transform.translation = Vec3::new(210.0, 130.0, 1.0);
    }
}
