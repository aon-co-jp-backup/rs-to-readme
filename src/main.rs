//! `rs-to-readme` CLI。
//!
//! 使い方:
//!   rs-to-readme [--path <クレートのルート>] [--entry <エントリファイル>]
//!                [--output <出力先>] [--check] [--html [出力先]]
//!
//! `--path`(既定`.`): `Cargo.toml`があるクレートのルートディレクトリ。
//! `--entry`(既定`src/lib.rs`。存在しなければ`src/main.rs`): ドキュメント
//!   抽出元のソースファイル。
//! `--output`(既定`README.md`): 生成結果の書き出し先。
//! `--check`: 書き出す代わりに、既存の`--output`と生成結果を比較する。
//!   一致しなければ終了コード1で失敗する(CIでの「README更新忘れ」検出用)。
//! `--html [出力先]`(既定の出力先: `--output`の拡張子を`.html`に変えたもの、
//!   例: `README.md`→`README.html`): 生成したMarkdownを、GitHubの
//!   リポジトリページのような見た目のスタンドアロンHTMLとしても書き出す。
//!   ブラウザで直接開けば、GitHubにpushする前にレンダリング結果を確認できる。

use rs_to_readme::{generate_readme, render_html, CargoMeta};
use std::path::PathBuf;
use std::process::ExitCode;

struct Args {
    path: PathBuf,
    entry: Option<PathBuf>,
    output: PathBuf,
    check: bool,
    html: Option<Option<PathBuf>>,
}

fn parse_args() -> Result<Args, String> {
    let mut path = PathBuf::from(".");
    let mut entry = None;
    let mut output = PathBuf::from("README.md");
    let mut check = false;
    let mut html = None;

    let raw: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < raw.len() {
        match raw[i].as_str() {
            "--path" => {
                path = PathBuf::from(raw.get(i + 1).ok_or("--pathには値が必要です")?);
                i += 2;
            }
            "--entry" => {
                entry = Some(PathBuf::from(raw.get(i + 1).ok_or("--entryには値が必要です")?));
                i += 2;
            }
            "--output" => {
                output = PathBuf::from(raw.get(i + 1).ok_or("--outputには値が必要です")?);
                i += 2;
            }
            "--check" => {
                check = true;
                i += 1;
            }
            "--html" => {
                // 次の引数が別のフラグ(`--`始まり)でなければ、明示的な出力先として消費する。
                match raw.get(i + 1) {
                    Some(next) if !next.starts_with("--") => {
                        html = Some(Some(PathBuf::from(next)));
                        i += 2;
                    }
                    _ => {
                        html = Some(None);
                        i += 1;
                    }
                }
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => return Err(format!("未知の引数です: '{other}'(--helpでヘルプ表示)")),
        }
    }

    Ok(Args { path, entry, output, check, html })
}

fn print_help() {
    println!(
        "rs-to-readme - Rustクレートのメタデータ・rustdocコメントからREADME.mdを生成する\n\n\
使い方:\n  \
rs-to-readme [--path <ルート>] [--entry <エントリファイル>] [--output <出力先>] [--check] [--html [出力先]]\n\n\
オプション:\n  \
--path <ルート>       Cargo.tomlがあるディレクトリ(既定: .)\n  \
--entry <ファイル>    ドキュメント抽出元(既定: src/lib.rs、無ければsrc/main.rs)\n  \
--output <ファイル>   出力先(既定: README.md)\n  \
--check               書き出さず、既存ファイルとの一致のみ検証する(CI向け)\n  \
--html [ファイル]     GitHub風の見た目でレンダリングしたHTMLも書き出す\n                        \
(既定: --outputの拡張子を.htmlに変えたもの)"
    );
}

/// `--output`のパスから、拡張子を`.html`に変えたパスを作る
/// (`README.md` → `README.html`、拡張子が無い場合は単に`.html`を付与)。
fn default_html_path(output: &std::path::Path) -> PathBuf {
    output.with_extension("html")
}

/// HTMLプレビューの`<title>`用に、クレート名を`Cargo.toml`から取得する。
/// ディレクトリ名(`args.path.file_name()`)は`--path .`(既定値)の場合
/// `None`を返してしまう(`.`に「通常のコンポーネント名」が無いため)ため、
/// 実際のCargo.tomlメタデータから取得するのが正しい。
fn crate_name_for_title(crate_root: &std::path::Path) -> String {
    std::fs::read_to_string(crate_root.join("Cargo.toml"))
        .ok()
        .and_then(|text| CargoMeta::parse(&text).ok())
        .map(|meta| meta.name)
        .unwrap_or_else(|| "README".to_string())
}

fn resolve_entry(path: &std::path::Path, entry: Option<PathBuf>) -> PathBuf {
    if let Some(e) = entry {
        return path.join(e);
    }
    let lib_rs = path.join("src/lib.rs");
    if lib_rs.exists() {
        return lib_rs;
    }
    path.join("src/main.rs")
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("エラー: {e}");
            return ExitCode::FAILURE;
        }
    };

    let entry_path = resolve_entry(&args.path, args.entry);
    let readme = match generate_readme(&args.path, &entry_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("生成に失敗しました: {e}");
            return ExitCode::FAILURE;
        }
    };

    let output_path = args.path.join(&args.output);

    // `--check`はCI向けの検証専用モードのため、HTMLプレビューの書き出しは
    // 通常の生成モード(--checkなし)でのみ行う——検証実行が意図せず
    // ファイルを増やさないようにする。
    if !args.check {
        if let Some(html_arg) = &args.html {
            let html_path = args.path.join(html_arg.clone().unwrap_or_else(|| default_html_path(&args.output)));
            let title = crate_name_for_title(&args.path);
            let html = render_html(&readme, &title);
            if let Err(e) = std::fs::write(&html_path, &html) {
                eprintln!("'{}'への書き込みに失敗しました: {e}", html_path.display());
                return ExitCode::FAILURE;
            }
            println!("'{}'にGitHub風プレビューを生成しました。", html_path.display());
        }
    }

    if args.check {
        let existing = std::fs::read_to_string(&output_path).unwrap_or_default();
        if existing == readme {
            println!("'{}'は最新です。", output_path.display());
            ExitCode::SUCCESS
        } else {
            eprintln!(
                "'{}'が古くなっています(Cargo.toml/rustdocコメントと一致しません)。\n\
                 `rs-to-readme --path {} --output {}`を実行して更新してください。",
                output_path.display(),
                args.path.display(),
                args.output.display()
            );
            ExitCode::FAILURE
        }
    } else {
        match std::fs::write(&output_path, &readme) {
            Ok(()) => {
                println!("'{}'を生成しました。", output_path.display());
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("'{}'への書き込みに失敗しました: {e}", output_path.display());
                ExitCode::FAILURE
            }
        }
    }
}
