# rs-to-readme

[![crates.io](https://img.shields.io/crates/v/rs-to-readme.svg)](https://crates.io/crates/rs-to-readme) [![docs.rs](https://docs.rs/rs-to-readme/badge.svg)](https://docs.rs/rs-to-readme) ![license](https://img.shields.io/badge/license-MIT-blue.svg) [![CI](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml/badge.svg)](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml)

📖 他言語 / Other languages: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md) ·
他プロジェクトへの導入は **[PORTING.md](PORTING.md)** 参照。

Generate a polished README.md for a Rust crate from its Cargo.toml metadata and rustdoc comments

`rs-to-readme`: Rustクレートの`Cargo.toml`メタデータとrustdocコメントから、
過不足のないREADME.mdを自動生成するライブラリ+CLIツール。

【設計方針】ソースコード(`Cargo.toml`のメタデータ、クレートトップレベルの
`//!`ドキュメントコメント、公開API各項目の`///`コメント)を「唯一の正」とし、
README.mdはそこから機械的に導出する。これにより、コードとドキュメントが
乖離する(README更新を忘れる)という典型的な問題を構造的に防ぐ。

CI等で`--check`モードを使えば、生成結果と既存のREADME.mdが一致するかを
検証でき、「コードは変更したがREADMEを更新し忘れた」プルリクエストを
機械的に検出できる。

## Installation

```bash
cargo install rs-to-readme
```

## API Overview

- `RsToReadmeError` (enum)
- `Result` (type)
- `generate_readme` (fn) — クレートのルートディレクトリ(`Cargo.toml`がある場所)から、README.md 本文を生成する。`entry_file`にはドキュメント抽出元のソースファイル (通常`src/lib.rs`、bin専用クレートなら`src/main.rs`)を指定する。

## License

Licensed under MIT.
