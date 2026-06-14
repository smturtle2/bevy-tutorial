use bevy::prelude::*;

const PLAYER_SPEED: f32 = 260.0;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    GameOver,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct GameplayEntity;

#[derive(Component)]
struct MenuUi;

#[derive(Component)]
struct PauseUi;

#[derive(Component)]
struct GameOverUi;

#[derive(Component)]
struct Health(i32);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::Menu), spawn_menu)
        .add_systems(OnExit(GameState::Menu), cleanup_entities::<MenuUi>)
        .add_systems(OnEnter(GameState::Paused), spawn_pause_ui)
        .add_systems(OnExit(GameState::Paused), cleanup_entities::<PauseUi>)
        .add_systems(OnEnter(GameState::GameOver), enter_game_over)
        .add_systems(OnExit(GameState::GameOver), cleanup_entities::<GameOverUi>)
        .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)))
        .add_systems(Update, playing_input.run_if(in_state(GameState::Playing)))
        .add_systems(
            Update,
            (move_player, debug_take_damage, game_over_when_dead)
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(Update, paused_input.run_if(in_state(GameState::Paused)))
        .add_systems(
            Update,
            game_over_input.run_if(in_state(GameState::GameOver)),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_menu(mut commands: Commands) {
    commands.spawn((
        MenuUi,
        Text::new("MENU\nEnter: start"),
        TextFont::from_font_size(42.0),
        TextColor(Color::srgb(0.92, 0.95, 1.0)),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: percent(40),
            left: percent(40),
            ..default()
        },
    ));
}

fn menu_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        spawn_gameplay(&mut commands, &asset_server);
        next_state.set(GameState::Playing);
    }
}

fn playing_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        next_state.set(GameState::Paused);
    }
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

fn debug_take_damage(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut health: Single<&mut Health, With<Player>>,
) {
    if keyboard.just_pressed(KeyCode::KeyH) {
        health.0 -= 1;
        info!("health: {}", health.0);
    }
}

fn game_over_when_dead(
    health: Single<&Health, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if health.0 <= 0 {
        next_state.set(GameState::GameOver);
    }
}

fn spawn_pause_ui(mut commands: Commands) {
    commands.spawn((
        PauseUi,
        Text::new("PAUSED\nP: resume | Esc: menu"),
        TextFont::from_font_size(38.0),
        TextColor(Color::srgb(1.0, 0.90, 0.50)),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: percent(40),
            left: percent(34),
            ..default()
        },
    ));
}

fn paused_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    gameplay: Query<Entity, With<GameplayEntity>>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        next_state.set(GameState::Playing);
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        for entity in &gameplay {
            commands.entity(entity).despawn();
        }
        next_state.set(GameState::Menu);
    }
}

fn enter_game_over(mut commands: Commands, gameplay: Query<Entity, With<GameplayEntity>>) {
    for entity in &gameplay {
        commands.entity(entity).despawn();
    }

    commands.spawn((
        GameOverUi,
        Text::new("GAME OVER\nR: restart | Esc: menu"),
        TextFont::from_font_size(38.0),
        TextColor(Color::srgb(1.0, 0.42, 0.42)),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: percent(40),
            left: percent(34),
            ..default()
        },
    ));
}

fn game_over_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        spawn_gameplay(&mut commands, &asset_server);
        next_state.set(GameState::Playing);
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}

fn spawn_gameplay(commands: &mut Commands, asset_server: &AssetServer) {
    commands.spawn((
        GameplayEntity,
        Player,
        Health(3),
        Sprite::from_image(asset_server.load("player.png")),
        Transform::from_xyz(0.0, 0.0, 2.0),
    ));

    commands.spawn((
        GameplayEntity,
        Text::new("Playing | WASD move | H damage | P pause"),
        TextFont::from_font_size(22.0),
        TextColor(Color::srgb(0.90, 0.94, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(16),
            left: px(16),
            ..default()
        },
    ));
}

fn cleanup_entities<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
