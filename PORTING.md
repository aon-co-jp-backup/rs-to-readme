# PORTING.md — rs-to-readme お引越しファイル

> このファイル 1 枚で、**どのRustプロジェクトにも `rs-to-readme` を導入**できます。

## 0. rs-to-readme とは

`Cargo.toml`のメタデータとrustdocコメント(クレートトップレベルの`//!`、
公開API各項目の`///`)を「唯一の正」として、README.mdを機械的に生成する
ライブラリ+CLIツール。手動でREADMEを書き換える運用を想定せず、
コードとドキュメントの乖離を構造的に防ぐ。

## 1. 導入方法

```bash
cargo install rs-to-readme
```

```bash
# クレートのルートディレクトリ(Cargo.tomlがある場所)で実行
rs-to-readme --entry src/lib.rs > README.md

# CI組み込み: 生成結果と既存README.mdの一致を検証(不一致ならexit非0)
rs-to-readme --entry src/lib.rs --check
```

## 2. ライブラリとしての利用

```rust
use rs_to_readme::generate_readme;

let readme = generate_readme(".", "src/lib.rs")?;
```

- `generate_readme(crate_root, entry_file) -> Result<String, RsToReadmeError>`
- `entry_file`は通常`src/lib.rs`(binクレート専用なら`src/main.rs`)

## 3. 依存

`syn` + `proc-macro2`(Rustソース解析)・`toml` + `serde`(Cargo.toml解析)・
`thiserror`(エラー型)のみ。最小依存方針。

## 4. 開発ルール

このプロジェクトのコーディング方針・関連プロジェクトは
[`CLAUDE.md`](CLAUDE.md)、開発ルールの正本は
[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z)の`CLAUDE.md`を参照。

## License

MIT.
