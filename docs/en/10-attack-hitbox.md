# 10. Attack Hitboxes


<div align="center">

[Index](index.md) · [← Previous: Enemy waves](09-enemy-waves.md) · [Next: Sprite assets →](11-sprite-assets.md)

</div>

---

Run:

```sh
cargo run --example 10_attack_hitbox
```

![The attack hitbox example shows the player, enemies, slash sprite, and translucent AttackHitbox Body spawned in front of the player.](../../assets/screenshots/ch10-attack-hitbox.png)

## Contract

An attack is not just a sprite. It is a short-lived entity with collision data.

```text
AttackHitbox = lifetime + damage
Body         = collision size
Transform    = where the attack exists
```

The visual slash image is only presentation.

## Data Contract

The player stores `Facing(Vec2)`. When the player attacks, the hitbox is spawned in front of the player:

```text
player position + facing direction * hitbox distance
```

The hitbox has its own `Body`, so the collision system can test it against enemies using the same AABB helper used elsewhere.

## Rust Point

`Facing(Vec2)` is a tuple struct. The `.0` field is the wrapped `Vec2`. Tuple structs are useful when one value needs a strong name in the type system.

## Bevy Point

`Commands` queues the hitbox spawn. The hitbox does not exist until Bevy applies commands, but it will be present for later systems in the schedule after command application points.

The combat set chains:

```text
attack_hits_enemies -> expire_attack_hitboxes
```

That order lets a fresh hitbox deal damage before its lifetime cleanup can remove it.

## Common Mistakes

- Checking attack collision directly inside input code mixes input and combat rules.
- Keeping old hitboxes forever causes repeated damage.
- Using only the slash image size as the collision rule makes gameplay depend on art.

## Change It

- Make hitboxes last longer.
- Increase `damage`.
- Despawn the hitbox after the first enemy only, or let it pierce multiple enemies.

---

<div align="center">

[← Previous: Enemy waves](09-enemy-waves.md) · [Index](index.md) · [Next: Sprite assets →](11-sprite-assets.md)

</div>
