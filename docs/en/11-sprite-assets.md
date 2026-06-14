# 11. Sprite Assets


<div align="center">

[Index](index.md) · [← Previous: Attack hitboxes](10-attack-hitbox.md) · [Next: Screen-space UI →](12-screen-space-ui.md)

</div>

---

Run:

```sh
cargo run --example 11_sprite_assets
```

![The sprite asset example shows the player, enemy, and gem loaded from image files through AssetServer handles.](../../assets/screenshots/ch11-sprite-assets.png)

## Contract

Prototype sprites can be colored rectangles. Domain sprites should move to image assets once identity matters.

```text
asset path  = relative to assets/
Handle<T>   = Bevy's reference to a loaded asset
Sprite      = component that renders an image handle
```

## Data Contract

`PlayerBundle::new(asset_server)` loads `player.png`. `DisplaySpriteBundle::new(path, position, asset_server)` demonstrates the same rule for `enemy.png` and `gem.png`.

The bundle owns the spawn shape. The system decides when to spawn it.

## Rust Point

The asset path is `&'static str` in `DisplaySpriteBundle::new`. That means the string literal lives for the whole program. It is appropriate for fixed tutorial asset paths.

## Bevy Point

`asset_server.load("player.png")` resolves to:

```text
assets/player.png
```

The returned handle can be stored in a component immediately. Bevy may finish loading the image asynchronously.

## Common Mistakes

- Writing `assets/player.png` inside `asset_server.load`; Bevy already starts from the asset root.
- Loading the same image path in every frame.
- Treat handles as asset-system references, and keep image bytes managed by Bevy.

## Change It

- Replace `enemy.png` with another asset.
- Add a new `DisplaySpriteBundle` spawn.
- Scale a sprite with `Transform::from_scale`.

---

<div align="center">

[← Previous: Attack hitboxes](10-attack-hitbox.md) · [Index](index.md) · [Next: Screen-space UI →](12-screen-space-ui.md)

</div>
