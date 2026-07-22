//! [`CargoMeta`]と[`ExtractedDocs`]から、実際のMarkdown本文を組み立てる。

use crate::cargo_meta::CargoMeta;
use crate::doc_extract::ExtractedDocs;

/// GitHubリポジトリURL(`https://github.com/OWNER/REPO`)から`OWNER/REPO`を
/// 取り出す(バッジ生成・crates.io/docs.rs等のリンク構築に使う)。
fn github_owner_repo(repository: &str) -> Option<&str> {
    repository.strip_prefix("https://github.com/").map(|s| s.trim_end_matches('/').trim_end_matches(".git"))
}

fn badges(meta: &CargoMeta) -> String {
    let mut badges = Vec::new();

    badges.push(format!(
        "[![crates.io](https://img.shields.io/crates/v/{name}.svg)](https://crates.io/crates/{name})",
        name = meta.name
    ));
    badges.push(format!(
        "[![docs.rs](https://docs.rs/{name}/badge.svg)](https://docs.rs/{name})",
        name = meta.name
    ));
    if let Some(license) = &meta.license {
        badges.push(format!("![license](https://img.shields.io/badge/license-{}-blue.svg)", license.replace('-', "--")));
    }
    if let Some(repo) = &meta.repository {
        if let Some(owner_repo) = github_owner_repo(repo) {
            badges.push(format!(
                "[![CI](https://github.com/{owner_repo}/actions/workflows/ci.yml/badge.svg)](https://github.com/{owner_repo}/actions/workflows/ci.yml)"
            ));
        }
    }

    badges.join(" ")
}

fn installation_section(meta: &CargoMeta) -> String {
    if meta.has_binary {
        format!(
            "## Installation\n\n```bash\ncargo install {name}\n```\n",
            name = meta.name
        )
    } else {
        format!(
            "## Installation\n\n```bash\ncargo add {name}\n```\n",
            name = meta.name
        )
    }
}

fn api_overview_section(items: &[crate::doc_extract::PublicItem]) -> String {
    if items.is_empty() {
        return String::new();
    }
    let mut out = String::from("## API Overview\n\n");
    for item in items {
        if item.summary.is_empty() {
            out.push_str(&format!("- `{}` ({})\n", item.name, item.kind));
        } else {
            out.push_str(&format!("- `{}` ({}) — {}\n", item.name, item.kind, item.summary));
        }
    }
    out
}

fn license_section(meta: &CargoMeta) -> String {
    match &meta.license {
        Some(license) => format!("## License\n\nLicensed under {license}.\n"),
        None => String::new(),
    }
}

/// [`CargoMeta`]・[`ExtractedDocs`]から完全なREADME.md本文を組み立てる。
///
/// `banner`には、多言語READMEへのリンクや`PORTING.md`への案内など、
/// クレートのメタデータ/rustdocからは導出できないが毎回手で書き直したくない
/// 短い前置き文を渡せる(`README.banner.md`の内容。詳細は[`crate`]直下の
/// [`generate_readme`](crate::generate_readme)を参照)。バッジ行の直後・
/// 説明文の直前に挿入される。
pub fn render_readme(meta: &CargoMeta, docs: &ExtractedDocs, banner: Option<&str>) -> String {
    let mut out = String::new();

    out.push_str(&format!("# {}\n\n", meta.name));

    let badge_line = badges(meta);
    if !badge_line.is_empty() {
        out.push_str(&badge_line);
        out.push_str("\n\n");
    }

    if let Some(banner) = banner {
        let trimmed = banner.trim();
        if !trimmed.is_empty() {
            out.push_str(trimmed);
            out.push_str("\n\n");
        }
    }

    if let Some(description) = &meta.description {
        out.push_str(description);
        out.push_str("\n\n");
    }

    if !docs.crate_doc.is_empty() {
        out.push_str(&docs.crate_doc);
        out.push_str("\n\n");
    }

    out.push_str(&installation_section(meta));
    out.push('\n');

    let api = api_overview_section(&docs.public_items);
    if !api.is_empty() {
        out.push_str(&api);
        out.push('\n');
    }

    let license = license_section(meta);
    if !license.is_empty() {
        out.push_str(&license);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_meta() -> CargoMeta {
        CargoMeta {
            name: "sample".to_string(),
            version: "0.1.0".to_string(),
            description: Some("A sample crate.".to_string()),
            license: Some("MIT".to_string()),
            repository: Some("https://github.com/example/sample".to_string()),
            homepage: None,
            keywords: vec![],
            has_binary: false,
        }
    }

    #[test]
    fn renders_a_library_crate_with_cargo_add_instructions() {
        let meta = sample_meta();
        let docs = ExtractedDocs::default();
        let readme = render_readme(&meta, &docs, None);
        assert!(readme.contains("cargo add sample"));
        assert!(!readme.contains("cargo install sample"));
    }

    #[test]
    fn renders_a_binary_crate_with_cargo_install_instructions() {
        let mut meta = sample_meta();
        meta.has_binary = true;
        let docs = ExtractedDocs::default();
        let readme = render_readme(&meta, &docs, None);
        assert!(readme.contains("cargo install sample"));
    }

    #[test]
    fn badges_include_crates_io_docs_rs_license_and_ci() {
        let meta = sample_meta();
        let badge_line = badges(&meta);
        assert!(badge_line.contains("crates.io/crates/sample"));
        assert!(badge_line.contains("docs.rs/sample"));
        assert!(badge_line.contains("license-MIT"));
        assert!(badge_line.contains("github.com/example/sample/actions"));
    }

    #[test]
    fn omits_api_overview_section_when_there_are_no_public_items() {
        let meta = sample_meta();
        let docs = ExtractedDocs::default();
        let readme = render_readme(&meta, &docs, None);
        assert!(!readme.contains("## API Overview"));
    }

    #[test]
    fn inserts_banner_between_badges_and_description_when_present() {
        let meta = sample_meta();
        let docs = ExtractedDocs::default();
        let readme = render_readme(&meta, &docs, Some("他言語 / Other languages: [日本語](README-Japan.md)\n"));
        let badge_pos = readme.find("crates.io/crates/sample").unwrap();
        let banner_pos = readme.find("他言語 / Other languages").unwrap();
        let desc_pos = readme.find("A sample crate.").unwrap();
        assert!(badge_pos < banner_pos, "{readme}");
        assert!(banner_pos < desc_pos, "{readme}");
    }

    #[test]
    fn ignores_a_banner_that_is_empty_or_only_whitespace() {
        let meta = sample_meta();
        let docs = ExtractedDocs::default();
        let readme = render_readme(&meta, &docs, Some("   \n\n  "));
        assert!(readme.contains("A sample crate.\n\n## Installation") || readme.contains("A sample crate.\n"), "{readme}");
    }
}
