use std::process::exit;
use slack_web_api::api::{SlackApiChatPostMessageRequest, SlackMessageAttachment};
use slack_web_api::{CompositionObjectText, SlackBlockHeader, SlackClient};

#[tokio::test]
async fn test() {
    // let res = SlackClient::new("")
    //     .user_conversations()
    //     .await;
    // dbg!(res);

    let token = std::env::var("SLACK_TOKEN_TEST").unwrap();

    let res = SlackClient::new(token.as_str())
        .files_upload()
        .await;
    //   let res = SlackClient::new(token.as_str())
        // .chat_post_message(&SlackApiChatPostMessageRequest {
        //     channel: "test-chanel".to_string(),
        //     attachments: Some(vec![SlackMessageAttachment {
        //         blocks: Some(vec![SlackBlockHeader {
        //             text: CompositionObjectText {
        //                 text: "aa".to_string(),
        //                 ..CompositionObjectText::default()
        //             },
        //             ..SlackBlockHeader::default()
        //         }
        //         .into()]),
        //         color: Some("#36a64f".to_string()),
        //     }]),
        //
        //     ..SlackApiChatPostMessageRequest::default()
        // })
        // .await;
    dbg!(res).expect("TODO: panic message");
}
