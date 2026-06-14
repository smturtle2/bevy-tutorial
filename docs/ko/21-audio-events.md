# 21. 오디오 이벤트

<div align="center">

[목차](index.md) · [← 이전: 대화](20-dialogue.md) · [다음: 씬 로딩 →](22-scene-loading.md)

</div>

---

## 이 장에서 만들 것

이 장이 끝나면 게임플레이 시스템이 오디오 이벤트를 보내고, 오디오 시스템 하나가 그 이벤트를 소리로 바꿉니다. 공격, 아이템 수집, 피격 피드백이 서로 다른 짧은 소리를 냅니다.

![공격, 수집, 피격 오디오 이벤트 횟수를 보여주는 장면](../../assets/screenshots/ch21-audio-events.png)

## 실행

```sh
cargo run --example 21_audio_events
```

WASD나 방향키로 움직입니다. Space는 공격 소리, 보석 수집은 수집 소리, H는 피격 소리를 냅니다.

## 구현 흐름 1: 오디오를 게임 이벤트로 정의하기

먼저 enum을 만듭니다.

```rust
#[derive(Message, Debug, Clone, Copy)]
enum GameAudioEvent {
    Attack,
    Pickup,
    Hurt,
}
```

그리고 App에 등록합니다.

```rust
.add_message::<GameAudioEvent>()
```

게임플레이 시스템이 정확한 주파수, 파일 경로, 재생 설정까지 알 필요는 없습니다. 게임플레이 시스템은 “무슨 일이 일어났는지”만 말하면 됩니다.

## 구현 흐름 2: 게임플레이 시스템에서 이벤트 보내기

공격 입력 시스템은 공격 소리 요청을 보냅니다.

```rust
if keyboard.just_pressed(KeyCode::Space) {
    audio_events.write(GameAudioEvent::Attack);
}
```

수집 시스템은 보석을 주웠을 때 수집 소리 요청을 보냅니다.

```rust
commands.entity(entity).despawn();
audio_events.write(GameAudioEvent::Pickup);
```

이렇게 하면 수집 규칙과 소리 재생 규칙이 섞이지 않습니다.

## 구현 흐름 3: 오디오 시스템 하나에서 이벤트 읽기

오디오 시스템은 이번 프레임에 들어온 이벤트를 모두 읽습니다.

```rust
for event in events.read() {
    let frequency = match event {
        GameAudioEvent::Attack => 360.0,
        GameAudioEvent::Pickup => 720.0,
        GameAudioEvent::Hurt => 180.0,
    };
}
```

`match` 하나가 게임 의미를 실제 소리 선택으로 바꿉니다.

## 구현 흐름 4: 일회성 오디오 엔티티 생성하기

Bevy 오디오 재생도 컴포넌트 기반입니다.

```rust
commands.spawn((
    AudioPlayer(pitch_assets.add(Pitch::new(frequency, Duration::from_millis(120)))),
    PlaybackSettings::DESPAWN,
));
```

`PlaybackSettings::DESPAWN`은 재생이 끝난 뒤 오디오 엔티티를 제거합니다. 짧은 효과음에는 이 정리 규칙이 맞습니다.

## 구현 흐름 5: 디버깅용 카운트 저장하기

예제는 각 소리가 몇 번 재생됐는지도 저장합니다.

```rust
#[derive(Resource, Default)]
struct AudioStats {
    attack_sounds: u32,
    pickup_sounds: u32,
    hurt_sounds: u32,
}
```

소리 자체에 꼭 필요한 값은 아닙니다. 다만 음소거 환경에서도 이벤트가 제대로 흐르는지 화면에서 확인할 수 있습니다.

## 구현 흐름 6: 오디오 계약 정하기

이 장의 오디오 규칙은 다음과 같습니다.

```text
게임플레이 시스템은 GameAudioEvent를 보냄
play_audio_events만 오디오 엔티티를 생성함
짧은 효과음은 PlaybackSettings::DESPAWN 사용
같은 프레임에 여러 이벤트가 오면 전부 재생
BGM은 별도의 오래 살아 있는 오디오 엔티티로 관리
```

파일 기반 사운드에도 같은 구조를 씁니다. `Pitch::new(...)` 대신 `AudioPlayer::new(asset_server.load("sounds/hit.ogg"))`를 쓰거나, 사운드 핸들을 리소스에 저장하면 됩니다.

## Rust로 보면

정해진 소리 의미 집합에는 enum이 잘 맞습니다.

```rust
enum GameAudioEvent {
    Attack,
    Pickup,
    Hurt,
}
```

`match`는 모든 variant를 처리해야 합니다. 나중에 `GameAudioEvent::MenuSelect`를 추가하면 Rust가 오디오 시스템에서 그 소리를 어떻게 처리할지 요구합니다.

## Bevy로 보면

메시지는 게임플레이와 표현을 분리합니다.

```text
공격 시스템    GameAudioEvent::Attack 전송
수집 시스템    GameAudioEvent::Pickup 전송
오디오 시스템  이벤트를 읽고 AudioPlayer 엔티티 생성
```

히트박스나 UI와 같은 설계 감각입니다. 게임플레이는 사실을 만들고, 표현 시스템은 그 사실에 반응합니다.

## 확인

실행합니다.

```sh
cargo run --example 21_audio_events
```

확인 기준:

- Space를 누르면 공격 소리가 나고 공격 카운트가 올라갑니다.
- 보석을 주우면 수집 소리가 나고 수집 카운트가 올라갑니다.
- H를 누르면 피격 소리가 나고 피격 카운트가 올라갑니다.
- 이벤트가 없으면 소리가 나지 않습니다.
- 소리 엔티티가 계속 쌓이지 않습니다.

## 바꿔보기

수집 소리 주파수를 바꿉니다.

```rust
GameAudioEvent::Pickup => 720.0,
```

이렇게 바꿉니다.

```rust
GameAudioEvent::Pickup => 960.0,
```

기대 결과: 수집 소리가 더 높은 음으로 들립니다. 수집 로직은 바뀌지 않습니다.

---

<div align="center">

[← 이전: 대화](20-dialogue.md) · [목차](index.md) · [다음: 씬 로딩 →](22-scene-loading.md)

</div>
