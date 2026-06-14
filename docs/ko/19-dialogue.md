# 19. 대화

<div align="center">

[목차](index.md) · [← 이전: 인벤토리](18-inventory.md) · [다음: 오디오 이벤트 →](20-audio-events.md)

</div>

---

## 이 장에서 만들 것

RPG 루프에 대화 모드를 추가합니다. NPC는 자기 이름과 대사를 가집니다. `DialogueState`는 현재 어떤 NPC와 몇 번째 줄을 보고 있는지 저장합니다. `GameState::Dialogue`에서는 일반 이동이 멈추고, 대화 입력이 줄 넘김과 닫기를 처리합니다.

![NPC와 대화하면 화면 아래 대화 패널이 열리는 장면](../../assets/screenshots/ch19-dialogue.png)

## 실행

```sh
cargo run --example 19_dialogue
```

조작:

```text
WASD / 방향키   이동
E               가까운 NPC와 대화
Space           다음 대사
Esc             대화 닫기
```

## 이어받는 계약

대화는 앞에서 배운 상태 시스템으로 다룹니다.

```text
GameState::Playing    이동과 탐색
GameState::Dialogue   대화 입력과 대화 UI
DialogueState         현재 대화 중인 NPC와 줄 번호
Npc 컴포넌트           화자 이름과 대사 데이터
```

모드는 `GameState`가 소유합니다. 대화 내용은 `Npc`가 소유합니다. 현재 진행 중인 대화는 `DialogueState`가 소유합니다.

## 구현 흐름 1: 대화 상태 추가하기

상태 enum에 두 모드를 둡니다.

```rust
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Playing,
    Dialogue,
}
```

derive들은 Bevy 상태 계약을 만족시킵니다. `Default`는 시작 상태를 `Playing`으로 정합니다.

## 구현 흐름 2: NPC에 대사 넣기

NPC는 화자 데이터를 가집니다.

```rust
#[derive(Component)]
struct Npc {
    name: &'static str,
    lines: &'static [&'static str],
}
```

이 예제의 대사는 Rust 코드 안에 직접 적혀 있으므로 문자열 리터럴을 빌려서 쓸 수 있습니다. 문자열 리터럴의 타입은 `&'static str`입니다. 프로그램이 실행되는 동안 계속 살아 있는 문자열 조각을 빌린다는 뜻입니다.

문자열 소유권은 이렇게 판단합니다.

```text
소스 코드에 직접 적은 고정 텍스트      &'static str
어딘가에서 잠깐 빌린 텍스트           해당 lifetime을 가진 &str
파일이나 네트워크에서 읽은 텍스트      String
코드에 직접 적은 대사 배열            &'static [&'static str]
씬 파일에서 읽은 대사                 Vec<String>
```

21장은 JSON을 파싱해서 실행 중에 문자열을 만들기 때문에 `Vec<String>`을 씁니다.

## 구현 흐름 3: 현재 대화 저장하기

현재 진행 중인 대화는 리소스로 둡니다.

```rust
#[derive(Resource, Default)]
struct DialogueState {
    active_npc: Option<Entity>,
    line_index: usize,
}
```

NPC는 대사를 소유하고, 리소스는 지금 어떤 대화가 열려 있는지 소유합니다.

## 구현 흐름 4: 상태로 이동 막기

이동 시스템은 현재 상태를 읽습니다.

```rust
if *state.get() != GameState::Playing {
    return;
}
```

여러 시스템이 같은 앱 모드를 기준으로 판단할 수 있으므로, 임의의 boolean을 여기저기 두는 것보다 구조가 분명합니다.

## 구현 흐름 5: 시작, 넘김, 종료 규칙 만들기

입력 시스템이 상태 전환 규칙을 가집니다.

```text
Playing에서 NPC 근처 E      active_npc = Some(entity), state = Dialogue
Dialogue에서 Space          line_index += 1
마지막 줄을 넘김             active_npc = None, state = Playing
Esc                         active_npc = None, state = Playing
```

선택된 NPC는 `Entity` ID로 저장합니다.

```rust
dialogue.active_npc = Some(entity);
dialogue.line_index = 0;
next_state.set(GameState::Dialogue);
```

## 구현 흐름 6: 대화 UI 그리기

UI 시스템은 `DialogueState`와 활성 `Npc`를 읽습니다.

```rust
let Some(entity) = dialogue.active_npc else {
    text.0.clear();
    return;
};

let Ok(npc) = npcs.get(entity) else {
    text.0.clear();
    return;
};
```

`let else`를 쓰면 활성 대화가 있는 경우를 들여쓰기 깊게 넣지 않고 바로 읽을 수 있습니다. 대화가 없으면 텍스트를 지우고 시스템을 끝냅니다.

## 통합 지점

대화는 이동 코드 안에 직접 끼어드는 것이 아니라 상태로 RPG 루프에 연결됩니다.

```text
Input      E/Space/Esc가 대화 전환을 결정
Movement   GameState::Playing일 때만 실행
Ui         현재 상태에 맞게 안내 문구와 대화 패널 표시
Scenes     로드된 NPC도 name + lines 데이터 모양을 사용
```

대화 중 전투도 멈추고 싶다면 전투 시스템에도 `in_state(GameState::Playing)`을 걸면 됩니다. 일시정지 메뉴가 대화보다 우선해야 한다면 그 규칙은 입력 상태 전환 시스템에서 정합니다.

## Rust로 보면

`Option<Entity>`는 “대화 없음”과 “이 NPC와 대화 중”을 나타냅니다.

```rust
active_npc: Option<Entity>
```

`Entity`는 참조가 아니라 ID입니다. 프레임을 넘어 오래 빌리는 대신 ID만 저장하고, 매 프레임 UI 시스템이 그 ID로 현재 NPC 데이터를 조회합니다.

`&'static str`에는 lifetime 문법이 들어 있습니다.

```rust
name: &'static str
```

작은따옴표로 시작하는 이름은 빌림이 얼마나 오래 유효한지를 나타냅니다. `'static`은 프로그램 전체 동안 데이터가 살아 있다는 뜻입니다. 코드에 직접 적은 문자열 리터럴은 프로그램 바이너리 안에 들어가므로 여기서 `'static`을 씁니다.

## 확인

실행합니다.

```sh
cargo run --example 19_dialogue
```

확인 기준:

- NPC 근처에서 안내 문구가 나옵니다.
- `E`를 누르면 대화 패널이 열리고 `GameState::Dialogue`로 들어갑니다.
- `Space`를 누르면 NPC 대사가 다음 줄로 넘어갑니다.
- `Esc`를 누르면 대화가 닫히고 `GameState::Playing`으로 돌아갑니다.
- 대화 중에는 이동이 멈춥니다.

## 바꿔보기

Mapper NPC에 대사를 하나 더 추가합니다.

```rust
"Dialogue data can grow without changing the UI system.",
```

기대 결과: 그 NPC와 대화할 때 `Space`로 한 줄을 더 넘길 수 있습니다.

---

<div align="center">

[← 이전: 인벤토리](18-inventory.md) · [목차](index.md) · [다음: 오디오 이벤트 →](20-audio-events.md)

</div>
