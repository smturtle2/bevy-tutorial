# 11. 스프라이트 에셋

[목차](index.md) | 이전: [공격 hitbox](10-attack-hitbox.md) | 다음: [screen-space UI](12-screen-space-ui.md)

실행:

```sh
cargo run --example 11_sprite_assets
```

이 장의 계약은 파일 경로 문자열을 스프라이트 컴포넌트에 직접 저장하지 않고, `AssetServer`가 돌려주는 `Handle<Image>`를 `Sprite`에 넣는 것입니다. 엔티티는 핸들을 들고 있고, Bevy의 에셋 시스템이 실제 이미지 로딩과 공유를 담당합니다.

## 핵심 ECS 계약

- `PlayerBundle`: 플레이어 마커, 이미지 스프라이트, 위치를 한 번에 생성합니다.
- `DisplaySpriteBundle`: 화면에 보여줄 단순 스프라이트와 위치를 묶습니다.
- `AssetLabel`: 로드된 에셋 상태를 표시하는 텍스트 엔티티를 찾는 마커입니다.
- `move_player`: 플레이어 위치를 갱신합니다.
- `update_asset_label`: 플레이어 위치를 읽어 텍스트를 갱신합니다.

`Sprite::from_image(asset_server.load("player.png"))`가 핵심입니다. `load`는 즉시 이미지 픽셀을 반환하지 않고 핸들을 반환합니다. Bevy는 핸들을 보고 필요한 시점에 에셋을 로드합니다.

## Rust 포인트

`DisplaySpriteBundle::new(path: &'static str, ...)`는 예제 안의 고정 문자열 경로만 받습니다. `"enemy.png"` 같은 문자열 리터럴은 프로그램 전체 수명 동안 살아 있으므로 `&'static str`에 맞습니다.

`for (path, x) in [("enemy.png", -160.0), ("gem.png", 160.0)]`는 작은 고정 데이터를 배열로 두고 반복합니다. 같은 스폰 코드를 두 번 쓰지 않고, 데이터만 바꿉니다.

## Bevy 포인트

`DefaultPlugins.set(ImagePlugin::default_nearest())`는 픽셀 아트가 흐려지지 않게 nearest sampling을 사용합니다. 작은 스프라이트를 확대하는 튜토리얼에서는 이 설정이 중요합니다.

핸들은 복사 가능한 파일 내용이 아니라 에셋 참조입니다. 같은 이미지를 여러 엔티티가 써도 각 엔티티가 이미지 전체를 새로 들고 있는 것이 아닙니다.

## 프레임 흐름

1. 시작 시 카메라를 생성합니다.
2. `AssetServer`로 `player.png`, `enemy.png`, `gem.png` 핸들을 얻습니다.
3. 각 핸들을 `Sprite`에 넣어 엔티티를 생성합니다.
4. 입력이 플레이어 위치를 바꿉니다.
5. 텍스트가 현재 플레이어 좌표와 에셋 사용 사실을 표시합니다.

## 흔한 실수

- `assets/`를 경로에 포함해 `asset_server.load("assets/player.png")`처럼 쓰지 마세요. Bevy의 기본 에셋 루트가 이미 `assets`입니다.
- 이미지가 보이지 않을 때는 파일명 대소문자와 실행 위치를 확인하세요.
- 픽셀 아트가 흐리면 `ImagePlugin::default_nearest()` 설정을 확인하세요.
- 에셋 핸들을 지역 변수로 만들었다는 이유만으로 로드가 취소되는 것은 아닙니다. 핸들이 `Sprite` 안에 저장되어 있으면 엔티티가 참조를 유지합니다.

## 작게 바꿔보기

- `enemy.png` 대신 다른 asset을 로드해보세요.
- `DisplaySpriteBundle`을 하나 더 생성하세요.
- `Transform::from_scale`로 sprite 크기를 바꿔보세요.
