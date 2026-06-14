use bevy::prelude::*;
use bevy_tutorial::tutorial_capture::{add_tutorial_screenshot, tutorial_capture_enabled};

const PLAYER_SPEED: f32 = 260.0;
const PLAYER_SIZE: Vec2 = Vec2::splat(40.0);
const ITEM_SIZE: Vec2 = Vec2::splat(28.0);

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Playing,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Collision,
    Ui,
}

#[derive(Component)]
struct GameplayEntity;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct InventoryText;

#[derive(Component)]
struct Body {
    half_size: Vec2,
}

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ItemKind {
    Gem,
    Key,
    Potion,
}

impl ItemKind {
    fn label(self) -> &'static str {
        match self {
            ItemKind::Gem => "gem",
            ItemKind::Key => "key",
            ItemKind::Potion => "potion",
        }
    }

    fn color(self) -> Color {
        match self {
            ItemKind::Gem => Color::srgb(0.20, 0.86, 0.72),
            ItemKind::Key => Color::srgb(1.0, 0.82, 0.20),
            ItemKind::Potion => Color::srgb(0.95, 0.24, 0.42),
        }
    }

    fn score_value(self) -> u32 {
        match self {
            ItemKind::Gem => 10,
            ItemKind::Key => 50,
            ItemKind::Potion => 0,
        }
    }
}

#[derive(Component)]
struct InventoryItem {
    kind: ItemKind,
}

#[derive(Resource, Default)]
struct Inventory {
    gems: u32,
    keys: u32,
    potions: u32,
    last_pickup: Option<ItemKind>,
}

impl Inventory {
    fn add(&mut self, kind: ItemKind) {
        match kind {
            ItemKind::Gem => self.gems += 1,
            ItemKind::Key => self.keys += 1,
            ItemKind::Potion => self.potions += 1,
        }

        self.last_pickup = Some(kind);
    }
}

#[derive(Resource, Default)]
struct RunStats {
    score: u32,
}

#[derive(Bundle)]
struct BodyBundle {
    body: Body,
    velocity: Velocity,
    transform: Transform,
}

impl BodyBundle {
    fn new(position: Vec3, size: Vec2) -> Self {
        Self {
            body: Body {
                half_size: size / 2.0,
            },
            velocity: Velocity(Vec2::ZERO),
            transform: Transform::from_translation(position),
        }
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    gameplay: GameplayEntity,
    player: Player,
    body: BodyBundle,
    sprite: Sprite,
}

impl PlayerBundle {
    fn new() -> Self {
        Self {
            gameplay: GameplayEntity,
            player: Player,
            body: BodyBundle::new(Vec3::new(-260.0, -80.0, 2.0), PLAYER_SIZE),
            sprite: Sprite::from_color(Color::srgb(0.25, 0.64, 1.0), PLAYER_SIZE),
        }
    }
}

#[derive(Bundle)]
struct ItemBundle {
    gameplay: GameplayEntity,
    item: InventoryItem,
    body: Body,
    sprite: Sprite,
    transform: Transform,
}

impl ItemBundle {
    fn new(kind: ItemKind, position: Vec3) -> Self {
        Self {
            gameplay: GameplayEntity,
            item: InventoryItem { kind },
            body: Body {
                half_size: ITEM_SIZE / 2.0,
            },
            sprite: Sprite::from_color(kind.color(), ITEM_SIZE),
            transform: Transform::from_translation(position),
        }
    }
}

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .init_resource::<Inventory>()
        .init_resource::<RunStats>()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .configure_sets(
            Update,
            (GameSet::Input, GameSet::Collision, GameSet::Ui).chain(),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            move_player
                .in_set(GameSet::Input)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            collect_items
                .in_set(GameSet::Collision)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            update_inventory_text
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::Playing)),
        );

    add_tutorial_screenshot(&mut app, "assets/screenshots/ch19-inventory.png", 20);
    app.run();
}

fn setup(mut commands: Commands, mut inventory: ResMut<Inventory>, mut stats: ResMut<RunStats>) {
    commands.spawn(Camera2d);
    commands.spawn(PlayerBundle::new());

    for (kind, position) in [
        (ItemKind::Gem, Vec3::new(-80.0, 90.0, 2.0)),
        (ItemKind::Gem, Vec3::new(180.0, -80.0, 2.0)),
        (ItemKind::Key, Vec3::new(260.0, 120.0, 2.0)),
        (ItemKind::Potion, Vec3::new(40.0, -180.0, 2.0)),
    ] {
        commands.spawn(ItemBundle::new(kind, position));
    }

    commands.spawn((
        InventoryText,
        Text::new(""),
        TextFont::from_font_size(24.0),
        TextColor(Color::srgb(0.92, 0.95, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(18),
            left: px(18),
            ..default()
        },
    ));

    if tutorial_capture_enabled() {
        inventory.add(ItemKind::Gem);
        inventory.add(ItemKind::Potion);
        stats.score = ItemKind::Gem.score_value() + ItemKind::Potion.score_value();
    }
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Single<(&mut Transform, &mut Velocity), With<Player>>,
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

    let (mut transform, mut velocity) = player.into_inner();
    velocity.0 = direction.normalize_or_zero() * PLAYER_SPEED;
    transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
}

fn collect_items(
    mut commands: Commands,
    player: Single<(&Transform, &Body), With<Player>>,
    items: Query<(Entity, &Transform, &Body, &InventoryItem)>,
    mut inventory: ResMut<Inventory>,
    mut stats: ResMut<RunStats>,
) {
    let (player_transform, player_body) = *player;

    for (entity, item_transform, item_body, item) in &items {
        if overlaps(player_transform, player_body, item_transform, item_body) {
            inventory.add(item.kind);
            stats.score += item.kind.score_value();
            commands.entity(entity).despawn();
        }
    }
}

fn update_inventory_text(
    inventory: Res<Inventory>,
    stats: Res<RunStats>,
    items: Query<(), With<InventoryItem>>,
    mut text: Single<&mut Text, With<InventoryText>>,
) {
    let last = inventory
        .last_pickup
        .map(|kind| format!("last pickup: {}", kind.label()))
        .unwrap_or_else(|| "last pickup: none".to_string());

    text.0 = format!(
        "WASD move and collect items\nScore: {} | Gems: {} | Keys: {} | Potions: {}\n{} | remaining items: {}",
        stats.score,
        inventory.gems,
        inventory.keys,
        inventory.potions,
        last,
        items.iter().count()
    );
}

fn overlaps(
    a_transform: &Transform,
    a_body: &Body,
    b_transform: &Transform,
    b_body: &Body,
) -> bool {
    let distance = (a_transform.translation - b_transform.translation)
        .truncate()
        .abs();
    let allowed = a_body.half_size + b_body.half_size;

    distance.x < allowed.x && distance.y < allowed.y
}
