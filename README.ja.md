# slack-web-api

[English](README.md) | 日本語

Slack Web API の全メソッド（330、`admin.*` を含む）を型付きで呼べる、非同期の Rust クライアント。

- 通信は reqwest + rustls（OpenSSL 不要）。HTTP/2 と接続プールを使い回す
- 全メソッドの引数・応答の型と、Block Kit の型を持つ
- 429 は `Retry-After` を見て自動で再送（既定 3 回）
- cursor のページ送り、ファイルのアップロード（`files.upload` の後継手順）をまとめて提供

Web API（こちらから呼ぶ側）専用。Socket Mode や Events API の受信には使えない。

## インストール

```toml
[dependencies]
slack-web-api = "0.2"
```

## 使い方

```rust
use slack_web_api::api::ChatPostMessageRequest;
use slack_web_api::blocks::{Block, SectionBlock, TextObject};
use slack_web_api::SlackClient;

#[tokio::main]
async fn main() -> Result<(), slack_web_api::SlackError> {
    // アプリ全体で1つ作って clone して使う（接続プールを共有する）
    let client = SlackClient::new(std::env::var("SLACK_TOKEN").unwrap());

    let res = client
        .chat_post_message(
            &ChatPostMessageRequest::new("C0123456789")
                .text("応募がありました")
                .blocks(vec![Block::from(SectionBlock::new().text(TextObject::mrkdwn("*応募がありました*")))]),
        )
        .await?;
    println!("{:?}", res.ts);
    Ok(())
}
```

メソッド名は `chat.postMessage` → `chat_post_message`、型は `ChatPostMessageRequest` / `ChatPostMessageResponse`。
引数は `new(必須の引数)` に、任意の引数を同名のメソッドで足していく。

### 既存の reqwest::Client と接続プールを共有する

```rust
let client = slack_web_api::SlackClient::builder()
    .http_client(app_http_client.clone())
    .token("xoxb-...")
    .build();
```

複数ワークスペースを扱うときは `client.with_token("xoxb-other")` で、接続プールを共有したままトークンだけ替える。

### ページ送り

```rust
use slack_web_api::api::ConversationsListRequest;

let mut pages = client.pages(ConversationsListRequest::new().limit(200));
while let Some(page) = pages.next_page().await {
    for channel in page?.channels {
        println!("{:?}", channel.name);
    }
}
```

### ファイルのアップロード

`files.upload` は 2025-11-12 に停止した。`upload_files` が `files.getUploadURLExternal` → アップロード →
`files.completeUploadExternal` をまとめて行う（複数ファイルは並行して送る）。

```rust
use slack_web_api::{FileUpload, UploadDestination};

client
    .upload_files(
        vec![FileUpload::new("report.csv", bytes)],
        UploadDestination::channel("C0123456789").initial_comment("今月分"),
    )
    .await?;
```

### 型の無い呼び出し

まだ型の無い新しいメソッドや引数は `call_raw` で呼べる。

```rust
let value = client.call_raw("chat.postMessage", &serde_json::json!({"channel": "C1", "text": "hi"})).await?;
```

### エラー

`SlackError::Api` は Slack が `"ok": false` を返したとき。`err.api_error()` で `channel_not_found` などの値が取れる。
`invalid_blocks` などの詳しい理由は `response_metadata.messages` に入る。

## 対応している API

<https://docs.slack.dev/reference/methods> にあるメソッドのうち、Slack が停止した `files.upload` を除く 330 メソッド。
`files.upload` の代わりには `upload_files` を使う。メソッドごとの一覧（Rust の関数名つき）は
[docs.rs の `api` モジュール](https://docs.rs/slack-web-api/latest/slack_web_api/api/)と [`llms-full.txt`](llms-full.txt) にある。

<!-- methods:start -->
| Category | Methods | Examples |
|---|---:|---|
| `agents.*` | 2 | `agents.sessions.rename`, `agents.sessions.setStatus` |
| `api.*` | 1 | `api.test` |
| `apps.*` | 23 | `apps.activities.list`, `apps.auth.external.delete`, `apps.auth.external.get`, ... |
| `assistant.*` | 5 | `assistant.search.context`, `assistant.search.info`, `assistant.threads.setStatus`, ... |
| `auth.*` | 3 | `auth.revoke`, `auth.teams.list`, `auth.test` |
| `blocks.*` | 1 | `blocks.validate` |
| `bookmarks.*` | 4 | `bookmarks.add`, `bookmarks.edit`, `bookmarks.list`, ... |
| `bots.*` | 1 | `bots.info` |
| `calls.*` | 6 | `calls.add`, `calls.end`, `calls.info`, ... |
| `canvases.*` | 7 | `canvases.access.delete`, `canvases.access.set`, `canvases.create`, ... |
| `chat.*` | 13 | `chat.appendStream`, `chat.delete`, `chat.deleteScheduledMessage`, ... |
| `conversations.*` | 28 | `conversations.acceptSharedInvite`, `conversations.approveSharedInvite`, `conversations.archive`, ... |
| `dialog.*` | 1 | `dialog.open` |
| `dnd.*` | 5 | `dnd.endDnd`, `dnd.endSnooze`, `dnd.info`, ... |
| `emoji.*` | 1 | `emoji.list` |
| `entity.*` | 3 | `entity.acknowledgeCommentAction`, `entity.presentComments`, `entity.presentDetails` |
| `files.*` | 14 | `files.comments.delete`, `files.completeUploadExternal`, `files.delete`, ... |
| `functions.*` | 8 | `functions.completeError`, `functions.completeSuccess`, `functions.distributions.permissions.add`, ... |
| `migration.*` | 1 | `migration.exchange` |
| `oauth.*` | 6 | `oauth.access`, `oauth.v2.access`, `oauth.v2.beginShortTokenRotation`, ... |
| `openid.*` | 2 | `openid.connect.token`, `openid.connect.userInfo` |
| `pins.*` | 3 | `pins.add`, `pins.list`, `pins.remove` |
| `reactions.*` | 4 | `reactions.add`, `reactions.get`, `reactions.list`, ... |
| `reminders.*` | 5 | `reminders.add`, `reminders.complete`, `reminders.delete`, ... |
| `rtm.*` | 2 | `rtm.connect`, `rtm.start` |
| `search.*` | 3 | `search.all`, `search.files`, `search.messages` |
| `slackLists.*` | 12 | `slackLists.access.delete`, `slackLists.access.set`, `slackLists.create`, ... |
| `stars.*` | 3 | `stars.add`, `stars.list`, `stars.remove` |
| `team.*` | 9 | `team.accessLogs`, `team.billableInfo`, `team.billing.info`, ... |
| `tooling.*` | 1 | `tooling.tokens.rotate` |
| `usergroups.*` | 7 | `usergroups.create`, `usergroups.disable`, `usergroups.enable`, ... |
| `users.*` | 13 | `users.conversations`, `users.deletePhoto`, `users.discoverableContacts.lookup`, ... |
| `views.*` | 4 | `views.open`, `views.publish`, `views.push`, ... |
| `workflows.*` | 8 | `workflows.featured.add`, `workflows.featured.list`, `workflows.featured.remove`, ... |
| `admin.*` | 121 | `admin.analytics.getFile`, `admin.analytics.messages.activity`, `admin.analytics.messages.metadata`, ... |
<!-- methods:end -->

Block Kit: 全ブロック・ブロック要素・rich text 要素・構成オブジェクトと、legacy attachment・view（modal / home）・
メッセージのメタデータ（`slack_web_api::blocks`）。

## 応答の型について

応答の型はドキュメントの応答例を全メソッド分重ねて作っている。項目は全部省略可能（`Option` か空の `Vec`）で、
Slack が同じ項目を文字列で返したり数値で返したりしても失敗しないよう、ゆるく読む（`ts` が数値で来ても文字列にする等）。
型に無い項目は読み飛ばす。全部見たいときは `call_raw` を使う。

## 型の再生成

型は `codegen/` で Slack 公式ドキュメント（docs.slack.dev の Markdown 版）から生成している。

```sh
# docs.slack.dev/reference/methods.md の一覧にある各 .md を <dir>/m に、objects の .md を <dir>/obj に保存してから
python3 codegen/parse_docs.py methods <dir>/m > codegen/spec/methods.json
python3 codegen/parse_docs.py objects <dir>/obj > codegen/spec/objects.json
python3 codegen/generate.py && cargo fmt
```

## License

MIT
