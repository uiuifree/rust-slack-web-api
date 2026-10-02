use slack_web_api::api::{ConversationsListRequest, ConversationsListResponse};
use slack_web_api::{SlackClient, SlackError};
use std::time::Duration;
use wiremock::matchers::{body_string, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn server() -> (MockServer, SlackClient) {
    let server = MockServer::start().await;
    let client = SlackClient::builder()
        .token("xoxb-1")
        .base_url(format!("{}/api", server.uri()))
        .build();
    (server, client)
}

fn json(body: &str) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_raw(body.to_owned(), "application/json")
}

#[tokio::test]
async fn sends_form_body_with_bearer_token() {
    let (server, client) = server().await;
    Mock::given(method("POST"))
        .and(path("/api/conversations.list"))
        .and(header("authorization", "Bearer xoxb-1"))
        .and(header("content-type", "application/x-www-form-urlencoded"))
        .and(body_string("limit=2&types=public_channel%2Cim"))
        .respond_with(json(
            r#"{"ok":true,"channels":[{"id":"C1","name":"general","is_channel":true}]}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;
    let res = client
        .conversations_list(
            &ConversationsListRequest::new()
                .limit(2)
                .types("public_channel,im"),
        )
        .await
        .unwrap();
    assert_eq!(res.channels[0].id.as_deref(), Some("C1"));
    assert_eq!(res.channels[0].is_channel, Some(true));
}

#[tokio::test]
async fn api_errors_carry_details() {
    let (server, client) = server().await;
    Mock::given(path("/api/chat.postMessage"))
        .respond_with(json(
            r#"{"ok":false,"error":"invalid_blocks","warning":"missing_charset","response_metadata":{"messages":["[ERROR] bad"]}}"#,
        ))
        .mount(&server)
        .await;
    let err = client
        .call_raw("chat.postMessage", &serde_json::json!({"channel": "C1"}))
        .await
        .unwrap_err();
    assert_eq!(err.api_error(), Some("invalid_blocks"));
    assert_eq!(err.to_string(), "slack api error: invalid_blocks");
    let SlackError::Api(api) = err else {
        panic!("expected api error")
    };
    assert_eq!(api.warning.as_deref(), Some("missing_charset"));
    assert_eq!(
        api.response_metadata.unwrap().messages,
        vec!["[ERROR] bad".to_string()]
    );
}

#[tokio::test]
async fn api_error_without_error_field() {
    let (server, client) = server().await;
    Mock::given(path("/api/api.test"))
        .respond_with(json(r#"{"ok":false}"#))
        .mount(&server)
        .await;
    let err = client.call_raw("api.test", &()).await.unwrap_err();
    assert_eq!(err.api_error(), Some(""));
}

#[tokio::test]
async fn retries_after_rate_limit() {
    let (server, client) = server().await;
    Mock::given(path("/api/api.test"))
        .respond_with(ResponseTemplate::new(429).insert_header("retry-after", "0"))
        .up_to_n_times(2)
        .mount(&server)
        .await;
    Mock::given(path("/api/api.test"))
        .respond_with(json(r#"{"ok":true,"args":{}}"#))
        .mount(&server)
        .await;
    let res = client.call_raw("api.test", &()).await.unwrap();
    assert_eq!(res["ok"], true);
}

#[tokio::test]
async fn rate_limit_without_retry_after_waits_one_second() {
    let (server, client) = server().await;
    Mock::given(path("/api/api.test"))
        .respond_with(ResponseTemplate::new(429))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(path("/api/api.test"))
        .respond_with(json(r#"{"ok":true}"#))
        .mount(&server)
        .await;
    let started = std::time::Instant::now();
    client.call_raw("api.test", &()).await.unwrap();
    assert!(started.elapsed() >= Duration::from_secs(1));
}

#[tokio::test]
async fn gives_up_after_max_retries() {
    let server = MockServer::start().await;
    let client = SlackClient::builder()
        .base_url(format!("{}/api/", server.uri()))
        .max_retries(0)
        .build();
    Mock::given(path("/api/api.test"))
        .respond_with(ResponseTemplate::new(429).insert_header("retry-after", "30"))
        .mount(&server)
        .await;
    let err = client.call_raw("api.test", &()).await.unwrap_err();
    assert!(
        matches!(err, SlackError::RateLimited { retry_after: Some(d) } if d == Duration::from_secs(30))
    );
    assert_eq!(err.api_error(), None);
}

#[tokio::test]
async fn non_json_error_status_is_http_error() {
    let (server, client) = server().await;
    Mock::given(path("/api/api.test"))
        .respond_with(ResponseTemplate::new(503).set_body_string("down"))
        .mount(&server)
        .await;
    let err = client.call_raw("api.test", &()).await.unwrap_err();
    assert!(matches!(err, SlackError::Http { status: 503, ref body } if body == "down"));
}

#[tokio::test]
async fn undecodable_bodies_are_decode_errors() {
    let (server, client) = server().await;
    Mock::given(path("/api/api.test"))
        .respond_with(ResponseTemplate::new(200).set_body_string("<html>"))
        .mount(&server)
        .await;
    Mock::given(path("/api/conversations.list"))
        .respond_with(json(r#"{"ok":true,"channels":[1]}"#))
        .mount(&server)
        .await;
    let err = client.call_raw("api.test", &()).await.unwrap_err();
    assert!(matches!(err, SlackError::Decode { ref body, .. } if body == "<html>"));
    let err = client
        .conversations_list(&ConversationsListRequest::new())
        .await
        .unwrap_err();
    assert!(err.to_string().starts_with("failed to decode response"));
}

#[tokio::test]
async fn encode_errors_are_reported_before_sending() {
    let (_server, client) = server().await;
    let err = client.call_raw("api.test", &1).await.unwrap_err();
    assert_eq!(
        err.to_string(),
        "failed to encode request: form body must be a struct or map"
    );
}

#[tokio::test]
async fn transport_errors_are_reported() {
    let client = SlackClient::builder().base_url("not a url").build();
    let err = client.call_raw("api.test", &()).await.unwrap_err();
    assert!(matches!(err, SlackError::Transport(_)));
}

#[tokio::test]
async fn with_token_shares_settings_and_swaps_token() {
    let (server, client) = server().await;
    Mock::given(path("/api/auth.test"))
        .and(header("authorization", "Bearer xoxb-2"))
        .respond_with(json(r#"{"ok":true,"team_id":"T2"}"#))
        .expect(1)
        .mount(&server)
        .await;
    let other = client.with_token("xoxb-2");
    let res = other
        .auth_test(&slack_web_api::api::AuthTestRequest::new())
        .await
        .unwrap();
    assert_eq!(res.team_id.as_deref(), Some("T2"));
}

#[test]
fn debug_hides_the_token() {
    let client = SlackClient::new("xoxb-secret");
    let debug = format!("{client:?}");
    assert!(!debug.contains("secret"));
    assert!(debug.contains("***"));
    assert!(debug.contains("https://slack.com/api/"));
    let anonymous = SlackClient::builder()
        .http_client(reqwest::Client::new())
        .build();
    assert!(format!("{anonymous:?}").contains("token: None"));
}

#[tokio::test]
async fn pages_follow_next_cursor() {
    let (server, client) = server().await;
    Mock::given(path("/api/conversations.list"))
        .and(body_string("limit=1"))
        .respond_with(json(
            r#"{"ok":true,"channels":[{"id":"C1"}],"response_metadata":{"next_cursor":"abc"}}"#,
        ))
        .mount(&server)
        .await;
    Mock::given(path("/api/conversations.list"))
        .and(body_string("cursor=abc&limit=1"))
        .respond_with(json(
            r#"{"ok":true,"channels":[{"id":"C2"}],"response_metadata":{"next_cursor":""}}"#,
        ))
        .mount(&server)
        .await;
    let mut pages = client.pages(ConversationsListRequest::new().limit(1));
    let mut ids = vec![];
    while let Some(page) = pages.next_page().await {
        let page: ConversationsListResponse = page.unwrap();
        ids.extend(page.channels.into_iter().filter_map(|c| c.id));
    }
    assert_eq!(ids, vec!["C1", "C2"]);
    assert!(pages.next_page().await.is_none());
}

#[tokio::test]
async fn pages_stop_after_an_error() {
    let (server, client) = server().await;
    Mock::given(path("/api/conversations.list"))
        .respond_with(json(r#"{"ok":false,"error":"invalid_cursor"}"#))
        .mount(&server)
        .await;
    let mut pages = client.pages(ConversationsListRequest::new());
    assert!(pages.next_page().await.unwrap().is_err());
    assert!(pages.next_page().await.is_none());
}

#[tokio::test]
async fn file_download_methods_return_bytes_or_errors() {
    use slack_web_api::api::AdminAnalyticsGetFileRequest;
    let (server, client) = server().await;
    let req = AdminAnalyticsGetFileRequest::new("member").date("2020-09-01");
    Mock::given(path("/api/admin.analytics.getFile"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(vec![0x1f, 0x8b, 0x08]))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    assert_eq!(
        &client.admin_analytics_get_file(&req).await.unwrap()[..],
        &[0x1f, 0x8b, 0x08]
    );

    Mock::given(path("/api/admin.analytics.getFile"))
        .respond_with(json(r#" {"ok":false,"error":"file_not_yet_available"}"#))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    let err = client.admin_analytics_get_file(&req).await.unwrap_err();
    assert_eq!(err.api_error(), Some("file_not_yet_available"));

    Mock::given(path("/api/admin.analytics.getFile"))
        .respond_with(ResponseTemplate::new(500).set_body_string("oops"))
        .mount(&server)
        .await;
    let err = client.admin_analytics_get_file(&req).await.unwrap_err();
    assert!(matches!(err, SlackError::Http { status: 500, .. }));
}

#[tokio::test]
async fn empty_block_lists_are_sent_as_json_arrays() {
    use slack_web_api::api::ChatUpdateRequest;
    let (server, client) = server().await;
    Mock::given(path("/api/chat.update"))
        .and(body_string(
            "channel=C1&ts=1.2&attachments=%5B%5D&blocks=%5B%5D&text=t",
        ))
        .respond_with(json(r#"{"ok":true,"channel":"C1","ts":"1.2"}"#))
        .expect(1)
        .mount(&server)
        .await;
    client
        .chat_update(
            &ChatUpdateRequest::new("C1", "1.2")
                .attachments(vec![])
                .blocks(vec![])
                .text("t"),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn metadata_without_payload_and_unknown_fields_are_kept() {
    use slack_web_api::api::{ConversationsHistoryRequest, EmojiListRequest, ReactionsAddRequest};
    let (server, client) = server().await;
    Mock::given(path("/api/conversations.history"))
        .respond_with(json(r#"{"ok":true,"messages":[{"ts":"1.2","text":"x","metadata":{"event_type":"task_created"}}]}"#))
        .mount(&server)
        .await;
    Mock::given(path("/api/emoji.list"))
        .respond_with(json(
            r#"{"ok":true,"emoji":{"party_parrot":"https://e/p.gif"}}"#,
        ))
        .mount(&server)
        .await;
    Mock::given(path("/api/reactions.add"))
        .respond_with(json(r#"{"ok":true,"undocumented":1}"#))
        .mount(&server)
        .await;
    let history = client
        .conversations_history(&ConversationsHistoryRequest::new("C1"))
        .await
        .unwrap();
    let metadata = history.messages[0].metadata.as_ref().unwrap();
    assert_eq!(metadata.event_type, "task_created");
    assert!(metadata.event_payload.is_null());
    let emoji = client.emoji_list(&EmojiListRequest::new()).await.unwrap();
    assert_eq!(emoji.emoji.unwrap()["party_parrot"], "https://e/p.gif");
    let added = client
        .reactions_add(&ReactionsAddRequest::new("C1", "thumbsup", "1.2"))
        .await
        .unwrap();
    assert_eq!(added.extra["undocumented"], 1);
}
