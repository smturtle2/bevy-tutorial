# 0. Project Setup

[Index](index.md) | Previous: [Index](index.md) | Next: [Rust for Bevy](01-rust-for-bevy.md)

This chapter establishes the project contract. We use a normal Cargo binary crate and Bevy `0.18.1`.

## Required Tools

Install Rust through `rustup` and confirm Cargo works:

```sh
rustc --version
cargo --version
```

This repository uses:

```toml
[package]
edition = "2024"

[dependencies]
bevy = "0.18.1"
```

## Create The Project

From scratch, the commands are:

```sh
cargo init --bin .
cargo add bevy@0.18.1
```

Bevy's first build is slow because you are compiling a large engine locally. Later builds are much faster because Cargo reuses compiled artifacts.

## Development Profile

This repo uses a common Bevy dev profile:

```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

The app code remains debuggable, while dependencies such as Bevy are optimized enough to run smoothly.

## Verify

Run:

```sh
cargo check
cargo run
```

Expected result:

- `cargo check` succeeds.
- `cargo run` opens a Bevy window.
- The first full build may take several minutes.

## What Matters

Cargo owns the build. Bevy owns the game loop. Your job is to register systems and data with Bevy's `App`.
