//! Post a message to a real Slack channel.
//!
//! Run it with a bot token that has `chat:write` and a channel the bot has joined:
//! ```sh
//! SLACK_BOT_TOKEN=xoxb-... cargo run --example post_message -- C0123456789 "Hello from Rust"
//! ```
//! This and `tests/live.rs` are the only places in the repository that call Slack;
//! the test suite runs against a mock server.
use slack_web_api::api::ChatPostMessageRequest;
use slack_web_api::blocks::{Block, SectionBlock, TextObject};
use slack_web_api::SlackClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("SLACK_BOT_TOKEN").expect("set the SLACK_BOT_TOKEN env var");
    let mut args = std::env::args().skip(1);
    let channel = args
        .next()
        .expect("usage: post_message <channel id> [text]");
    let text = args
        .next()
        .unwrap_or_else(|| "Hello from slack-web-api".to_string());

    let client = SlackClient::new(token);
    let res = client
        .chat_post_message(
            &ChatPostMessageRequest::new(channel)
                .text(text.clone())
                .blocks(vec![Block::from(
                    SectionBlock::new().text(TextObject::mrkdwn(text)),
                )]),
        )
        .await?;
    println!("posted: channel={:?} ts={:?}", res.channel, res.ts);
    Ok(())
}
