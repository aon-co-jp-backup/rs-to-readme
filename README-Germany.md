# rs-to-readme

[![crates.io](https://img.shields.io/crates/v/rs-to-readme.svg)](https://crates.io/crates/rs-to-readme) [![docs.rs](https://docs.rs/rs-to-readme/badge.svg)](https://docs.rs/rs-to-readme) ![license](https://img.shields.io/badge/license-MIT-blue.svg) [![CI](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml/badge.svg)](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml)

📖 Andere Sprachen: [日本語](README-Japan.md) / [English](README-English.md) / [中文](README-Chinese.md) /
[한국어](README-Korea.md) / [Español](README-Spain.md) / [Français](README-France.md) /
[Italiano](README-Italy.md) / [Русский](README-Russia.md) / [العربية](README-Arabic.md) ·
Zur Integration in ein anderes Projekt siehe **[PORTING.md](PORTING.md)**.

Erzeugt eine vollständige README.md für ein Rust-Crate aus dessen
`Cargo.toml`-Metadaten und rustdoc-Kommentaren.

**Designprinzip**: Der Quellcode (`Cargo.toml`-Metadaten, der `//!`-Dokumentations-
kommentar auf oberster Ebene des Crates sowie der `///`-Kommentar jedes
öffentlichen Elements) gilt als einzige Quelle der Wahrheit, aus der die
README.md mechanisch abgeleitet wird. Dies verhindert strukturell das
klassische Problem „Code geändert, README-Update vergessen".

Der `--check`-Modus in CI prüft, ob die generierte Ausgabe mit der
bestehenden README.md übereinstimmt, und erkennt so mechanisch Pull
Requests, die Code geändert, aber vergessen haben, die README neu zu
generieren.

## Installation

```bash
cargo install rs-to-readme
```

## API Overview

- `RsToReadmeError` (enum)
- `Result` (type)
- `generate_readme` (fn) — erzeugt den README.md-Inhalt aus dem Wurzel-
  verzeichnis eines Crates (wo `Cargo.toml` liegt). `entry_file` ist die
  Quelldatei, aus der die Dokumentation extrahiert wird (üblicherweise
  `src/lib.rs`, bei reinen Bin-Crates `src/main.rs`).

## License

Lizenziert unter MIT.
