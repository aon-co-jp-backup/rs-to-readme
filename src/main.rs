//! `rs-to-readme` CLI。
//!
//! 使い方:
//!   rs-to-readme [--path <クレートのルート>] [--entry <エントリファイル>]
//!                [--output <出力先>] [--check]
//!
//! `--path`(既定`.`): `Cargo.toml`があるクレートのルートディレクトリ。
//! `--entry`(既定`src/lib.rs`。存在しなければ`src/main.rs`): ドキュメント
//!   抽出元のソースファイル。
//! `--output`(既定`README.md`): 生成結果の書き出し先。
//! `--check`: 書き出す代わりに、既存の`--output`と生成結果を比較する。
//!   一致しなければ終了コード1で失敗する(CIでの「README更新忘れ」検出用)。

use rs_to_readme::generate_readme;
use std::path::PathBuf;
use std::process::ExitCode;

struct Args {
    path: PathBuf,
    entry: Option<PathBuf>,
    output: PathBuf,
    check: bool,
}

fn parse_args() -> Result<Args, String> {
    let mut path = PathBuf::from(".");
    let mut entry = None;
    let mut output = PathBuf::from("README.md");
    let mut check = false;

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
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => return Err(format!("未知の引数です: '{other}'(--helpでヘルプ表示)")),
        }
    }

    Ok(Args { path, entry, output, check })
}

fn print_help() {
    println!(
        "rs-to-readme - Rustクレートのメタデータ・rustdocコメントからREADME.mdを生成する\n\n\
使い方:\n  \
rs-to-readme [--path <ルート>] [--entry <エントリファイル>] [--output <出力先>] [--check]\n\n\
オプション:\n  \
--path <ルート>       Cargo.tomlがあるディレクトリ(既定: .)\n  \
--entry <ファイル>    ドキュメント抽出元(既定: src/lib.rs、無ければsrc/main.rs)\n  \
--output <ファイル>   出力先(既定: README.md)\n  \
--check               書き出さず、既存ファイルとの一致のみ検証する(CI向け)"
    );
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
