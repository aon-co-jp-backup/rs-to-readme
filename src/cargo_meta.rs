//! `Cargo.toml`から、README生成に必要なメタデータだけを取り出す。

use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CargoMeta {
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub repository: Option<String>,
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    /// `[[bin]]`が1つ以上あるか(実行ファイルクレートかどうかで
    /// 「インストール」節の文面を変える)。
    #[serde(skip)]
    pub has_binary: bool,
}

#[derive(Debug, Deserialize)]
struct RawManifest {
    package: RawPackage,
    #[serde(rename = "bin", default)]
    bins: Vec<toml::Value>,
}

#[derive(Debug, Deserialize)]
struct RawPackage {
    name: String,
    #[serde(default)]
    version: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    license: Option<String>,
    #[serde(default)]
    repository: Option<String>,
    #[serde(default)]
    homepage: Option<String>,
    #[serde(default)]
    keywords: Vec<String>,
}

impl CargoMeta {
    pub fn parse(cargo_toml_text: &str) -> Result<Self, toml::de::Error> {
        let raw: RawManifest = toml::from_str(cargo_toml_text)?;
        // `[[bin]]`が明示されていなくても、`src/main.rs`が存在すれば実行ファイル
        // クレートになる(cargo既定の慣習)。ここではマニフェストのみから
        // 判断できる範囲(明示的な`[[bin]]`)に留め、`src/main.rs`の有無は
        // 呼び出し側(`generate_readme`のentry_file)で判断する。
        let has_binary = !raw.bins.is_empty();
        Ok(Self {
            name: raw.package.name,
            version: raw.package.version,
            description: raw.package.description,
            license: raw.package.license,
            repository: raw.package.repository,
            homepage: raw.package.homepage,
            keywords: raw.package.keywords,
            has_binary,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_typical_manifest() {
        let text = r#"
[package]
name = "example-crate"
version = "0.3.0"
description = "Does example things"
license = "Apache-2.0"
repository = "https://github.com/example/example-crate"
keywords = ["example", "demo"]
"#;
        let meta = CargoMeta::parse(text).unwrap();
        assert_eq!(meta.name, "example-crate");
        assert_eq!(meta.version, "0.3.0");
        assert_eq!(meta.description.as_deref(), Some("Does example things"));
        assert_eq!(meta.license.as_deref(), Some("Apache-2.0"));
        assert_eq!(meta.keywords, vec!["example", "demo"]);
        assert!(!meta.has_binary);
    }

    #[test]
    fn detects_explicit_bin_targets() {
        let text = r#"
[package]
name = "example-cli"
version = "0.1.0"

[[bin]]
name = "example-cli"
path = "src/main.rs"
"#;
        let meta = CargoMeta::parse(text).unwrap();
        assert!(meta.has_binary);
    }

    #[test]
    fn missing_optional_fields_default_to_none_or_empty() {
        let text = r#"
[package]
name = "bare"
version = "0.1.0"
"#;
        let meta = CargoMeta::parse(text).unwrap();
        assert_eq!(meta.description, None);
        assert_eq!(meta.license, None);
        assert_eq!(meta.repository, None);
        assert!(meta.keywords.is_empty());
    }
}
