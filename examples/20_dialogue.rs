use bevy::prelude::*;
use bevy_tutorial::tutorial_capture::{add_tutorial_screenshot, tutorial_capture_enabled};

const PLAYER_SPEED: f32 = 250.0;
const PLAYER_SIZE: Vec2 = Vec2::splat(40.0);
const NPC_SIZE: Vec2 = Vec2::splat(42.0);
const INTERACT_DISTANCE: f32 = 82.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Npc {
    name: &'static str,
    lines: &'static [&'static str],
}

#[derive(Component)]
struct PromptText;

#[derive(Component)]
struct DialogueText;

#[derive(Resource, Default)]
struct DialogueState {
    active_npc: Option<Entity>,
    line_index: usize,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .init_resource::<DialogueState>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_player,
                update_prompt_text,
                dialogue_input,
                update_dialogue_text,
            )
                .chain(),
        );

    add_tutorial_screenshot(&mut app, "assets/screenshots/ch20-dialogue.png", 20);
    app.run();
}

fn setup(mut commands: Commands, mut dialogue: ResMut<DialogueState>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Player,
        Sprite::from_color(Color::srgb(0.25, 0.64, 1.0), PLAYER_SIZE),
        Transform::from_xyz(-260.0, -40.0, 2.0),
    ));

    let mut first_npc = None;

    for (position, name, lines) in [
        (
            Vec3::new(80.0, 80.0, 2.0),
            "Mapper",
            &[
                "A scene file can decide where I stand.",
                "Code decides what talking to me means.",
            ][..],
        ),
        (
            Vec3::new(260.0, -120.0, 2.0),
            "Smith",
            &[
                "Keep dialogue state out of the NPC component.",
                "The NPC owns lines; the resource owns the current conversation.",
            ][..],
        ),
    ] {
        let entity = commands
            .spawn((
                Npc { name, lines },
                Sprite::from_color(Color::srgb(0.95, 0.68, 0.30), NPC_SIZE),
                Transform::from_translation(position),
            ))
            .id();

        first_npc.get_or_insert(entity);
    }

    commands.spawn((
        PromptText,
        Text::new(""),
        TextFont::from_font_size(22.0),
        TextColor(Color::srgb(0.92, 0.95, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(18),
            left: px(18),
            ..default()
        },
    ));

    commands.spawn((
        DialogueText,
        Text::new(""),
        TextFont::from_font_size(24.0),
        TextColor(Color::srgb(1.0, 0.92, 0.62)),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(28),
            left: px(32),
            right: px(32),
            padding: UiRect::all(px(14)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.06, 0.07, 0.10, 0.88)),
    ));

    if tutorial_capture_enabled() {
        dialogue.active_npc = first_npc;
        dialogue.line_index = 0;
    }
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    dialogue: Res<DialogueState>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    if dialogue.active_npc.is_some() {
        return;
    }

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

fn update_prompt_text(
    player: Single<&Transform, With<Player>>,
    npcs: Query<(&Transform, &Npc)>,
    dialogue: Res<DialogueState>,
    mut text: Single<&mut Text, With<PromptText>>,
) {
    if dialogue.active_npc.is_some() {
        text.0 = "Space: next line | Esc: close dialogue".to_string();
        return;
    }

    let nearby = npcs.iter().find(|(transform, _)| {
        player
            .translation
            .truncate()
            .distance(transform.translation.truncate())
            <= INTERACT_DISTANCE
    });

    text.0 = match nearby {
        Some((_, npc)) => format!("WASD move | E: talk to {}", npc.name),
        None => "WASD move | stand near an NPC".to_string(),
    };
}

fn dialogue_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Single<&Transform, With<Player>>,
    npcs: Query<(Entity, &Transform, &Npc)>,
    mut dialogue: ResMut<DialogueState>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        dialogue.active_npc = None;
        dialogue.line_index = 0;
        return;
    }

    if let Some(active_npc) = dialogue.active_npc {
        if keyboard.just_pressed(KeyCode::Space) {
            let Ok((_, _, npc)) = npcs.get(active_npc) else {
                dialogue.active_npc = None;
                dialogue.line_index = 0;
                return;
            };

            dialogue.line_index += 1;

            if dialogue.line_index >= npc.lines.len() {
                dialogue.active_npc = None;
                dialogue.line_index = 0;
            }
        }

        return;
    }

    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    let nearest = npcs.iter().find(|(_, transform, _)| {
        player
            .translation
            .truncate()
            .distance(transform.translation.truncate())
            <= INTERACT_DISTANCE
    });

    if let Some((entity, _, _)) = nearest {
        dialogue.active_npc = Some(entity);
        dialogue.line_index = 0;
    }
}

fn update_dialogue_text(
    dialogue: Res<DialogueState>,
    npcs: Query<&Npc>,
    mut text: Single<&mut Text, With<DialogueText>>,
) {
    let Some(entity) = dialogue.active_npc else {
        text.0.clear();
        return;
    };

    let Ok(npc) = npcs.get(entity) else {
        text.0.clear();
        return;
    };

    let line = npc.lines.get(dialogue.line_index).copied().unwrap_or("");
    text.0 = format!("{}:\n{}\n\nSpace: continue | Esc: close", npc.name, line);
}
