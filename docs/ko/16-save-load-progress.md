# 16. 진행도 저장과 불러오기


<div align="center">

[목차](index.md) · [← 이전: 게임 상태](15-game-states.md) · [다음: 완성 RPG 조각 →](17-complete-rpg-slice.md)

</div>

---

실행:

```sh
cargo run --example 16_save_load_progress
```

![진행도 저장/불러오기 예제는 화면에 영구 Progress 데이터, 단축키, JSON 저장 경로를 표시합니다.](../../assets/screenshots/ch16-save-load-progress.png)

이 장의 계약은 저장 가능한 게임 진행도를 Bevy 리소스로 들고, 필요할 때 JSON 파일로 직렬화하는 것입니다. 예제는 `target/tutorial-save/progress.json`을 사용합니다.

## 핵심 ECS 계약

- `Progress`: 저장할 데이터입니다. 최고 점수, 해금 웨이브, 플레이어 이름을 가집니다.
- `SaveStatus`: 마지막 저장/불러오기 결과 메시지입니다.
- `ProgressText`, `SaveStatusText`: UI 텍스트를 찾는 마커입니다.
- `edit_progress`: 키 입력으로 메모리의 진행도를 바꾸거나 저장/불러오기를 실행합니다.
- `update_text`: 리소스 상태를 화면에 표시합니다.

저장 파일은 ECS 월드 밖에 있습니다. ECS 안에서는 `Progress` 리소스가 원본이고, 파일은 그 원본을 오래 보관하기 위한 입출력 결과입니다.

## Rust 포인트

`#[derive(Serialize, Deserialize)]`는 `serde`가 `Progress`를 JSON 문자열로 바꾸고 다시 구조체로 복원할 수 있게 합니다. `Debug`, `Clone`은 디버그와 값 복사가 필요할 때 유용합니다.

`load_progress_from_disk`는 실패를 정상 경로로 처리합니다.

```rust
fs::read_to_string(SAVE_PATH)
    .ok()
    .and_then(|text| serde_json::from_str(&text).ok())
    .unwrap_or_default()
```

파일이 없거나 JSON이 깨졌으면 기본 진행도로 시작합니다. 튜토리얼 예제에서는 시작 실패보다 복구 가능한 기본값이 낫습니다.

## Bevy 포인트

`.insert_resource(load_progress_from_disk())`는 앱 시작 시 디스크에서 읽은 값을 월드 리소스로 넣습니다. 이후 시스템은 파일을 매 프레임 읽지 않고 `Res`/`ResMut`로 리소스를 다룹니다.

`ResMut<Progress>`에 `*progress = load_progress_from_disk();`를 대입하면 리소스 내부 값을 통째로 교체합니다.

## 프레임 흐름

1. 시작 시 저장 파일을 읽어 `Progress` 리소스를 만듭니다.
2. S/W 키가 메모리의 진행도 값을 바꿉니다.
3. F5가 현재 리소스를 JSON 파일로 저장합니다.
4. F9가 파일에서 다시 읽어 리소스를 교체합니다.
5. Delete가 기본값으로 되돌리고 저장 파일을 제거합니다.
6. UI가 현재 진행도와 상태 메시지를 표시합니다.

## 흔한 실수

- 저장 전에 부모 디렉터리를 만들지 않으면 `fs::write`가 실패합니다. 예제는 `fs::create_dir_all(parent)`를 먼저 호출합니다.
- 파일을 저장했다고 메모리 값이 자동으로 바뀌지는 않습니다. 원본은 여전히 `Progress` 리소스입니다.
- `unwrap()`으로 파일 입출력을 처리하면 저장 파일이 없는 첫 실행에서 앱이 종료될 수 있습니다.
- `target/` 아래 저장은 튜토리얼용입니다. 배포 게임에서는 플랫폼별 사용자 데이터 경로를 써야 합니다.

## 작게 바꿔보기

- `Progress`에 `gold` 필드를 추가하세요.
- `last_selected_character` 같은 문자열 필드를 저장해보세요.
- F5가 아니라 게임오버 시점에 자동 저장하도록 바꿔보세요.

---

<div align="center">

[← 이전: 게임 상태](15-game-states.md) · [목차](index.md) · [다음: 완성 RPG 조각 →](17-complete-rpg-slice.md)

</div>
