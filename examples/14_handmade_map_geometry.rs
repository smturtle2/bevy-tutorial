use bevy::prelude::*;

const PLAYER_SPEED: f32 = 280.0;
const PLAYER_SIZE: Vec2 = Vec2::splat(42.0);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
    Collision,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Body {
    half_size: Vec2,
}

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Bundle)]
struct BodyBundle {
    body: Body,
    transform: Transform,
}

impl BodyBundle {
    fn new(position: Vec3, size: Vec2) -> Self {
        Self {
            body: Body {
                half_size: size / 2.0,
            },
            transform: Transform::from_translation(position),
        }
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    body: BodyBundle,
    velocity: Velocity,
    sprite: Sprite,
}

impl PlayerBundle {
    fn new(asset_server: &AssetServer) -> Self {
        Self {
            player: Player,
            body: BodyBundle::new(Vec3::new(-280.0, -160.0, 3.0), PLAYER_SIZE),
            velocity: Velocity(Vec2::ZERO),
            sprite: Sprite::from_image(asset_server.load("player.png")),
        }
    }
}

#[derive(Bundle)]
struct WallBundle {
    wall: Wall,
    body: BodyBundle,
    sprite: Sprite,
}

impl WallBundle {
    fn new(position: Vec3, size: Vec2) -> Self {
        Self {
            wall: Wall,
            body: BodyBundle::new(position, size),
            sprite: Sprite::from_color(Color::srgb(0.28, 0.33, 0.42), size),
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .configure_sets(
            Update,
            (GameSet::Input, GameSet::Movement, GameSet::Collision).chain(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, player_input.in_set(GameSet::Input))
        .add_systems(Update, move_player.in_set(GameSet::Movement))
        .add_systems(Update, resolve_wall_collisions.in_set(GameSet::Collision))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(PlayerBundle::new(&asset_server));
    spawn_floor(&mut commands);
    spawn_walls(&mut commands);

    commands.spawn((
        Text::new("Handmade map geometry: floor is visual, walls have Body + Wall"),
        TextFont::from_font_size(21.0),
        TextColor(Color::srgb(0.90, 0.94, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(16),
            left: px(16),
            ..default()
        },
    ));
}

fn player_input(
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
        velocity.0 = direction.normalize_or_zero() * PLAYER_SPEED;
    }
}

fn move_player(time: Res<Time>, mut players: Query<(&mut Transform, &Velocity), With<Player>>) {
    for (mut transform, velocity) in &mut players {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
    }
}

fn resolve_wall_collisions(
    mut bodies: Query<(&mut Transform, &Body, Option<&Player>, Option<&Wall>)>,
) {
    let mut walls = Vec::new();

    for (transform, body, _, wall) in &mut bodies {
        if wall.is_some() {
            walls.push((transform.translation.truncate(), body.half_size));
        }
    }

    for (mut player_transform, player_body, player, _) in &mut bodies {
        if player.is_none() {
            continue;
        }

        for (wall_position, wall_half_size) in &walls {
            let player_position = player_transform.translation.truncate();
            let delta = player_position - *wall_position;
            let overlap = player_body.half_size + *wall_half_size - delta.abs();

            if overlap.x <= 0.0 || overlap.y <= 0.0 {
                continue;
            }

            if overlap.x < overlap.y {
                player_transform.translation.x += overlap.x * delta.x.signum();
            } else {
                player_transform.translation.y += overlap.y * delta.y.signum();
            }
        }
    }
}

fn spawn_floor(commands: &mut Commands) {
    let tile_size = Vec2::splat(80.0);

    for x in -5..=5 {
        for y in -3..=3 {
            let color = if (x + y) % 2 == 0 {
                Color::srgb(0.13, 0.16, 0.20)
            } else {
                Color::srgb(0.15, 0.18, 0.23)
            };

            commands.spawn((
                Sprite::from_color(color, tile_size - Vec2::splat(2.0)),
                Transform::from_xyz(x as f32 * tile_size.x, y as f32 * tile_size.y, 0.0),
            ));
        }
    }
}

fn spawn_walls(commands: &mut Commands) {
    for (position, size) in [
        (Vec3::new(0.0, 300.0, 2.0), Vec2::new(900.0, 40.0)),
        (Vec3::new(0.0, -300.0, 2.0), Vec2::new(900.0, 40.0)),
        (Vec3::new(-460.0, 0.0, 2.0), Vec2::new(40.0, 640.0)),
        (Vec3::new(460.0, 0.0, 2.0), Vec2::new(40.0, 640.0)),
        (Vec3::new(-130.0, 80.0, 2.0), Vec2::new(260.0, 36.0)),
        (Vec3::new(210.0, -120.0, 2.0), Vec2::new(260.0, 36.0)),
        (Vec3::new(90.0, 140.0, 2.0), Vec2::new(36.0, 210.0)),
    ] {
        commands.spawn(WallBundle::new(position, size));
    }
}
