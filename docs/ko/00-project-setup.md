# 0. 프로젝트 설정

<div align="center">

[목차](index.md) · [← 이전: 목차](index.md) · [다음: Bevy에 필요한 Rust →](01-rust-for-bevy.md)

</div>

---

## 이 장에서 만들 것

이 장이 끝나면 저장소는 Bevy `0.18.1`을 컴파일할 수 있는 평범한 Rust 바이너리 크레이트가 됩니다. Cargo가 어떤 파일을 읽고, 예제 파일이 어떻게 실행되는지도 같이 확인합니다.

## 실행

```sh
cargo check
cargo run --example 01_empty_app
```

기대 결과는 이렇습니다.

- `cargo check`가 성공합니다.
- `cargo run --example 01_empty_app`를 실행하면 어두운 배경의 Bevy 창이 열립니다.
- 처음 전체 빌드는 몇 분 걸릴 수 있습니다. Bevy 엔진을 로컬에서 한 번 컴파일하기 때문입니다.

## Rust 설치

Rust는 `rustup`으로 설치하고, 설치가 끝나면 버전을 확인합니다.

```sh
rustc --version
cargo --version
```

Cargo는 Rust의 빌드 도구입니다. 이 튜토리얼에서는 주로 네 가지를 담당합니다.

```text
Cargo.toml       패키지와 의존성을 선언합니다
Cargo.lock       실제로 선택된 의존성 버전을 기록합니다
examples/*.rs    장별로 독립 실행되는 예제입니다
src/main.rs      일반 앱 바이너리의 진입점입니다
```

## 프로젝트 만들기

빈 디렉토리에서 시작한다면 명령은 이렇습니다.

```sh
cargo init --bin .
cargo add bevy@0.18.1
```

이 저장소의 `Cargo.toml`도 같은 의존성을 사용합니다.

```toml
[package]
edition = "2024"

[dependencies]
bevy = "0.18.1"
```

버전을 고정하는 이유는 코드, 스크린샷, API 이름이 모두 같은 Bevy 릴리스에 맞아야 하기 때문입니다.

## 개발 프로필 추가

Bevy 프로젝트에서는 개발 빌드 프로필을 자주 조정합니다.

```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

내가 작성하는 코드는 디버깅하기 쉽게 두고, Bevy 같은 의존성 코드는 조금 더 최적화해서 실행 성능을 챙기는 설정입니다.

## 예제 하나 실행하기

첫 예제를 실행합니다.

```sh
cargo run --example 01_empty_app
```

Cargo는 `examples/01_empty_app.rs`를 찾아서 별도 실행 파일처럼 컴파일하고 실행합니다. 그래서 이 튜토리얼은 장마다 하나의 실행 가능한 파일을 둘 수 있습니다.

## Rust로 보면

Rust 프로젝트의 파일 위치에는 뚜렷한 관례가 있습니다.

```text
src/main.rs            일반 앱의 시작점
examples/name.rs       `cargo run --example name`으로 실행되는 예제
src/lib.rs             여러 예제가 같이 쓰는 라이브러리 코드
```

의존성은 이렇게 나뉩니다.

```text
Cargo.toml은 원하는 의존성을 말합니다.
Cargo.lock은 Cargo가 실제로 고른 정확한 버전을 말합니다.
```

이 튜토리얼 저장소에서는 둘 다 커밋하는 것이 맞습니다. 그래야 다른 사람이 같은 의존성 그래프로 빌드할 수 있습니다.

## Bevy로 보면

Bevy는 별도의 에디터 프로젝트 파일을 만드는 엔진이 아닙니다. Rust 코드가 `App`을 만들고, 플러그인과 시스템을 등록하고, 그 Rust 프로그램 안에서 Bevy의 엔진 루프가 실행됩니다.

## 확인

다음 두 명령이 성공하면 다음 장으로 넘어갈 준비가 된 것입니다.

```sh
cargo check
cargo run --example 01_empty_app
```

Rust 코드가 컴파일되기 전에 실패한다면 툴체인 설치나 OS 그래픽 의존성을 확인합니다. 빌드는 성공했는데 창이 열리지 않으면 그래픽 데스크톱 세션에서 실행하고 있는지 확인합니다.

## 바꿔보기

`Cargo.toml`에서 이 줄을 찾아봅니다.

```toml
bevy = "0.18.1"
```

튜토리얼 중에는 바꾸지 않습니다. 여기서 볼 점은 Bevy 버전이 숨겨진 엔진 설정이 아니라 평범한 Rust 의존성 데이터라는 사실입니다.

---

<div align="center">

[← 이전: 목차](index.md) · [목차](index.md) · [다음: Bevy에 필요한 Rust →](01-rust-for-bevy.md)

</div>
