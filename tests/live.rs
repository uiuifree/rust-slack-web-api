//! Live tests against the real Slack Web API.
//!
//! Every test here is `#[ignore]`d: `cargo test` skips them, so CI and everyday runs stay offline.
//! Run them deliberately with a bot token and a test channel the bot has joined:
//!
//! ```sh
//! SLACK_BOT_TOKEN=xoxb-... SLACK_TEST_CHANNEL=C0123456789 \
//!   cargo test --test live -- --ignored --nocapture --test-threads=1
//! ```
//!
//! Bot token scopes: `chat:write`, `channels:read`, `channels:history`, `files:write`, `users:read`.
//! The tests post, update and delete one message and upload and delete two files in the test channel.
//!
//! They check what the mock server cannot: that Slack accepts the form bodies this crate builds
//! (comma-separated lists, JSON-encoded blocks, an empty `[]` to clear blocks), that the upload flow
//! works end to end, that real errors are classified, and they print response fields that the typed
//! model does not cover yet.
use serde_json::Value;
use slack_web_api::api::{
    AuthTestRequest, ChatDeleteRequest, ChatGetPermalinkRequest, ChatPostMessageRequest,
    ChatUpdateRequest, ConversationsHistoryRequest, ConversationsInfoRequest,
    ConversationsListRequest, FilesDeleteRequest, UsersInfoRequest,
};
use slack_web_api::blocks::{Block, DividerBlock, MessageMetadata, SectionBlock, TextObject};
use slack_web_api::{FileUpload, SlackClient, SlackError, UploadDestination};

/// 実 API のトークン。無ければ「実行の仕方を間違えた」ので落とす
fn client() -> SlackClient {
    let token = std::env::var("SLACK_BOT_TOKEN").expect(
        "live tests need SLACK_BOT_TOKEN and SLACK_TEST_CHANNEL: see the top of tests/live.rs",
    );
    SlackClient::new(token)
}

fn channel() -> String {
    std::env::var("SLACK_TEST_CHANNEL")
        .expect("set SLACK_TEST_CHANNEL to a channel ID the bot has joined")
}

/// 生の応答にあって、型に読んで書き戻すと消える項目を列挙する（型が拾えていない項目の検出）
fn uncovered(raw: &Value, typed: &Value, at: &str, out: &mut Vec<String>) {
    match (raw, typed) {
        (Value::Object(r), Value::Object(t)) => {
            for (k, v) in r {
                let path = format!("{at}.{k}");
                match t.get(k) {
                    Some(w) => uncovered(v, w, &path, out),
                    None if v.is_null()
                        || v == &Value::Array(vec![])
                        || v == &Value::Object(Default::default()) => {}
                    None => out.push(path),
                }
            }
        }
        (Value::Array(r), Value::Array(t)) => {
            for (i, (v, w)) in r.iter().zip(t).enumerate() {
                uncovered(v, w, &format!("{at}[{i}]"), out);
            }
        }
        _ => {}
    }
}

fn report_uncovered<T: serde::Serialize>(method: &str, raw: &Value, typed: &T) {
    let mut missing = vec![];
    uncovered(raw, &serde_json::to_value(typed).unwrap(), "", &mut missing);
    missing.retain(|p| p != ".ok");
    if missing.is_empty() {
        println!("{method}: every returned field is covered by the typed response");
    } else {
        // Slack が足した新しい項目。失敗にはせず、型の作り直しの材料として出す
        println!("{method}: fields not covered by the typed response: {missing:?}");
    }
}

#[tokio::test]
#[ignore = "calls the real Slack API; needs SLACK_BOT_TOKEN"]
async fn トークンが通り型付きの応答が読める() {
    let client = client();
    let res = client
        .auth_test(&AuthTestRequest::new())
        .await
        .expect("auth.test failed");
    println!("team {:?} / bot user {:?}", res.team_id, res.user_id);
    assert!(res.team_id.is_some() && res.user_id.is_some());

    let raw = client
        .call_raw("auth.test", &serde_json::json!({}))
        .await
        .unwrap();
    report_uncovered("auth.test", &raw, &res);

    let user_id = res.user_id.unwrap();
    let user = client
        .users_info(&UsersInfoRequest::new().user(user_id.clone()))
        .await
        .expect("users.info failed");
    assert_eq!(
        user.user.as_ref().and_then(|u| u.id.as_deref()),
        Some(user_id.as_str())
    );
    let raw = client
        .call_raw("users.info", &serde_json::json!({ "user": user_id }))
        .await
        .unwrap();
    report_uncovered("users.info", &raw, &user);
}

#[tokio::test]
#[ignore = "calls the real Slack API; posts and deletes a message; needs SLACK_BOT_TOKEN"]
async fn メッセージを投稿し更新し削除できる() {
    let client = client();
    let channel = channel();

    let posted = client
        .chat_post_message(
            &ChatPostMessageRequest::new(&channel)
                .text("slack-web-api live test")
                .blocks(vec![
                    Block::from(
                        SectionBlock::new().text(TextObject::mrkdwn("*slack-web-api* live test")),
                    ),
                    Block::from(DividerBlock::new()),
                ])
                .metadata(MessageMetadata::new(
                    "live_test",
                    serde_json::json!({ "crate": "slack-web-api" }),
                )),
        )
        .await
        .expect("chat.postMessage failed");
    let ts = posted.ts.expect("chat.postMessage returned no ts");
    assert_eq!(posted.message.as_ref().map(|m| m.blocks.len()), Some(2));

    let link = client
        .chat_get_permalink(&ChatGetPermalinkRequest::new(&channel, &ts))
        .await
        .expect("chat.getPermalink failed");
    println!("posted: {:?}", link.permalink);

    // 空の blocks は `[]` で送られ、Slack はブロックを消す
    let updated = client
        .chat_update(
            &ChatUpdateRequest::new(&channel, &ts)
                .blocks(vec![])
                .text("updated"),
        )
        .await
        .expect("chat.update failed");
    assert_eq!(updated.ts.as_deref(), Some(ts.as_str()));

    let history = client
        .conversations_history(
            &ConversationsHistoryRequest::new(&channel)
                .latest(&ts)
                .inclusive(true)
                .limit(1)
                .include_all_metadata(true),
        )
        .await
        .expect("conversations.history failed");
    let message = history
        .messages
        .first()
        .expect("the posted message is not in the history");
    assert_eq!(message.text.as_deref(), Some("updated"));
    assert!(
        message.blocks.is_empty()
            || message
                .blocks
                .iter()
                .all(|b| !matches!(b, Block::Section(_)))
    );
    assert_eq!(
        message.metadata.as_ref().map(|m| m.event_type.as_str()),
        Some("live_test")
    );

    client
        .chat_delete(&ChatDeleteRequest::new(&channel, &ts))
        .await
        .expect("chat.delete failed");
}

#[tokio::test]
#[ignore = "calls the real Slack API; needs SLACK_BOT_TOKEN"]
async fn カーソルでページを送れる() {
    let client = client();
    let mut pages = client.pages(
        ConversationsListRequest::new()
            .limit(1)
            .types("public_channel"),
    );
    let mut ids = vec![];
    while let Some(page) = pages.next_page().await {
        let page = page.expect("conversations.list failed");
        ids.extend(page.channels.into_iter().filter_map(|c| c.id));
        if ids.len() >= 3 {
            break;
        }
    }
    println!("first channels: {ids:?}");
    assert!(!ids.is_empty());
    let unique: std::collections::HashSet<_> = ids.iter().collect();
    assert_eq!(
        unique.len(),
        ids.len(),
        "the cursor returned the same channel twice"
    );

    let info = client
        .conversations_info(&ConversationsInfoRequest::new(&ids[0]))
        .await
        .expect("conversations.info failed");
    let raw = client
        .call_raw(
            "conversations.info",
            &serde_json::json!({ "channel": ids[0] }),
        )
        .await
        .unwrap();
    report_uncovered("conversations.info", &raw, &info);
}

#[tokio::test]
#[ignore = "calls the real Slack API; uploads and deletes two files; needs SLACK_BOT_TOKEN"]
async fn ファイルをアップロードして削除できる() {
    let client = client();
    let res = client
        .upload_files(
            vec![
                FileUpload::new("live-test.txt", "slack-web-api live test\n"),
                FileUpload::new("live-test.csv", "a,b\n1,2\n").title("Live test CSV"),
            ],
            UploadDestination::channel(channel()).initial_comment("slack-web-api live test upload"),
        )
        .await
        .expect("upload_files failed");
    let ids: Vec<String> = res.files.into_iter().filter_map(|f| f.id).collect();
    println!("uploaded: {ids:?}");
    assert_eq!(ids.len(), 2);
    for id in ids {
        client
            .files_delete(&FilesDeleteRequest::new(id))
            .await
            .expect("files.delete failed");
    }
}

#[tokio::test]
#[ignore = "calls the real Slack API; needs SLACK_BOT_TOKEN"]
async fn 実際のエラーを分類できる() {
    let client = client();
    let err = client
        .conversations_info(&ConversationsInfoRequest::new("C00000000"))
        .await
        .expect_err("a channel that does not exist should fail");
    assert_eq!(err.api_error(), Some("channel_not_found"));

    let err = client
        .with_token("xoxb-invalid")
        .auth_test(&AuthTestRequest::new())
        .await
        .expect_err("an invalid token should fail");
    assert!(matches!(err, SlackError::Api(_)), "{err:?}");
    assert_eq!(err.api_error(), Some("invalid_auth"));
}
