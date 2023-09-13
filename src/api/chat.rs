use crate::{
    SlackApiResponse, SlackBlock, SlackClient,
};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SlackMessageAttachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<SlackBlock>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl SlackClient {
    /// https://api.slack.com/methods/chat.delete
    pub async fn chat_delete(&self, request: &SlackApiChatDeleteRequest) -> SlackApiResponse<SlackApiChatDeleteResponse> {
        self.http_post("chat.delete", request).await
    }
    /// https://api.slack.com/methods/chat.deleteScheduledMessage
    pub async fn chat_delete_scheduled_message(&self, request: &SlackApiChatDeleteScheduleMessageRequest) -> SlackApiResponse<SlackApiChatDeleteScheduleMessageResponse> {
        self.http_post("chat.deleteScheduledMessage", request).await
    }
    /// https://api.slack.com/methods/chat.getPermalink
    pub async fn chat_get_permalink(&self, request: &SlackApiChatGetPermalinkRequest) -> SlackApiResponse<SlackApiChatGetPermalinkResponse> {
        self.http_post("chat.getPermalink", request).await
    }
    /// https://api.slack.com/methods/chat.meMessage
    pub async fn chat_me_message(&self, request: &SlackApiChatMeMessageRequest) -> SlackApiResponse<SlackApiChatMeMessageResponse> {
        self.http_post("chat.meMessage", request).await
    }
    /// https://api.slack.com/methods/chat.postEphemeral
    pub async fn chat_post_ephemeral(&self, request: &SlackApiChatPostEphemeralRequest) -> SlackApiResponse<SlackApiChatPostEphemeralResponse> {
        self.http_post("chat.postEphemeral", request).await
    }

    /// https://api.slack.com/methods/chat.postMessage
    pub async fn chat_post_message(
        &self,
        request: &SlackApiChatPostMessageRequest,
    ) -> SlackApiResponse<SlackApiChatPostMessageResponse> {
        self.http_post("chat.postMessage", request).await
    }
    /// https://api.slack.com/methods/chat.scheduleMessage
    pub async fn chat_schedule_message(
        &self,
        request: &SlackApiChatScheduleMessageRequest,
    ) -> SlackApiResponse<SlackApiChatScheduleMessageResponse> {
        self.http_post("chat.scheduleMessage", request).await
    }
    /// https://api.slack.com/methods/chat.unfurl
    pub async fn chat_unfurl(
        &self,
        request: &SlackApiChatUnfurlRequest,
    ) -> SlackApiResponse<SlackApiChatUnfurlResponse> {
        self.http_post("chat.unfurl", request).await
    }
    /// https://api.slack.com/methods/chat.update
    pub async fn chat_update(
        &self,
        request: &SlackApiChatUpdateRequest,
    ) -> SlackApiResponse<SlackApiChatUpdateResponse> {
        self.http_post("chat.update", request).await
    }
    /// https://api.slack.com/methods/chat.scheduledMessages.list
    pub async fn chat_schedule_message_list(
        &self,
        request: &SlackApiChatScheduledMessagesListRequest,
    ) -> SlackApiResponse<SlackApiChatScheduledMessagesListResponse> {
        self.http_post("chat.scheduledMessages.list", request).await
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatDeleteRequest {
    pub channel: String,
    pub ts: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<bool>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatDeleteResponse {
    pub ok: bool,
    pub channel: String,
    pub ts: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatDeleteScheduleMessageRequest {
    pub channel: String,
    pub scheduled_message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<bool>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatDeleteScheduleMessageResponse {
    pub ok: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatGetPermalinkRequest {
    pub channel: String,
    pub message_ts: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatGetPermalinkResponse {
    pub ok: bool,
    pub channel: String,
    pub permalink: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatMeMessageRequest {
    pub channel: String,
    pub text: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatMeMessageResponse {
    pub ok: bool,
    pub channel: String,
    pub ts: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatPostEphemeralRequest {
    pub channel: String,
    pub text: String,
    pub user: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<SlackMessageAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<SlackBlock>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_names: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatPostEphemeralResponse {
    pub ok: bool,
    pub message_ts: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatPostMessageRequest {
    pub channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<SlackMessageAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<SlackBlock>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_names: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mrkdwn: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_links: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatPostMessageResponse {
    pub ok: bool,
    pub channel: String,
    pub ts: String,
    pub message: Value,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatScheduleMessageRequest {
    pub channel: String,
    pub post_at: i32,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<SlackMessageAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<SlackBlock>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_names: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_links: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_media: Option<bool>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatScheduleMessageResponse {
    pub ok: bool,
    pub channel: String,
    pub scheduled_message_id: String,
    pub post_at: String,
    pub message: Value,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatUnfurlRequest {
    pub channel: String,
    pub ts: String,
    pub unfurls: String,
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_auth_blocks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_auth_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_auth_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_auth_url: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatUnfurlResponse {
    pub ok: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatUpdateRequest {
    pub channel: String,
    pub ts: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<Vec<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<SlackMessageAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<SlackBlock>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_names: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatUpdateResponse {
    pub ok: bool,
    pub channel: String,
    pub ts: String,
    pub text: String,
    pub message: Value,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatScheduledMessagesListRequest {
    pub channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,

}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SlackApiChatScheduledMessagesListResponse {
    pub ok: bool,
    pub scheduled_messages: Value,
    pub response_metadata: Value,
}
