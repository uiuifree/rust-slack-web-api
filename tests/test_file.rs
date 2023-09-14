use slack_web_api::api::{SlackApiChatDeleteRequest, SlackApiChatGetPermalinkRequest, SlackApiChatMeMessageRequest, SlackApiChatMeMessageResponse, SlackApiChatPostEphemeralRequest, SlackApiChatPostMessageRequest, SlackApiChatScheduledMessagesListRequest, SlackApiChatScheduleMessageRequest, SlackApiChatUnfurlRequest, SlackApiChatUpdateRequest, SlackApiFilesUploadRequest, SlackMessageAttachment};
use slack_web_api::{CompositionObjectText, SlackAttachmentBuilder, SlackBlockContext, SlackBlockFile, SlackBlockHeader, SlackClient, SlackMessageBuilder};
use std::process::exit;

#[tokio::test]
async fn test() {
    let token = std::env::var("SLACK_TOKEN_TEST").unwrap();
    let channel = "test-chanel";
    let client = SlackClient::new(token.as_str());

    let res = client.files_upload(&SlackApiFilesUploadRequest {
        channels: Some(channel.to_string()),
        file: Some("/home/uiuifree/CLionProjects/rust-slack-web-api/test.txt".to_string()),
        ..SlackApiFilesUploadRequest::default()
    }).await;
    assert!(res.is_ok());
    // dbg!(res).expect(" panic message");
}
