# rs-to-readme

[![crates.io](https://img.shields.io/crates/v/rs-to-readme.svg)](https://crates.io/crates/rs-to-readme) [![docs.rs](https://docs.rs/rs-to-readme/badge.svg)](https://docs.rs/rs-to-readme) ![license](https://img.shields.io/badge/license-MIT-blue.svg) [![CI](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml/badge.svg)](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml)

📖 Altre lingue: [日本語](README-Japan.md) / [English](README-English.md) / [中文](README-Chinese.md) /
[한국어](README-Korea.md) / [Español](README-Spain.md) / [Français](README-France.md) /
[Deutsch](README-Germany.md) / [Русский](README-Russia.md) / [العربية](README-Arabic.md) ·
Per integrarlo in un altro progetto, vedi **[PORTING.md](PORTING.md)**.

Genera un README.md completo per un crate Rust a partire dai metadati del suo
`Cargo.toml` e dai commenti rustdoc.

**Filosofia di design**: il codice sorgente (metadati di `Cargo.toml`, il
commento di documentazione `//!` di primo livello del crate e il commento
`///` di ogni elemento pubblico) è trattato come unica fonte di verità, e il
README.md ne viene derivato meccanicamente. Questo previene strutturalmente
il classico problema "codice modificato, README dimenticato".

Usa la modalità `--check` in CI per verificare che l'output generato
corrisponda al README.md esistente, individuando meccanicamente le pull
request che hanno modificato il codice senza rigenerare il README.

## Installation

```bash
cargo install rs-to-readme
```

## API Overview

- `RsToReadmeError` (enum)
- `Result` (type)
- `generate_readme` (fn) — genera il corpo del README.md dalla directory
  radice di un crate (dove si trova `Cargo.toml`). `entry_file` è il file
  sorgente da cui estrarre la documentazione (di solito `src/lib.rs`, o
  `src/main.rs` per i crate solo-bin).

## License

Rilasciato sotto licenza MIT.
