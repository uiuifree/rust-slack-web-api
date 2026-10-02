// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`assistant.search.context`](https://docs.slack.dev/reference/methods/assistant.search.context): Searches messages, files, channels and users across your Slack organization.
///
/// Send it with [`SlackClient::assistant_search_context`].
#[doc(alias = "assistant.search.context")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AssistantSearchContextRequest {
    /// User prompt or search query
    pub query: String,
    /// Send `action_token` as received in a message event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_token: Option<String>,
    /// Mix and match channel types by providing a comma-separated list of any combination of `public_channel`, `private_channel`, `mpim`, `im`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_types: Option<Vec<String>>,
    /// Content types to include, a comma-separated list of any combination of `messages`, `files`, `channels`, `users`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_types: Option<Vec<String>>,
    /// Whether the results should include bots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_bots: Option<bool>,
    /// Whether to include deleted users in the user search results. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deleted_users: Option<bool>,
    /// UNIX timestamp filter. If present, filters for results before this date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<i64>,
    /// UNIX timestamp filter. If present, filters for results after this date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<i64>,
    /// Whether to include context messages surrounding the main message result. Defaults to false if unspecified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_context_messages: Option<bool>,
    /// Context channel ID to support scoping the search when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_channel_id: Option<String>,
    /// The cursor returned by the API. Leave this blank for the first request and use this to get the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Number of results to return, up to a max of 20. Defaults to 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The field to sort the results by. Defaults to score. Can be one of: score, timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// The direction to sort the results by. Defaults to desc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_dir: Option<String>,
    /// Whether to return the message blocks in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_message_blocks: Option<bool>,
    /// Whether to highlight the search query in the results. Defaults to false if unspecified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<bool>,
    /// A list of term clauses. A term clause is a string with search terms. Search results returned will match every term clause specified (i.e., conjunctive normal form).
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub term_clauses: Option<Vec<serde_json::Value>>,
    /// A string containing only modifiers in the format of `modifier:value`. Search results returned will match the modifier value. For now modifiers only affect term clauses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<String>,
    /// Whether to include archived channels in the search results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_archived_channels: Option<bool>,
    /// Whether to disable semantic search. When true, only keyword-based search is used. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_semantic_search: Option<bool>,
}

impl AssistantSearchContextRequest {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            action_token: None,
            channel_types: None,
            content_types: None,
            include_bots: None,
            include_deleted_users: None,
            before: None,
            after: None,
            include_context_messages: None,
            context_channel_id: None,
            cursor: None,
            limit: None,
            sort: None,
            sort_dir: None,
            include_message_blocks: None,
            highlight: None,
            term_clauses: None,
            modifiers: None,
            include_archived_channels: None,
            disable_semantic_search: None,
        }
    }

    pub fn action_token(mut self, action_token: impl Into<String>) -> Self {
        self.action_token = Some(action_token.into());
        self
    }

    pub fn channel_types(mut self, channel_types: Vec<String>) -> Self {
        self.channel_types = Some(channel_types);
        self
    }

    pub fn content_types(mut self, content_types: Vec<String>) -> Self {
        self.content_types = Some(content_types);
        self
    }

    pub fn include_bots(mut self, include_bots: bool) -> Self {
        self.include_bots = Some(include_bots);
        self
    }

    pub fn include_deleted_users(mut self, include_deleted_users: bool) -> Self {
        self.include_deleted_users = Some(include_deleted_users);
        self
    }

    pub fn before(mut self, before: i64) -> Self {
        self.before = Some(before);
        self
    }

    pub fn after(mut self, after: i64) -> Self {
        self.after = Some(after);
        self
    }

    pub fn include_context_messages(mut self, include_context_messages: bool) -> Self {
        self.include_context_messages = Some(include_context_messages);
        self
    }

    pub fn context_channel_id(mut self, context_channel_id: impl Into<String>) -> Self {
        self.context_channel_id = Some(context_channel_id.into());
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    pub fn sort_dir(mut self, sort_dir: impl Into<String>) -> Self {
        self.sort_dir = Some(sort_dir.into());
        self
    }

    pub fn include_message_blocks(mut self, include_message_blocks: bool) -> Self {
        self.include_message_blocks = Some(include_message_blocks);
        self
    }

    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = Some(highlight);
        self
    }

    pub fn term_clauses(mut self, term_clauses: Vec<serde_json::Value>) -> Self {
        self.term_clauses = Some(term_clauses);
        self
    }

    pub fn modifiers(mut self, modifiers: impl Into<String>) -> Self {
        self.modifiers = Some(modifiers.into());
        self
    }

    pub fn include_archived_channels(mut self, include_archived_channels: bool) -> Self {
        self.include_archived_channels = Some(include_archived_channels);
        self
    }

    pub fn disable_semantic_search(mut self, disable_semantic_search: bool) -> Self {
        self.disable_semantic_search = Some(disable_semantic_search);
        self
    }
}

impl SlackApiMethod for AssistantSearchContextRequest {
    const METHOD: &'static str = "assistant.search.context";
    type Response = AssistantSearchContextResponse;
}

impl CursorPaginated for AssistantSearchContextRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AssistantSearchContextResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`assistant.search.context`](https://docs.slack.dev/reference/methods/assistant.search.context).
#[doc(alias = "assistant.search.context")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AssistantSearchContextResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub results: Option<AssistantSearchContextResponseResults>,
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
pub struct AssistantSearchContextResponseResults {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub messages: Vec<AssistantSearchContextResponseResultsMessages>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub files: Vec<AssistantSearchContextResponseResultsFiles>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub channels: Vec<AssistantSearchContextResponseResultsChannels>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AssistantSearchContextResponseResultsMessages {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub author_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub author_user_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub message_ts: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub content: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_author_bot: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub permalink: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocks: Vec<crate::blocks::Block>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub context_messages: Option<AssistantSearchContextResponseResultsMessagesContextMessages>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AssistantSearchContextResponseResultsMessagesContextMessages {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub before: Vec<AssistantSearchContextResponseResultsMessagesContextMessagesBefore>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub after: Vec<AssistantSearchContextResponseResultsMessagesContextMessagesAfter>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AssistantSearchContextResponseResultsMessagesContextMessagesBefore {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub text: Option<String>,
    #[serde(
        rename = "user_id:",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub ts: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocks: Vec<crate::blocks::Block>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AssistantSearchContextResponseResultsMessagesContextMessagesAfter {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub text: Option<String>,
    #[serde(
        rename = "user_id:",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub ts: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocks: Vec<crate::blocks::Block>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AssistantSearchContextResponseResultsFiles {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub uploader_user_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub author_user_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub author_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub file_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_updated: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub file_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub permalink: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AssistantSearchContextResponseResultsChannels {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub creator_user_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub creator_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_updated: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub topic: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub purpose: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub permalink: Option<String>,
}

/// Arguments for the Slack Web API method [`assistant.search.info`](https://docs.slack.dev/reference/methods/assistant.search.info): Returns search capabilities on a given team.
///
/// Send it with [`SlackClient::assistant_search_info`].
#[doc(alias = "assistant.search.info")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AssistantSearchInfoRequest {}

impl AssistantSearchInfoRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl SlackApiMethod for AssistantSearchInfoRequest {
    const METHOD: &'static str = "assistant.search.info";
    type Response = AssistantSearchInfoResponse;
}

/// Successful response of the Slack Web API method [`assistant.search.info`](https://docs.slack.dev/reference/methods/assistant.search.info).
#[doc(alias = "assistant.search.info")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AssistantSearchInfoResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_ai_search_enabled: Option<bool>,
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

/// Arguments for the Slack Web API method [`assistant.threads.setStatus`](https://docs.slack.dev/reference/methods/assistant.threads.setStatus): Set the status for an AI assistant thread.
///
/// Send it with [`SlackClient::assistant_threads_set_status`].
#[doc(alias = "assistant.threads.setStatus")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AssistantThreadsSetStatusRequest {
    /// Channel ID containing the assistant thread.
    pub channel_id: String,
    /// Message timestamp of the thread of where to set the status.
    pub thread_ts: String,
    /// Status of the specified bot user, e.g., 'is thinking...'. A two minute timeout applies, which will cause the status to be removed if no message has been sent.
    pub status: String,
    /// The list of messages to rotate through as a loading indicator. Maximum of 10 messages.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub loading_messages: Option<Vec<serde_json::Value>>,
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

impl AssistantThreadsSetStatusRequest {
    pub fn new(
        channel_id: impl Into<String>,
        thread_ts: impl Into<String>,
        status: impl Into<String>,
    ) -> Self {
        Self {
            channel_id: channel_id.into(),
            thread_ts: thread_ts.into(),
            status: status.into(),
            loading_messages: None,
            icon_emoji: None,
            icon_url: None,
            username: None,
        }
    }

    pub fn loading_messages(mut self, loading_messages: Vec<serde_json::Value>) -> Self {
        self.loading_messages = Some(loading_messages);
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

impl SlackApiMethod for AssistantThreadsSetStatusRequest {
    const METHOD: &'static str = "assistant.threads.setStatus";
    type Response = AssistantThreadsSetStatusResponse;
}

/// Successful response of the Slack Web API method [`assistant.threads.setStatus`](https://docs.slack.dev/reference/methods/assistant.threads.setStatus).
#[doc(alias = "assistant.threads.setStatus")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AssistantThreadsSetStatusResponse {
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

/// Arguments for the Slack Web API method [`assistant.threads.setSuggestedPrompts`](https://docs.slack.dev/reference/methods/assistant.threads.setSuggestedPrompts): Set suggested prompts for the given assistant thread
///
/// Send it with [`SlackClient::assistant_threads_set_suggested_prompts`].
#[doc(alias = "assistant.threads.setSuggestedPrompts")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AssistantThreadsSetSuggestedPromptsRequest {
    /// Channel ID containing the assistant thread.
    pub channel_id: String,
    /// Each prompt should be supplied with its `title` and `message` attribute.
    pub prompts: String,
    /// Message timestamp of the thread to set suggested prompts for. If not provided, the prompts will be set for the latest message in the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
    /// Title for the list of provided prompts. For example: Suggested Prompts, Related Questions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl AssistantThreadsSetSuggestedPromptsRequest {
    pub fn new(channel_id: impl Into<String>, prompts: impl Into<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
            prompts: prompts.into(),
            thread_ts: None,
            title: None,
        }
    }

    pub fn thread_ts(mut self, thread_ts: impl Into<String>) -> Self {
        self.thread_ts = Some(thread_ts.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl SlackApiMethod for AssistantThreadsSetSuggestedPromptsRequest {
    const METHOD: &'static str = "assistant.threads.setSuggestedPrompts";
    type Response = AssistantThreadsSetSuggestedPromptsResponse;
}

/// Successful response of the Slack Web API method [`assistant.threads.setSuggestedPrompts`](https://docs.slack.dev/reference/methods/assistant.threads.setSuggestedPrompts).
#[doc(alias = "assistant.threads.setSuggestedPrompts")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AssistantThreadsSetSuggestedPromptsResponse {
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

/// Arguments for the Slack Web API method [`assistant.threads.setTitle`](https://docs.slack.dev/reference/methods/assistant.threads.setTitle): Set the title for the given assistant thread
///
/// Send it with [`SlackClient::assistant_threads_set_title`].
#[doc(alias = "assistant.threads.setTitle")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AssistantThreadsSetTitleRequest {
    /// Channel ID containing the assistant thread.
    pub channel_id: String,
    /// Message timestamp of the thread to set suggested prompts for.
    pub thread_ts: String,
    /// The title to use for the thread.
    pub title: String,
}

impl AssistantThreadsSetTitleRequest {
    pub fn new(
        channel_id: impl Into<String>,
        thread_ts: impl Into<String>,
        title: impl Into<String>,
    ) -> Self {
        Self {
            channel_id: channel_id.into(),
            thread_ts: thread_ts.into(),
            title: title.into(),
        }
    }
}

impl SlackApiMethod for AssistantThreadsSetTitleRequest {
    const METHOD: &'static str = "assistant.threads.setTitle";
    type Response = AssistantThreadsSetTitleResponse;
}

/// Successful response of the Slack Web API method [`assistant.threads.setTitle`](https://docs.slack.dev/reference/methods/assistant.threads.setTitle).
#[doc(alias = "assistant.threads.setTitle")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AssistantThreadsSetTitleResponse {
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

impl SlackClient {
    /// Calls the Slack Web API method [`assistant.search.context`](https://docs.slack.dev/reference/methods/assistant.search.context): Searches messages, files, channels and users across your Slack organization.
    ///
    /// Required scopes:
    ///
    /// - bot token: `search:read.public`, `search:read.files`, `search:read.users`
    /// - user token: `search:read.public`, `search:read.private`, `search:read.im`, `search:read.mpim`, `search:read.files`, `search:read.users`
    ///
    /// Rate limit: Tier 5 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "assistant.search.context")]
    pub async fn assistant_search_context(
        &self,
        request: &AssistantSearchContextRequest,
    ) -> Result<AssistantSearchContextResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`assistant.search.info`](https://docs.slack.dev/reference/methods/assistant.search.info): Returns search capabilities on a given team.
    ///
    /// Required scopes:
    ///
    /// - bot token: `search:read.public`
    /// - user token: `search:read.public`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "assistant.search.info")]
    pub async fn assistant_search_info(
        &self,
        request: &AssistantSearchInfoRequest,
    ) -> Result<AssistantSearchInfoResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`assistant.threads.setStatus`](https://docs.slack.dev/reference/methods/assistant.threads.setStatus): Set the status for an AI assistant thread.
    ///
    /// Required scopes:
    ///
    /// - bot token: `chat:write`
    ///
    /// Rate limit: Tier 5 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "assistant.threads.setStatus")]
    pub async fn assistant_threads_set_status(
        &self,
        request: &AssistantThreadsSetStatusRequest,
    ) -> Result<AssistantThreadsSetStatusResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`assistant.threads.setSuggestedPrompts`](https://docs.slack.dev/reference/methods/assistant.threads.setSuggestedPrompts): Set suggested prompts for the given assistant thread
    ///
    /// Required scopes:
    ///
    /// - bot token: `assistant:write`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "assistant.threads.setSuggestedPrompts")]
    pub async fn assistant_threads_set_suggested_prompts(
        &self,
        request: &AssistantThreadsSetSuggestedPromptsRequest,
    ) -> Result<AssistantThreadsSetSuggestedPromptsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`assistant.threads.setTitle`](https://docs.slack.dev/reference/methods/assistant.threads.setTitle): Set the title for the given assistant thread
    ///
    /// Required scopes:
    ///
    /// - bot token: `assistant:write`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "assistant.threads.setTitle")]
    pub async fn assistant_threads_set_title(
        &self,
        request: &AssistantThreadsSetTitleRequest,
    ) -> Result<AssistantThreadsSetTitleResponse, SlackError> {
        self.call(request).await
    }
}
