# 11. 스프라이트 에셋

<div align="center">

[목차](index.md) · [← 이전: 공격 히트박스](10-attack-hitbox.md) · [다음: 화면 고정 UI →](12-screen-space-ui.md)

</div>

---

## 이 장에서 만들 것

이 장이 끝나면 임시 색상 사각형 대신 `assets/` 디렉토리의 이미지 에셋으로 플레이어, 적, 보석을 그립니다.

![플레이어, 적, 보석 스프라이트가 이미지 에셋으로 로드됩니다.](../../assets/screenshots/ch11-sprite-assets.png)

## 실행

```sh
cargo run --example 11_sprite_assets
```

WASD/방향키로 플레이어를 움직입니다. 이미지 스프라이트는 엔티티에 붙어 있고, 상태 텍스트는 플레이어 위치를 따라 갱신됩니다.

## 구현 흐름 1: 픽셀 아트에 맞는 이미지 설정 쓰기

앱은 최근접 샘플링(nearest-neighbor sampling)을 씁니다.

```rust
.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
```

픽셀 아트가 확대될 때 흐려지지 않게 하는 설정입니다.

## 구현 흐름 2: 플레이어 이미지 로드하기

플레이어 bundle은 이미지 핸들로 만든 sprite를 저장합니다.

```rust
impl PlayerBundle {
    fn new(asset_server: &AssetServer) -> Self {
        Self {
            player: Player,
            sprite: Sprite::from_image(asset_server.load("player.png")),
            transform: Transform::from_xyz(0.0, -60.0, 2.0),
        }
    }
}
```

`asset_server.load("player.png")`는 이 파일을 가리킵니다.

```text
assets/player.png
```

엔티티는 `Sprite` 컴포넌트를 갖고, 실제 로딩과 캐싱은 애셋 서버가 맡습니다.

## 구현 흐름 3: 표시용 스프라이트 생성 재사용하기

예제는 정적 표시용 스프라이트 bundle을 하나 둡니다.

```rust
#[derive(Bundle)]
struct DisplaySpriteBundle {
    sprite: Sprite,
    transform: Transform,
}

impl DisplaySpriteBundle {
    fn new(path: &'static str, position: Vec3, asset_server: &AssetServer) -> Self {
        Self {
            sprite: Sprite::from_image(asset_server.load(path)),
            transform: Transform::from_translation(position),
        }
    }
}
```

그러면 setup에서 여러 에셋을 간단히 생성할 수 있습니다.

```rust
for (path, x) in [("enemy.png", -160.0), ("gem.png", 160.0)] {
    commands.spawn(DisplaySpriteBundle::new(
        path,
        Vec3::new(x, 100.0, 2.0),
        &asset_server,
    ));
}
```

경로만 데이터로 바뀌고 spawn 모양은 같습니다.

## 구현 흐름 4: 게임플레이 데이터와 그림 분리하기

플레이어 이동은 여전히 `Transform`을 수정합니다.

```rust
player.translation +=
    (direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs()).extend(0.0);
```

이동 시스템은 플레이어가 색상 사각형으로 그려지는지, 이미지로 그려지는지 몰라도 됩니다.

## Rust로 보면

이 시그니처에는 static 수명을 가진 문자열 슬라이스가 나옵니다.

```rust
fn new(path: &'static str, position: Vec3, asset_server: &AssetServer) -> Self
```

`"enemy.png"`와 `"gem.png"` 같은 문자열 리터럴은 프로그램 내내 살아 있으므로 `&'static str`에 맞습니다.

Bundle은 `AssetServer`를 빌려서 핸들을 만들고, `Sprite` 안에는 그 핸들만 저장합니다.

## Bevy로 보면

스프라이트 에셋도 결국 컴포넌트 데이터입니다.

```text
Sprite { image: Handle<Image>, ... }
Transform
```

그림을 바꿨다고 이동, 충돌, AI, 저장 코드가 바뀌면 구조가 좋지 않은 겁니다. 게임플레이 컴포넌트와 렌더링 컴포넌트는 같은 엔티티에 함께 붙을 수 있지만, 시스템 책임은 분리하는 것이 좋습니다.

## 확인

실행합니다.

```sh
cargo run --example 11_sprite_assets
```

기대 결과:

- `player.png`, `enemy.png`, `gem.png`가 보입니다.
- 플레이어가 움직입니다.
- 상태 텍스트가 플레이어 위치를 표시합니다.

## 바꿔보기

표시 에셋 순서를 바꿔 봅니다.

```rust
for (path, x) in [("gem.png", -160.0), ("enemy.png", 160.0)] {
```

기대 결과: 왼쪽과 오른쪽에 보이는 스프라이트가 바뀝니다. 게임플레이 코드는 바뀌지 않습니다.

---

<div align="center">

[← 이전: 공격 히트박스](10-attack-hitbox.md) · [목차](index.md) · [다음: 화면 고정 UI →](12-screen-space-ui.md)

</div>
