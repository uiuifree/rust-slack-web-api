# AGENTS.md

Slack Web API の Rust クライアント（crate: slack-web-api）。

## 構成

- 手書き: `src/client.rs`（送受信・429 再送・ページ送り）/ `src/form.rs`（form-urlencoded の直列化）/
  `src/de.rs`（応答のゆるい読み取り）/ `src/upload.rs`（files.getUploadURLExternal → 送信 → completeUploadExternal）/
  `src/error.rs` / `src/response.rs` / `src/lib.rs` / `src/blocks/`（Block Kit）
- 生成物（手で編集しない）: `src/api/*.rs`・`src/objects.rs`・`tests/generated_methods.rs`。
  生成器は `codegen/generate.py`、入力は `codegen/spec/*.json`（`codegen/parse_docs.py` が docs.slack.dev から作る）

## 役割

- **レビュー**（`codex exec review`）: 読み取り専用。指摘だけを返し、ファイルは変更しない。
  「実際に壊れる」ものだけを出す（スタイルの好み・推測の将来要件は出さない）。
  - 重点: 手書き部分と `codegen/*.py`。生成物は1行ずつ読まず、生成器のバグが生成物に出ていないか（型の取り違え・
    必須引数の欠落・送信形式の誤り）を見る
  - 観点: Slack への送信形式（form-urlencoded・配列の扱い）、エラーの取りこぼし、429 再送で二重投稿にならないか、
    ページ送りの停止条件、並行アップロードの順序とエラー、応答の読み取りで不要に失敗しないか、公開 API の型安全
  - 各指摘は `[severity: high|medium|low] path:line — 問題（1文）` + 再現/根拠（どういう入力・状態で壊れるか）の形式で
    確信度の高い順に並べ、最後に「見た範囲・見ていない範囲」を1行で書く。指摘が無ければ「指摘なし」とだけ書く
- commit / push / タグ作成をしない。Slack 本番 API へのアクセスはしない
