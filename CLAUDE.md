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
- **コンテキストウインドウ・5時間利用制限・その他のセッション中断が
  発生し、その後リミットが解除されて新しいセッションが開始された場合、
  「続けてよろしいですか」等の確認を挟まず、毎回自動的に前回セッションの
  続きの作業を再開すること**(ユーザー指示、2026-07-18、正本は
  open-raid-zのCLAUDE.md参照)。「続けてよろしければそのまま自動開発を
  継続します」のような、続行そのものを尋ねる確認は今後一切行わない。
  セッション開始時は`git status`/`git log`とHANDOFF節の「次にすべき
  こと」で未完了作業を裏取りし、あれば確認なしに完了させ、無ければ
  次の項目へ確認なしに着手する。
- **WEB/UIを持つ機能を実装した後は、ビルド成功・テストだけで「完了」と
  報告せず、実際に画面が正しく表示される(白画面・エラーが無い)ところ
  まで確認すること**(ユーザー指示、2026-07-19、正本はopen-raid-zの
  CLAUDE.md参照)。ブラウザ操作ができない場合は`curl`等でHTMLボディの
  中身に期待される文字列が含まれるかを確認し、ステータスコードのみで
  判断しない。不具合が見つかった場合は確認を求めず自動的に修正する。
- **中断した作業を再開する際、「どちらを優先しますか」のように選択肢を
  尋ねて手を止めないこと**(ユーザー指示、2026-07-19)。中断前に進めて
  いた作業がある場合はそれを最優先で再開し、他にも並行して進めるべき
  作業があれば、ユーザーへの確認を挟まず追加で着手する(既存の
  「確認を求めて手を止めない」方針・[[feedback-dont-stop-for-confirmation]]
  の一部として扱う)。

## HANDOFF

- **2026-07-19 GitHub風HTMLプレビュー出力(`--html`)を追加**:
  生成したMarkdownをターミナル/プレーンテキストエディタで見ると
  GitHub上でのレンダリング結果と乖離が大きい、という指摘を受け、
  `src/html_render.rs`(新規)で`pulldown-cmark`を使い、GitHubの
  `github-markdown-css`の主要ルールを手作業で抜粋・埋め込んだ
  スタンドアロンHTMLへの変換機能を追加。CLIに`--html [出力先]`
  フラグを追加(既定の出力先は`--output`の拡張子を`.html`に変えた
  もの)。ライト/ダーク両モード対応(`prefers-color-scheme`)。
  `--check`モード実行時はHTMLプレビューを書き出さない(CI検証の
  副作用でファイルを増やさないため)。
  - **実装中に発見・修正したバグ**: 当初`<title>`をディレクトリ名
    (`args.path.file_name()`)から取っていたが、既定値`--path .`
    (カレントディレクトリ)の場合`.`には「通常のコンポーネント名」が
    無く`file_name()`が`None`を返すため、常に`README`という無意味な
    タイトルになっていた。`Cargo.toml`を実際に読んで`CargoMeta::parse`
    でクレート名を取得する方式に修正(`crate_name_for_title`)。
    実行して`<title>rs-to-readme</title>`と正しく表示されることを
    確認済み。
  - **正直な開示**: このHTMLレンダリングはGitHubの実CSSをピクセル
    単位で複製したものではなく、見出し・コードブロック・表・引用・
    リンクの見た目が「雰囲気として近い」程度の簡易版。
  - **検証**: `cargo test`で新規4件(HTML変換のタイトル設定・タイトル
    属性のエスケープ・表・コードブロック)含め計16件全green。実際に
    バイナリを実行し(スクラッチディレクトリへ出力、リポジトリ本体の
    README.mdを誤って上書きしないよう注意)、生成された`.html`ファイルの
    中身に`<title>rs-to-readme</title>`・`<h1>rs-to-readme</h1>`・
    `markdown-body`クラスが実際に含まれることを確認済み(ビルド成功・
    ステータスコードのみでの完了報告ではない)。
  - 次にすべきこと: 特に緊急の課題はない。将来的に、GitHub本家の
    CSSとの差分が気になる場合は`github-markdown-css`パッケージの
    最新版と突き合わせて更新を検討。

- **2026-07-22 バックグラウンドエージェント異常終了後の復旧・
  `README.banner.md`フックの実用化**: 直前セッションが異常終了し、
  未コミットの`src/lib.rs`/`src/render.rs`の変更(`generate_readme`/
  `render_readme`に`README.banner.md`読み込みフックを追加。多言語
  READMEリンクや`PORTING.md`案内など、Cargo.toml/rustdocから導出
  できない前置き文を、バッジ行直後・説明文直前に挿入する仕組み)が
  残っていた。テスト付きで完成していたため、`cargo build`/
  `cargo test`(19件全green)で健全性確認の上そのまま採用。
  - **発見した齟齬**: 実装は完成していたのに、当のrs-to-readme
    リポジトリ自身が`README.banner.md`を置いておらず、`--check`が
    実際に失敗する状態だった(README.mdに手書きの多言語バナーは
    あるが、位置が「説明文の後」でツールの生成順序「説明文の前」と
    不一致)。ドキュメント上は「後方互換を壊さない」とあるだけで、
    自己適用(dogfooding)されていなかった。
  - **対処**: `README.banner.md`を新規作成しバナー文を移設、
    `rs-to-readme --path . --output README.md`で実際に再生成し、
    `--check`が green(「README.mdは最新です」)になることを確認。
    エコシステム内の他リポジトリ(open-raid-z/RCosmo/RPoem/
    open-easy-web/open-web-server)を確認したが、いずれも
    CLAUDE.md上でrs-to-readmeに言及するのみで、`README.banner.md`や
    CI組み込み(`--check`実行)は未導入。今回はrs-to-readme本体側の
    整合性修正に留め、他リポジトリへの導入は大規模変更になるため
    見送った。個人情報のハードコードは無し。
  - 次にすべきこと: 他リポジトリ(特にopen-easy-web、10ヶ国語README
    運用中)へ`README.banner.md` + CI `--check`の導入を検討。

## API 方針: REST 全廃・gRPC / Connect へ (2026-11-04)

### 日本語
- **REST は全廃する。新しい API は REST で作らない。** ブラウザや `curl` から呼ぶ API は **Connect の単純呼び出し**(`POST /<パッケージ>.<Service>/<Method>`、`content-type: application/json`、本文は JSON)、サービス間は **gRPC / gRPC-web**(RPoem の管理API `open_runo.admin.v1.Admin` と同じ流儀)。ストリーミングは、必要になってから。
- **既存の REST は、触るときに順に移す。** 移したら旧 REST の経路は消す。移行が終わるまで残るが、**新規には追加しない**。
- **JSON は RS-JSON に統一**する(読み込みは `from_str_secure` / `parse_secure` でサイズ・深さ・重複キーの上限つき、書き出しは `rust_json::fast`)。ブラウザの WASM に `prost` は入れない(サイズが増えるため)。
- **例外(自分たちの API ではなく、相手の規格が決めているもの)**: ACME、OAuth/OIDC のコールバック、第三者(GitHub など)からの Webhook、ヘルスチェック、静的ファイル配信、OTLP など。これらは規格どおりの HTTP を受ける。
- 参考実装: `RPoem`(`open_runo.admin.v1.Admin`)、`open-easy-web`(`server/src/connect.rs`、`proto/open_easy_web/admin/v1/admin.proto`)。
- ※ このリポジトリのコードが既に方針どおりかどうかは、各リポジトリの現状による。この節は方針の記録であり、コードの変更を意味しない。

### English
- **REST is being abolished. New APIs are not built as REST.** APIs called from browsers or `curl` use **Connect unary calls** (`POST /<package>.<Service>/<Method>`, `content-type: application/json`, JSON body); service-to-service calls use **gRPC / gRPC-web** (the same style as RPoem's admin API `open_runo.admin.v1.Admin`). Streaming only when it is actually needed.
- **Existing REST endpoints are migrated one by one, when they are touched.** The old REST route is removed once migrated. Until migration is finished they remain, but **no new ones are added**.
- **JSON is unified on RS-JSON**: reading with `from_str_secure` / `parse_secure` (size, depth and duplicate-key limits), writing with `rust_json::fast`. `prost` is not put into browser WASM (it would grow the binary).
- **Exceptions (shapes dictated by someone else's standard, not our own API)**: ACME, OAuth/OIDC callbacks, webhooks from third parties (e.g. GitHub), health checks, static file serving, OTLP, and so on. These accept plain HTTP as the standard requires.
- Reference implementations: `RPoem` (`open_runo.admin.v1.Admin`) and `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Note: whether the code in a given repository already follows this policy depends on that repository. This section records the policy; it does not mean the code has been changed.

### Deutsch (KI-Übersetzung, noch nicht von Muttersprachlern geprüft)
- **REST wird abgeschafft. Neue APIs werden nicht als REST gebaut.** APIs, die aus dem Browser oder mit `curl` aufgerufen werden, nutzen **Connect-Unary-Aufrufe** (`POST /<Paket>.<Service>/<Methode>`, `content-type: application/json`, JSON-Body); zwischen Diensten gilt **gRPC / gRPC-web** (wie bei der Admin-API von RPoem, `open_runo.admin.v1.Admin`). Streaming erst, wenn es wirklich nötig ist.
- **Bestehende REST-Endpunkte werden einzeln migriert, sobald man sie anfasst.** Nach der Migration wird die alte REST-Route entfernt. Bis dahin bleiben sie bestehen, es werden aber **keine neuen hinzugefügt**.
- **JSON wird auf RS-JSON vereinheitlicht**: Lesen mit `from_str_secure` / `parse_secure` (Grenzen für Größe, Tiefe und doppelte Schlüssel), Schreiben mit `rust_json::fast`. `prost` kommt nicht in Browser-WASM (die Datei würde größer).
- **Ausnahmen (Formate, die ein fremder Standard vorgibt, nicht unsere eigene API)**: ACME, OAuth/OIDC-Callbacks, Webhooks von Dritten (z. B. GitHub), Health-Checks, statische Dateien, OTLP usw. Diese nehmen schlichtes HTTP entgegen, wie es der Standard verlangt.
- Referenzimplementierungen: `RPoem` (`open_runo.admin.v1.Admin`) und `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Hinweis: Ob der Code eines Repositorys diese Richtlinie schon befolgt, hängt vom jeweiligen Repository ab. Dieser Abschnitt hält die Richtlinie fest; er bedeutet nicht, dass der Code bereits geändert wurde.

### Русский (машинный перевод, носителями языка ещё не проверен)
- **REST полностью отменяется. Новые API не создаются как REST.** API, вызываемые из браузера или через `curl`, используют **унарные вызовы Connect** (`POST /<пакет>.<Service>/<Method>`, `content-type: application/json`, тело — JSON); между сервисами — **gRPC / gRPC-web** (так же, как административный API RPoem, `open_runo.admin.v1.Admin`). Потоковая передача — только когда она действительно нужна.
- **Существующие REST-эндпоинты переносятся по одному, когда до них доходят руки.** После переноса старый REST-маршрут удаляется. До завершения переноса они остаются, но **новые не добавляются**.
- **JSON унифицируется на RS-JSON**: чтение через `from_str_secure` / `parse_secure` (ограничения на размер, глубину и повторяющиеся ключи), запись через `rust_json::fast`. `prost` не добавляется в браузерный WASM (размер вырос бы).
- **Исключения (форматы, заданные чужим стандартом, а не наш собственный API)**: ACME, обратные вызовы OAuth/OIDC, вебхуки сторонних сервисов (например, GitHub), проверки работоспособности, раздача статических файлов, OTLP и т. п. Они принимают обычный HTTP, как требует стандарт.
- Эталонные реализации: `RPoem` (`open_runo.admin.v1.Admin`) и `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Примечание: соответствует ли код конкретного репозитория этой политике, зависит от самого репозитория. Этот раздел фиксирует политику и не означает, что код уже изменён.
