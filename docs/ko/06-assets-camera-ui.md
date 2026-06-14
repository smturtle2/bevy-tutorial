# 6. 에셋, 카메라, UI

[목차](index.md) | 이전: [번들, 플러그인, 세트](05-bundles-plugins-sets.md) | 다음: [RPG 기초 조각](07-rpg-slice.md)

초기 예제는 색 sprite를 사용합니다. 첫 ECS 수업에서 asset loading을 제외할 수 있기 때문입니다. 이 장은 게임 로직을 작게 유지하면서 흔한 presentation 기능 세 가지를 추가합니다.

- `AssetServer`로 이미지 로드
- 플레이어를 따라가는 카메라
- `Text2d`로 world-space HUD 텍스트 표시

실행합니다.

```sh
cargo run --example 06_assets_camera_ui
```

이미지 기반 플레이어, 큰 배경 사각형, 그리고 플레이어 위를 따라다니며 위치를 표시하는 텍스트가 보여야 합니다.

## 둘러보기: `06_assets_camera_ui`

예제에는 marker component가 두 개 있습니다.

```rust
#[derive(Component)]
struct Player;

#[derive(Component)]
struct HudText;
```

`Player`는 움직일 sprite를 표시합니다. `HudText`는 플레이어 위치를 표시하는 텍스트 엔티티를 표시합니다.

앱은 startup 시스템 하나와 순서 있는 update 시스템 네 개를 등록합니다.

```rust
.add_systems(Startup, setup)
.add_systems(
    Update,
    (
        move_player,
        follow_player,
        update_hud_text,
        position_hud_text,
    )
        .chain(),
)
```

순서는 의도적입니다.

```text
move_player       player Transform 변경
follow_player     camera를 새 player 위치로 이동
update_hud_text   text 내용 갱신
position_hud_text text를 player 기준 위치로 이동
```

`.chain()`이 없으면 Bevy는 호환 가능한 시스템을 다른 순서로 실행할 수 있습니다. 이 예제에서 HUD와 카메라는 최신 player position을 사용해야 하므로 chain도 동작의 일부입니다.

## `AssetServer`로 Sprite 로드

setup 시스템은 `AssetServer`를 리소스로 요청합니다.

```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle::new(&asset_server));
}
```

번들이 생성 모양을 소유합니다.

```rust
#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    sprite: Sprite,
    transform: Transform,
}

impl PlayerBundle {
    fn new(asset_server: &AssetServer) -> Self {
        Self {
            player: Player,
            sprite: Sprite::from_image(asset_server.load("player.png")),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
        }
    }
}
```

기본적으로 Bevy는 asset path를 저장소의 `assets/` 디렉터리 아래에서 해석합니다. 이 코드는 다음 파일을 로드합니다.

```text
assets/player.png
```

`asset_server.load("player.png")`는 handle을 반환합니다. asset loading은 비동기로 완료될 수 있습니다. 시스템은 보통 이미지 파일을 직접 읽지 않고 handle을 저장하고 사용합니다.

단계에 따라 두 sprite 생성자를 씁니다.

```rust
Sprite::from_color(color, size)        // fast prototype visual
Sprite::from_image(asset_server.load("player.png")) // image asset
```

예제는 배경에는 여전히 색 sprite를 씁니다.

```rust
Sprite::from_color(Color::srgb(0.18, 0.22, 0.28), Vec2::new(900.0, 540.0))
```

이것은 자연스러운 조합입니다. 정체성이 중요한 곳에는 asset을 쓰고, 단순한 shape이면 colored primitive로 충분합니다.

## 정확히 하나의 플레이어를 위한 `Single`

이동 시스템은 `Single`을 씁니다.

```rust
fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    // ...
}
```

`Single<&mut Transform, With<Player>>`의 뜻:

```text
There must be exactly one entity with Player and Transform.
Give this system mutable access to that Transform.
```

`Query<&mut Transform, With<Player>>`보다 강한 계약입니다. `Query`는 0개, 1개, 여러 개 엔티티를 매칭할 수 있습니다. 예제나 기능이 정말 정확히 하나의 match를 요구할 때 `Single`을 쓰세요.

## Camera Follow

카메라도 그냥 엔티티입니다.

```rust
commands.spawn(Camera2d);
```

follow 시스템은 플레이어의 x/y 위치를 카메라에 복사합니다.

```rust
fn follow_player(
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}
```

`Without<Camera2d>`는 player query가 camera entity를 제외한다고 Bevy에게 알려줍니다. 두 파라미터가 모두 `Transform`에 접근하고 하나는 mutable이므로 중요합니다.

카메라의 z 값은 그대로 둡니다. 2D에서 x/y는 카메라가 바라보는 위치를 제어하고, z와 projection 설정은 view가 렌더링되는 방식을 제어합니다.

## `Text2d`로 World-Space HUD 만들기

HUD 텍스트도 평범한 ECS 엔티티로 spawn됩니다.

```rust
commands.spawn((
    HudText,
    Text2d::new("Position: 0, 0"),
    TextFont::from_font_size(24.0),
    TextColor(Color::srgb(0.86, 0.91, 0.98)),
    Transform::from_xyz(0.0, 230.0, 2.0),
));
```

이것은 screen-space UI가 아닙니다. 월드 안의 2D 텍스트입니다. 두 시스템이 HUD를 유지합니다. 하나는 텍스트 내용을 갱신하고, 하나는 텍스트 위치를 플레이어 위로 옮깁니다.

```rust
fn update_hud_text(
    player: Single<&Transform, With<Player>>,
    mut hud: Single<&mut Text2d, With<HudText>>,
) {
    hud.0 = format!(
        "Position: {:.0}, {:.0}",
        player.translation.x, player.translation.y
    );
}

fn position_hud_text(
    player: Single<&Transform, (With<Player>, Without<HudText>)>,
    mut hud: Single<&mut Transform, (With<HudText>, Without<Player>)>,
) {
    hud.translation.x = player.translation.x;
    hud.translation.y = player.translation.y + 230.0;
}
```

`Without` 필터는 장식이 아닙니다. `position_hud_text`는 player `Transform`을 읽고 HUD `Transform`을 수정합니다. 필터는 두 쿼리가 같은 엔티티에 접근하지 않는다는 사실을 Bevy에 증명합니다.

`Text2d`는 tuple struct이므로 문자열은 `text.0`에 저장됩니다.

텍스트 transform도 함께 갱신됩니다. 카메라가 플레이어를 따라가므로 HUD 같은 텍스트가 화면 위쪽 근처에 보이게 됩니다.

## 연습

작은 변경을 시도하세요.

1. HUD offset을 `+ 230.0`에서 `+ 120.0`으로 바꿔 보세요.
2. `.chain()`을 제거하고 어떤 시스템이 이전 프레임의 위치를 읽을 수 있는지 생각해 보세요.
3. `Sprite::from_image(...)`를 `Sprite::from_color(...)`로 바꾸고 movement, camera follow, HUD text가 asset에 의존하지 않는지 확인하세요.

## 흔한 실수

- `AssetServer::load`에 `"player.png"` 대신 `"assets/player.png"`를 사용함.
- 카메라를 잊고 아무것도 보이지 않음.
- 그 순간 0개나 여러 개 엔티티가 유효한데 `Single`을 사용함.
- 같은 엔티티를 매칭할 수 없다는 것을 증명하는 필터 없이 `Transform`에 대한 mutable query 두 개를 만듦.
- `Text2d`가 고정 screen-space UI처럼 동작한다고 기대함. 이 예제에서는 world-space text입니다.
