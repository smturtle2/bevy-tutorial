use bevy::prelude::*;

const PLAYER_SPEED: f32 = 260.0;
const PLAYER_SCALE: f32 = 3.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum PlayerAnimState {
    Idle,
    Run,
    Attack,
}

#[derive(Component)]
struct PlayerAnimation {
    state: PlayerAnimState,
    frame_timer: Timer,
    attack_timer: Timer,
    run_frame: usize,
}

impl Default for PlayerAnimation {
    fn default() -> Self {
        Self {
            state: PlayerAnimState::Idle,
            frame_timer: Timer::from_seconds(0.14, TimerMode::Repeating),
            attack_timer: Timer::from_seconds(0.20, TimerMode::Once),
            run_frame: 1,
        }
    }
}

#[derive(Component)]
struct AnimationLabel;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (player_input, animate_player, update_label).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    let texture = asset_server.load("player_sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Player,
        Velocity(Vec2::ZERO),
        PlayerAnimation::default(),
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(PLAYER_SCALE)),
    ));

    commands.spawn((
        AnimationLabel,
        Text::new("Idle"),
        TextFont::from_font_size(24.0),
        TextColor(Color::srgb(0.92, 0.95, 1.0)),
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
    time: Res<Time>,
    player: Single<(&mut Transform, &mut Velocity, &mut PlayerAnimation), With<Player>>,
) {
    let (mut transform, mut velocity, mut animation) = player.into_inner();
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

    let normalized = direction.normalize_or_zero();
    velocity.0 = normalized * PLAYER_SPEED;
    transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);

    if keyboard.just_pressed(KeyCode::Space) {
        animation.state = PlayerAnimState::Attack;
        animation.attack_timer.reset();
    } else if animation.state != PlayerAnimState::Attack {
        animation.state = if normalized == Vec2::ZERO {
            PlayerAnimState::Idle
        } else {
            PlayerAnimState::Run
        };
    }
}

fn animate_player(
    time: Res<Time>,
    player: Single<(&mut Sprite, &Velocity, &mut PlayerAnimation), With<Player>>,
) {
    let (mut sprite, velocity, mut animation) = player.into_inner();
    let Some(atlas) = &mut sprite.texture_atlas else {
        return;
    };

    match animation.state {
        PlayerAnimState::Idle => {
            atlas.index = 0;
        }
        PlayerAnimState::Run => {
            animation.frame_timer.tick(time.delta());

            if animation.frame_timer.just_finished() {
                animation.run_frame = if animation.run_frame == 1 { 2 } else { 1 };
            }

            atlas.index = animation.run_frame;
        }
        PlayerAnimState::Attack => {
            atlas.index = 3;
            animation.attack_timer.tick(time.delta());

            if animation.attack_timer.is_finished() {
                animation.state = if velocity.0 == Vec2::ZERO {
                    PlayerAnimState::Idle
                } else {
                    PlayerAnimState::Run
                };
            }
        }
    }
}

fn update_label(
    player: Single<&PlayerAnimation, With<Player>>,
    mut label: Single<&mut Text, With<AnimationLabel>>,
) {
    label.0 = format!(
        "Animation state: {:?} | Move with WASD / arrows | Space attack",
        player.state
    );
}
