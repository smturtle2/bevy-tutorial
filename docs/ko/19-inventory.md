# 19. 인벤토리

<div align="center">

[목차](index.md) · [← 이전: 발사체](18-projectiles.md) · [다음: 대화 →](20-dialogue.md)

</div>

---

## 이 장에서 만들 것

RPG 데이터 모델에 인벤토리를 추가합니다. 맵 위의 아이템은 위치와 충돌 범위를 가진 엔티티로 남고, 수집되는 순간 두 데이터가 바뀝니다. `Inventory`는 플레이어가 무엇을 갖고 있는지 저장하고, `RunStats`는 이번 플레이의 점수를 저장합니다.

![아이템을 수집하면 인벤토리 숫자가 바뀌는 장면](../../assets/screenshots/ch19-inventory.png)

## 실행

```sh
cargo run --example 19_inventory
```

WASD나 방향키로 움직여 아이템에 닿아 봅니다.

## 이어받는 계약

이 장은 인벤토리 기능에 필요한 단계만 둡니다.

```text
GameState::Playing      게임플레이 시스템 실행 조건
GameSet                 Input -> Collision -> Ui
GameplayEntity          생성된 게임플레이 오브젝트 표시
Body                    아이템 수집 충돌에 쓰는 충돌 크기
RunStats                이번 플레이의 점수
```

새로 생기는 데이터 주인은 셋입니다.

```text
InventoryItem 컴포넌트   맵 위 아이템이 무엇인지
ItemKind enum            가능한 아이템 종류
Inventory 리소스         플레이어가 가진 아이템 개수
```

## 구현 흐름 1: 아이템 종류와 보유 수량 분리하기

`ItemKind`는 아이템 종류를 나타냅니다.

```rust
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ItemKind {
    Gem,
    Key,
    Potion,
}
```

각 derive에는 이유가 있습니다.

```text
Component    필요하면 enum 값을 직접 컴포넌트로 붙일 수 있음
Debug        인벤토리 규칙을 디버깅할 때 출력 가능
Clone/Copy   작은 enum 값을 컴포넌트에서 복사해 꺼낼 수 있음
PartialEq/Eq 비교 가능
Hash         HashMap이나 set 방식 규칙에 바로 사용 가능
```

플레이어의 현재 보유 수량은 리소스에 둡니다.

```rust
#[derive(Resource, Default)]
struct Inventory {
    gems: u32,
    keys: u32,
    potions: u32,
    last_pickup: Option<ItemKind>,
}
```

`ItemKind::Gem`은 아이템의 정체성이고, `inventory.gems`는 현재 플레이어 상태입니다.

## 구현 흐름 2: 아이템을 월드 엔티티로 두기

바닥에 놓인 아이템은 충돌과 화면 표현을 가집니다.

```rust
#[derive(Component)]
struct InventoryItem {
    kind: ItemKind,
}
```

번들은 맵 배치 코드를 한 번의 생성 호출로 만듭니다.

```rust
commands.spawn(ItemBundle::new(ItemKind::Key, Vec3::new(260.0, 120.0, 2.0)));
```

아이템은 여전히 월드 오브젝트입니다. 이동시킬 수도 있고, 제거할 수도 있고, 씬 파일에서 불러올 수도 있고, 충돌 시스템으로 검사할 수도 있습니다.

## 구현 흐름 3: 인벤토리 변경을 한 메서드에 모으기

인벤토리 변경 규칙은 `Inventory::add`가 책임집니다.

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

`match`는 모든 아이템 종류를 다룹니다. `ItemKind::Coin`을 추가하면 Rust가 코인을 어떻게 저장할지 코드로 정하라고 요구합니다.

## 구현 흐름 4: 수집 규칙 정하기

수집은 보유 수량과 점수를 함께 바꿉니다.

```rust
if overlaps(player_transform, player_body, item_transform, item_body) {
    inventory.add(item.kind);
    stats.score += item.kind.score_value();
    commands.entity(entity).despawn();
}
```

이 장의 게임플레이 계약은 이렇습니다.

```text
플레이어와 아이템이 겹침
-> Inventory 변경
-> ItemKind에 따라 RunStats.score 변경
-> 아이템 엔티티 제거
```

이제 보석은 인벤토리 아이템이면서 점수 아이템입니다. 둘을 연결하는 규칙이 `ItemKind::score_value`입니다.

## 구현 흐름 5: 화면 고정 UI로 보여주기

UI 시스템은 리소스를 읽고 텍스트를 씁니다.

```rust
text.0 = format!(
    "Score: {} | Gems: {} | Keys: {} | Potions: {}",
    stats.score,
    inventory.gems,
    inventory.keys,
    inventory.potions,
);
```

HUD는 `Inventory`와 `RunStats`의 현재 값을 읽어 화면에 보여줍니다.

## 통합 지점

인벤토리는 앞뒤 장과 이렇게 연결됩니다.

```text
17장 점수        RunStats.score는 이번 플레이의 가치를 계속 기록
19장 인벤토리    Inventory는 플레이어가 가진 아이템을 기록
22장 씬 로딩     씬 파일이 InventoryItem 엔티티를 생성 가능
저장/불러오기    오래 유지할 인벤토리 필드는 Progress에 추가 가능
```

저장 데이터는 소유자를 기준으로 정합니다. 오래 유지되는 플레이어 진행도는 저장하고, 씬에서 다시 만들 수 있는 임시 월드 엔티티는 저장하지 않습니다.

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

`map`은 `Some(kind)`를 처리합니다. `unwrap_or_else`는 `None`일 때 쓸 문장을 만듭니다.

`ItemKind`는 `Copy`라서 `inventory.add(item.kind)`처럼 컴포넌트 안의 enum 값을 복사해서 넘길 수 있습니다. 작은 enum에는 적절한 선택입니다. 파일에서 읽은 큰 아이템 데이터라면 ID를 쓰거나 필요한 시점에 명시적으로 복제하는 편이 낫습니다.

## Bevy로 보면

이 예제에서는 플레이어가 한 명이므로 인벤토리를 리소스로 둡니다.

```text
맵 위 아이템          Component
플레이어 보유 수량     Resource
이번 플레이 점수       Resource
HUD 표시              UI entity
```

멀티플레이 게임이라면 `Inventory`를 각 플레이어 엔티티의 컴포넌트로 두는 쪽이 자연스럽습니다. 데이터 소유자는 게임 규칙이 정합니다.

## 확인

실행합니다.

```sh
cargo run --example 19_inventory
```

확인 기준:

- 플레이어가 아이템에 닿을 수 있습니다.
- 수집한 아이템은 사라집니다.
- 해당 인벤토리 수량이 올라갑니다.
- `ItemKind::score_value`에 따라 점수가 오릅니다.
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
