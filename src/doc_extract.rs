//! Rustソースファイル(通常`src/lib.rs`)を`syn`で解析し、README生成に
//! 必要なドキュメント情報(クレートトップレベルのコメント、公開API一覧)を
//! 取り出す。

use syn::{Attribute, Item, Meta};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExtractedDocs {
    /// クレートトップレベルの`//!`ドキュメントコメント(改行含む、
    /// 段落構造を保ったまま)。
    pub crate_doc: String,
    /// トップレベルの公開API一覧(モジュール直下のみ、v1では再帰しない)。
    pub public_items: Vec<PublicItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicItem {
    /// "fn" | "struct" | "enum" | "trait" | "type" のいずれか。
    pub kind: &'static str,
    pub name: String,
    /// `///`コメントの最初の段落(概要行)。無ければ空文字列。
    pub summary: String,
}

/// 1つの`Attribute`が`#[doc = "..."]`(`///`/`//!`の展開形)であれば、
/// その文字列内容を返す。
fn doc_attr_text(attr: &Attribute) -> Option<String> {
    if !attr.path().is_ident("doc") {
        return None;
    }
    let Meta::NameValue(nv) = &attr.meta else { return None };
    if let syn::Expr::Lit(expr_lit) = &nv.value {
        if let syn::Lit::Str(s) = &expr_lit.lit {
            return Some(s.value());
        }
    }
    None
}

/// 複数の`#[doc = "..."]`属性(1行=1属性)を結合し、行頭の余分な半角
/// スペース1つ(rustdocコメントの`/// `の慣習)を取り除いた本文にする。
fn join_doc_lines(attrs: &[Attribute]) -> String {
    let lines: Vec<String> = attrs
        .iter()
        .filter_map(doc_attr_text)
        .map(|line| line.strip_prefix(' ').map(str::to_string).unwrap_or(line))
        .collect();
    lines.join("\n").trim().to_string()
}

/// ドキュメント本文の最初の段落(最初の空行まで)を1行に畳んで返す
/// (README中の一覧・概要表示用)。
fn first_paragraph(doc: &str) -> String {
    doc.split("\n\n")
        .next()
        .unwrap_or("")
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

fn item_name_and_kind(item: &Item) -> Option<(&'static str, String)> {
    match item {
        Item::Fn(f) if matches!(f.vis, syn::Visibility::Public(_)) => Some(("fn", f.sig.ident.to_string())),
        Item::Struct(s) if matches!(s.vis, syn::Visibility::Public(_)) => Some(("struct", s.ident.to_string())),
        Item::Enum(e) if matches!(e.vis, syn::Visibility::Public(_)) => Some(("enum", e.ident.to_string())),
        Item::Trait(t) if matches!(t.vis, syn::Visibility::Public(_)) => Some(("trait", t.ident.to_string())),
        Item::Type(t) if matches!(t.vis, syn::Visibility::Public(_)) => Some(("type", t.ident.to_string())),
        _ => None,
    }
}

fn item_attrs(item: &Item) -> &[Attribute] {
    match item {
        Item::Fn(f) => &f.attrs,
        Item::Struct(s) => &s.attrs,
        Item::Enum(e) => &e.attrs,
        Item::Trait(t) => &t.attrs,
        Item::Type(t) => &t.attrs,
        _ => &[],
    }
}

/// Rustソース全文を解析し、[`ExtractedDocs`]を返す。
pub fn extract(source_text: &str) -> syn::Result<ExtractedDocs> {
    let file = syn::parse_file(source_text)?;

    let crate_doc = join_doc_lines(&file.attrs);

    let mut public_items = Vec::new();
    for item in &file.items {
        if let Some((kind, name)) = item_name_and_kind(item) {
            let doc = join_doc_lines(item_attrs(item));
            public_items.push(PublicItem { kind, name, summary: first_paragraph(&doc) });
        }
    }

    Ok(ExtractedDocs { crate_doc, public_items })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_crate_level_doc_comment_across_multiple_lines() {
        let src = r#"//! First line of the crate overview.
//! Second line, same paragraph.
//!
//! A second paragraph.

pub fn noop() {}
"#;
        let docs = extract(src).unwrap();
        assert_eq!(
            docs.crate_doc,
            "First line of the crate overview.\nSecond line, same paragraph.\n\nA second paragraph."
        );
    }

    #[test]
    fn extracts_public_functions_structs_enums_and_traits_with_summaries() {
        let src = r#"//! doc

/// Adds two numbers.
///
/// More detail that should not appear in the summary.
pub fn add(a: i32, b: i32) -> i32 { a + b }

fn private_helper() {}

/// A point in 2D space.
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Either red, green, or blue.
pub enum Color { Red, Green, Blue }

/// Something that can be rendered.
pub trait Renderable {
    fn render(&self);
}
"#;
        let docs = extract(src).unwrap();
        assert_eq!(docs.public_items.len(), 4);

        let add = docs.public_items.iter().find(|i| i.name == "add").unwrap();
        assert_eq!(add.kind, "fn");
        assert_eq!(add.summary, "Adds two numbers.");

        let point = docs.public_items.iter().find(|i| i.name == "Point").unwrap();
        assert_eq!(point.kind, "struct");

        let color = docs.public_items.iter().find(|i| i.name == "Color").unwrap();
        assert_eq!(color.kind, "enum");

        let renderable = docs.public_items.iter().find(|i| i.name == "Renderable").unwrap();
        assert_eq!(renderable.kind, "trait");

        assert!(!docs.public_items.iter().any(|i| i.name == "private_helper"));
    }

    #[test]
    fn items_without_doc_comments_get_an_empty_summary_rather_than_failing() {
        let src = "pub fn undocumented() {}";
        let docs = extract(src).unwrap();
        assert_eq!(docs.public_items.len(), 1);
        assert_eq!(docs.public_items[0].summary, "");
    }

    #[test]
    fn invalid_rust_source_returns_a_parse_error_instead_of_panicking() {
        let src = "this is not valid rust {{{";
        assert!(extract(src).is_err());
    }
}
