A simple and ergonomic Rust client for the Slack Web API.
> Currently under development, priority API requests will be accepted.

## Features
* Asynchronous API requests using tokio.
* Lightweight with minimal dependencies.
* Supports Slack's chat and files API methods.
* Easy integration with existing Rust projects.

## Installation
To use this library, add the following to your Cargo.toml file:

```toml
[dependencies]
slack-web-api = "0.1"
```


## Usage
Here's a simple example that sends a message to a channel:
```rust
#[tokio::main]
async fn main() {
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
}
```


## Roadmap
|API|Support|
|---|---|
| admin | |  
| api | |  
| apps | |  
| auth | |  
| bookmark | |  
| bots | |  
| calls | |  
| chat |️✔️ |  
| conversations | |  
| dialog | |  
| dnd | |  
| emoji | |  
| files | ✔️|  
| functions | |  
| oauth | |  
| openid | |  
| pins | |  
| reactions | |  
| reminder | |  
| rtm | |  
| search | |  
| star | |  
| team | |  
| usergroup | |  
| users | |  
| view | |  
| workflow | |  
