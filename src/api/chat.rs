// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`chat.appendStream`](https://docs.slack.dev/reference/methods/chat.appendStream): Append text to an existing streaming conversation
///
/// Send it with [`SlackClient::chat_append_stream`].
#[doc(alias = "chat.appendStream")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ChatAppendStreamRequest {
    /// An encoded ID that represents a channel, private group, or DM
    pub channel: String,
    /// The timestamp of the streaming message.
    pub ts: String,
    /// Array of streaming chunks.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub chunks: Option<Vec<serde_json::Value>>,
    /// Accepts message text formatted in markdown. Limit this field to 12,000 characters. This text is what will be appended to the message received so far.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown_text: Option<String>,
}

impl ChatAppendStreamRequest {
    pub fn new(channel: impl Into<String>, ts: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            ts: ts.into(),
            chunks: None,
            markdown_text: None,
        }
    }

    pub fn chunks(mut self, chunks: Vec<serde_json::Value>) -> Self {
        self.chunks = Some(chunks);
        self
    }

    pub fn markdown_text(mut self, markdown_text: impl Into<String>) -> Self {
        self.markdown_text = Some(markdown_text.into());
        self
    }
}

impl SlackApiMethod for ChatAppendStreamRequest {
    const METHOD: &'static str = "chat.appendStream";
    type Response = ChatAppendStreamResponse;
}

/// Successful response of the Slack Web API method [`chat.appendStream`](https://docs.slack.dev/reference/methods/chat.appendStream).
#[doc(alias = "chat.appendStream")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatAppendStreamResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub ts: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

/// Arguments for the Slack Web API method [`chat.delete`](https://docs.slack.dev/reference/methods/chat.delete): Deletes a message.
///
/// Send it with [`SlackClient::chat_delete`].
#[doc(alias = "chat.delete")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ChatDeleteRequest {
    /// Channel containing the message to be deleted.
    pub channel: String,
    /// Timestamp of the message to be deleted.
    pub ts: String,
    /// (Legacy) Pass true to delete the message as the authed user with `chat:write:user` scope. Bot users in this context are considered authed users. See [legacy `as_user` parameter](#legacy_as_user) below.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<bool>,
}

impl ChatDeleteRequest {
    pub fn new(channel: impl Into<String>, ts: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            ts: ts.into(),
            as_user: None,
        }
    }

    pub fn as_user(mut self, as_user: bool) -> Self {
        self.as_user = Some(as_user);
        self
    }
}

impl SlackApiMethod for ChatDeleteRequest {
    const METHOD: &'static str = "chat.delete";
    type Response = ChatDeleteResponse;
}

/// Successful response of the Slack Web API method [`chat.delete`](https://docs.slack.dev/reference/methods/chat.delete).
#[doc(alias = "chat.delete")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatDeleteResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub ts: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

/// Arguments for the Slack Web API method [`chat.deleteScheduledMessage`](https://docs.slack.dev/reference/methods/chat.deleteScheduledMessage): Deletes a pending scheduled message from the queue.
///
/// Send it with [`SlackClient::chat_delete_scheduled_message`].
#[doc(alias = "chat.deleteScheduledMessage")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ChatDeleteScheduledMessageRequest {
    /// The channel the scheduled\_message is posting to
    pub channel: String,
    /// `scheduled_message_id` returned from call to chat.scheduleMessage
    pub scheduled_message_id: String,
    /// Pass true to delete the message as the authed user with `chat:write:user` scope. Bot users in this context are considered authed users. If unused or false, the message will be deleted with `chat:write:bot` scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<bool>,
}

impl ChatDeleteScheduledMessageRequest {
    pub fn new(channel: impl Into<String>, scheduled_message_id: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            scheduled_message_id: scheduled_message_id.into(),
            as_user: None,
        }
    }

    pub fn as_user(mut self, as_user: bool) -> Self {
        self.as_user = Some(as_user);
        self
    }
}

impl SlackApiMethod for ChatDeleteScheduledMessageRequest {
    const METHOD: &'static str = "chat.deleteScheduledMessage";
    type Response = ChatDeleteScheduledMessageResponse;
}

/// Successful response of the Slack Web API method [`chat.deleteScheduledMessage`](https://docs.slack.dev/reference/methods/chat.deleteScheduledMessage).
#[doc(alias = "chat.deleteScheduledMessage")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatDeleteScheduledMessageResponse {
    /// Fields returned by Slack. This method has no documented response example.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

/// Arguments for the Slack Web API method [`chat.getPermalink`](https://docs.slack.dev/reference/methods/chat.getPermalink): Retrieve a permalink URL for a specific extant message
///
/// Send it with [`SlackClient::chat_get_permalink`].
#[doc(alias = "chat.getPermalink")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ChatGetPermalinkRequest {
    /// The ID of the conversation or channel containing the message
    pub channel: String,
    /// A message's `ts` value, uniquely identifying it within a channel
    pub message_ts: String,
}

impl ChatGetPermalinkRequest {
    pub fn new(channel: impl Into<String>, message_ts: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            message_ts: message_ts.into(),
        }
    }
}

impl SlackApiMethod for ChatGetPermalinkRequest {
    const METHOD: &'static str = "chat.getPermalink";
    type Response = ChatGetPermalinkResponse;
}

/// Successful response of the Slack Web API method [`chat.getPermalink`](https://docs.slack.dev/reference/methods/chat.getPermalink).
#[doc(alias = "chat.getPermalink")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatGetPermalinkResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub permalink: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

/// Arguments for the Slack Web API method [`chat.meMessage`](https://docs.slack.dev/reference/methods/chat.meMessage): Share a me message into a channel.
///
/// Send it with [`SlackClient::chat_me_message`].
#[doc(alias = "chat.meMessage")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ChatMeMessageRequest {
    /// Channel to send message to. Can be a public channel, private group or IM channel. Can be an encoded ID, or a name.
    pub channel: String,
    /// Text of the message to send.
    pub text: String,
}

impl ChatMeMessageRequest {
    pub fn new(channel: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            text: text.into(),
        }
    }
}

impl SlackApiMethod for ChatMeMessageRequest {
    const METHOD: &'static str = "chat.meMessage";
    type Response = ChatMeMessageResponse;
}

/// Successful response of the Slack Web API method [`chat.meMessage`](https://docs.slack.dev/reference/methods/chat.meMessage).
#[doc(alias = "chat.meMessage")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatMeMessageResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub ts: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

/// Arguments for the Slack Web API method [`chat.postEphemeral`](https://docs.slack.dev/reference/methods/chat.postEphemeral): Sends an ephemeral message to a user in a channel.
///
/// Send it with [`SlackClient::chat_post_ephemeral`].
#[doc(alias = "chat.postEphemeral")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ChatPostEphemeralRequest {
    /// Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name.
    pub channel: String,
    /// `id` of the user who will receive the ephemeral message. The user should be in the channel specified by the `channel` argument.
    pub user: String,
    /// (Legacy) Pass true to post the message as the authed user. Defaults to true if the chat:write:bot scope is not included. Otherwise, defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<bool>,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub attachments: Option<Vec<crate::blocks::Attachment>>,
    /// A JSON-based array of structured blocks, presented as a URL-encoded string.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub blocks: Option<Vec<crate::blocks::Block>>,
    /// Emoji to use as the icon for this message. Overrides `icon_url`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_emoji: Option<String>,
    /// URL to an image to use as the icon for this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Find and link channel names and usernames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_names: Option<bool>,
    /// Accepts message text formatted in markdown. This argument should not be used in conjunction with `blocks` or `text`. Limit this field to 12,000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown_text: Option<String>,
    /// JSON object with an `entities` array of work object entity metadata, presented as a URL-encoded string. Only entity metadata is supported: ephemeral messages are not persisted and never dispatch `message_metadata_*` events, so `event_type`/`event_payload` message metadata is ignored here. Each entity requires `entity_type`, `entity_payload`, `external_ref`, and `url`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::blocks::MessageMetadata>,
    /// Change how messages are treated. Defaults to `none`. See [below](#formatting).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<String>,
    /// How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Provide another message's `ts` value to post this message in a thread. Avoid using a reply's `ts` value; use its parent's value instead. Ephemeral messages in threads are only shown if there is already an active thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
    /// Set your bot's user name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl ChatPostEphemeralRequest {
    pub fn new(channel: impl Into<String>, user: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            user: user.into(),
            as_user: None,
            attachments: None,
            blocks: None,
            icon_emoji: None,
            icon_url: None,
            link_names: None,
            markdown_text: None,
            metadata: None,
            parse: None,
            text: None,
            thread_ts: None,
            username: None,
        }
    }

    pub fn as_user(mut self, as_user: bool) -> Self {
        self.as_user = Some(as_user);
        self
    }

    pub fn attachments(mut self, attachments: Vec<crate::blocks::Attachment>) -> Self {
        self.attachments = Some(attachments);
        self
    }

    pub fn blocks(mut self, blocks: Vec<crate::blocks::Block>) -> Self {
        self.blocks = Some(blocks);
        self
    }

    pub fn icon_emoji(mut self, icon_emoji: impl Into<String>) -> Self {
        self.icon_emoji = Some(icon_emoji.into());
        self
    }

    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }

    pub fn link_names(mut self, link_names: bool) -> Self {
        self.link_names = Some(link_names);
        self
    }

    pub fn markdown_text(mut self, markdown_text: impl Into<String>) -> Self {
        self.markdown_text = Some(markdown_text.into());
        self
    }

    pub fn metadata(mut self, metadata: crate::blocks::MessageMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn parse(mut self, parse: impl Into<String>) -> Self {
        self.parse = Some(parse.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn thread_ts(mut self, thread_ts: impl Into<String>) -> Self {
        self.thread_ts = Some(thread_ts.into());
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }
}

impl SlackApiMethod for ChatPostEphemeralRequest {
    const METHOD: &'static str = "chat.postEphemeral";
    type Response = ChatPostEphemeralResponse;
}

/// Successful response of the Slack Web API method [`chat.postEphemeral`](https://docs.slack.dev/reference/methods/chat.postEphemeral).
#[doc(alias = "chat.postEphemeral")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatPostEphemeralResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub message_ts: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

/// Arguments for the Slack Web API method [`chat.postMessage`](https://docs.slack.dev/reference/methods/chat.postMessage): Sends a message to a channel.
///
/// Send it with [`SlackClient::chat_post_message`].
#[doc(alias = "chat.postMessage")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ChatPostMessageRequest {
    /// An encoded ID or channel name that represents a channel, private group, or IM channel to send the message to. See [below](#channels) for more details.
    pub channel: String,
    /// (Legacy) Pass true to post the message as the authed user instead of as a bot. Defaults to false. Can only be used by classic apps. See [legacy `as_user` parameter](#legacy_as_user) below.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<bool>,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub attachments: Option<Vec<crate::blocks::Attachment>>,
    /// A JSON-based array of structured blocks, presented as a URL-encoded string.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub blocks: Option<Vec<crate::blocks::Block>>,
    /// This field represents the timestamp of the draft's last update at the time this API is called. If the current message is a draft, this field can be provided to ensure synchronization with the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_draft_last_updated_ts: Option<String>,
    /// Emoji to use as the icon for this message. Overrides `icon_url`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_emoji: Option<String>,
    /// URL to an image to use as the icon for this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Find and link user groups. No longer supports linking individual users; use syntax shown in [Mentioning Users](https://docs.slack.dev/messaging/formatting-message-text.md#mentioning-users) instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_names: Option<bool>,
    /// Accepts message text formatted in markdown. This argument should not be used in conjunction with `blocks` or `text`. Limit this field to 12,000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown_text: Option<String>,
    /// JSON object with event\_type and event\_payload fields, presented as a URL-encoded string. You can also provide Work Object entity metadata using this parameter. Metadata you post to Slack is accessible to any app or user who is a member of that workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::blocks::MessageMetadata>,
    /// Disable Slack markup parsing by setting to `false`. Enabled by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mrkdwn: Option<bool>,
    /// Change how messages are treated. See [below](#formatting).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<String>,
    /// Used in conjunction with `thread_ts` and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_broadcast: Option<bool>,
    /// How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Provide another message's `ts` value to make this message a reply. Avoid using a reply's `ts` value; use its parent instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
    /// Pass true to enable unfurling of primarily text-based content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_links: Option<bool>,
    /// Pass false to disable unfurling of media content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_media: Option<bool>,
    /// Set your bot's user name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Pass true to unfurl links from installed apps, or false to prevent app links from unfurling. When omitted, app links follow the `unfurl_links` setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_app_links: Option<bool>,
}

impl ChatPostMessageRequest {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            as_user: None,
            attachments: None,
            blocks: None,
            current_draft_last_updated_ts: None,
            icon_emoji: None,
            icon_url: None,
            link_names: None,
            markdown_text: None,
            metadata: None,
            mrkdwn: None,
            parse: None,
            reply_broadcast: None,
            text: None,
            thread_ts: None,
            unfurl_links: None,
            unfurl_media: None,
            username: None,
            unfurl_app_links: None,
        }
    }

    pub fn as_user(mut self, as_user: bool) -> Self {
        self.as_user = Some(as_user);
        self
    }

    pub fn attachments(mut self, attachments: Vec<crate::blocks::Attachment>) -> Self {
        self.attachments = Some(attachments);
        self
    }

    pub fn blocks(mut self, blocks: Vec<crate::blocks::Block>) -> Self {
        self.blocks = Some(blocks);
        self
    }

    pub fn current_draft_last_updated_ts(
        mut self,
        current_draft_last_updated_ts: impl Into<String>,
    ) -> Self {
        self.current_draft_last_updated_ts = Some(current_draft_last_updated_ts.into());
        self
    }

    pub fn icon_emoji(mut self, icon_emoji: impl Into<String>) -> Self {
        self.icon_emoji = Some(icon_emoji.into());
        self
    }

    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }

    pub fn link_names(mut self, link_names: bool) -> Self {
        self.link_names = Some(link_names);
        self
    }

    pub fn markdown_text(mut self, markdown_text: impl Into<String>) -> Self {
        self.markdown_text = Some(markdown_text.into());
        self
    }

    pub fn metadata(mut self, metadata: crate::blocks::MessageMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn mrkdwn(mut self, mrkdwn: bool) -> Self {
        self.mrkdwn = Some(mrkdwn);
        self
    }

    pub fn parse(mut self, parse: impl Into<String>) -> Self {
        self.parse = Some(parse.into());
        self
    }

    pub fn reply_broadcast(mut self, reply_broadcast: bool) -> Self {
        self.reply_broadcast = Some(reply_broadcast);
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn thread_ts(mut self, thread_ts: impl Into<String>) -> Self {
        self.thread_ts = Some(thread_ts.into());
        self
    }

    pub fn unfurl_links(mut self, unfurl_links: bool) -> Self {
        self.unfurl_links = Some(unfurl_links);
        self
    }

    pub fn unfurl_media(mut self, unfurl_media: bool) -> Self {
        self.unfurl_media = Some(unfurl_media);
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn unfurl_app_links(mut self, unfurl_app_links: bool) -> Self {
        self.unfurl_app_links = Some(unfurl_app_links);
        self
    }
}

impl SlackApiMethod for ChatPostMessageRequest {
    const METHOD: &'static str = "chat.postMessage";
    type Response = ChatPostMessageResponse;
}

/// Successful response of the Slack Web API method [`chat.postMessage`](https://docs.slack.dev/reference/methods/chat.postMessage).
#[doc(alias = "chat.postMessage")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatPostMessageResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub ts: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub message: Option<Message>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

/// Arguments for the Slack Web API method [`chat.scheduleMessage`](https://docs.slack.dev/reference/methods/chat.scheduleMessage): Schedules a message to be sent to a channel.
///
/// Send it with [`SlackClient::chat_schedule_message`].
#[doc(alias = "chat.scheduleMessage")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ChatScheduleMessageRequest {
    /// Channel, private group, or DM channel to send message to. Can be an encoded ID, or a name. See [below](#channels) for more details.
    pub channel: String,
    /// Unix timestamp representing the future time the message should post to Slack.
    pub post_at: i64,
    /// Set to `true` to post the message as the authed user, instead of as a bot. Defaults to false. Cannot be used by [new Slack apps](https://docs.slack.dev/quickstart.md). See [chat.postMessage](chat.postMessage#authorship).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<bool>,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub attachments: Option<Vec<crate::blocks::Attachment>>,
    /// A JSON-based array of structured blocks, presented as a URL-encoded string.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub blocks: Option<Vec<crate::blocks::Block>>,
    /// Find and link user groups. No longer supports linking individual users; use syntax shown in [Mentioning Users](https://docs.slack.dev/messaging/formatting-message-text.md#mentioning-users) instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_names: Option<bool>,
    /// Accepts message text formatted in markdown. This argument should not be used in conjunction with `blocks` or `text`. Limit this field to 12,000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown_text: Option<String>,
    /// Change how messages are treated. See [chat.postMessage](chat.postMessage#formatting).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<String>,
    /// Used in conjunction with `thread_ts` and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_broadcast: Option<bool>,
    /// How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Provide another message's `ts` value to make this message a reply. Avoid using a reply's `ts` value; use its parent instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
    /// Pass true to enable unfurling of primarily text-based content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_links: Option<bool>,
    /// Pass false to disable unfurling of media content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_media: Option<bool>,
    /// JSON object with event\_type and event\_payload fields, presented as a URL-encoded string. Metadata you post to Slack is accessible to any app or user who is a member of that workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::blocks::MessageMetadata>,
}

impl ChatScheduleMessageRequest {
    pub fn new(channel: impl Into<String>, post_at: i64) -> Self {
        Self {
            channel: channel.into(),
            post_at,
            as_user: None,
            attachments: None,
            blocks: None,
            link_names: None,
            markdown_text: None,
            parse: None,
            reply_broadcast: None,
            text: None,
            thread_ts: None,
            unfurl_links: None,
            unfurl_media: None,
            metadata: None,
        }
    }

    pub fn as_user(mut self, as_user: bool) -> Self {
        self.as_user = Some(as_user);
        self
    }

    pub fn attachments(mut self, attachments: Vec<crate::blocks::Attachment>) -> Self {
        self.attachments = Some(attachments);
        self
    }

    pub fn blocks(mut self, blocks: Vec<crate::blocks::Block>) -> Self {
        self.blocks = Some(blocks);
        self
    }

    pub fn link_names(mut self, link_names: bool) -> Self {
        self.link_names = Some(link_names);
        self
    }

    pub fn markdown_text(mut self, markdown_text: impl Into<String>) -> Self {
        self.markdown_text = Some(markdown_text.into());
        self
    }

    pub fn parse(mut self, parse: impl Into<String>) -> Self {
        self.parse = Some(parse.into());
        self
    }

    pub fn reply_broadcast(mut self, reply_broadcast: bool) -> Self {
        self.reply_broadcast = Some(reply_broadcast);
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn thread_ts(mut self, thread_ts: impl Into<String>) -> Self {
        self.thread_ts = Some(thread_ts.into());
        self
    }

    pub fn unfurl_links(mut self, unfurl_links: bool) -> Self {
        self.unfurl_links = Some(unfurl_links);
        self
    }

    pub fn unfurl_media(mut self, unfurl_media: bool) -> Self {
        self.unfurl_media = Some(unfurl_media);
        self
    }

    pub fn metadata(mut self, metadata: crate::blocks::MessageMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl SlackApiMethod for ChatScheduleMessageRequest {
    const METHOD: &'static str = "chat.scheduleMessage";
    type Response = ChatScheduleMessageResponse;
}

/// Successful response of the Slack Web API method [`chat.scheduleMessage`](https://docs.slack.dev/reference/methods/chat.scheduleMessage).
#[doc(alias = "chat.scheduleMessage")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatScheduleMessageResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_message_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub post_at: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub message: Option<ChatScheduleMessageResponseMessage>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatScheduleMessageResponseMessage {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub text: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub username: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub bot_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attachments: Vec<MessageAttachment>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub subtype: Option<String>,
}

/// Arguments for the Slack Web API method [`chat.scheduledMessages.list`](https://docs.slack.dev/reference/methods/chat.scheduledMessages.list): Returns a list of scheduled messages.
///
/// Send it with [`SlackClient::chat_scheduled_messages_list`].
#[doc(alias = "chat.scheduledMessages.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct ChatScheduledMessagesListRequest {
    /// The channel of the scheduled messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// For pagination purposes, this is the `cursor` value returned from a previous call to `chat.scheduledmessages.list` indicating where you want to start this call from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// A Unix timestamp of the latest value in the time range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<String>,
    /// Maximum number of original entries to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A Unix timestamp of the oldest value in the time range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest: Option<String>,
    /// encoded team id to list channels in, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl ChatScheduledMessagesListRequest {
    pub fn new() -> Self {
        Self {
            channel: None,
            cursor: None,
            latest: None,
            limit: None,
            oldest: None,
            team_id: None,
        }
    }

    pub fn channel(mut self, channel: impl Into<String>) -> Self {
        self.channel = Some(channel.into());
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn latest(mut self, latest: impl Into<String>) -> Self {
        self.latest = Some(latest.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn oldest(mut self, oldest: impl Into<String>) -> Self {
        self.oldest = Some(oldest.into());
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for ChatScheduledMessagesListRequest {
    const METHOD: &'static str = "chat.scheduledMessages.list";
    type Response = ChatScheduledMessagesListResponse;
}

impl CursorPaginated for ChatScheduledMessagesListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for ChatScheduledMessagesListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`chat.scheduledMessages.list`](https://docs.slack.dev/reference/methods/chat.scheduledMessages.list).
#[doc(alias = "chat.scheduledMessages.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatScheduledMessagesListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scheduled_messages: Vec<ChatScheduledMessagesListResponseScheduledMessages>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatScheduledMessagesListResponseScheduledMessages {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub post_at: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub text: Option<String>,
}

/// Arguments for the Slack Web API method [`chat.startStream`](https://docs.slack.dev/reference/methods/chat.startStream): Start a new streaming conversation
///
/// Send it with [`SlackClient::chat_start_stream`].
#[doc(alias = "chat.startStream")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ChatStartStreamRequest {
    /// An encoded ID that represents a channel, thread, or DM.
    pub channel: String,
    /// Array of streaming chunks. Can include [markdown text chunk](#markdown_text-chunks) objects, [task update chunk](#task_update-chunks) objects, [plan update chunks](#plan_update-chunks), or [blocks chunks](#blocks-chunks).
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub chunks: Option<Vec<serde_json::Value>>,
    /// Accepts message text formatted in markdown. Limit this field to 12,000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown_text: Option<String>,
    /// Provide another message's `ts` value to reply to. Omit it to stream a top-level message instead of a thread reply; this is only supported in channels where the whole channel is one session, such as Slack Code, and returns `invalid_thread_ts` elsewhere. Passing `"0"` is equivalent to omitting it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
    /// The encoded ID of the user to receive the streaming text. Required when streaming to channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_user_id: Option<String>,
    /// The encoded ID of the team the user receiving the streaming text belongs to. Required when streaming to channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_team_id: Option<String>,
    /// Specifies how tasks are displayed in the message. `timeline` task updates render as individual task cards interleaved with streamed text. `plan` task updates render together in a plan block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_display_mode: Option<String>,
    /// Emoji to use as the icon for this message. Overrides `icon_url`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_emoji: Option<String>,
    /// Image URL to use as the icon for this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// The bot's username to display.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl ChatStartStreamRequest {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            chunks: None,
            markdown_text: None,
            thread_ts: None,
            recipient_user_id: None,
            recipient_team_id: None,
            task_display_mode: None,
            icon_emoji: None,
            icon_url: None,
            username: None,
        }
    }

    pub fn chunks(mut self, chunks: Vec<serde_json::Value>) -> Self {
        self.chunks = Some(chunks);
        self
    }

    pub fn markdown_text(mut self, markdown_text: impl Into<String>) -> Self {
        self.markdown_text = Some(markdown_text.into());
        self
    }

    pub fn thread_ts(mut self, thread_ts: impl Into<String>) -> Self {
        self.thread_ts = Some(thread_ts.into());
        self
    }

    pub fn recipient_user_id(mut self, recipient_user_id: impl Into<String>) -> Self {
        self.recipient_user_id = Some(recipient_user_id.into());
        self
    }

    pub fn recipient_team_id(mut self, recipient_team_id: impl Into<String>) -> Self {
        self.recipient_team_id = Some(recipient_team_id.into());
        self
    }

    pub fn task_display_mode(mut self, task_display_mode: impl Into<String>) -> Self {
        self.task_display_mode = Some(task_display_mode.into());
        self
    }

    pub fn icon_emoji(mut self, icon_emoji: impl Into<String>) -> Self {
        self.icon_emoji = Some(icon_emoji.into());
        self
    }

    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }
}

impl SlackApiMethod for ChatStartStreamRequest {
    const METHOD: &'static str = "chat.startStream";
    type Response = ChatStartStreamResponse;
}

/// Successful response of the Slack Web API method [`chat.startStream`](https://docs.slack.dev/reference/methods/chat.startStream).
#[doc(alias = "chat.startStream")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatStartStreamResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub ts: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

/// Arguments for the Slack Web API method [`chat.stopStream`](https://docs.slack.dev/reference/methods/chat.stopStream): Stop a streaming conversation
///
/// Send it with [`SlackClient::chat_stop_stream`].
#[doc(alias = "chat.stopStream")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ChatStopStreamRequest {
    /// An encoded ID that represents a channel, private group, or DM
    pub channel: String,
    /// The timestamp of the streaming message.
    pub ts: String,
    /// Array of streaming chunks. Can include [markdown text chunk](#markdown_text-chunks) objects, [task update chunk](#task_update-chunks) objects, [plan update chunks](#plan_update-chunks), or [blocks chunks](#blocks-chunks).
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub chunks: Option<Vec<serde_json::Value>>,
    /// Accepts message text formatted in markdown. Limit this field to 12,000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown_text: Option<String>,
    /// A list of blocks that will be rendered at the bottom of the finalized message.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub blocks: Option<Vec<crate::blocks::Block>>,
    /// JSON object with event\_type and event\_payload fields, presented as a URL-encoded string. Metadata you post to Slack is accessible to any app or user who is a member of that workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::blocks::MessageMetadata>,
    /// The session status to set after stopping the stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_status: Option<String>,
}

impl ChatStopStreamRequest {
    pub fn new(channel: impl Into<String>, ts: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            ts: ts.into(),
            chunks: None,
            markdown_text: None,
            blocks: None,
            metadata: None,
            session_status: None,
        }
    }

    pub fn chunks(mut self, chunks: Vec<serde_json::Value>) -> Self {
        self.chunks = Some(chunks);
        self
    }

    pub fn markdown_text(mut self, markdown_text: impl Into<String>) -> Self {
        self.markdown_text = Some(markdown_text.into());
        self
    }

    pub fn blocks(mut self, blocks: Vec<crate::blocks::Block>) -> Self {
        self.blocks = Some(blocks);
        self
    }

    pub fn metadata(mut self, metadata: crate::blocks::MessageMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn session_status(mut self, session_status: impl Into<String>) -> Self {
        self.session_status = Some(session_status.into());
        self
    }
}

impl SlackApiMethod for ChatStopStreamRequest {
    const METHOD: &'static str = "chat.stopStream";
    type Response = ChatStopStreamResponse;
}

/// Successful response of the Slack Web API method [`chat.stopStream`](https://docs.slack.dev/reference/methods/chat.stopStream).
#[doc(alias = "chat.stopStream")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatStopStreamResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub ts: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub message: Option<Message>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

/// Arguments for the Slack Web API method [`chat.unfurl`](https://docs.slack.dev/reference/methods/chat.unfurl): Provide custom unfurl behavior for user-posted URLs
///
/// Send it with [`SlackClient::chat_unfurl`].
#[doc(alias = "chat.unfurl")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct ChatUnfurlRequest {
    /// Channel ID of the message. Both `channel` and `ts` must be provided together, _or_ `unfurl_id` and `source` must be provided together. Required for public channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// Timestamp of the message to add unfurl behavior to. Required for public channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    /// URL-encoded JSON map with keys set to URLs featured in the message, pointing to their unfurl blocks or message attachments. Required for public channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurls: Option<String>,
    /// Provide a simply-formatted string to send as an ephemeral message to the user as invitation to authenticate further and enable full unfurling behavior. Provides two buttons, `Not now` or `Never ask me again`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_auth_message: Option<String>,
    /// Set to `true` or `1` to indicate the user must install your Slack app to trigger unfurls for this domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_auth_required: Option<bool>,
    /// Send users to this custom URL where they will complete authentication in your app to fully trigger unfurling. Value should be properly URL-encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_auth_url: Option<String>,
    /// Provide a JSON based array of structured blocks presented as URL-encoded string to send as an ephemeral message to the user as invitation to authenticate further and enable full unfurling behavior
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub user_auth_blocks: Option<Vec<crate::blocks::Block>>,
    /// The ID of the link to unfurl. Both `unfurl_id` and `source` must be provided together, _or_ `channel` and `ts` must be provided together.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_id: Option<String>,
    /// The source of the link to unfurl. The source may either be `composer`, when the link is inside the message composer, or `conversations_history`, when the link has been posted to a conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl ChatUnfurlRequest {
    pub fn new() -> Self {
        Self {
            channel: None,
            ts: None,
            unfurls: None,
            user_auth_message: None,
            user_auth_required: None,
            user_auth_url: None,
            user_auth_blocks: None,
            unfurl_id: None,
            source: None,
        }
    }

    pub fn channel(mut self, channel: impl Into<String>) -> Self {
        self.channel = Some(channel.into());
        self
    }

    pub fn ts(mut self, ts: impl Into<String>) -> Self {
        self.ts = Some(ts.into());
        self
    }

    pub fn unfurls(mut self, unfurls: impl Into<String>) -> Self {
        self.unfurls = Some(unfurls.into());
        self
    }

    pub fn user_auth_message(mut self, user_auth_message: impl Into<String>) -> Self {
        self.user_auth_message = Some(user_auth_message.into());
        self
    }

    pub fn user_auth_required(mut self, user_auth_required: bool) -> Self {
        self.user_auth_required = Some(user_auth_required);
        self
    }

    pub fn user_auth_url(mut self, user_auth_url: impl Into<String>) -> Self {
        self.user_auth_url = Some(user_auth_url.into());
        self
    }

    pub fn user_auth_blocks(mut self, user_auth_blocks: Vec<crate::blocks::Block>) -> Self {
        self.user_auth_blocks = Some(user_auth_blocks);
        self
    }

    pub fn unfurl_id(mut self, unfurl_id: impl Into<String>) -> Self {
        self.unfurl_id = Some(unfurl_id.into());
        self
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }
}

impl SlackApiMethod for ChatUnfurlRequest {
    const METHOD: &'static str = "chat.unfurl";
    type Response = ChatUnfurlResponse;
}

/// Successful response of the Slack Web API method [`chat.unfurl`](https://docs.slack.dev/reference/methods/chat.unfurl).
#[doc(alias = "chat.unfurl")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatUnfurlResponse {
    /// Fields returned by Slack. This method has no documented response example.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

/// Arguments for the Slack Web API method [`chat.update`](https://docs.slack.dev/reference/methods/chat.update): Updates a message.
///
/// Send it with [`SlackClient::chat_update`].
#[doc(alias = "chat.update")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ChatUpdateRequest {
    /// Channel containing the message to be updated. For direct messages, ensure that this value is a DM ID (starts with `D`) instead of a User ID (starts with either `U` or `W`).
    pub channel: String,
    /// Timestamp of the message to be updated.
    pub ts: String,
    /// Pass true to update the message as the authed user. Bot users in this context are considered authed users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<bool>,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub attachments: Option<Vec<crate::blocks::Attachment>>,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurled_attachments: Option<String>,
    /// A JSON-based array of structured blocks, presented as a URL-encoded string.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub blocks: Option<Vec<crate::blocks::Block>>,
    /// Accepts message text formatted in markdown. This argument should not be used in conjunction with `blocks` or `text`. Limit this field to 12,000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown_text: Option<String>,
    /// JSON object with event\_type and event\_payload fields, presented as a URL-encoded string. If you don't include this field, the message's previous `metadata` will be retained. To remove previous `metadata`, include an empty object for this field. Metadata you post to Slack is accessible to any app or user who is a member of that workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::blocks::MessageMetadata>,
    /// Find and link channel names and usernames. Defaults to `none`. If you do not specify a value for this field, the original value set for the message will be overwritten with the default, `none`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_names: Option<bool>,
    /// Change how messages are treated. Defaults to `client`, unlike `chat.postMessage`. Accepts either `none` or `full`. If you do not specify a value for this field, the original value set for the message will be overwritten with the default, `client`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<String>,
    /// How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Broadcast an existing thread reply to make it visible to everyone in the channel or conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_broadcast: Option<bool>,
    /// Array of new file ids that will be sent with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
}

impl ChatUpdateRequest {
    pub fn new(channel: impl Into<String>, ts: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            ts: ts.into(),
            as_user: None,
            attachments: None,
            unfurled_attachments: None,
            blocks: None,
            markdown_text: None,
            metadata: None,
            link_names: None,
            parse: None,
            text: None,
            reply_broadcast: None,
            file_ids: None,
        }
    }

    pub fn as_user(mut self, as_user: bool) -> Self {
        self.as_user = Some(as_user);
        self
    }

    pub fn attachments(mut self, attachments: Vec<crate::blocks::Attachment>) -> Self {
        self.attachments = Some(attachments);
        self
    }

    pub fn unfurled_attachments(mut self, unfurled_attachments: impl Into<String>) -> Self {
        self.unfurled_attachments = Some(unfurled_attachments.into());
        self
    }

    pub fn blocks(mut self, blocks: Vec<crate::blocks::Block>) -> Self {
        self.blocks = Some(blocks);
        self
    }

    pub fn markdown_text(mut self, markdown_text: impl Into<String>) -> Self {
        self.markdown_text = Some(markdown_text.into());
        self
    }

    pub fn metadata(mut self, metadata: crate::blocks::MessageMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn link_names(mut self, link_names: bool) -> Self {
        self.link_names = Some(link_names);
        self
    }

    pub fn parse(mut self, parse: impl Into<String>) -> Self {
        self.parse = Some(parse.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn reply_broadcast(mut self, reply_broadcast: bool) -> Self {
        self.reply_broadcast = Some(reply_broadcast);
        self
    }

    pub fn file_ids(mut self, file_ids: Vec<String>) -> Self {
        self.file_ids = Some(file_ids);
        self
    }
}

impl SlackApiMethod for ChatUpdateRequest {
    const METHOD: &'static str = "chat.update";
    type Response = ChatUpdateResponse;
}

/// Successful response of the Slack Web API method [`chat.update`](https://docs.slack.dev/reference/methods/chat.update).
#[doc(alias = "chat.update")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatUpdateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub ts: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub text: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub message: Option<ChatUpdateResponseMessage>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatUpdateResponseMessage {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub text: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<String>,
}

impl SlackClient {
    /// Calls the Slack Web API method [`chat.appendStream`](https://docs.slack.dev/reference/methods/chat.appendStream): Append text to an existing streaming conversation
    ///
    /// Required scopes:
    ///
    /// - bot token: `chat:write`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "chat.appendStream")]
    pub async fn chat_append_stream(
        &self,
        request: &ChatAppendStreamRequest,
    ) -> Result<ChatAppendStreamResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`chat.delete`](https://docs.slack.dev/reference/methods/chat.delete): Deletes a message.
    ///
    /// Required scopes:
    ///
    /// - bot token: `chat:write`
    /// - user token: `chat:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "chat.delete")]
    pub async fn chat_delete(
        &self,
        request: &ChatDeleteRequest,
    ) -> Result<ChatDeleteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`chat.deleteScheduledMessage`](https://docs.slack.dev/reference/methods/chat.deleteScheduledMessage): Deletes a pending scheduled message from the queue.
    ///
    /// Required scopes:
    ///
    /// - bot token: `chat:write`
    /// - user token: `chat:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "chat.deleteScheduledMessage")]
    pub async fn chat_delete_scheduled_message(
        &self,
        request: &ChatDeleteScheduledMessageRequest,
    ) -> Result<ChatDeleteScheduledMessageResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`chat.getPermalink`](https://docs.slack.dev/reference/methods/chat.getPermalink): Retrieve a permalink URL for a specific extant message
    ///
    /// Rate limit: Tier 5 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "chat.getPermalink")]
    pub async fn chat_get_permalink(
        &self,
        request: &ChatGetPermalinkRequest,
    ) -> Result<ChatGetPermalinkResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`chat.meMessage`](https://docs.slack.dev/reference/methods/chat.meMessage): Share a me message into a channel.
    ///
    /// Required scopes:
    ///
    /// - bot token: `chat:write`
    /// - user token: `chat:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "chat.meMessage")]
    pub async fn chat_me_message(
        &self,
        request: &ChatMeMessageRequest,
    ) -> Result<ChatMeMessageResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`chat.postEphemeral`](https://docs.slack.dev/reference/methods/chat.postEphemeral): Sends an ephemeral message to a user in a channel.
    ///
    /// Required scopes:
    ///
    /// - bot token: `chat:write`
    /// - user token: `chat:write`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "chat.postEphemeral")]
    pub async fn chat_post_ephemeral(
        &self,
        request: &ChatPostEphemeralRequest,
    ) -> Result<ChatPostEphemeralResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`chat.postMessage`](https://docs.slack.dev/reference/methods/chat.postMessage): Sends a message to a channel.
    ///
    /// Required scopes:
    ///
    /// - bot token: `chat:write`
    /// - user token: `chat:write`
    ///
    /// Rate limit: Tier 5 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "chat.postMessage")]
    pub async fn chat_post_message(
        &self,
        request: &ChatPostMessageRequest,
    ) -> Result<ChatPostMessageResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`chat.scheduleMessage`](https://docs.slack.dev/reference/methods/chat.scheduleMessage): Schedules a message to be sent to a channel.
    ///
    /// Required scopes:
    ///
    /// - bot token: `chat:write`
    /// - user token: `chat:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "chat.scheduleMessage")]
    pub async fn chat_schedule_message(
        &self,
        request: &ChatScheduleMessageRequest,
    ) -> Result<ChatScheduleMessageResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`chat.scheduledMessages.list`](https://docs.slack.dev/reference/methods/chat.scheduledMessages.list): Returns a list of scheduled messages.
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "chat.scheduledMessages.list")]
    pub async fn chat_scheduled_messages_list(
        &self,
        request: &ChatScheduledMessagesListRequest,
    ) -> Result<ChatScheduledMessagesListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`chat.startStream`](https://docs.slack.dev/reference/methods/chat.startStream): Start a new streaming conversation
    ///
    /// Required scopes:
    ///
    /// - bot token: `chat:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "chat.startStream")]
    pub async fn chat_start_stream(
        &self,
        request: &ChatStartStreamRequest,
    ) -> Result<ChatStartStreamResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`chat.stopStream`](https://docs.slack.dev/reference/methods/chat.stopStream): Stop a streaming conversation
    ///
    /// Required scopes:
    ///
    /// - bot token: `chat:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "chat.stopStream")]
    pub async fn chat_stop_stream(
        &self,
        request: &ChatStopStreamRequest,
    ) -> Result<ChatStopStreamResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`chat.unfurl`](https://docs.slack.dev/reference/methods/chat.unfurl): Provide custom unfurl behavior for user-posted URLs
    ///
    /// Required scopes:
    ///
    /// - bot token: `links:write`
    /// - user token: `links:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "chat.unfurl")]
    pub async fn chat_unfurl(
        &self,
        request: &ChatUnfurlRequest,
    ) -> Result<ChatUnfurlResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`chat.update`](https://docs.slack.dev/reference/methods/chat.update): Updates a message.
    ///
    /// Required scopes:
    ///
    /// - bot token: `chat:write`
    /// - user token: `chat:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "chat.update")]
    pub async fn chat_update(
        &self,
        request: &ChatUpdateRequest,
    ) -> Result<ChatUpdateResponse, SlackError> {
        self.call(request).await
    }
}
