# 21. Audio Events

<div align="center">

[Index](index.md) · [← Previous: Dialogue](20-dialogue.md) · [Next: Scene loading →](22-scene-loading.md)

</div>

---

## Outcome

This chapter adds audio without letting gameplay systems know how sound is played. Attack, pickup, and hurt systems emit typed messages. One audio system reads those messages and spawns short-lived audio players.

![Attack, pickup, and hurt events are staged with visible sound markers.](../../assets/screenshots/ch21-audio-events.png)

## Run

```sh
cargo run --example 21_audio_events
```

Controls:

```text
WASD / Arrow keys   move
Space               attack
touch gems          pickup event
touch enemy         hurt event
```

## Continuity Contract

Audio connects to actual gameplay events:

```text
attack input       spawns AttackHitbox and emits GameAudioEvent::Attack
gem collection     despawns Gem and emits GameAudioEvent::Pickup
enemy contact      damages player and emits GameAudioEvent::Hurt
audio system       reads events and spawns AudioPlayer entities
```

The collision and combat systems publish what happened. The audio system owns frequencies, audio handles, and playback settings.

## Build Step 1: Define Audio As A Message Type

The event payload is a small enum:

```rust
#[derive(Message, Debug, Clone, Copy)]
enum GameAudioEvent {
    Attack,
    Pickup,
    Hurt,
}
```

The derives have direct jobs:

```text
Message      Bevy can send this type through the message/event system
Debug        values can be printed during debugging
Clone/Copy   tiny event values can be duplicated cheaply
```

The app registers the message type:

```rust
.add_message::<GameAudioEvent>()
```

That registration creates the channel used by `MessageWriter<GameAudioEvent>` and `MessageReader<GameAudioEvent>`.

## Build Step 2: Emit Events From Gameplay Rules

The attack input system emits `Attack` when it creates the hitbox:

```rust
commands.spawn((AttackHitbox { ... }, ...));
audio_events.write(GameAudioEvent::Attack);
```

The pickup system emits `Pickup` only when a gem is actually collected:

```rust
if overlaps(player_transform, player_body, gem_transform, gem_body) {
    commands.entity(entity).despawn();
    audio_events.write(GameAudioEvent::Pickup);
}
```

The hurt system emits `Hurt` only when enemy contact damages the player:

```rust
health.current = (health.current - 1).max(0);
audio_events.write(GameAudioEvent::Hurt);
```

## Build Step 3: Read Events In One Audio System

One system converts gameplay events into sound:

```rust
for event in events.read() {
    let frequency = match event {
        GameAudioEvent::Attack => 360.0,
        GameAudioEvent::Pickup => 720.0,
        GameAudioEvent::Hurt => 180.0,
    };

    commands.spawn((
        AudioPlayer(pitch_assets.add(Pitch::new(frequency, Duration::from_millis(120)))),
        PlaybackSettings::DESPAWN,
    ));
}
```

The `match` is exhaustive. If you add `GameAudioEvent::MenuSelect`, Rust requires the audio system to choose what that event sounds like.

## Build Step 4: Keep Playback Entities Short-Lived

The spawned audio entity uses:

```rust
PlaybackSettings::DESPAWN
```

That tells Bevy to remove the audio entity after playback. Gameplay systems should not track one-shot sound lifetimes.

## Build Step 5: Make Audio Visible

The example also updates `AudioStats`:

```rust
#[derive(Resource, Default)]
struct AudioStats {
    attack_sounds: u32,
    pickup_sounds: u32,
    hurt_sounds: u32,
}
```

The counters are a teaching aid. They prove that audio events came from real gameplay rules even if the machine is muted.

## Integration Points

The audio feature should attach to gameplay facts, not input keys:

```text
attack system       writes Attack
pickup system       writes Pickup
damage system       writes Hurt
audio system        reads all GameAudioEvent values
UI/debug system     may read AudioStats for visibility
```

For file-based sounds, replace `Pitch::new(...)` with handles from `AssetServer`, for example `asset_server.load("sounds/hit.ogg")`. The event contract stays the same.

## Rust Lens

An enum is a closed vocabulary:

```rust
enum GameAudioEvent {
    Attack,
    Pickup,
    Hurt,
}
```

This is stronger than sending strings like `"attack"`. Misspelling a variant is a compile error. Adding a variant forces every exhaustive `match` to be updated.

Messages are also typed:

```rust
MessageWriter<GameAudioEvent>
MessageReader<GameAudioEvent>
```

Those system parameters say exactly which event stream the system writes or reads.

## Check

Run:

```sh
cargo run --example 21_audio_events
```

Expected result:

- `Space` spawns an attack hitbox and increments the attack sound counter.
- Touching a gem despawns it and increments the pickup sound counter.
- Touching the enemy damages the player and increments the hurt sound counter.
- Audio entities do not accumulate forever.
- The UI counters match the gameplay events.

## Change

Change the pickup frequency:

```rust
GameAudioEvent::Pickup => 720.0,
```

to:

```rust
GameAudioEvent::Pickup => 960.0,
```

Expected result: pickup sounds become higher pitched without changing the pickup system.

---

<div align="center">

[← Previous: Dialogue](20-dialogue.md) · [Index](index.md) · [Next: Scene loading →](22-scene-loading.md)

</div>
