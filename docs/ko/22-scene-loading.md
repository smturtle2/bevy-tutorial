# 22. 씬 로딩

<div align="center">

[목차](index.md) · [← 이전: 오디오 이벤트](21-audio-events.md) · [기여하기 →](https://github.com/smturtle2/bevy-tutorial)

</div>

---

## 이 장에서 만들 것

이 장이 끝나면 게임이 JSON 파일에서 레벨 데이터를 읽습니다. 씬 파일은 플레이어 시작 위치, 벽, 보석, NPC를 정하고, Bevy 시스템은 그 데이터를 엔티티로 바꿉니다.

![두 개의 데이터 기반 아레나를 전환하는 씬 로딩 장면](../../assets/screenshots/ch22-scene-loading.png)

## 실행

```sh
cargo run --example 22_scene_loading
```

1 또는 2를 눌러 다른 씬 파일을 불러옵니다. WASD나 방향키로 움직입니다.

## 구현 흐름 1: 이 장에서 말하는 씬 정의하기

이 장은 Bevy의 전체 reflection 기반 `DynamicScene` 포맷이 아니라, 튜토리얼용 레벨 데이터 포맷을 씁니다. 여기서 씬은 맵 데이터입니다.

```rust
#[derive(Deserialize)]
struct SceneData {
    name: String,
    player_start: [f32; 2],
    walls: Vec<RectData>,
    gems: Vec<PointData>,
    npcs: Vec<NpcData>,
}
```

데이터 파일은 이런 모양입니다.

```json
{
  "name": "Training Yard",
  "player_start": [-260.0, -120.0],
  "walls": [
    { "x": 0.0, "y": 260.0, "w": 760.0, "h": 34.0 }
  ],
  "gems": [
    { "x": -120.0, "y": 140.0 }
  ],
  "npcs": [
    { "name": "Mapper", "x": 190.0, "y": 120.0 }
  ]
}
```

계약은 단순합니다. 씬 파일은 무엇이 어디서 시작하는지 설명합니다. 그것들이 어떻게 움직이고 충돌하고 수집되는지는 Rust 시스템이 정합니다.

## 구현 흐름 2: 씬 소유 엔티티 표시하기

씬 데이터에서 생성된 모든 엔티티에는 marker를 붙입니다.

```rust
#[derive(Component)]
struct SceneEntity;
```

새 씬을 불러올 때 이전 씬 엔티티를 제거합니다.

```rust
for entity in &entities {
    commands.entity(entity).despawn();
}
```

이 규칙이 있어야 이전 벽, 보석, NPC가 새 씬에 남지 않습니다.

## 구현 흐름 3: 씬 파일 읽고 파싱하기

로더는 `assets/scenes/...`에서 파일을 읽습니다.

```rust
let fs_path = format!("assets/{asset_path}");
let text = match fs::read_to_string(&fs_path) {
    Ok(text) => text,
    Err(error) => return format!("Failed to read {asset_path}: {error}"),
};
let scene = match serde_json::from_str::<SceneData>(&text) {
    Ok(scene) => scene,
    Err(error) => return format!("Failed to parse {asset_path}: {error}"),
};
```

예제는 실패해도 panic하지 않고 상태 메시지를 돌려줍니다. 이 메시지는 HUD에 표시되므로 로딩 실패를 화면에서 바로 볼 수 있습니다.

## 구현 흐름 4: 데이터에서 엔티티 생성하기

씬 데이터는 일반 ECS 엔티티로 변환됩니다.

```rust
for wall in &scene.walls {
    let size = Vec2::new(wall.w, wall.h);
    commands.spawn((
        SceneEntity,
        Wall,
        Body { half_size: size / 2.0 },
        Sprite::from_color(Color::srgb(0.28, 0.33, 0.42), size),
        Transform::from_xyz(wall.x, wall.y, 2.0),
    ));
}
```

플레이어, 보석, NPC도 같은 방식으로 생성합니다. 씬 로딩은 마법이 아니라 데이터 기반 생성입니다.

## 구현 흐름 5: 런타임 규칙은 코드에 남기기

씬 파일에는 충돌 함수, 이동 코드, UI 시스템을 넣지 않습니다. 그런 규칙은 Rust에 남습니다.

```rust
fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Option<Single<&mut Transform, With<Player>>>,
    walls: Query<(&Transform, &Body), (With<Wall>, Without<Player>)>,
)
```

씬은 데이터를 바꿉니다. 시스템은 행동을 유지합니다.

## 구현 흐름 6: 단축키로 씬 바꾸기

단축키 시스템은 불러올 경로를 고릅니다.

```rust
let next_path = if keyboard.just_pressed(KeyCode::Digit1) {
    Some("scenes/arena_a.json")
} else if keyboard.just_pressed(KeyCode::Digit2) {
    Some("scenes/arena_b.json")
} else {
    None
};
```

그 다음 이전 씬 엔티티를 지우고 새 파일을 읽습니다. 이것이 기본적인 방 전환 계약입니다.

## Rust로 보면

중첩 struct는 중첩 데이터를 그대로 닮습니다.

```rust
struct SceneData {
    walls: Vec<RectData>,
    gems: Vec<PointData>,
    npcs: Vec<NpcData>,
}
```

`Vec<T>`는 씬마다 개수가 달라도 된다는 뜻입니다. `serde_json::from_str::<SceneData>`는 JSON을 정확히 이 Rust 타입으로 파싱해 달라는 요청입니다.

## Bevy로 보면

씬 로딩은 세 종류의 데이터를 분리합니다.

```text
영구 진행도       저장되는 플레이어 진행 상황
씬 데이터         벽, 보석, NPC 위치
런타임 엔티티      실제 월드에 생성된 ECS 오브젝트
```

marker 컴포넌트는 씬 데이터와 정리 규칙을 연결합니다. 일단 올바른 컴포넌트만 붙어 있으면, 게임플레이 시스템은 그 엔티티가 코드에서 왔는지 JSON에서 왔는지 신경 쓰지 않습니다.

## 확인

실행합니다.

```sh
cargo run --example 22_scene_loading
```

확인 기준:

- 시작하면 씬 1이 로드됩니다.
- 2를 누르면 다른 배치로 바뀝니다.
- 이전 벽, 보석, NPC는 새 씬이 생성되기 전에 사라집니다.
- 플레이어는 씬 파일의 시작 위치에서 시작합니다.
- HUD는 로드된 씬 이름과 개수를 보여줍니다.

## 바꿔보기

`assets/scenes/arena_a.json`을 열고 보석을 하나 추가합니다.

```json
{ "x": 40.0, "y": 210.0 }
```

기대 결과: Rust 코드를 바꾸지 않아도 예제를 다시 실행하면 보석이 하나 더 보입니다.

---

<div align="center">

[← 이전: 오디오 이벤트](21-audio-events.md) · [목차](index.md) · [기여하기 →](https://github.com/smturtle2/bevy-tutorial)

</div>
