# 20. Dialogue

<div align="center">

[Index](index.md) · [← Previous: Inventory](19-inventory.md) · [Next: Audio events →](21-audio-events.md)

</div>

---

## Outcome

At the end of this chapter, the player can approach NPCs, start dialogue, advance lines, and close the conversation. Dialogue text appears in screen-space UI while the world remains ECS-driven.

![A dialogue panel opens when the player talks to an NPC.](../../assets/screenshots/ch20-dialogue.png)

## Run

```sh
cargo run --example 20_dialogue
```

Move near an NPC with WASD or arrow keys. Press E to talk, Space to advance, and Esc to close.

## Build Step 1: Put Lines On The NPC

An NPC owns its static dialogue data:

```rust
#[derive(Component)]
struct Npc {
    name: &'static str,
    lines: &'static [&'static str],
}
```

The lines are string slices with a `'static` lifetime because the example writes them directly in code:

```rust
Npc {
    name: "Mapper",
    lines: &[
        "A scene file can decide where I stand.",
        "Code decides what talking to me means.",
    ],
}
```

For dialogue loaded from files, you would use `String` values instead.

## Build Step 2: Store The Current Conversation In A Resource

The current dialogue is global mode-like state:

```rust
#[derive(Resource, Default)]
struct DialogueState {
    active_npc: Option<Entity>,
    line_index: usize,
}
```

This does not belong on the NPC. The NPC has lines. The conversation resource says which NPC is currently active and which line is visible.

## Build Step 3: Detect A Nearby NPC

The prompt system searches NPCs near the player:

```rust
let nearby = npcs.iter().find(|(transform, _)| {
    player
        .translation
        .truncate()
        .distance(transform.translation.truncate())
        <= INTERACT_DISTANCE
});
```

If an NPC is near, the UI shows an interaction prompt. If no NPC is near, it tells the player to move closer.

## Build Step 4: Start, Advance, And End Dialogue

The input system has three branches:

```text
Esc while talking       close dialogue
Space while talking     advance to the next line
E while near an NPC     start dialogue
```

The active NPC is stored as an `Entity`:

```rust
dialogue.active_npc = Some(entity);
dialogue.line_index = 0;
```

When Space advances past the last line, the dialogue closes:

```rust
if dialogue.line_index >= npc.lines.len() {
    dialogue.active_npc = None;
    dialogue.line_index = 0;
}
```

## Build Step 5: Pause Movement During Dialogue

The movement system reads `DialogueState`:

```rust
if dialogue.active_npc.is_some() {
    return;
}
```

This is a simple mode gate. In a larger game, this same idea usually becomes a Bevy `State`, such as `GameState::Dialogue`.

## Build Step 6: Render The Dialogue Panel

The dialogue UI is a normal Bevy UI entity:

```rust
commands.spawn((
    DialogueText,
    Text::new(""),
    Node {
        position_type: PositionType::Absolute,
        bottom: px(28),
        left: px(32),
        right: px(32),
        padding: UiRect::all(px(14)),
        ..default()
    },
    BackgroundColor(Color::srgba(0.06, 0.07, 0.10, 0.88)),
));
```

The update system writes the speaker and line:

```rust
text.0 = format!("{}:\n{}", npc.name, line);
```

## Rust Lens

`Option<Entity>` represents conversation state:

```rust
active_npc: Option<Entity>
```

The code uses `let else` to leave early when no dialogue is active:

```rust
let Some(entity) = dialogue.active_npc else {
    text.0.clear();
    return;
};
```

That keeps the active case flat and readable.

## Bevy Lens

Dialogue crosses three ECS responsibilities:

```text
NPC component       static speaker data
Dialogue resource   current conversation state
UI entity           screen-space presentation
```

Do not store the current line index inside every NPC. Only one conversation is active, so one resource is the clearer owner.

## Check

Run:

```sh
cargo run --example 20_dialogue
```

Expected result:

- A prompt appears near an NPC.
- E opens the dialogue panel.
- Space advances through the NPC's lines.
- Esc closes the dialogue.
- Movement pauses while dialogue is active.

## Change

Add a new line to the Mapper NPC:

```rust
"Dialogue data can grow without changing the UI system.",
```

Expected result: Space now advances through one more line for that NPC.

---

<div align="center">

[← Previous: Inventory](19-inventory.md) · [Index](index.md) · [Next: Audio events →](21-audio-events.md)

</div>
