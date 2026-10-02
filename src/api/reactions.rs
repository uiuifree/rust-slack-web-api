// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`reactions.add`](https://docs.slack.dev/reference/methods/reactions.add): Adds a reaction to an item.
///
/// Send it with [`SlackClient::reactions_add`].
#[doc(alias = "reactions.add")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ReactionsAddRequest {
    /// Channel where the message to add reaction to was posted.
    pub channel: String,
    /// Reaction (emoji) name
    pub name: String,
    /// Timestamp of the message to add reaction to.
    pub timestamp: String,
}

impl ReactionsAddRequest {
    pub fn new(
        channel: impl Into<String>,
        name: impl Into<String>,
        timestamp: impl Into<String>,
    ) -> Self {
        Self {
            channel: channel.into(),
            name: name.into(),
            timestamp: timestamp.into(),
        }
    }
}

impl SlackApiMethod for ReactionsAddRequest {
    const METHOD: &'static str = "reactions.add";
    type Response = ReactionsAddResponse;
}

/// Successful response of the Slack Web API method [`reactions.add`](https://docs.slack.dev/reference/methods/reactions.add).
#[doc(alias = "reactions.add")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ReactionsAddResponse {
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

/// Arguments for the Slack Web API method [`reactions.get`](https://docs.slack.dev/reference/methods/reactions.get): Gets reactions for an item.
///
/// Send it with [`SlackClient::reactions_get`].
#[doc(alias = "reactions.get")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct ReactionsGetRequest {
    /// Channel where the message to get reactions for was posted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// File to get reactions for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// File comment to get reactions for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_comment: Option<String>,
    /// If true always return the complete reaction list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full: Option<bool>,
    /// Timestamp of the message to get reactions for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl ReactionsGetRequest {
    pub fn new() -> Self {
        Self {
            channel: None,
            file: None,
            file_comment: None,
            full: None,
            timestamp: None,
        }
    }

    pub fn channel(mut self, channel: impl Into<String>) -> Self {
        self.channel = Some(channel.into());
        self
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn file_comment(mut self, file_comment: impl Into<String>) -> Self {
        self.file_comment = Some(file_comment.into());
        self
    }

    pub fn full(mut self, full: bool) -> Self {
        self.full = Some(full);
        self
    }

    pub fn timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
}

impl SlackApiMethod for ReactionsGetRequest {
    const METHOD: &'static str = "reactions.get";
    type Response = ReactionsGetResponse;
}

/// Successful response of the Slack Web API method [`reactions.get`](https://docs.slack.dev/reference/methods/reactions.get).
#[doc(alias = "reactions.get")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ReactionsGetResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub message: Option<Message>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<String>,
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

/// Arguments for the Slack Web API method [`reactions.list`](https://docs.slack.dev/reference/methods/reactions.list): Lists reactions made by a user.
///
/// Send it with [`SlackClient::reactions_list`].
#[doc(alias = "reactions.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct ReactionsListRequest {
    /// Show reactions made by this user. Defaults to the authed user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// If true always return the complete reaction list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Parameter for pagination. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first "page" of the collection. See [pagination](https://docs.slack.dev/apis/web-api/pagination.md) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// encoded team id to list reactions in, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl ReactionsListRequest {
    pub fn new() -> Self {
        Self {
            user: None,
            full: None,
            count: None,
            page: None,
            cursor: None,
            limit: None,
            team_id: None,
        }
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn full(mut self, full: bool) -> Self {
        self.full = Some(full);
        self
    }

    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }

    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
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

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for ReactionsListRequest {
    const METHOD: &'static str = "reactions.list";
    type Response = ReactionsListResponse;
}

impl CursorPaginated for ReactionsListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for ReactionsListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`reactions.list`](https://docs.slack.dev/reference/methods/reactions.list).
#[doc(alias = "reactions.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ReactionsListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ReactionsListResponseItems>,
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
pub struct ReactionsListResponseItems {
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
    pub channel: Option<String>,
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
    pub comment: Option<ReactionsListResponseItemsComment>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub file: Option<ReactionsListResponseItemsFile>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ReactionsListResponseItemsComment {
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
    pub comment: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub created: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reactions: Vec<Reaction>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub timestamp: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ReactionsListResponseItemsFile {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub channels: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub comments_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub created: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reactions: Vec<Reaction>,
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
    pub user: Option<String>,
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
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub size: Option<i64>,
}

/// Arguments for the Slack Web API method [`reactions.remove`](https://docs.slack.dev/reference/methods/reactions.remove): Removes a reaction from an item.
///
/// Send it with [`SlackClient::reactions_remove`].
#[doc(alias = "reactions.remove")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ReactionsRemoveRequest {
    /// Reaction (emoji) name.
    pub name: String,
    /// File to remove reaction from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// File comment to remove reaction from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_comment: Option<String>,
    /// Channel where the message to remove reaction from was posted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// Timestamp of the message to remove reaction from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl ReactionsRemoveRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            file: None,
            file_comment: None,
            channel: None,
            timestamp: None,
        }
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn file_comment(mut self, file_comment: impl Into<String>) -> Self {
        self.file_comment = Some(file_comment.into());
        self
    }

    pub fn channel(mut self, channel: impl Into<String>) -> Self {
        self.channel = Some(channel.into());
        self
    }

    pub fn timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
}

impl SlackApiMethod for ReactionsRemoveRequest {
    const METHOD: &'static str = "reactions.remove";
    type Response = ReactionsRemoveResponse;
}

/// Successful response of the Slack Web API method [`reactions.remove`](https://docs.slack.dev/reference/methods/reactions.remove).
#[doc(alias = "reactions.remove")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ReactionsRemoveResponse {
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
    /// Calls the Slack Web API method [`reactions.add`](https://docs.slack.dev/reference/methods/reactions.add): Adds a reaction to an item.
    ///
    /// Required scopes:
    ///
    /// - bot token: `reactions:write`
    /// - user token: `reactions:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "reactions.add")]
    pub async fn reactions_add(
        &self,
        request: &ReactionsAddRequest,
    ) -> Result<ReactionsAddResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`reactions.get`](https://docs.slack.dev/reference/methods/reactions.get): Gets reactions for an item.
    ///
    /// Required scopes:
    ///
    /// - bot token: `reactions:read`
    /// - user token: `reactions:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "reactions.get")]
    pub async fn reactions_get(
        &self,
        request: &ReactionsGetRequest,
    ) -> Result<ReactionsGetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`reactions.list`](https://docs.slack.dev/reference/methods/reactions.list): Lists reactions made by a user.
    ///
    /// Required scopes:
    ///
    /// - bot token: `reactions:read`
    /// - user token: `reactions:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "reactions.list")]
    pub async fn reactions_list(
        &self,
        request: &ReactionsListRequest,
    ) -> Result<ReactionsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`reactions.remove`](https://docs.slack.dev/reference/methods/reactions.remove): Removes a reaction from an item.
    ///
    /// Required scopes:
    ///
    /// - bot token: `reactions:write`
    /// - user token: `reactions:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "reactions.remove")]
    pub async fn reactions_remove(
        &self,
        request: &ReactionsRemoveRequest,
    ) -> Result<ReactionsRemoveResponse, SlackError> {
        self.call(request).await
    }
}
