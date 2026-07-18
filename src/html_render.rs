//! 生成したREADME(Markdown)を、GitHubのリポジトリページのような見た目で
//! 表示できるスタンドアロンHTMLへ変換する。
//!
//! 【動機】`rs-to-readme`が生成するのはあくまでMarkdownのテキストであり、
//! ターミナルやプレーンテキストエディタで開くと「GitHub上でレンダリング
//! された見た目」とは程遠い。GitHub本体にpushして初めて綺麗に表示される、
//! という往復を待たずに、ローカルで生成直後の見た目を確認できるように
//! この変換を提供する。
//!
//! スタイルは`github-markdown-css`(GitHubが実際に使っているCSS)の主要な
//! ルールを参考に、外部CDN依存なしで埋め込んだ簡易版。ピクセル単位での
//! 完全一致は目指さず、見出し・コードブロック・表・引用・リンクの見た目が
//! 「GitHubで見ているのと同じ雰囲気」になる程度を目標とする。

/// MarkdownをGitHubスタイルのスタンドアロンHTMLドキュメントに変換する。
/// `title`はブラウザタブに表示されるタイトル。
pub fn render_html(markdown: &str, title: &str) -> String {
    let parser = pulldown_cmark::Parser::new_ext(markdown, pulldown_cmark::Options::ENABLE_TABLES | pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    let mut body_html = String::new();
    pulldown_cmark::html::push_html(&mut body_html, parser);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<style>
{css}
</style>
</head>
<body>
<article class="markdown-body">
{body_html}
</article>
</body>
</html>"#,
        title = html_escape_attr(title),
        css = GITHUB_STYLE_CSS,
        body_html = body_html,
    )
}

fn html_escape_attr(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// GitHubの`github-markdown-css`(https://github.com/sindresorhus/github-markdown-css)
/// の代表的なルールを手作業で抜粋・簡略化した埋め込みCSS。ライト/ダーク両モード対応。
const GITHUB_STYLE_CSS: &str = r#"
:root { color-scheme: light dark; }
body {
  margin: 0;
  padding: 2rem;
  background: #ffffff;
  display: flex;
  justify-content: center;
}
@media (prefers-color-scheme: dark) {
  body { background: #0d1117; }
}
.markdown-body {
  max-width: 900px;
  width: 100%;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans", Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
  font-size: 16px;
  line-height: 1.5;
  color: #1f2328;
  word-wrap: break-word;
}
@media (prefers-color-scheme: dark) {
  .markdown-body { color: #e6edf3; }
}
.markdown-body h1, .markdown-body h2, .markdown-body h3,
.markdown-body h4, .markdown-body h5, .markdown-body h6 {
  margin-top: 24px;
  margin-bottom: 16px;
  font-weight: 600;
  line-height: 1.25;
}
.markdown-body h1 { font-size: 2em; padding-bottom: 0.3em; border-bottom: 1px solid #d1d9e0; }
.markdown-body h2 { font-size: 1.5em; padding-bottom: 0.3em; border-bottom: 1px solid #d1d9e0; }
.markdown-body h3 { font-size: 1.25em; }
@media (prefers-color-scheme: dark) {
  .markdown-body h1, .markdown-body h2 { border-bottom-color: #21262d; }
}
.markdown-body p, .markdown-body ul, .markdown-body ol, .markdown-body blockquote,
.markdown-body table, .markdown-body pre {
  margin-top: 0;
  margin-bottom: 16px;
}
.markdown-body a { color: #0969da; text-decoration: none; }
.markdown-body a:hover { text-decoration: underline; }
@media (prefers-color-scheme: dark) {
  .markdown-body a { color: #4493f8; }
}
.markdown-body code, .markdown-body pre {
  font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace;
  font-size: 85%;
}
.markdown-body code {
  background: rgba(175, 184, 193, 0.2);
  padding: 0.2em 0.4em;
  border-radius: 6px;
}
.markdown-body pre {
  background: #f6f8fa;
  padding: 16px;
  overflow: auto;
  border-radius: 6px;
  line-height: 1.45;
}
.markdown-body pre code {
  background: none;
  padding: 0;
  font-size: 100%;
}
@media (prefers-color-scheme: dark) {
  .markdown-body code { background: rgba(110, 118, 129, 0.4); }
  .markdown-body pre { background: #161b22; }
}
.markdown-body blockquote {
  padding: 0 1em;
  color: #59636e;
  border-left: 0.25em solid #d1d9e0;
}
@media (prefers-color-scheme: dark) {
  .markdown-body blockquote { color: #9198a1; border-left-color: #3d444d; }
}
.markdown-body table {
  border-collapse: collapse;
  width: max-content;
  max-width: 100%;
  display: block;
  overflow: auto;
}
.markdown-body table th, .markdown-body table td {
  padding: 6px 13px;
  border: 1px solid #d1d9e0;
}
.markdown-body table tr { background: #ffffff; border-top: 1px solid #d1d9e0; }
.markdown-body table tr:nth-child(2n) { background: #f6f8fa; }
@media (prefers-color-scheme: dark) {
  .markdown-body table th, .markdown-body table td { border-color: #3d444d; }
  .markdown-body table tr { background: #0d1117; border-top-color: #3d444d; }
  .markdown-body table tr:nth-child(2n) { background: #151b23; }
}
.markdown-body img { max-width: 100%; box-sizing: border-box; }
.markdown-body hr {
  height: 0.25em;
  padding: 0;
  margin: 24px 0;
  background-color: #d1d9e0;
  border: 0;
}
@media (prefers-color-scheme: dark) {
  .markdown-body hr { background-color: #3d444d; }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_html_wraps_markdown_body_and_sets_title() {
        let html = render_html("# Hello\n\nSome **bold** text.", "my-crate");
        assert!(html.contains("<title>my-crate</title>"));
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains("markdown-body"));
    }

    #[test]
    fn render_html_escapes_title_attribute() {
        let html = render_html("body", "<script>alert(1)</script>");
        assert!(!html.contains("<script>alert(1)</script>"));
        assert!(html.contains("&lt;script&gt;"));
    }

    #[test]
    fn render_html_supports_tables() {
        let md = "| a | b |\n|---|---|\n| 1 | 2 |\n";
        let html = render_html(md, "t");
        assert!(html.contains("<table>"));
        assert!(html.contains("<td>1</td>"));
    }

    #[test]
    fn render_html_renders_fenced_code_blocks() {
        let md = "```rust\nfn main() {}\n```";
        let html = render_html(md, "t");
        assert!(html.contains("<pre>"));
        assert!(html.contains("fn main()"));
    }
}
