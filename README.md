# slack-web-api

English | [日本語](README.ja.md)

[![crates.io](https://img.shields.io/crates/v/slack-web-api.svg)](https://crates.io/crates/slack-web-api)
[![docs.rs](https://img.shields.io/docsrs/slack-web-api)](https://docs.rs/slack-web-api)
[![CI](https://github.com/uiuifree/rust-slack-web-api/actions/workflows/ci.yml/badge.svg)](https://github.com/uiuifree/rust-slack-web-api/actions/workflows/ci.yml)
[![MSRV 1.88](https://img.shields.io/badge/MSRV-1.88-blue.svg)](https://blog.rust-lang.org/)
[![license: MIT](https://img.shields.io/crates/l/slack-web-api.svg)](LICENSE)

Typed async Rust client for **every Slack Web API method** — all 330 of them, including `admin.*` — built on reqwest and rustls.

- No OpenSSL: reqwest 0.13 + rustls, HTTP/2 and a shared connection pool
- Request and response types for every method, plus typed Block Kit
- 30 second request timeout by default; HTTP 429 is returned with its `Retry-After`, and can be retried for you (`max_retries`)
- Cursor pagination and file upload (the replacement for the retired `files.upload`) built in

It covers the Web API only. For Socket Mode or receiving Events API payloads, use another crate.

## Installation

```toml
[dependencies]
slack-web-api = "0.2"
```

## Usage

```rust
use slack_web_api::api::ChatPostMessageRequest;
use slack_web_api::blocks::{Block, SectionBlock, TextObject};
use slack_web_api::SlackClient;

#[tokio::main]
async fn main() -> Result<(), slack_web_api::SlackError> {
    // Create one client per application and clone it: clones share the connection pool.
    let client = SlackClient::new(std::env::var("SLACK_TOKEN").unwrap());

    let res = client
        .chat_post_message(
            &ChatPostMessageRequest::new("C0123456789")
                .text("New application received")
                .blocks(vec![Block::from(SectionBlock::new().text(TextObject::mrkdwn("*New application received*")))]),
        )
        .await?;
    println!("{:?}", res.ts);
    Ok(())
}
```

Method `chat.postMessage` becomes `chat_post_message`, with types `ChatPostMessageRequest` / `ChatPostMessageResponse`.
Required arguments go to `new(...)`; optional ones are set with methods of the same name.

### Sharing your reqwest::Client

```rust
let client = slack_web_api::SlackClient::builder()
    .http_client(app_http_client.clone())
    .token("xoxb-...")
    .build();
```

A client you pass keeps its own settings, so give it a timeout: the built-in client uses 30 seconds
(10 seconds to connect), but `reqwest::Client::new()` has none.
For apps installed in many workspaces, `client.with_token("xoxb-other")` switches the token while keeping the pool.

### Pagination

```rust
use slack_web_api::api::ConversationsListRequest;

let mut pages = client.pages(ConversationsListRequest::new().limit(200));
while let Some(page) = pages.next_page().await {
    for channel in page?.channels {
        println!("{:?}", channel.name);
    }
}
```

### Uploading files

`files.upload` stopped working on 2025-11-12. `upload_files` runs `files.getUploadURLExternal`, the upload and
`files.completeUploadExternal` for you, uploading several files concurrently.

```rust
use slack_web_api::{FileUpload, UploadDestination};

client
    .upload_files(
        vec![FileUpload::new("report.csv", bytes)],
        UploadDestination::channel("C0123456789").initial_comment("Monthly report"),
    )
    .await?;
```

### Untyped calls

New methods or arguments that are not in the generated types yet can be called with `call_raw`:

```rust
let value = client.call_raw("chat.postMessage", &serde_json::json!({"channel": "C1", "text": "hi"})).await?;
```

### Errors

`SlackError::Api` means Slack answered `"ok": false`; `err.api_error()` returns the code such as `channel_not_found`.
Details for errors like `invalid_blocks` are in `response_metadata.messages`.

## Supported APIs

Every method listed at <https://docs.slack.dev/reference/methods> (330 methods) except `files.upload`,
which Slack retired; use `upload_files` instead. The per-method list with Rust function names is in the
[`api` module docs](https://docs.rs/slack-web-api/latest/slack_web_api/api/) and in [`llms-full.txt`](llms-full.txt).

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

Block Kit: every block, block element, rich text element and composition object, plus legacy attachments,
views (modal / home) and message metadata, in `slack_web_api::blocks`.

## How the response types are built

Response types are inferred from the documented response examples of all methods, and the core objects
(Message, Conversation, User, File, ...) are shared across methods. Every field is optional (`Option` or an empty `Vec`),
and scalar fields are read leniently: Slack sometimes returns the same field as a string in one place and a number in
another, and that must not fail the whole response. Unknown fields are skipped; use `call_raw` to see everything.
A test calls every method against a mock server returning its documented examples and checks no value is lost.

## Development

```sh
cargo test                                     # unit, mock-server and generated tests; no network needed
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
SLACK_BOT_TOKEN=xoxb-... cargo run --example post_message -- C0123456789 "Hello from Rust"
```

`cargo test` never calls Slack: every method is exercised against a mock server that returns the documented
response examples, and the test checks that no value is lost when reading them.

### Live tests

`tests/live.rs` talks to the real Slack API. The tests are `#[ignore]`d, so `cargo test` and CI skip them.
Run them with a bot token (`chat:write`, `channels:read`, `channels:history`, `files:write`, `users:read`) and a
channel the bot has joined; they post, update and delete one message and upload and delete two files:

```sh
SLACK_BOT_TOKEN=xoxb-... SLACK_TEST_CHANNEL=C0123456789 \
  cargo test --test live -- --ignored --nocapture --test-threads=1
```

They check what a mock cannot: that Slack accepts the form bodies this crate builds, that the upload flow works
end to end, that real errors are classified — and they print response fields the typed model does not cover yet.

Changes are recorded in [CHANGELOG.md](CHANGELOG.md).

## Regenerating the types

The types are generated in `codegen/` from the Markdown version of the official docs (docs.slack.dev).

```sh
# Save each .md listed in docs.slack.dev/reference/methods.md into <dir>/m and the object pages into <dir>/obj, then:
python3 codegen/parse_docs.py methods <dir>/m > codegen/spec/methods.json
python3 codegen/parse_docs.py objects <dir>/obj > codegen/spec/objects.json
python3 codegen/generate.py && cargo fmt
```

## License

MIT
