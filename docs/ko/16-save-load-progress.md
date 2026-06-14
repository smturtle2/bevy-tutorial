# 16. 진행도 저장과 불러오기

<div align="center">

[목차](index.md) · [← 이전: 게임 상태](15-game-states.md) · [다음: 완성된 RPG 예제 →](17-complete-rpg-slice.md)

</div>

---

## 이 장에서 만들 것

이 장이 끝나면 명시적인 진행도 데이터를 JSON으로 저장하고, 키 입력으로 저장/불러오기/초기화를 할 수 있습니다.

![진행도 데이터가 메모리에서 바뀌고 JSON 파일로 저장됩니다.](../../assets/screenshots/ch16-save-load-progress.png)

## 실행

```sh
cargo run --example 16_save_load_progress
```

조작은 이렇습니다.

```text
S       메모리에서 score 증가
W       메모리에서 unlocked wave 증가
F5      진행도 저장
F9      진행도 불러오기
Delete  진행도 초기화 및 저장 파일 삭제
```

저장 경로는 다음과 같습니다.

```text
target/tutorial-save/progress.json
```

## 구현 흐름 1: 월드 전체가 아니라 진행도만 저장하기

진행도 리소스는 직렬화 가능한 평범한 데이터입니다.

```rust
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
struct Progress {
    best_score: u32,
    unlocked_wave: u32,
    player_name: String,
}
```

기본값도 명확히 둡니다.

```rust
impl Default for Progress {
    fn default() -> Self {
        Self {
            best_score: 0,
            unlocked_wave: 1,
            player_name: "Bevy Learner".to_string(),
        }
    }
}
```

저장 데이터는 일부러 작게 둡니다. 카메라 transform, UI entity, 애셋 핸들, 임시 적 같은 것은 저장하지 않습니다.

## 구현 흐름 2: 앱 시작 때 불러오기

앱 등록에서 디스크의 진행도를 리소스로 넣습니다.

```rust
.insert_resource(load_progress_from_disk())
```

불러오기 함수는 파일 읽기, JSON 파싱, 기본값 순서로 처리합니다.

```rust
fn load_progress_from_disk() -> Progress {
    fs::read_to_string(SAVE_PATH)
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}
```

흐름은 이렇습니다.

```text
파일을 읽을 수 있음 -> JSON 파싱 시도
파싱 성공           -> 파싱된 Progress 사용
어디선가 실패        -> Progress::default() 사용
```

## 구현 흐름 3: 메모리의 진행도 바꾸기

수정 시스템은 리소스를 수정합니다.

```rust
fn edit_progress(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut progress: ResMut<Progress>,
    mut status: ResMut<SaveStatus>,
) {
    if keyboard.just_pressed(KeyCode::KeyS) {
        progress.best_score += 100;
        status.message = "Changed best_score in memory".to_string();
    }
}
```

리소스를 바꾼다고 자동으로 파일에 저장되지는 않습니다. 디스크 IO는 저장 키를 눌렀을 때만 일어납니다.

## 구현 흐름 4: `Result`로 저장 실패 처리하기

저장 함수는 `Result<(), String>`을 반환합니다.

```rust
fn save_progress_to_disk(progress: &Progress) -> Result<(), String> {
    let Some(parent) = Path::new(SAVE_PATH).parent() else {
        return Err("save path has no parent directory".to_string());
    };

    fs::create_dir_all(parent).map_err(|error| error.to_string())?;

    let json = serde_json::to_string_pretty(progress).map_err(|error| error.to_string())?;
    fs::write(SAVE_PATH, json).map_err(|error| error.to_string())
}
```

디렉토리를 만들고, JSON으로 변환하고, 파일을 씁니다. 실패하면 오류 메시지를 돌려줍니다.

## 구현 흐름 5: 저장 상태를 UI에 보여주기

입력 시스템은 저장 결과를 상태 메시지로 바꿉니다.

```rust
if keyboard.just_pressed(KeyCode::F5) {
    match save_progress_to_disk(&progress) {
        Ok(()) => status.message = format!("Saved to {SAVE_PATH}"),
        Err(error) => status.message = format!("Save failed: {error}"),
    }
}
```

표시 시스템은 진행도와 상태 메시지를 모두 화면에 씁니다.

```rust
progress_text.0 = format!(
    "Progress\nbest_score: {}\nunlocked_wave: {}\nplayer_name: {}",
    progress.best_score, progress.unlocked_wave, progress.player_name
);
```

## Rust로 보면

`Serialize`와 `Deserialize`는 `serde`에서 옵니다.

```rust
use serde::{Deserialize, Serialize};
```

이 trait 덕분에 `serde_json`이 Rust struct와 JSON 문자열을 서로 변환할 수 있습니다.

`?`는 오류 전파 문법입니다.

```rust
fs::create_dir_all(parent).map_err(|error| error.to_string())?;
```

디렉토리 생성이 실패하면 함수는 바로 `Err(...)`를 반환합니다.

## Bevy로 보면

진행도는 앱 전체에 하나인 저장 프로필이므로 리소스가 맞습니다.

```text
Progress resource     오래 유지되는 플레이어 진행도
SaveStatus resource   저장/불러오기 피드백
ProgressText entity   진행도 표시
SaveStatusText entity 저장 상태 표시
```

저장은 명시적인 데이터 구조를 대상으로 해야 합니다. ECS 월드 전체에는 임시 데이터와 엔진 소유 데이터가 많이 들어 있으므로, 보통 그대로 저장 대상이 아닙니다.

## 확인

실행합니다.

```sh
cargo run --example 16_save_load_progress
```

기대 결과:

- `S`, `W`를 누르면 값이 바뀝니다.
- `F5`를 누르면 `target/tutorial-save/` 아래에 JSON 파일이 생깁니다.
- 예제를 다시 실행하면 저장된 값이 로드됩니다.
- Delete를 누르면 값이 초기화되고 파일이 삭제됩니다.

## 바꿔보기

필드를 하나 추가해 봅니다.

```rust
gold: u32,
```

그다음 `Default`, `update_text`, 필요한 저장 상태 표시를 함께 수정합니다. 기대 결과: 저장하면 JSON 파일에 새 필드가 포함됩니다.

---

<div align="center">

[← 이전: 게임 상태](15-game-states.md) · [목차](index.md) · [다음: 완성된 RPG 예제 →](17-complete-rpg-slice.md)

</div>
