# rs-to-readme

[![crates.io](https://img.shields.io/crates/v/rs-to-readme.svg)](https://crates.io/crates/rs-to-readme) [![docs.rs](https://docs.rs/rs-to-readme/badge.svg)](https://docs.rs/rs-to-readme) ![license](https://img.shields.io/badge/license-MIT-blue.svg) [![CI](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml/badge.svg)](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml)

📖 Otros idiomas: [日本語](README-Japan.md) / [English](README-English.md) / [中文](README-Chinese.md) /
[한국어](README-Korea.md) / [Français](README-France.md) / [Deutsch](README-Germany.md) /
[Italiano](README-Italy.md) / [Русский](README-Russia.md) / [العربية](README-Arabic.md) ·
Para integrarlo en otro proyecto, consulta **[PORTING.md](PORTING.md)**.

Genera un README.md completo para un crate de Rust a partir de los metadatos
de su `Cargo.toml` y sus comentarios rustdoc.

**Diseño**: el código fuente (metadatos de `Cargo.toml`, el comentario de
documentación `//!` de nivel superior del crate, y el comentario `///` de
cada elemento público) se trata como la única fuente de verdad, y el
README.md se deriva mecánicamente de él. Esto previene estructuralmente el
problema clásico de "el código cambió, pero se olvidó actualizar el README".

Usa el modo `--check` en CI para verificar que la salida generada coincide
con el README.md existente, detectando mecánicamente pull requests que
cambiaron código pero olvidaron regenerar el README.

## Installation

```bash
cargo install rs-to-readme
```

## API Overview

- `RsToReadmeError` (enum)
- `Result` (type)
- `generate_readme` (fn): genera el cuerpo del README.md desde el directorio
  raíz de un crate (donde está `Cargo.toml`). `entry_file` es el archivo
  fuente del que extraer la documentación (normalmente `src/lib.rs`, o
  `src/main.rs` para crates solo-bin).

## License

Licenciado bajo MIT.
