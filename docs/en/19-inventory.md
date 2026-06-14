# 19. Inventory

<div align="center">

[Index](index.md) · [← Previous: Projectiles](18-projectiles.md) · [Next: Dialogue →](20-dialogue.md)

</div>

---

## Outcome

At the end of this chapter, the player collects world items into an inventory resource. Pickups disappear from the map, item counts increase, and the HUD shows the current inventory.

![Inventory counts update as the player collects items.](../../assets/screenshots/ch19-inventory.png)

## Run

```sh
cargo run --example 19_inventory
```

Move with WASD or arrow keys and touch the items.

## Build Step 1: Separate Item Kind From Inventory State

The world item stores what it is:

```rust
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ItemKind {
    Gem,
    Key,
    Potion,
}
```

The inventory stores how many the player owns:

```rust
#[derive(Resource, Default)]
struct Inventory {
    gems: u32,
    keys: u32,
    potions: u32,
    last_pickup: Option<ItemKind>,
}
```

Those are different responsibilities. `ItemKind::Gem` is the definition. `inventory.gems` is the player's current state.

## Build Step 2: Make Pickups World Entities

An item on the ground has position and collision:

```rust
#[derive(Component)]
struct InventoryItem {
    kind: ItemKind,
}
```

The item bundle combines data and presentation:

```rust
struct ItemBundle {
    item: InventoryItem,
    body: Body,
    sprite: Sprite,
    transform: Transform,
}
```

The bundle constructor receives the item kind and a position. That makes level setup readable:

```rust
commands.spawn(ItemBundle::new(ItemKind::Key, Vec3::new(260.0, 120.0, 2.0)));
```

## Build Step 3: Add Items Through One Method

The inventory update rule lives in one method:

```rust
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
```

The `match` makes every item kind explicit. If you add `ItemKind::Coin`, Rust will force you to decide how the inventory stores it.

## Build Step 4: Pick Up Items With Collision

The pickup system reads the player and scans item entities:

```rust
fn collect_items(
    mut commands: Commands,
    player: Single<(&Transform, &Body), With<Player>>,
    items: Query<(Entity, &Transform, &Body, &InventoryItem)>,
    mut inventory: ResMut<Inventory>,
) {
    let (player_transform, player_body) = *player;

    for (entity, item_transform, item_body, item) in &items {
        if overlaps(player_transform, player_body, item_transform, item_body) {
            inventory.add(item.kind);
            commands.entity(entity).despawn();
        }
    }
}
```

The rule is complete:

```text
player overlaps pickup -> inventory changes -> pickup entity despawns
```

## Build Step 5: Show Inventory In Screen-Space UI

The UI system reads the resource and writes text:

```rust
text.0 = format!(
    "Gems: {} | Keys: {} | Potions: {}",
    inventory.gems,
    inventory.keys,
    inventory.potions,
);
```

This is screen-space UI, so the inventory display stays fixed while the player moves.

## Build Step 6: Define The Inventory Contract

This chapter uses a simple contract:

```text
items are stackable
there is no slot limit
pickup always succeeds
inventory is runtime memory only
save/load integration comes from the progress chapter
```

If you need limited slots, add the rule to `Inventory::add`. Do not scatter slot rules through pickup systems.

## Rust Lens

`Option<ItemKind>` means there may or may not be a last pickup:

```rust
last_pickup: Option<ItemKind>
```

The UI converts that optional value into text:

```rust
let last = inventory
    .last_pickup
    .map(|kind| format!("last pickup: {}", kind.label()))
    .unwrap_or_else(|| "last pickup: none".to_string());
```

`map` handles the `Some` case. `unwrap_or_else` handles the `None` case.

## Bevy Lens

Inventory is a resource in this example because there is one player and one inventory:

```text
world pickup item      Component
player-owned counts    Resource
HUD display            UI entity
```

In a multiplayer game, each player would likely have an `Inventory` component instead. The data ownership follows the game rule.

## Check

Run:

```sh
cargo run --example 19_inventory
```

Expected result:

- The player can move into items.
- Collected items disappear.
- The matching item count increases.
- The last pickup label changes.
- The remaining item count decreases.

## Change

Add another pickup in `setup`:

```rust
(ItemKind::Potion, Vec3::new(-180.0, 170.0, 2.0)),
```

Expected result: the new potion appears, can be collected, and increases the potion count through the same system.

---

<div align="center">

[← Previous: Projectiles](18-projectiles.md) · [Index](index.md) · [Next: Dialogue →](20-dialogue.md)

</div>
