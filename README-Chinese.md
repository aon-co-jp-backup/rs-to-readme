# rs-to-readme

[![crates.io](https://img.shields.io/crates/v/rs-to-readme.svg)](https://crates.io/crates/rs-to-readme) [![docs.rs](https://docs.rs/rs-to-readme/badge.svg)](https://docs.rs/rs-to-readme) ![license](https://img.shields.io/badge/license-MIT-blue.svg) [![CI](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml/badge.svg)](https://github.com/aon-co-jp/rs-to-readme/actions/workflows/ci.yml)

📖 其他语言: [日本語](README-Japan.md) / [English](README-English.md) / [한국어](README-Korea.md) /
[Español](README-Spain.md) / [Français](README-France.md) / [Deutsch](README-Germany.md) /
[Italiano](README-Italy.md) / [Русский](README-Russia.md) / [العربية](README-Arabic.md) ·
移植到其他项目请参阅 **[PORTING.md](PORTING.md)**。

从 Rust crate 的 `Cargo.toml` 元数据和 rustdoc 注释自动生成完整准确的
README.md 的库 + CLI 工具。

**设计理念**：将源代码(`Cargo.toml` 元数据、crate 顶层的 `//!` 文档注释、
每个公开项的 `///` 注释)视为唯一真实来源,README.md 从中机械化派生。
这从结构上避免了"代码改了但忘记更新 README"这一常见问题。

在 CI 中使用 `--check` 模式,可以验证生成结果是否与现有 README.md
一致,从而机械化检测出"改了代码却忘记重新生成 README"的 PR。

## Installation

```bash
cargo install rs-to-readme
```

## API Overview

- `RsToReadmeError`(枚举)
- `Result`(类型)
- `generate_readme`(函数)— 从 crate 根目录(`Cargo.toml` 所在处)生成
  README.md 正文。`entry_file` 指定文档提取来源文件(通常为
  `src/lib.rs`,纯 bin crate 则为 `src/main.rs`)。

## License

MIT 许可证。
