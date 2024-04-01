use slack_web_api::api::{SlackApiFilesUploadRequest};
use slack_web_api::{SlackClient};

#[tokio::test]
async fn test() {
    let token = std::env::var("SLACK_TOKEN_TEST").unwrap();
    let channel = "test-chanel";
    let client = SlackClient::new(token.as_str());

    let res = client.files_upload(&SlackApiFilesUploadRequest {
        channels: Some(channel.to_string()),
        file: Some("./test.txt".to_string()),
        ..SlackApiFilesUploadRequest::default()
    }).await;
    assert!(res.is_ok());
    // dbg!(res).expect(" panic message");
}
