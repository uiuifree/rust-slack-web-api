use crate::{SlackApiUserConversionResponse, SlackClient};
use serde_json::Value;

impl SlackClient {
    pub async fn user_conversations(&self) -> SlackApiUserConversionResponse {
        self.http_get(
            self.context.token.clone().unwrap().as_str(),
            "users.conversations",
            &Value::default(),
        )
        .await
        .unwrap()
    }
}
