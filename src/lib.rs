//! `rs-to-readme`: Rustクレートの`Cargo.toml`メタデータとrustdocコメントから、
//! 過不足のないREADME.mdを自動生成するライブラリ+CLIツール。
//!
//! 【設計方針】ソースコード(`Cargo.toml`のメタデータ、クレートトップレベルの
//! `//!`ドキュメントコメント、公開API各項目の`///`コメント)を「唯一の正」とし、
//! README.mdはそこから機械的に導出する。これにより、コードとドキュメントが
//! 乖離する(README更新を忘れる)という典型的な問題を構造的に防ぐ。
//!
//! CI等で`--check`モードを使えば、生成結果と既存のREADME.mdが一致するかを
//! 検証でき、「コードは変更したがREADMEを更新し忘れた」プルリクエストを
//! 機械的に検出できる。

pub mod cargo_meta;
pub mod doc_extract;
pub mod html_render;
pub mod render;

pub use cargo_meta::CargoMeta;
pub use doc_extract::{ExtractedDocs, PublicItem};
pub use html_render::render_html;
pub use render::render_readme;

use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum RsToReadmeError {
    #[error("'{0}'の読み込みに失敗しました: {1}")]
    ReadFile(String, std::io::Error),
    #[error("Cargo.tomlの解析に失敗しました: {0}")]
    ParseCargoToml(#[from] toml::de::Error),
    #[error("Rustソースの解析に失敗しました: {0}")]
    ParseRust(#[from] syn::Error),
}

pub type Result<T> = std::result::Result<T, RsToReadmeError>;

/// クレートのルートディレクトリ(`Cargo.toml`がある場所)から、README.md
/// 本文を生成する。`entry_file`にはドキュメント抽出元のソースファイル
/// (通常`src/lib.rs`、bin専用クレートなら`src/main.rs`)を指定する。
///
/// `crate_root`直下に`README.banner.md`が存在する場合、その内容がバッジ行の
/// 直後・説明文の直前に挿入される。多言語READMEへのリンクや`PORTING.md`
/// への案内など、`Cargo.toml`/rustdocからは導出できないが毎回手で書き直す
/// 運用にはしたくない短い前置き文のためのフックで、ファイルが無ければ何も
/// 挿入されない(既存クレートへの後方互換を壊さない)。これにより`--check`
/// モードは、こうした前置き文を含むREADME.mdに対しても正しく機能する。
pub fn generate_readme(crate_root: &Path, entry_file: &Path) -> Result<String> {
    let cargo_toml_path = crate_root.join("Cargo.toml");
    let cargo_toml_text = std::fs::read_to_string(&cargo_toml_path)
        .map_err(|e| RsToReadmeError::ReadFile(cargo_toml_path.display().to_string(), e))?;
    let meta = CargoMeta::parse(&cargo_toml_text)?;

    let entry_text = std::fs::read_to_string(entry_file)
        .map_err(|e| RsToReadmeError::ReadFile(entry_file.display().to_string(), e))?;
    let docs = doc_extract::extract(&entry_text)?;

    let banner = std::fs::read_to_string(crate_root.join("README.banner.md")).ok();

    Ok(render_readme(&meta, &docs, banner.as_deref()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn generate_readme_produces_expected_sections_for_a_minimal_crate() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("Cargo.toml"),
            r#"
[package]
name = "widgetizer"
version = "1.2.3"
description = "Turns raw gadgets into polished widgets"
license = "MIT"
repository = "https://github.com/example/widgetizer"
"#,
        )
        .unwrap();
        std::fs::create_dir(dir.path().join("src")).unwrap();
        let lib_path = dir.path().join("src/lib.rs");
        let mut f = std::fs::File::create(&lib_path).unwrap();
        write!(
            f,
            r#"//! Widgetizer turns raw gadgets into polished widgets.
//!
//! This is the second paragraph of the crate overview.

/// Turns a gadget into a widget.
pub fn widgetize(input: &str) -> String {{
    input.to_string()
}}

/// A configuration knob for widgetization.
pub struct WidgetConfig {{
    pub shine: bool,
}}
"#
        )
        .unwrap();

        let readme = generate_readme(dir.path(), &lib_path).unwrap();

        assert!(readme.contains("# widgetizer"), "{readme}");
        assert!(readme.contains("Turns raw gadgets into polished widgets"), "{readme}");
        assert!(readme.contains("Widgetizer turns raw gadgets into polished widgets."), "{readme}");
        assert!(readme.contains("This is the second paragraph"), "{readme}");
        assert!(readme.contains("widgetize"), "{readme}");
        assert!(readme.contains("WidgetConfig"), "{readme}");
        assert!(readme.contains("cargo add widgetizer"), "{readme}");
        assert!(readme.contains("MIT"), "{readme}");
        assert!(readme.contains("https://github.com/example/widgetizer"), "{readme}");
    }

    #[test]
    fn includes_readme_banner_md_content_when_present_in_crate_root() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("Cargo.toml"),
            r#"
[package]
name = "widgetizer"
version = "1.2.3"
description = "Turns raw gadgets into polished widgets"
license = "MIT"
"#,
        )
        .unwrap();
        std::fs::create_dir(dir.path().join("src")).unwrap();
        let lib_path = dir.path().join("src/lib.rs");
        std::fs::write(&lib_path, "//! Widgetizer.\n").unwrap();
        std::fs::write(
            dir.path().join("README.banner.md"),
            "他言語 / Other languages: [日本語](README-Japan.md)\n",
        )
        .unwrap();

        let readme = generate_readme(dir.path(), &lib_path).unwrap();

        assert!(readme.contains("他言語 / Other languages"), "{readme}");
    }
}
