use slack_web_api::api::{SlackApiChatDeleteRequest, SlackApiChatGetPermalinkRequest, SlackApiChatMeMessageRequest, SlackApiChatMeMessageResponse, SlackApiChatPostEphemeralRequest, SlackApiChatPostMessageRequest, SlackApiChatScheduledMessagesListRequest, SlackApiChatScheduleMessageRequest, SlackApiChatUnfurlRequest, SlackApiChatUpdateRequest, SlackMessageAttachment};
use slack_web_api::{CompositionObjectText, SlackAttachmentBuilder, SlackBlockContext, SlackBlockFile, SlackBlockHeader, SlackClient, SlackMessageBuilder};
use std::process::exit;

#[tokio::test]
async fn test() {
    let token = std::env::var("SLACK_TOKEN_TEST").unwrap();
    let channel = "test-chanel";
    let client = SlackClient::new(token.as_str());
    let ac = SlackAttachmentBuilder::new()
        .color("#ff0000")
        .block(SlackBlockHeader::new_text("Header Block"))
        .block(
            SlackBlockContext::new()
                .element(CompositionObjectText::new("Content1"))
                .element(CompositionObjectText::new("Content2"))
        )
        .block(
            SlackBlockContext::new()
                .element(CompositionObjectText::new("Content3"))
        );

    let res = client.chat_post_message(
        SlackMessageBuilder::new()
            .channel(channel)
            .icon_emoji(":test:")
            .username("test-user")
            .attachments(vec![ac])
    ).await;
    assert!(res.is_ok());
    let ts = res.unwrap().ts;
    let user = "U05SL9643G8";
    let channel = "C05RQL3CY6S";
    let res = client.chat_me_message(&SlackApiChatMeMessageRequest {
        channel: channel.to_string(), text: ts.to_string() }).await;
    assert!(res.is_ok());
    let res = client.chat_post_ephemeral(&SlackApiChatPostEphemeralRequest {
        channel: channel.to_string(), text: ts.to_string(),
        user: user.to_string(),
        ..SlackApiChatPostEphemeralRequest::default()
    }).await;
    assert!(res.is_ok());
    let res = client.chat_schedule_message(&SlackApiChatScheduleMessageRequest {
        channel: channel.to_string(),
        text: ts.to_string(),
        post_at: 1694682740,
        ..SlackApiChatScheduleMessageRequest::default()
    }).await;
    assert!(res.is_ok());
    let res = client.chat_update(&SlackApiChatUpdateRequest {
        channel: channel.to_string(),
        ts: ts.to_string(),
        text: Some("hoge!".to_string()),
        ..SlackApiChatUpdateRequest::default()
    }).await;
    assert!(res.is_ok());
    let res = client.chat_schedule_message_list(&SlackApiChatScheduledMessagesListRequest {
        channel: channel.to_string(),
        ..SlackApiChatScheduledMessagesListRequest::default()
    }).await;
    assert!(res.is_ok());
    let res = client.chat_delete(&SlackApiChatDeleteRequest {
        channel: channel.to_string(),
        ts,
        as_user: None,
    }).await;
    assert!(res.is_ok());
    // let res = client.files_upload().await;
    dbg!(res).expect("TODO: panic message");
}
