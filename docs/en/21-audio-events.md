# 21. Audio Events

<div align="center">

[Index](index.md) · [← Previous: Dialogue](20-dialogue.md) · [Next: Scene loading →](22-scene-loading.md)

</div>

---

## Outcome

At the end of this chapter, gameplay systems emit audio events and one audio system turns those events into sound. Attacks, pickups, and hurt feedback produce different short sounds.

![Audio event counters show attack, pickup, and hurt sounds.](../../assets/screenshots/ch21-audio-events.png)

## Run

```sh
cargo run --example 21_audio_events
```

Move with WASD or arrow keys. Press Space for an attack sound, collect gems for pickup sounds, and press H for a hurt sound.

## Build Step 1: Define Audio As Gameplay Events

The example starts with an enum:

```rust
#[derive(Message, Debug, Clone, Copy)]
enum GameAudioEvent {
    Attack,
    Pickup,
    Hurt,
}
```

Register it:

```rust
.add_message::<GameAudioEvent>()
```

Gameplay systems should not decide exact frequencies, files, or playback settings. They should say what happened.

## Build Step 2: Emit Events From Gameplay Systems

The attack input system emits an attack sound request:

```rust
if keyboard.just_pressed(KeyCode::Space) {
    audio_events.write(GameAudioEvent::Attack);
}
```

The pickup system emits a pickup request when a gem is collected:

```rust
commands.entity(entity).despawn();
audio_events.write(GameAudioEvent::Pickup);
```

This keeps the pickup rule separate from audio playback.

## Build Step 3: Read Events In One Audio System

The audio system reads every event from this frame:

```rust
for event in events.read() {
    let frequency = match event {
        GameAudioEvent::Attack => 360.0,
        GameAudioEvent::Pickup => 720.0,
        GameAudioEvent::Hurt => 180.0,
    };
}
```

One `match` maps gameplay meaning to sound choice.

## Build Step 4: Spawn One-Shot Audio Entities

Bevy audio playback is component-based:

```rust
commands.spawn((
    AudioPlayer(pitch_assets.add(Pitch::new(frequency, Duration::from_millis(120)))),
    PlaybackSettings::DESPAWN,
));
```

`PlaybackSettings::DESPAWN` removes the audio entity after playback. This is the correct cleanup rule for short SFX.

## Build Step 5: Keep Counts For Debugging

The example also tracks how often each event played:

```rust
#[derive(Resource, Default)]
struct AudioStats {
    attack_sounds: u32,
    pickup_sounds: u32,
    hurt_sounds: u32,
}
```

This is not required for audio itself. It makes the behavior visible even when sound output is unavailable or muted.

## Build Step 6: Define The Audio Contract

This chapter uses a clear audio contract:

```text
gameplay systems emit GameAudioEvent
play_audio_events is the only system that spawns audio
short SFX use PlaybackSettings::DESPAWN
multiple events in one frame are all played
BGM would be a separate long-lived audio entity
```

The same structure works with file assets. Replace `Pitch::new(...)` with `AudioPlayer::new(asset_server.load("sounds/hit.ogg"))` or store sound handles in a resource.

## Rust Lens

`enum` is the right type when there is a closed set of sound meanings:

```rust
enum GameAudioEvent {
    Attack,
    Pickup,
    Hurt,
}
```

The `match` expression is exhaustive. If you add `GameAudioEvent::MenuSelect`, Rust will require the audio system to choose a sound for it.

## Bevy Lens

Messages decouple gameplay from presentation:

```text
attack system    writes GameAudioEvent::Attack
pickup system    writes GameAudioEvent::Pickup
audio system     reads events and spawns AudioPlayer entities
```

This is the same design idea as hitboxes and UI updates: gameplay writes facts, presentation systems react to those facts.

## Check

Run:

```sh
cargo run --example 21_audio_events
```

Expected result:

- Space produces an attack sound and increments the attack count.
- Picking up a gem produces a pickup sound and increments the pickup count.
- H produces a hurt sound and increments the hurt count.
- No sound plays when no event is emitted.
- Sound entities do not accumulate forever.

## Change

Change the pickup frequency:

```rust
GameAudioEvent::Pickup => 720.0,
```

to:

```rust
GameAudioEvent::Pickup => 960.0,
```

Expected result: pickup sounds become higher pitched without changing pickup logic.

---

<div align="center">

[← Previous: Dialogue](20-dialogue.md) · [Index](index.md) · [Next: Scene loading →](22-scene-loading.md)

</div>
