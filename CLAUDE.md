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

### Deutsch (KI-Übersetzung, redaktionell überarbeitet; noch nicht von Muttersprachlern geprüft)
- **REST wird vollständig abgeschafft; neue APIs entstehen nicht mehr als REST.** APIs, die aus dem Browser oder per `curl` aufgerufen werden, verwenden **unäre Connect-Aufrufe** (`POST /<Paket>.<Service>/<Methode>`, `content-type: application/json`, JSON-Body). Zwischen Diensten kommen **gRPC bzw. gRPC-web** zum Einsatz, im selben Stil wie die Admin-API von RPoem (`open_runo.admin.v1.Admin`). Streaming wird erst ergänzt, wenn es tatsächlich gebraucht wird.
- **Bestehende REST-Endpunkte werden schrittweise umgestellt, und zwar immer dann, wenn man ohnehin an ihnen arbeitet.** Sobald ein Endpunkt umgestellt ist, wird die alte REST-Route entfernt. Bis zum Abschluss der Migration bleiben die übrigen bestehen, es werden aber **keine neuen mehr hinzugefügt**.
- **JSON wird einheitlich über RS-JSON verarbeitet**: Gelesen wird mit `from_str_secure` bzw. `parse_secure` (mit Obergrenzen für Größe, Verschachtelungstiefe und doppelte Schlüssel), geschrieben mit `rust_json::fast`. `prost` wird nicht in das Browser-WASM aufgenommen, da es die Datei vergrößern würde.
- **Ausnahmen (Schnittstellen, deren Form ein fremder Standard vorgibt und die nicht zu unserer eigenen API gehören)**: ACME, OAuth/OIDC-Callbacks, Webhooks von Drittanbietern (z. B. GitHub), Health-Checks, die Auslieferung statischer Dateien, OTLP und Ähnliches. Diese nehmen, wie vom jeweiligen Standard verlangt, gewöhnliches HTTP entgegen.
- Referenzimplementierungen: `RPoem` (`open_runo.admin.v1.Admin`) und `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Hinweis: Ob der Code eines einzelnen Repositorys diese Richtlinie bereits erfüllt, hängt vom jeweiligen Repository ab. Dieser Abschnitt dokumentiert lediglich die Richtlinie; er besagt nicht, dass der Code schon angepasst wurde.

### Русский (перевод выполнен с помощью ИИ и отредактирован; носителями языка пока не проверен)
- **От REST мы полностью отказываемся; новые API больше не создаются в стиле REST.** API, к которым обращаются из браузера или через `curl`, используют **унарные вызовы Connect** (`POST /<пакет>.<Service>/<Method>`, `content-type: application/json`, тело запроса — JSON). Для взаимодействия между сервисами применяются **gRPC и gRPC-web** — в том же стиле, что и административный API RPoem (`open_runo.admin.v1.Admin`). Потоковая передача данных добавляется лишь тогда, когда в ней действительно возникает необходимость.
- **Существующие REST-эндпоинты переводятся поэтапно — по мере того, как с ними приходится работать.** После перевода старый REST-маршрут удаляется. Пока миграция не завершена, оставшиеся эндпоинты сохраняются, но **новые не добавляются**.
- **Работа с JSON унифицирована на основе RS-JSON**: чтение — через `from_str_secure` или `parse_secure` (с ограничениями на размер, глубину вложенности и повторяющиеся ключи), запись — через `rust_json::fast`. `prost` в браузерный WASM не включается, поскольку это увеличило бы размер файла.
- **Исключения (интерфейсы, форму которых определяет чужой стандарт, а не наш собственный API)**: ACME, обратные вызовы OAuth/OIDC, вебхуки сторонних сервисов (например, GitHub), проверки работоспособности, раздача статических файлов, OTLP и тому подобное. Такие интерфейсы принимают обычный HTTP, как того требует соответствующий стандарт.
- Эталонные реализации: `RPoem` (`open_runo.admin.v1.Admin`) и `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Примечание: соответствует ли код конкретного репозитория этой политике, зависит от самого репозитория. Данный раздел лишь фиксирует политику и не означает, что код уже изменён.

## API 方針の補足: 用語の整理と使い分け (2026-11-04)

### 日本語
**1つに統一するものではなく、別の層の話です。** 「REST の代わり」になるのは Connect・gRPC・GraphQL だけで、WASM は画面を作る技術です。

| 用語 | 何か | REST の代わりか |
|---|---|---|
| REST | URL と HTTP メソッドで操作を表す API の作り方 | 全廃する側 |
| **Connect** | `POST /サービス名/メソッド名` に JSON を送る形式。`curl` でもブラウザの `fetch` でも呼べる | はい(**採用**) |
| **gRPC** | 同じ「サービス/メソッド」の考え方を、バイナリ(protobuf)と HTTP/2 で行う形式。速く、ストリーミングも得意 | はい(**サービス同士に採用**) |
| gRPC-web | gRPC のブラウザ向けの変形版 | はい(必要なら) |
| GraphQL | 欲しい項目を問い合わせる形式 | 一部だけ(**問い合わせ(読み取り)専用**) |
| WASM | Rust をブラウザの中で動かす仕組み | いいえ(画面を作る技術) |

- **ブラウザの画面や `curl` から呼ぶ API は Connect**(POST、JSON を RS-JSON で読み書き)。ブラウザの WASM は素の gRPC を話せず、Connect なら `fetch` のままで、`prost` を足さずに済む(画面のサイズも増えない)。
- **サービス同士の通信(サーバー間、重い処理、ストリーミング)は gRPC。** 多くは Connect で足りるので、ストリーミングが必要になった所だけ gRPC に上げる。
- **GraphQL は、まとめて問い合わせたいときの読み取り専用。** 書き込み(操作)には使わない。
- **WASM は画面を作る技術として、そのまま使う。** 画面から呼ぶ API が Connect になっただけ。
- 外部の規格が形を決めるもの(ACME、OAuth のコールバック、第三者の Webhook、ヘルスチェック、静的ファイル、OTLP)は、規格どおりの HTTP のまま。
- **迷ったら Connect。**

### English
**These are not alternatives to unify; they sit at different layers.** Only Connect, gRPC and GraphQL can replace REST; WASM is a way to build the UI.

| Term | What it is | Replaces REST? |
|---|---|---|
| REST | API style that expresses operations through URLs and HTTP methods | The thing being abolished |
| **Connect** | `POST /Service/Method` with a JSON body; callable from `curl` and from a browser `fetch` | Yes (**adopted**) |
| **gRPC** | The same service/method idea over binary (protobuf) and HTTP/2; fast, good at streaming | Yes (**for service-to-service**) |
| gRPC-web | A browser-friendly variant of gRPC | Yes (if needed) |
| GraphQL | A query language: ask for exactly the fields you want | Partly (**read-only queries**) |
| WASM | Runs Rust inside the browser | No (a UI technology) |

- **APIs called from browser screens or `curl` use Connect** (POST, JSON read and written with RS-JSON). Browser WASM cannot speak plain gRPC; with Connect it stays a `fetch`, and `prost` is not needed (the binary does not grow).
- **Service-to-service calls (server to server, heavy work, streaming) use gRPC.** Connect is enough in most places; move a call to gRPC only where streaming is actually needed.
- **GraphQL is read-only**, for queries that gather several pieces of data. It is not used for writes (operations).
- **WASM stays as the way to build the UI.** The API it calls simply became Connect.
- Interfaces whose shape an outside standard dictates (ACME, OAuth callbacks, third-party webhooks, health checks, static files, OTLP) stay plain HTTP.
- **When in doubt, use Connect.**

### Deutsch (KI-Übersetzung, sprachlich überarbeitet; noch nicht von Muttersprachlern geprüft)
**Es geht nicht darum, eine einzige Technik auszuwählen – die Begriffe liegen auf unterschiedlichen Ebenen.** Als Ersatz für REST kommen nur Connect, gRPC und GraphQL in Frage; WASM dagegen ist eine Technik, mit der die Benutzeroberfläche gebaut wird.

| Begriff | Was ist das? | Ersatz für REST? |
|---|---|---|
| REST | Ein API-Stil, bei dem Operationen über URLs und HTTP-Methoden ausgedrückt werden | wird abgeschafft |
| **Connect** | `POST /Dienst/Methode` mit JSON im Body; aufrufbar mit `curl` ebenso wie mit `fetch` im Browser | ja (**gewählt**) |
| **gRPC** | Dasselbe Prinzip aus Dienst und Methode, aber über binäres protobuf und HTTP/2; schnell und gut für Streaming geeignet | ja (**zwischen Diensten**) |
| gRPC-web | Eine für den Browser angepasste Variante von gRPC | ja (bei Bedarf) |
| GraphQL | Abfragesprache, mit der man genau die benötigten Felder anfordert | teilweise (**nur für lesende Abfragen**) |
| WASM | Führt Rust-Code direkt im Browser aus | nein (Technik für die Oberfläche) |

- **APIs, die von Browseroberflächen oder per `curl` aufgerufen werden, verwenden Connect** (POST; JSON wird mit RS-JSON gelesen und geschrieben). WASM im Browser kann kein reines gRPC sprechen. Mit Connect bleibt es bei einem gewöhnlichen `fetch`, und `prost` ist nicht nötig – die WASM-Datei wird dadurch nicht größer.
- **Für die Kommunikation zwischen Diensten (Server zu Server, rechenintensive Aufgaben, Streaming) wird gRPC eingesetzt.** In den meisten Fällen reicht Connect aus; auf gRPC wird nur dort umgestellt, wo Streaming tatsächlich gebraucht wird.
- **GraphQL dient ausschließlich lesenden Abfragen**, wenn mehrere Informationen gebündelt abgefragt werden sollen. Für Schreibzugriffe (Operationen) wird es nicht verwendet.
- **WASM bleibt die Technik für die Oberfläche.** Geändert hat sich lediglich, dass die aufgerufene API nun auf Connect beruht.
- Schnittstellen, deren Form durch einen externen Standard festgelegt ist (ACME, OAuth-Callbacks, Webhooks von Drittanbietern, Health-Checks, statische Dateien, OTLP), bleiben gewöhnliches HTTP.
- **Im Zweifel: Connect.**

### Русский (перевод выполнен с помощью ИИ и литературно отредактирован; носителями языка пока не проверен)
**Это не варианты, из которых нужно выбрать один, а технологии разных уровней.** Заменить REST могут только Connect, gRPC и GraphQL; WASM же — это способ создания пользовательского интерфейса.

| Термин | Что это | Заменяет REST? |
|---|---|---|
| REST | Стиль построения API, при котором операции выражаются через URL и HTTP-методы | отменяется |
| **Connect** | `POST /Сервис/Метод` с телом в формате JSON; вызывается как из `curl`, так и из браузерного `fetch` | да (**выбран**) |
| **gRPC** | Тот же принцип «сервис — метод», но поверх двоичного формата protobuf и HTTP/2; работает быстро и хорошо подходит для потоковой передачи | да (**для связи между сервисами**) |
| gRPC-web | Адаптированный для браузера вариант gRPC | да (при необходимости) |
| GraphQL | Язык запросов: клиент запрашивает ровно те поля, которые ему нужны | частично (**только запросы на чтение**) |
| WASM | Позволяет выполнять код на Rust прямо в браузере | нет (технология для интерфейса) |

- **API, которые вызываются из браузерных интерфейсов или через `curl`, строятся на Connect** (POST; JSON читается и записывается через RS-JSON). Браузерный WASM не умеет работать с «чистым» gRPC. С Connect достаточно обычного `fetch`, а `prost` не требуется — размер WASM-файла не увеличивается.
- **Для обмена между сервисами (сервер — сервер, ресурсоёмкие задачи, потоковая передача) применяется gRPC.** В большинстве случаев достаточно Connect; на gRPC переходят только там, где потоковая передача действительно необходима.
- **GraphQL используется только для чтения**, когда нужно получить сразу несколько связанных данных. Для записи (операций) он не применяется.
- **WASM по-прежнему остаётся технологией для интерфейса.** Изменилось лишь то, что вызываемый им API теперь построен на Connect.
- Интерфейсы, форма которых задана внешним стандартом (ACME, обратные вызовы OAuth, вебхуки сторонних сервисов, проверки работоспособности, статические файлы, OTLP), остаются обычным HTTP.
- **Если есть сомнения — выбирайте Connect.**
