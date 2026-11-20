# rs-to-readme

[![crates.io](https://img.shields.io/crates/v/rs-to-readme.svg)](https://crates.io/crates/rs-to-readme) [![docs.rs](https://docs.rs/rs-to-readme/badge.svg)](https://docs.rs/rs-to-readme) ![license](https://img.shields.io/badge/license-MIT-blue.svg) [![CI](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml/badge.svg)](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml)

📖 다른 언어: [日本語](README-Japan.md) / [English](README-English.md) / [中文](README-Chinese.md) /
[Español](README-Spain.md) / [Français](README-France.md) / [Deutsch](README-Germany.md) /
[Italiano](README-Italy.md) / [Русский](README-Russia.md) / [العربية](README-Arabic.md) ·
다른 프로젝트로의 이식은 **[PORTING.md](PORTING.md)** 참조.

Rust crate의 `Cargo.toml` 메타데이터와 rustdoc 주석으로부터 부족함 없는
README.md를 자동 생성하는 라이브러리 + CLI 도구입니다.

**설계 방침**: 소스 코드(`Cargo.toml` 메타데이터, crate 최상위 `//!` 문서
주석, 각 공개 API의 `///` 주석)를 유일한 정본으로 삼고, README.md는 그로부터
기계적으로 도출합니다. 이를 통해 코드와 문서가 어긋나는(README 갱신을
잊어버리는) 전형적인 문제를 구조적으로 방지합니다.

CI 등에서 `--check` 모드를 사용하면 생성 결과와 기존 README.md가 일치하는지
검증할 수 있어, "코드는 바꿨지만 README 갱신을 잊은" PR을 기계적으로
탐지할 수 있습니다.

## Installation

```bash
cargo install rs-to-readme
```

## API Overview

- `RsToReadmeError`(enum)
- `Result`(type)
- `generate_readme`(fn) — crate 루트 디렉터리(`Cargo.toml`이 있는 곳)에서
  README.md 본문을 생성합니다. `entry_file`에는 문서 추출 대상 소스 파일
  (보통 `src/lib.rs`, bin 전용 crate라면 `src/main.rs`)을 지정합니다.

## License

MIT 라이선스.
