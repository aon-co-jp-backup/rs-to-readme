# rs-to-readme

[![crates.io](https://img.shields.io/crates/v/rs-to-readme.svg)](https://crates.io/crates/rs-to-readme) [![docs.rs](https://docs.rs/rs-to-readme/badge.svg)](https://docs.rs/rs-to-readme) ![license](https://img.shields.io/badge/license-MIT-blue.svg) [![CI](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml/badge.svg)](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml)

📖 Other languages: [日本語](README-Japan.md) / [中文](README-Chinese.md) / [한국어](README-Korea.md) /
[Español](README-Spain.md) / [Français](README-France.md) / [Deutsch](README-Germany.md) /
[Italiano](README-Italy.md) / [Русский](README-Russia.md) / [العربية](README-Arabic.md) ·
To integrate into another project, see **[PORTING.md](PORTING.md)**.

Generate a polished README.md for a Rust crate from its Cargo.toml metadata
and rustdoc comments.

**Design**: source code (`Cargo.toml` metadata, the crate's top-level `//!`
doc comment, and each public item's `///` comment) is treated as the single
source of truth, and README.md is mechanically derived from it. This
structurally prevents the classic "code changed, README forgot to update"
drift.

Use `--check` mode in CI to verify the generated output matches the existing
README.md, mechanically catching pull requests that changed code but forgot
to regenerate the README.

## Installation

```bash
cargo install rs-to-readme
```

## API Overview

- `RsToReadmeError` (enum)
- `Result` (type)
- `generate_readme` (fn) — generates the README.md body from a crate's root
  directory (where `Cargo.toml` lives). `entry_file` is the source file to
  extract docs from (usually `src/lib.rs`, or `src/main.rs` for bin-only
  crates).

## License

Licensed under MIT.
