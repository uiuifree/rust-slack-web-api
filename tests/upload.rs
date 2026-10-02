use slack_web_api::{FileUpload, SlackClient, SlackError, UploadDestination};
use wiremock::matchers::{body_string, body_string_contains, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn json(body: String) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_raw(body, "application/json")
}

async fn setup() -> (MockServer, SlackClient) {
    let server = MockServer::start().await;
    let client = SlackClient::builder()
        .token("xoxb-1")
        .base_url(format!("{}/api", server.uri()))
        .build();
    (server, client)
}

async fn mount_upload_url(server: &MockServer, filename: &str, file_id: &str) {
    Mock::given(path("/api/files.getUploadURLExternal"))
        .and(body_string_contains(format!("filename={filename}")))
        .respond_with(json(format!(
            r#"{{"ok":true,"upload_url":"{}/upload/{file_id}","file_id":"{file_id}"}}"#,
            server.uri()
        )))
        .mount(server)
        .await;
}

#[tokio::test]
async fn uploads_files_in_parallel_and_completes_in_order() {
    let (server, client) = setup().await;
    mount_upload_url(&server, "a.txt", "F1").await;
    mount_upload_url(&server, "b.png", "F2").await;
    Mock::given(method("POST"))
        .and(path("/upload/F1"))
        .and(body_string("hello"))
        .respond_with(ResponseTemplate::new(200).set_body_string("OK - 5"))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/upload/F2"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(path("/api/files.completeUploadExternal"))
        .and(body_string(
            "files=%5B%7B%22id%22%3A%22F1%22%2C%22title%22%3A%22a.txt%22%7D%2C%7B%22id%22%3A%22F2%22%2C%22title%22%3A%22Chart%22%7D%5D\
             &channel_id=C1&thread_ts=1.2&initial_comment=see",
        ))
        .respond_with(json(r#"{"ok":true,"files":[{"id":"F1","title":"a.txt"},{"id":"F2","title":"Chart"}]}"#.into()))
        .expect(1)
        .mount(&server)
        .await;

    let res = client
        .upload_files(
            vec![
                FileUpload::new("a.txt", "hello"),
                FileUpload::new("b.png", vec![1u8, 2, 3])
                    .title("Chart")
                    .alt_txt("chart")
                    .snippet_type("png"),
            ],
            UploadDestination::channel("C1")
                .thread_ts("1.2")
                .initial_comment("see"),
        )
        .await
        .unwrap();
    let ids: Vec<_> = res.files.iter().filter_map(|f| f.id.as_deref()).collect();
    assert_eq!(ids, vec!["F1", "F2"]);
}

#[tokio::test]
async fn missing_upload_url_is_an_error() {
    let (server, client) = setup().await;
    Mock::given(path("/api/files.getUploadURLExternal"))
        .respond_with(json(r#"{"ok":true}"#.into()))
        .mount(&server)
        .await;
    let err = client
        .upload_files(
            vec![FileUpload::new("a.txt", "x")],
            UploadDestination::default(),
        )
        .await
        .unwrap_err();
    assert!(matches!(err, SlackError::MissingField(_)));
    assert!(err.to_string().starts_with("response is missing"));
}

#[tokio::test]
async fn failed_upload_stops_before_completing() {
    let (server, client) = setup().await;
    mount_upload_url(&server, "a.txt", "F1").await;
    Mock::given(path("/upload/F1"))
        .respond_with(ResponseTemplate::new(413).set_body_string("too large"))
        .mount(&server)
        .await;
    Mock::given(path("/api/files.completeUploadExternal"))
        .respond_with(json(r#"{"ok":true}"#.into()))
        .expect(0)
        .mount(&server)
        .await;
    let err = client
        .upload_files(
            vec![FileUpload::new("a.txt", "x")],
            UploadDestination::default(),
        )
        .await
        .unwrap_err();
    assert!(matches!(err, SlackError::Http { status: 413, ref body } if body == "too large"));
}

#[tokio::test]
async fn upload_url_errors_propagate() {
    let (server, client) = setup().await;
    Mock::given(path("/api/files.getUploadURLExternal"))
        .respond_with(json(r#"{"ok":false,"error":"not_authed"}"#.into()))
        .mount(&server)
        .await;
    let err = client
        .upload_files(
            vec![FileUpload::new("a.txt", "x")],
            UploadDestination::default(),
        )
        .await
        .unwrap_err();
    assert_eq!(err.api_error(), Some("not_authed"));
}

#[tokio::test]
async fn transport_failure_on_upload() {
    let (server, client) = setup().await;
    Mock::given(path("/api/files.getUploadURLExternal"))
        .respond_with(json(
            r#"{"ok":true,"upload_url":"not a url","file_id":"F1"}"#.into(),
        ))
        .mount(&server)
        .await;
    let err = client
        .upload_files(
            vec![FileUpload::new("a.txt", "x")],
            UploadDestination::default(),
        )
        .await
        .unwrap_err();
    assert!(matches!(err, SlackError::Transport(_)));
}

#[tokio::test]
async fn blocks_are_sent_with_the_completion() {
    use slack_web_api::blocks::{Block, DividerBlock};
    let (server, client) = setup().await;
    mount_upload_url(&server, "a.txt", "F1").await;
    Mock::given(path("/upload/F1"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;
    Mock::given(path("/api/files.completeUploadExternal"))
        .and(body_string_contains(
            "blocks=%5B%7B%22type%22%3A%22divider%22%7D%5D",
        ))
        .respond_with(json(r#"{"ok":true}"#.into()))
        .expect(1)
        .mount(&server)
        .await;
    client
        .upload_files(
            vec![FileUpload::new("a.txt", "x")],
            UploadDestination::channel("C1").blocks(vec![Block::from(DividerBlock::new())]),
        )
        .await
        .unwrap();
}

#[test]
fn builders_keep_values() {
    let a = FileUpload::new("a", "x").title("t");
    assert_eq!(a.clone(), a);
    assert!(format!("{a:?}").contains("title: Some(\"t\")"));
    let d = UploadDestination::channel("C1");
    assert_eq!(d.clone(), d);
    assert!(format!("{d:?}").contains("C1"));
}
