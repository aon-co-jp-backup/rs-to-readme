# rs-to-readme

[![crates.io](https://img.shields.io/crates/v/rs-to-readme.svg)](https://crates.io/crates/rs-to-readme) [![docs.rs](https://docs.rs/rs-to-readme/badge.svg)](https://docs.rs/rs-to-readme) ![license](https://img.shields.io/badge/license-MIT-blue.svg) [![CI](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml/badge.svg)](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml)

📖 Другие языки: [日本語](README-Japan.md) / [English](README-English.md) / [中文](README-Chinese.md) /
[한국어](README-Korea.md) / [Español](README-Spain.md) / [Français](README-France.md) /
[Deutsch](README-Germany.md) / [Italiano](README-Italy.md) / [العربية](README-Arabic.md) ·
Для переноса в другой проект см. **[PORTING.md](PORTING.md)**.

Библиотека + CLI-инструмент, генерирующий полноценный README.md для Rust
крейта на основе метаданных `Cargo.toml` и комментариев rustdoc.

**Принцип проектирования**: исходный код (метаданные `Cargo.toml`,
документационный комментарий `//!` верхнего уровня крейта, комментарий `///`
каждого публичного элемента) считается единственным источником истины, а
README.md механически выводится из него. Это структурно предотвращает
классическую проблему «код изменился, а README обновить забыли».

Используйте режим `--check` в CI, чтобы проверить соответствие
сгенерированного результата существующему README.md — это механически
выявляет pull request'ы, в которых код изменили, а README забыли
перегенерировать.

## Installation

```bash
cargo install rs-to-readme
```

## API Overview

- `RsToReadmeError` (enum)
- `Result` (type)
- `generate_readme` (fn) — генерирует содержимое README.md из корневого
  каталога крейта (где находится `Cargo.toml`). `entry_file` — исходный
  файл, из которого извлекается документация (обычно `src/lib.rs`, либо
  `src/main.rs` для чисто bin-крейтов).

## License

Лицензия MIT.
