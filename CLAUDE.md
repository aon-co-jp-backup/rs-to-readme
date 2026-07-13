# 技術スタック・開発ルール(rs-to-readme)

このリポジトリは、Rustクレートの`Cargo.toml`メタデータとrustdocコメント
から過不足のないREADME.mdを自動生成するCLIツール。作業ドライブは
`F:\open-runo`(関連プロジェクトと同じ方針、詳細は`open-raid-z`の
`CLAUDE.md`参照)。

## 技術スタック

- **Rust**(メイン言語、唯一の言語): https://www.rust-lang.org/ja/
- `syn` + `proc-macro2`: Rustソースの解析(crate内トップレベルの`//!`・
  各公開項目の`///`コメント抽出)
- `toml` + `serde`: `Cargo.toml`の解析
- `thiserror`: エラー型

## 設計方針

- ソースコード(`Cargo.toml`のメタデータ、rustdocコメント)を「唯一の正」
  とし、README.mdはそこから機械的に導出する。手動でREADMEを書き換える
  運用は想定しない。
- `--check`モードでCIに組み込み、「コードは変更したがREADMEを更新し
  忘れた」プルリクエストを機械的に検出できるようにする。
- 依存クレートは最小限に留める(`syn`/`toml`/`serde`/`thiserror`のみ)。

## 関連プロジェクト

- **open-raid-z**: https://github.com/aon-co-jp/open-raid-z
- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **aruaru-db**: https://github.com/aon-co-jp/aruaru-db

## 運用ルール

- **開発中はこの`CLAUDE.md`を、コード変更のコミット/pushと必ず一緒に
  push する**。
- 実装で迷った場合は、学習データからの推測より公式ドキュメントを優先する。
- **ローカル作業ドライブ(`F:\open-runo`)上の各リポジトリは、常にリモート
  (GitHub)の最新コミットに追従させておくこと**(`git fetch`/`git pull`を
  こまめに実行する。ローカルにのみ存在する未コミット変更がある場合は、
  上書き前に必ず内容を確認し、必要なら `git stash` で退避してから最新化
  する)。
- **ドキュメントに「未調査」「未着手」「意図的に先送り」「将来検討」等と
  記録されている項目を見つけた場合、それは承認待ちの提案ではなく、次に
  着手すべき実装対象そのものである**(ユーザー指示、2026-07-12、
  open-raid-z正本より転記)。「ご指示はありますか?」のように追加の確認を
  求めて手を止めず、確認不要でそのまま実装を進めること。優先度・実現
  可能性の判断は自分で行ってよいが、「未着手だから今回は見送る」という
  判断そのものをユーザーへの確認なしに下してはならない。
