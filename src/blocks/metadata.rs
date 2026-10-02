use serde::{Deserialize, Serialize};

/// Metadata attached to a message. <https://docs.slack.dev/messaging/message-metadata>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageMetadata {
    pub event_type: String,
    // conversations.history などは include_all_metadata を付けないと event_payload を返さない
    #[serde(default)]
    pub event_payload: serde_json::Value,
}

impl MessageMetadata {
    pub fn new(event_type: impl Into<String>, event_payload: serde_json::Value) -> Self {
        Self {
            event_type: event_type.into(),
            event_payload,
        }
    }
}
