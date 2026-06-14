# 0. Project Setup

<div align="center">

[Index](index.md) · [← Previous: Index](index.md) · [Next: Rust for Bevy →](01-rust-for-bevy.md)

</div>

---

## Outcome

At the end of this chapter, the repository is a normal Rust binary crate that can compile Bevy `0.18.1`. You will also know which files Cargo reads when it builds the examples.

## Run

```sh
cargo check
cargo run --example 01_empty_app
```

Expected result:

- `cargo check` finishes successfully.
- `cargo run --example 01_empty_app` opens a Bevy window with a dark background.
- The first full build may take several minutes because Cargo is compiling Bevy locally.

## Install Rust

Install Rust with `rustup`, then confirm the toolchain:

```sh
rustc --version
cargo --version
```

Cargo is Rust's build tool. In this tutorial Cargo does four jobs:

```text
Cargo.toml       declares the package and dependencies
Cargo.lock       records exact dependency versions
examples/*.rs    builds standalone tutorial examples
src/main.rs      builds the normal app binary
```

## Create The Project

From an empty directory, the project starts like this:

```sh
cargo init --bin .
cargo add bevy@0.18.1
```

This repository uses the same dependency directly in `Cargo.toml`:

```toml
[package]
edition = "2024"

[dependencies]
bevy = "0.18.1"
```

The version is pinned so the tutorial code, screenshots, and API names all match one Bevy release.

## Add The Bevy Development Profile

Bevy projects commonly tune the development profile:

```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

This keeps your own code easy to debug while compiling dependencies such as Bevy with more optimization. The result is still a debug build, but the engine code runs more smoothly.

## Build One Example

Run the first example:

```sh
cargo run --example 01_empty_app
```

Cargo looks for `examples/01_empty_app.rs`, compiles it as its own executable, and runs it. That is why this tutorial can keep each chapter as a runnable file.

## Rust Lens

The first Rust rule is file ownership by convention:

```text
src/main.rs            normal app entry point
examples/name.rs       runnable example named `name`
src/lib.rs             shared library code imported by examples
```

The second rule is dependency ownership:

```text
Cargo.toml says what you want.
Cargo.lock says exactly what Cargo chose.
```

Commit both for this tutorial repository so another reader gets the same dependency graph.

## Bevy Lens

Bevy is a library dependency. There is no separate Bevy editor project file here. Your Rust code creates an `App`, registers plugins and systems, and Bevy runs the engine loop from that Rust program.

## Check

You are ready for chapter 1 when these commands work:

```sh
cargo check
cargo run --example 01_empty_app
```

If the build fails before Rust code is compiled, check toolchain installation and OS graphics dependencies. If the app builds but no window opens, run the command from a graphical desktop session.

## Change

Open `Cargo.toml` and find `bevy = "0.18.1"`. Do not change it for the tutorial. The point is to notice that the Bevy version is ordinary Rust dependency data, not hidden engine state.

---

<div align="center">

[← Previous: Index](index.md) · [Index](index.md) · [Next: Rust for Bevy →](01-rust-for-bevy.md)

</div>
