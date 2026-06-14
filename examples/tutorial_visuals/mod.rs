#![allow(dead_code)]

use bevy::prelude::*;

pub const PLAYER_DRAW_SIZE: Vec2 = Vec2::splat(46.0);
pub const ENEMY_DRAW_SIZE: Vec2 = Vec2::splat(42.0);
pub const GEM_DRAW_SIZE: Vec2 = Vec2::splat(34.0);
pub const SLASH_DRAW_SIZE: Vec2 = Vec2::new(78.0, 52.0);

pub const PANEL_BG: Color = Color::srgba(0.045, 0.055, 0.075, 0.90);
pub const PANEL_BORDER: Color = Color::srgba(0.36, 0.45, 0.58, 0.75);
pub const TEXT_MAIN: Color = Color::srgb(0.92, 0.95, 1.0);
pub const TEXT_ACCENT: Color = Color::srgb(1.0, 0.90, 0.52);

#[derive(Resource, Clone)]
pub struct TutorialSprites {
    pub player_sheet: Handle<Image>,
    pub player_layout: Handle<TextureAtlasLayout>,
    pub enemy: Handle<Image>,
    pub gem: Handle<Image>,
    pub slash: Handle<Image>,
}

impl TutorialSprites {
    pub fn load(
        asset_server: &AssetServer,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        let player_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(32),
            4,
            1,
            None,
            None,
        ));

        Self {
            player_sheet: asset_server.load("player_sheet.png"),
            player_layout,
            enemy: asset_server.load("enemy.png"),
            gem: asset_server.load("gem.png"),
            slash: asset_server.load("slash.png"),
        }
    }
}

pub fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera2d);
}

pub fn spawn_arena_backdrop(commands: &mut Commands) {
    spawn_floor(commands, -5..=5, -3..=3);

    for (position, size) in [
        (Vec3::new(0.0, 300.0, 1.0), Vec2::new(900.0, 40.0)),
        (Vec3::new(0.0, -300.0, 1.0), Vec2::new(900.0, 40.0)),
        (Vec3::new(-460.0, 0.0, 1.0), Vec2::new(40.0, 640.0)),
        (Vec3::new(460.0, 0.0, 1.0), Vec2::new(40.0, 640.0)),
        (Vec3::new(-125.0, 82.0, 1.0), Vec2::new(250.0, 34.0)),
        (Vec3::new(215.0, -120.0, 1.0), Vec2::new(250.0, 34.0)),
        (Vec3::new(90.0, 150.0, 1.0), Vec2::new(34.0, 205.0)),
    ] {
        spawn_wall(commands, position, size);
    }
}

pub fn spawn_floor(
    commands: &mut Commands,
    xs: std::ops::RangeInclusive<i32>,
    ys: std::ops::RangeInclusive<i32>,
) {
    let tile_size = Vec2::splat(80.0);

    for x in xs {
        for y in ys.clone() {
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

pub fn spawn_wall(commands: &mut Commands, position: Vec3, size: Vec2) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.30, 0.37, 0.48), size),
        Transform::from_translation(position),
    ));

    commands.spawn((
        Sprite::from_color(
            Color::srgba(0.08, 0.10, 0.13, 0.55),
            size + Vec2::splat(8.0),
        ),
        Transform::from_xyz(position.x + 5.0, position.y - 5.0, position.z - 0.1),
    ));
}

pub fn spawn_world_label(commands: &mut Commands, text: impl Into<String>, position: Vec3) {
    commands.spawn((
        Text2d::new(text),
        TextFont::from_font_size(18.0),
        TextColor(Color::srgb(0.72, 0.80, 0.92)),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_translation(position),
    ));
}

pub fn spawn_health_bar(commands: &mut Commands, position: Vec3, current: i32, max: i32) {
    let width = 42.0;
    let fraction = (current.max(0) as f32 / max.max(1) as f32).clamp(0.0, 1.0);

    commands.spawn((
        Sprite::from_color(Color::srgb(0.08, 0.10, 0.13), Vec2::new(width + 6.0, 8.0)),
        Transform::from_translation(position),
    ));
    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.30, 0.92, 0.47),
            Vec2::new(width * fraction, 4.0),
        ),
        Transform::from_xyz(
            position.x - width * (1.0 - fraction) / 2.0,
            position.y,
            position.z + 0.1,
        ),
    ));
}

pub fn spawn_impact_burst(commands: &mut Commands, position: Vec3, color: Color) {
    for (offset, size) in [
        (Vec2::new(-22.0, 18.0), Vec2::splat(8.0)),
        (Vec2::new(24.0, 14.0), Vec2::splat(6.0)),
        (Vec2::new(-18.0, -18.0), Vec2::splat(6.0)),
        (Vec2::new(18.0, -22.0), Vec2::splat(7.0)),
    ] {
        commands.spawn((
            Sprite::from_color(color, size),
            Transform::from_xyz(position.x + offset.x, position.y + offset.y, position.z),
        ));
    }
}

pub fn player_sprite(assets: &TutorialSprites) -> Sprite {
    Sprite {
        image: assets.player_sheet.clone(),
        texture_atlas: Some(TextureAtlas {
            layout: assets.player_layout.clone(),
            index: 0,
        }),
        custom_size: Some(PLAYER_DRAW_SIZE),
        ..default()
    }
}

pub fn npc_sprite(assets: &TutorialSprites) -> Sprite {
    Sprite {
        image: assets.player_sheet.clone(),
        texture_atlas: Some(TextureAtlas {
            layout: assets.player_layout.clone(),
            index: 1,
        }),
        color: Color::srgb(1.0, 0.78, 0.36),
        custom_size: Some(PLAYER_DRAW_SIZE),
        ..default()
    }
}

pub fn enemy_sprite(assets: &TutorialSprites) -> Sprite {
    Sprite {
        image: assets.enemy.clone(),
        custom_size: Some(ENEMY_DRAW_SIZE),
        ..default()
    }
}

pub fn gem_sprite(assets: &TutorialSprites) -> Sprite {
    Sprite {
        image: assets.gem.clone(),
        custom_size: Some(GEM_DRAW_SIZE),
        ..default()
    }
}

pub fn slash_sprite(assets: &TutorialSprites) -> Sprite {
    Sprite {
        image: assets.slash.clone(),
        custom_size: Some(SLASH_DRAW_SIZE),
        ..default()
    }
}

pub fn projectile_sprite() -> Sprite {
    Sprite::from_color(Color::srgb(1.0, 0.78, 0.25), Vec2::new(42.0, 12.0))
}

pub fn spawn_status_panel<T: Component>(
    commands: &mut Commands,
    marker: T,
    initial_text: impl Into<String>,
    width: f32,
) {
    commands.spawn((
        marker,
        Text::new(initial_text),
        TextFont::from_font_size(19.0),
        TextColor(TEXT_MAIN),
        Node {
            position_type: PositionType::Absolute,
            top: px(18),
            left: px(18),
            width: px(width),
            padding: UiRect::axes(px(14), px(10)),
            border: UiRect::all(px(1)),
            ..default()
        },
        BackgroundColor(PANEL_BG),
        BorderColor::all(PANEL_BORDER),
    ));
}

pub fn spawn_dialogue_panel<T: Component>(commands: &mut Commands, marker: T) {
    commands.spawn((
        marker,
        Text::new(""),
        TextFont::from_font_size(24.0),
        TextColor(TEXT_ACCENT),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(26),
            left: px(40),
            right: px(40),
            padding: UiRect::axes(px(18), px(14)),
            border: UiRect::all(px(1)),
            ..default()
        },
        BackgroundColor(PANEL_BG),
        BorderColor::all(PANEL_BORDER),
    ));
}

pub fn spawn_sound_ring(commands: &mut Commands, position: Vec3, color: Color, radius: f32) {
    commands.spawn((
        Sprite::from_color(color.with_alpha(0.28), Vec2::splat(radius)),
        Transform::from_translation(position),
    ));
    commands.spawn((
        Sprite::from_color(PANEL_BG.with_alpha(0.70), Vec2::splat(radius - 10.0)),
        Transform::from_xyz(position.x, position.y, position.z + 0.1),
    ));
}
