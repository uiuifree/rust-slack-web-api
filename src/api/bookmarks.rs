// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`bookmarks.add`](https://docs.slack.dev/reference/methods/bookmarks.add): Add bookmark to a channel.
///
/// Send it with [`SlackClient::bookmarks_add`].
#[doc(alias = "bookmarks.add")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct BookmarksAddRequest {
    /// Title for the bookmark.
    pub title: String,
    /// Type of the bookmark i.e link.
    pub r#type: String,
    /// Channel to add bookmark in. Required for public channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// Link to bookmark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// Emoji tag to apply to the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// ID of the entity being bookmarked. Only applies to message and file types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// The level that we are setting the file's permission to (read or write)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<String>,
    /// Id of this bookmark's parent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

impl BookmarksAddRequest {
    pub fn new(title: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            r#type: r#type.into(),
            channel_id: None,
            link: None,
            emoji: None,
            entity_id: None,
            access_level: None,
            parent_id: None,
        }
    }

    pub fn channel_id(mut self, channel_id: impl Into<String>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    pub fn link(mut self, link: impl Into<String>) -> Self {
        self.link = Some(link.into());
        self
    }

    pub fn emoji(mut self, emoji: impl Into<String>) -> Self {
        self.emoji = Some(emoji.into());
        self
    }

    pub fn entity_id(mut self, entity_id: impl Into<String>) -> Self {
        self.entity_id = Some(entity_id.into());
        self
    }

    pub fn access_level(mut self, access_level: impl Into<String>) -> Self {
        self.access_level = Some(access_level.into());
        self
    }

    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }
}

impl SlackApiMethod for BookmarksAddRequest {
    const METHOD: &'static str = "bookmarks.add";
    type Response = BookmarksAddResponse;
}

/// Successful response of the Slack Web API method [`bookmarks.add`](https://docs.slack.dev/reference/methods/bookmarks.add).
#[doc(alias = "bookmarks.add")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BookmarksAddResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub bookmark: Option<Bookmark>,
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

/// Arguments for the Slack Web API method [`bookmarks.edit`](https://docs.slack.dev/reference/methods/bookmarks.edit): Edit bookmark.
///
/// Send it with [`SlackClient::bookmarks_edit`].
#[doc(alias = "bookmarks.edit")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct BookmarksEditRequest {
    /// Channel to update bookmark in. Required for public channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// Bookmark to update. Required for public channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookmark_id: Option<String>,
    /// Title for the bookmark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Link to bookmark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// Emoji tag to apply to the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
}

impl BookmarksEditRequest {
    pub fn new() -> Self {
        Self {
            channel_id: None,
            bookmark_id: None,
            title: None,
            link: None,
            emoji: None,
        }
    }

    pub fn channel_id(mut self, channel_id: impl Into<String>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    pub fn bookmark_id(mut self, bookmark_id: impl Into<String>) -> Self {
        self.bookmark_id = Some(bookmark_id.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn link(mut self, link: impl Into<String>) -> Self {
        self.link = Some(link.into());
        self
    }

    pub fn emoji(mut self, emoji: impl Into<String>) -> Self {
        self.emoji = Some(emoji.into());
        self
    }
}

impl SlackApiMethod for BookmarksEditRequest {
    const METHOD: &'static str = "bookmarks.edit";
    type Response = BookmarksEditResponse;
}

/// Successful response of the Slack Web API method [`bookmarks.edit`](https://docs.slack.dev/reference/methods/bookmarks.edit).
#[doc(alias = "bookmarks.edit")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BookmarksEditResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub bookmark: Option<Bookmark>,
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

/// Arguments for the Slack Web API method [`bookmarks.list`](https://docs.slack.dev/reference/methods/bookmarks.list): List bookmark for the channel.
///
/// Send it with [`SlackClient::bookmarks_list`].
#[doc(alias = "bookmarks.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct BookmarksListRequest {
    /// Channel to list bookmarks in. Required for public channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
}

impl BookmarksListRequest {
    pub fn new() -> Self {
        Self { channel_id: None }
    }

    pub fn channel_id(mut self, channel_id: impl Into<String>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }
}

impl SlackApiMethod for BookmarksListRequest {
    const METHOD: &'static str = "bookmarks.list";
    type Response = BookmarksListResponse;
}

/// Successful response of the Slack Web API method [`bookmarks.list`](https://docs.slack.dev/reference/methods/bookmarks.list).
#[doc(alias = "bookmarks.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BookmarksListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bookmarks: Vec<Bookmark>,
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

/// Arguments for the Slack Web API method [`bookmarks.remove`](https://docs.slack.dev/reference/methods/bookmarks.remove): Remove bookmark from the channel.
///
/// Send it with [`SlackClient::bookmarks_remove`].
#[doc(alias = "bookmarks.remove")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct BookmarksRemoveRequest {
    /// Channel to remove bookmark. Required for public channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// Bookmark to remove. Required for public channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookmark_id: Option<String>,
    /// Quip section ID to unbookmark
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quip_section_id: Option<String>,
}

impl BookmarksRemoveRequest {
    pub fn new() -> Self {
        Self {
            channel_id: None,
            bookmark_id: None,
            quip_section_id: None,
        }
    }

    pub fn channel_id(mut self, channel_id: impl Into<String>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    pub fn bookmark_id(mut self, bookmark_id: impl Into<String>) -> Self {
        self.bookmark_id = Some(bookmark_id.into());
        self
    }

    pub fn quip_section_id(mut self, quip_section_id: impl Into<String>) -> Self {
        self.quip_section_id = Some(quip_section_id.into());
        self
    }
}

impl SlackApiMethod for BookmarksRemoveRequest {
    const METHOD: &'static str = "bookmarks.remove";
    type Response = BookmarksRemoveResponse;
}

/// Successful response of the Slack Web API method [`bookmarks.remove`](https://docs.slack.dev/reference/methods/bookmarks.remove).
#[doc(alias = "bookmarks.remove")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BookmarksRemoveResponse {
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
    /// Calls the Slack Web API method [`bookmarks.add`](https://docs.slack.dev/reference/methods/bookmarks.add): Add bookmark to a channel.
    ///
    /// Required scopes:
    ///
    /// - bot token: `bookmarks:write`
    /// - user token: `bookmarks:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "bookmarks.add")]
    pub async fn bookmarks_add(
        &self,
        request: &BookmarksAddRequest,
    ) -> Result<BookmarksAddResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`bookmarks.edit`](https://docs.slack.dev/reference/methods/bookmarks.edit): Edit bookmark.
    ///
    /// Required scopes:
    ///
    /// - bot token: `bookmarks:write`
    /// - user token: `bookmarks:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "bookmarks.edit")]
    pub async fn bookmarks_edit(
        &self,
        request: &BookmarksEditRequest,
    ) -> Result<BookmarksEditResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`bookmarks.list`](https://docs.slack.dev/reference/methods/bookmarks.list): List bookmark for the channel.
    ///
    /// Required scopes:
    ///
    /// - bot token: `bookmarks:read`
    /// - user token: `bookmarks:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "bookmarks.list")]
    pub async fn bookmarks_list(
        &self,
        request: &BookmarksListRequest,
    ) -> Result<BookmarksListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`bookmarks.remove`](https://docs.slack.dev/reference/methods/bookmarks.remove): Remove bookmark from the channel.
    ///
    /// Required scopes:
    ///
    /// - bot token: `bookmarks:write`
    /// - user token: `bookmarks:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "bookmarks.remove")]
    pub async fn bookmarks_remove(
        &self,
        request: &BookmarksRemoveRequest,
    ) -> Result<BookmarksRemoveResponse, SlackError> {
        self.call(request).await
    }
}
