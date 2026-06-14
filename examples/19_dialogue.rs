use bevy::prelude::*;
use bevy_tutorial::tutorial_capture::{add_tutorial_screenshot, tutorial_capture_enabled};

mod tutorial_visuals;
use tutorial_visuals::{
    TutorialSprites, npc_sprite, player_sprite, spawn_arena_backdrop, spawn_camera,
    spawn_dialogue_panel, spawn_status_panel, spawn_world_label,
};

const PLAYER_SPEED: f32 = 250.0;
const INTERACT_DISTANCE: f32 = 82.0;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Playing,
    Dialogue,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Ui,
}

#[derive(Component)]
struct GameplayEntity;

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
        .init_state::<GameState>()
        .configure_sets(Update, (GameSet::Input, GameSet::Ui).chain())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (move_player, dialogue_input).chain().in_set(GameSet::Input),
        )
        .add_systems(
            Update,
            (
                start_capture_dialogue,
                update_prompt_text,
                update_dialogue_text,
            )
                .chain()
                .in_set(GameSet::Ui),
        );

    add_tutorial_screenshot(&mut app, "assets/screenshots/ch19-dialogue.png", 20);
    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut dialogue: ResMut<DialogueState>,
) {
    spawn_camera(&mut commands);
    spawn_arena_backdrop(&mut commands);

    let assets = TutorialSprites::load(&asset_server, &mut texture_atlas_layouts);
    commands.insert_resource(assets.clone());

    commands.spawn((
        GameplayEntity,
        Player,
        player_sprite(&assets),
        Transform::from_xyz(18.0, 74.0, 3.0),
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
                GameplayEntity,
                Npc { name, lines },
                npc_sprite(&assets),
                Transform::from_translation(position),
            ))
            .id();

        first_npc.get_or_insert(entity);
        spawn_world_label(
            &mut commands,
            name,
            Vec3::new(position.x, position.y + 42.0, 4.0),
        );
    }

    spawn_status_panel(
        &mut commands,
        PromptText,
        "Dialogue uses GameState plus a DialogueState resource",
        600.0,
    );
    spawn_dialogue_panel(&mut commands, DialogueText);

    if tutorial_capture_enabled() {
        dialogue.active_npc = first_npc;
        dialogue.line_index = 0;
        spawn_world_label(
            &mut commands,
            "player is inside interact range",
            Vec3::new(58.0, 18.0, 4.0),
        );
    }
}

fn start_capture_dialogue(
    mut done: Local<bool>,
    dialogue: Res<DialogueState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *done || !tutorial_capture_enabled() || dialogue.active_npc.is_none() {
        return;
    }

    *done = true;
    next_state.set(GameState::Dialogue);
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    state: Res<State<GameState>>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    if *state.get() != GameState::Playing {
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
    state: Res<State<GameState>>,
    mut text: Single<&mut Text, With<PromptText>>,
) {
    if *state.get() == GameState::Dialogue {
        text.0 = "Dialogue state | Space: next line | Esc: close".to_string();
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
        Some((_, npc)) => format!("Playing state | WASD move | E: talk to {}", npc.name),
        None => "Playing state | WASD move | stand near an NPC".to_string(),
    };
}

fn dialogue_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Single<&Transform, With<Player>>,
    npcs: Query<(Entity, &Transform, &Npc)>,
    mut dialogue: ResMut<DialogueState>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        dialogue.active_npc = None;
        dialogue.line_index = 0;
        next_state.set(GameState::Playing);
        return;
    }

    if *state.get() == GameState::Dialogue {
        if keyboard.just_pressed(KeyCode::Space) {
            let Some(active_npc) = dialogue.active_npc else {
                next_state.set(GameState::Playing);
                return;
            };

            let Ok((_, _, npc)) = npcs.get(active_npc) else {
                dialogue.active_npc = None;
                dialogue.line_index = 0;
                next_state.set(GameState::Playing);
                return;
            };

            dialogue.line_index += 1;

            if dialogue.line_index >= npc.lines.len() {
                dialogue.active_npc = None;
                dialogue.line_index = 0;
                next_state.set(GameState::Playing);
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
        next_state.set(GameState::Dialogue);
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
