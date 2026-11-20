# rs-to-readme

[![crates.io](https://img.shields.io/crates/v/rs-to-readme.svg)](https://crates.io/crates/rs-to-readme) [![docs.rs](https://docs.rs/rs-to-readme/badge.svg)](https://docs.rs/rs-to-readme) ![license](https://img.shields.io/badge/license-MIT-blue.svg) [![CI](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml/badge.svg)](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml)

📖 Autres langues : [日本語](README-Japan.md) / [English](README-English.md) / [中文](README-Chinese.md) /
[한국어](README-Korea.md) / [Español](README-Spain.md) / [Deutsch](README-Germany.md) /
[Italiano](README-Italy.md) / [Русский](README-Russia.md) / [العربية](README-Arabic.md) ·
Pour l'intégrer à un autre projet, voir **[PORTING.md](PORTING.md)**.

Génère un README.md complet pour un crate Rust à partir des métadonnées de
son `Cargo.toml` et de ses commentaires rustdoc.

**Conception** : le code source (métadonnées de `Cargo.toml`, commentaire de
documentation `//!` de niveau supérieur du crate, et commentaire `///` de
chaque élément public) est considéré comme l'unique source de vérité, et le
README.md en est dérivé mécaniquement. Cela évite structurellement le
problème classique du « code modifié, mais README oublié ».

Utilisez le mode `--check` en CI pour vérifier que la sortie générée
correspond au README.md existant, détectant ainsi mécaniquement les pull
requests qui ont modifié le code sans régénérer le README.

## Installation

```bash
cargo install rs-to-readme
```

## API Overview

- `RsToReadmeError` (enum)
- `Result` (type)
- `generate_readme` (fn) : génère le corps du README.md depuis le répertoire
  racine d'un crate (où se trouve `Cargo.toml`). `entry_file` est le fichier
  source dont extraire la documentation (généralement `src/lib.rs`, ou
  `src/main.rs` pour les crates bin uniquement).

## License

Sous licence MIT.
