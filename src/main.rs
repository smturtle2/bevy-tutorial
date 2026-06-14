use bevy::prelude::*;

mod body;
mod player;

use body::BodyPlugin;
use player::PlayerPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.11)))
            .configure_sets(Update, (GameSet::Input, GameSet::Movement).chain())
            .add_plugins(BodyPlugin)
            .add_plugins(PlayerPlugin)
            .add_systems(Startup, setup_camera);
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
}
