// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`stars.add`](https://docs.slack.dev/reference/methods/stars.add): Save an item for later. Formerly known as adding a star.
///
/// Send it with [`SlackClient::stars_add`].
#[doc(alias = "stars.add")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct StarsAddRequest {
    /// Channel to add star to, or channel where the message to add star to was posted (used with `timestamp`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// File to add star to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// File comment to add star to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_comment: Option<String>,
    /// Timestamp of the message to add star to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl StarsAddRequest {
    pub fn new() -> Self {
        Self {
            channel: None,
            file: None,
            file_comment: None,
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

    pub fn timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
}

impl SlackApiMethod for StarsAddRequest {
    const METHOD: &'static str = "stars.add";
    type Response = StarsAddResponse;
}

/// Successful response of the Slack Web API method [`stars.add`](https://docs.slack.dev/reference/methods/stars.add).
#[doc(alias = "stars.add")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StarsAddResponse {
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

/// Arguments for the Slack Web API method [`stars.list`](https://docs.slack.dev/reference/methods/stars.list): Listed a user's saved items, formerly known as stars.
///
/// Send it with [`SlackClient::stars_list`].
#[doc(alias = "stars.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct StarsListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Parameter for pagination. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first "page" of the collection. See [pagination](https://docs.slack.dev/apis/web-api/pagination.md) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// encoded team id to list stars in, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl StarsListRequest {
    pub fn new() -> Self {
        Self {
            count: None,
            cursor: None,
            limit: None,
            page: None,
            team_id: None,
        }
    }

    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
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

    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for StarsListRequest {
    const METHOD: &'static str = "stars.list";
    type Response = StarsListResponse;
}

impl CursorPaginated for StarsListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for StarsListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`stars.list`](https://docs.slack.dev/reference/methods/stars.list).
#[doc(alias = "stars.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StarsListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<StarsListResponseItems>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub paging: Option<StarsListResponsePaging>,
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
pub struct StarsListResponseItems {
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
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_create: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StarsListResponsePaging {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub per_page: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub spill: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub page: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub total: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub pages: Option<i64>,
}

/// Arguments for the Slack Web API method [`stars.remove`](https://docs.slack.dev/reference/methods/stars.remove): Removes a saved item (star) from an item.
///
/// Send it with [`SlackClient::stars_remove`].
#[doc(alias = "stars.remove")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct StarsRemoveRequest {
    /// Channel to remove star from, or channel where the message to remove star from was posted (used with `timestamp`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// File to remove star from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// File comment to remove star from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_comment: Option<String>,
    /// Timestamp of the message to remove star from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl StarsRemoveRequest {
    pub fn new() -> Self {
        Self {
            channel: None,
            file: None,
            file_comment: None,
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

    pub fn timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
}

impl SlackApiMethod for StarsRemoveRequest {
    const METHOD: &'static str = "stars.remove";
    type Response = StarsRemoveResponse;
}

/// Successful response of the Slack Web API method [`stars.remove`](https://docs.slack.dev/reference/methods/stars.remove).
#[doc(alias = "stars.remove")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StarsRemoveResponse {
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
    /// Calls the Slack Web API method [`stars.add`](https://docs.slack.dev/reference/methods/stars.add): Save an item for later. Formerly known as adding a star.
    ///
    /// Required scopes:
    ///
    /// - user token: `stars:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "stars.add")]
    pub async fn stars_add(
        &self,
        request: &StarsAddRequest,
    ) -> Result<StarsAddResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`stars.list`](https://docs.slack.dev/reference/methods/stars.list): Listed a user's saved items, formerly known as stars.
    ///
    /// Required scopes:
    ///
    /// - user token: `stars:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "stars.list")]
    pub async fn stars_list(
        &self,
        request: &StarsListRequest,
    ) -> Result<StarsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`stars.remove`](https://docs.slack.dev/reference/methods/stars.remove): Removes a saved item (star) from an item.
    ///
    /// Required scopes:
    ///
    /// - user token: `stars:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "stars.remove")]
    pub async fn stars_remove(
        &self,
        request: &StarsRemoveRequest,
    ) -> Result<StarsRemoveResponse, SlackError> {
        self.call(request).await
    }
}
