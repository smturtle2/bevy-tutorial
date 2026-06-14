# 19. 인벤토리

<div align="center">

[목차](index.md) · [← 이전: 발사체](18-projectiles.md) · [다음: 대화 →](20-dialogue.md)

</div>

---

## 이 장에서 만들 것

이 장이 끝나면 플레이어가 월드에 놓인 아이템을 인벤토리에 담을 수 있습니다. 아이템은 맵에서 사라지고, 보유 수량은 올라가며, HUD는 현재 인벤토리를 보여줍니다.

![아이템을 수집하면 인벤토리 숫자가 바뀌는 장면](../../assets/screenshots/ch19-inventory.png)

## 실행

```sh
cargo run --example 19_inventory
```

WASD나 방향키로 움직여 아이템에 닿아 봅니다.

## 구현 흐름 1: 아이템 종류와 보유 상태 분리하기

월드 아이템은 자신이 무엇인지 저장합니다.

```rust
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ItemKind {
    Gem,
    Key,
    Potion,
}
```

인벤토리는 플레이어가 몇 개 갖고 있는지 저장합니다.

```rust
#[derive(Resource, Default)]
struct Inventory {
    gems: u32,
    keys: u32,
    potions: u32,
    last_pickup: Option<ItemKind>,
}
```

둘은 다른 책임입니다. `ItemKind::Gem`은 아이템 정의이고, `inventory.gems`는 현재 보유 상태입니다.

## 구현 흐름 2: 맵 위 아이템을 엔티티로 만들기

바닥에 놓인 아이템은 위치와 충돌 범위를 가집니다.

```rust
#[derive(Component)]
struct InventoryItem {
    kind: ItemKind,
}
```

아이템 Bundle은 데이터와 화면 표현을 함께 묶습니다.

```rust
struct ItemBundle {
    item: InventoryItem,
    body: Body,
    sprite: Sprite,
    transform: Transform,
}
```

Bundle 생성자는 아이템 종류와 위치를 받습니다.

```rust
commands.spawn(ItemBundle::new(ItemKind::Key, Vec3::new(260.0, 120.0, 2.0)));
```

이렇게 하면 맵 배치 코드가 컴포넌트 나열이 아니라 게임 오브젝트 배치처럼 읽힙니다.

## 구현 흐름 3: 추가 규칙을 한 메서드에 모으기

인벤토리 변경 규칙은 `Inventory::add`에 둡니다.

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

`match`는 모든 아이템 종류를 명시합니다. 나중에 `ItemKind::Coin`을 추가하면 Rust가 “코인은 어떻게 저장할 건데?”라고 컴파일 단계에서 묻습니다.

## 구현 흐름 4: 충돌로 아이템 줍기

수집 시스템은 플레이어를 읽고 아이템 엔티티를 검사합니다.

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

규칙은 이렇습니다.

```text
플레이어와 아이템이 겹침 -> 인벤토리 변경 -> 아이템 엔티티 제거
```

## 구현 흐름 5: 화면 고정 UI로 보여주기

UI 시스템은 인벤토리 리소스를 읽고 텍스트를 씁니다.

```rust
text.0 = format!(
    "Gems: {} | Keys: {} | Potions: {}",
    inventory.gems,
    inventory.keys,
    inventory.potions,
);
```

이 UI는 화면 고정 UI입니다. 플레이어가 움직여도 인벤토리 표시는 화면 같은 자리에 남습니다.

## 구현 흐름 6: 인벤토리 계약 정하기

이 장의 인벤토리는 다음 규칙을 가집니다.

```text
아이템은 stack 가능
슬롯 제한 없음
줍기는 항상 성공
인벤토리는 실행 중 메모리에만 있음
저장/불러오기와 연결하려면 진행도 저장 장의 구조를 사용
```

슬롯 제한이 필요하면 `Inventory::add`에 규칙을 넣습니다. 수집 시스템 곳곳에 슬롯 규칙을 흩뿌리면 나중에 유지보수가 어려워집니다.

## Rust로 보면

`Option<ItemKind>`는 마지막으로 주운 아이템이 있을 수도 있고 없을 수도 있다는 뜻입니다.

```rust
last_pickup: Option<ItemKind>
```

UI는 이 값을 텍스트로 바꿉니다.

```rust
let last = inventory
    .last_pickup
    .map(|kind| format!("last pickup: {}", kind.label()))
    .unwrap_or_else(|| "last pickup: none".to_string());
```

`map`은 `Some`일 때 실행됩니다. `unwrap_or_else`는 `None`일 때 기본 문장을 만듭니다.

## Bevy로 보면

이 예제에서는 플레이어가 한 명이고 인벤토리도 하나라서 리소스가 맞습니다.

```text
맵 위의 아이템       Component
플레이어의 보유 수량  Resource
화면 표시            UI entity
```

멀티플레이 게임이라면 플레이어마다 `Inventory` 컴포넌트를 붙이는 편이 더 자연스럽습니다. 데이터 소유자는 게임 규칙이 정합니다.

## 확인

실행합니다.

```sh
cargo run --example 19_inventory
```

확인 기준:

- 플레이어가 아이템에 닿을 수 있습니다.
- 주운 아이템은 사라집니다.
- 해당 아이템 수량이 올라갑니다.
- 마지막으로 주운 아이템 문구가 바뀝니다.
- 남은 아이템 수가 줄어듭니다.

## 바꿔보기

`setup`에 아이템을 하나 더 추가합니다.

```rust
(ItemKind::Potion, Vec3::new(-180.0, 170.0, 2.0)),
```

기대 결과: 새 포션이 나타나고, 같은 수집 시스템으로 포션 수량이 올라갑니다.

---

<div align="center">

[← 이전: 발사체](18-projectiles.md) · [목차](index.md) · [다음: 대화 →](20-dialogue.md)

</div>
