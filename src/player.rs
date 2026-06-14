use bevy::prelude::*;

use crate::GameSet;
use crate::body::{BodyBundle, Velocity};

#[derive(Component)]
struct Player;

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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, handle_player_input.in_set(GameSet::Input));
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
