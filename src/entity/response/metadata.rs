use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SlackApiResponseMetadata {
    pub next_cursor: Option<String>,
}
