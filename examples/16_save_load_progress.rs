use std::{fs, path::Path};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

const SAVE_PATH: &str = "target/tutorial-save/progress.json";

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
struct Progress {
    best_score: u32,
    unlocked_wave: u32,
    player_name: String,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            best_score: 0,
            unlocked_wave: 1,
            player_name: "Bevy Learner".to_string(),
        }
    }
}

#[derive(Component)]
struct ProgressText;

#[derive(Component)]
struct SaveStatusText;

#[derive(Resource)]
struct SaveStatus {
    message: String,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .insert_resource(load_progress_from_disk())
        .insert_resource(SaveStatus {
            message: format!("Save path: {SAVE_PATH}"),
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (edit_progress, update_text).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        ProgressText,
        Text::new(""),
        TextFont::from_font_size(28.0),
        TextColor(Color::srgb(0.92, 0.95, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(26),
            left: px(24),
            ..default()
        },
    ));

    commands.spawn((
        SaveStatusText,
        Text::new(""),
        TextFont::from_font_size(20.0),
        TextColor(Color::srgb(1.0, 0.82, 0.30)),
        Node {
            position_type: PositionType::Absolute,
            top: px(130),
            left: px(24),
            ..default()
        },
    ));
}

fn edit_progress(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut progress: ResMut<Progress>,
    mut status: ResMut<SaveStatus>,
) {
    if keyboard.just_pressed(KeyCode::KeyS) {
        progress.best_score += 100;
        status.message = "Changed best_score in memory".to_string();
    }

    if keyboard.just_pressed(KeyCode::KeyW) {
        progress.unlocked_wave += 1;
        status.message = "Changed unlocked_wave in memory".to_string();
    }

    if keyboard.just_pressed(KeyCode::F5) {
        match save_progress_to_disk(&progress) {
            Ok(()) => status.message = format!("Saved to {SAVE_PATH}"),
            Err(error) => status.message = format!("Save failed: {error}"),
        }
    }

    if keyboard.just_pressed(KeyCode::F9) {
        *progress = load_progress_from_disk();
        status.message = format!("Loaded from {SAVE_PATH}");
    }

    if keyboard.just_pressed(KeyCode::Delete) {
        *progress = Progress::default();
        let _ = fs::remove_file(SAVE_PATH);
        status.message = "Reset progress and removed save file".to_string();
    }
}

fn update_text(
    progress: Res<Progress>,
    status: Res<SaveStatus>,
    mut progress_text: Single<&mut Text, (With<ProgressText>, Without<SaveStatusText>)>,
    mut save_status_text: Single<&mut Text, (With<SaveStatusText>, Without<ProgressText>)>,
) {
    progress_text.0 = format!(
        "Progress\nbest_score: {}\nunlocked_wave: {}\nplayer_name: {}",
        progress.best_score, progress.unlocked_wave, progress.player_name
    );
    save_status_text.0 = format!(
        "S: +score | W: +wave | F5: save | F9: load | Delete: reset\n{}",
        status.message
    );
}

fn load_progress_from_disk() -> Progress {
    fs::read_to_string(SAVE_PATH)
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

fn save_progress_to_disk(progress: &Progress) -> Result<(), String> {
    let Some(parent) = Path::new(SAVE_PATH).parent() else {
        return Err("save path has no parent directory".to_string());
    };

    fs::create_dir_all(parent).map_err(|error| error.to_string())?;

    let json = serde_json::to_string_pretty(progress).map_err(|error| error.to_string())?;
    fs::write(SAVE_PATH, json).map_err(|error| error.to_string())
}
