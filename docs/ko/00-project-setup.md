# 0. 프로젝트 설정

[목차](index.md) | 이전: [목차](index.md) | 다음: [Bevy를 위한 Rust](01-rust-for-bevy.md)

이 장은 프로젝트의 기준을 정합니다. 일반 Cargo 바이너리 크레이트를 사용하고, Bevy 버전은 `0.18.1`입니다.

## 필요한 도구

Rust는 `rustup`으로 설치하고 Cargo가 동작하는지 확인합니다.

```sh
rustc --version
cargo --version
```

이 저장소의 기준은 다음과 같습니다.

```toml
[package]
edition = "2024"

[dependencies]
bevy = "0.18.1"
```

## 프로젝트 생성

처음부터 만든다면 명령은 다음과 같습니다.

```sh
cargo init --bin .
cargo add bevy@0.18.1
```

Bevy의 첫 빌드는 오래 걸립니다. 로컬에서 엔진 대부분을 컴파일하기 때문입니다. 이후 빌드는 Cargo 캐시를 사용하므로 훨씬 빨라집니다.

## 개발 프로필

이 저장소는 Bevy에서 자주 쓰는 개발 프로필을 사용합니다.

```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

앱 코드는 디버깅하기 쉽게 두고, Bevy 같은 의존성은 어느 정도 최적화해서 실행 성능을 확보합니다.

## 검증

```sh
cargo check
cargo run
```

완료 조건:

- `cargo check`가 성공한다.
- `cargo run`이 Bevy 창을 연다.
- 첫 전체 빌드는 몇 분 걸릴 수 있다.

## 중요한 점

Cargo는 빌드를 담당합니다. Bevy는 게임 루프를 담당합니다. 우리는 Bevy의 `App`에 시스템과 데이터를 등록합니다.
