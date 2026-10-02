# Changelog

このプロジェクトの注目すべき変更はこのファイルに記録する。
形式は [Keep a Changelog](https://keepachangelog.com/ja/1.1.0/) に、
バージョニングは [Semantic Versioning](https://semver.org/lang/ja/) に準拠する。

## [Unreleased]

## [0.2.1] - 2026-10-02

既定の挙動を2つ変えた（公開 API の形は変わらない）。0.2.0 から `cargo update` で上がるので、
429 の自動再送に頼っていた場合は `max_retries(n)` を指定する。

### Changed

- **429 を既定では再送しない。** `Retry-After` の値つきで `SlackError::RateLimited` をすぐ返す。
  `max_retries(n)` を指定したときだけ `Retry-After` だけ待って再送する（0.2.0 は既定で 3 回再送し、
  `Retry-After: 30` なら最大 90 秒止まっていた）。待ってよい時間は呼び出し側にしか決められないため
- 5xx・通信失敗は従来どおり再送しない（二重投稿を避けるため）

### Fixed

- 自前で作る HTTP クライアントに時間切れが無く、Slack が応答しないと待ち続けていた。
  全体 30 秒・接続 10 秒の時間切れを付けた。`http_client` で渡したクライアントはその設定のまま使う

### Added

- `CHANGELOG.md` / `LICENSE` / GitHub Actions の CI（fmt・clippy・test・doc と MSRV 1.88 のビルド検査）
- `rust-version = "1.88"`（MSRV）
- 実際の Slack に投稿する例 `examples/post_message.rs`
- 実際の Slack に対するライブテスト `tests/live.rs`（全て `#[ignore]`。
  `SLACK_BOT_TOKEN=... SLACK_TEST_CHANNEL=... cargo test --test live -- --ignored --nocapture --test-threads=1` で実行）。
  投稿・更新（空の blocks で消す）・削除、cursor のページ送り、アップロードと削除、実際のエラーの分類、
  型が拾えていない応答の項目の検出を見る

## [0.2.0] - 2026-10-02

全面的な作り直し。0.1 系とは公開 API の互換性が無い。

### Added

- Slack Web API の全メソッド（`admin.*` を含む 330）の引数・応答の型と、メソッドごとの関数
  （`chat.postMessage` → `SlackClient::chat_post_message`）。docs.slack.dev から生成する（`codegen/`）
- Message・Conversation・User・File などの主要オブジェクト（`objects`）
- Block Kit の全ブロック・要素・rich text・構成オブジェクトと、legacy attachment・view・メッセージのメタデータ（`blocks`）
- `SlackClient::upload_files`: 停止した `files.upload` の代わり（`files.getUploadURLExternal` → 送信 →
  `files.completeUploadExternal`。複数ファイルは並行して送る）
- `SlackClient::pages`: cursor によるページ送り
- `SlackClient::call_raw`: 型の無いメソッド・引数を名前で呼ぶ
- `SlackClient::with_token`: 接続プールを共有したままトークンだけ替える
- `SlackClientBuilder`: `reqwest::Client` の共有・送信先（GovSlack・テスト用）・再送回数の設定
- 429 を受けたら `Retry-After` だけ待って再送する（既定 3 回）。5xx・通信失敗は二重投稿を避けるため再送しない
- docs.rs の検索で Slack のメソッド名から関数・型が引ける（doc alias）。`llms.txt` / `llms-full.txt`
- 英語の `README.md` と日本語の `README.ja.md`

### Changed

- 通信を hyper 0.14 + hyper-tls（OpenSSL）から reqwest 0.13 + rustls（HTTP/2・gzip）に替えた。
  クライアントは接続プールを持ち、clone しても同じプールを使う（0.1 は呼ぶたびにクライアントを作り直していた）
- 送信は全メソッド `application/x-www-form-urlencoded`。文字列の配列はカンマ区切り、
  blocks・attachments・オブジェクトの配列は空でも JSON 配列で送る
- 応答は全項目を省略可能にし、Slack が同じ項目を文字列で返したり数値で返したりしても失敗しないように読む。
  応答例に項目が無いメソッドは、受け取った値を全部 `extra` に残す
- エラーを `SlackError`（`Api` / `RateLimited` / `Http` / `Transport` / `Decode` / `Encode` / `MissingField` / `Task`）にまとめた
- 型名から `Slack` 接頭辞を外した（`SlackApiChatPostMessageRequest` → `api::ChatPostMessageRequest` など）

### Removed

- `files_upload`（Slack が 2025-11-12 に `files.upload` を停止したため）。`upload_files` を使う
- `SlackMessageBuilder` / `SlackAttachmentBuilder`（各リクエスト型の `new` と同名のメソッドで組み立てる）
- 依存の `hyper` 0.14・`hyper-tls`・`mpart-async`（と、将来の Rust で通らなくなる `multipart` / `buf_redux` / `traitobject`）

## [0.1.1] - 2024-04-01

- chat（9 メソッド）・files（14 メソッド）の型と呼び出し。この時点までの細かい変更の記録は無い

[Unreleased]: https://github.com/uiuifree/rust-slack-web-api/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/uiuifree/rust-slack-web-api/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/uiuifree/rust-slack-web-api/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/uiuifree/rust-slack-web-api/releases/tag/v0.1.1
