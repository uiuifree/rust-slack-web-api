// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`admin.emoji.add`](https://docs.slack.dev/reference/methods/admin.emoji.add): Add an emoji.
///
/// Send it with [`SlackClient::admin_emoji_add`].
#[doc(alias = "admin.emoji.add")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminEmojiAddRequest {
    /// The name of the emoji to be added (using lower-case letters only). Colons (`:myemoji:`) around the value are not required, although they may be included.
    pub name: String,
    /// The URL of a file to use as an image for the emoji. Square images under 128KB and with transparent backgrounds work best.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl AdminEmojiAddRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: None,
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

impl SlackApiMethod for AdminEmojiAddRequest {
    const METHOD: &'static str = "admin.emoji.add";
    type Response = AdminEmojiAddResponse;
}

/// Successful response of the Slack Web API method [`admin.emoji.add`](https://docs.slack.dev/reference/methods/admin.emoji.add).
#[doc(alias = "admin.emoji.add")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminEmojiAddResponse {
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

/// Arguments for the Slack Web API method [`admin.emoji.addAlias`](https://docs.slack.dev/reference/methods/admin.emoji.addAlias): Add an emoji alias.
///
/// Send it with [`SlackClient::admin_emoji_add_alias`].
#[doc(alias = "admin.emoji.addAlias")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminEmojiAddAliasRequest {
    /// The new alias for the specified emoji. Any wrapping whitespace or colons will be automatically trimmed.
    pub name: String,
    /// Name of the emoji for which the alias is being made. Any wrapping whitespace or colons will be automatically trimmed.
    pub alias_for: String,
}

impl AdminEmojiAddAliasRequest {
    pub fn new(name: impl Into<String>, alias_for: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            alias_for: alias_for.into(),
        }
    }
}

impl SlackApiMethod for AdminEmojiAddAliasRequest {
    const METHOD: &'static str = "admin.emoji.addAlias";
    type Response = AdminEmojiAddAliasResponse;
}

/// Successful response of the Slack Web API method [`admin.emoji.addAlias`](https://docs.slack.dev/reference/methods/admin.emoji.addAlias).
#[doc(alias = "admin.emoji.addAlias")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminEmojiAddAliasResponse {
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

/// Arguments for the Slack Web API method [`admin.emoji.list`](https://docs.slack.dev/reference/methods/admin.emoji.list): List emoji for an Enterprise organization.
///
/// Send it with [`SlackClient::admin_emoji_list`].
#[doc(alias = "admin.emoji.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminEmojiListRequest {
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The maximum number of items to return. Must be between 1 - 1000 both inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl AdminEmojiListRequest {
    pub fn new() -> Self {
        Self {
            cursor: None,
            limit: None,
        }
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl SlackApiMethod for AdminEmojiListRequest {
    const METHOD: &'static str = "admin.emoji.list";
    type Response = AdminEmojiListResponse;
}

impl CursorPaginated for AdminEmojiListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminEmojiListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.emoji.list`](https://docs.slack.dev/reference/methods/admin.emoji.list).
#[doc(alias = "admin.emoji.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminEmojiListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub emoji: Option<std::collections::HashMap<String, AdminEmojiListResponseEmojiValue>>,
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
pub struct AdminEmojiListResponseEmojiValue {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
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
    pub uploaded_by: Option<String>,
}

/// Arguments for the Slack Web API method [`admin.emoji.remove`](https://docs.slack.dev/reference/methods/admin.emoji.remove): Remove an emoji across an Enterprise organization
///
/// Send it with [`SlackClient::admin_emoji_remove`].
#[doc(alias = "admin.emoji.remove")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminEmojiRemoveRequest {
    /// The name of the emoji to be removed. Colons (`:myemoji:`) around the value are not required, although they may be included.
    pub name: String,
}

impl AdminEmojiRemoveRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl SlackApiMethod for AdminEmojiRemoveRequest {
    const METHOD: &'static str = "admin.emoji.remove";
    type Response = AdminEmojiRemoveResponse;
}

/// Successful response of the Slack Web API method [`admin.emoji.remove`](https://docs.slack.dev/reference/methods/admin.emoji.remove).
#[doc(alias = "admin.emoji.remove")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminEmojiRemoveResponse {
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

/// Arguments for the Slack Web API method [`admin.emoji.rename`](https://docs.slack.dev/reference/methods/admin.emoji.rename): Rename an emoji.
///
/// Send it with [`SlackClient::admin_emoji_rename`].
#[doc(alias = "admin.emoji.rename")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminEmojiRenameRequest {
    /// The name of the emoji to be renamed. Colons (`:myemoji:`) around the value are not required, although they may be included.
    pub name: String,
    /// The new name of the emoji.
    pub new_name: String,
}

impl AdminEmojiRenameRequest {
    pub fn new(name: impl Into<String>, new_name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            new_name: new_name.into(),
        }
    }
}

impl SlackApiMethod for AdminEmojiRenameRequest {
    const METHOD: &'static str = "admin.emoji.rename";
    type Response = AdminEmojiRenameResponse;
}

/// Successful response of the Slack Web API method [`admin.emoji.rename`](https://docs.slack.dev/reference/methods/admin.emoji.rename).
#[doc(alias = "admin.emoji.rename")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminEmojiRenameResponse {
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
    /// Calls the Slack Web API method [`admin.emoji.add`](https://docs.slack.dev/reference/methods/admin.emoji.add): Add an emoji.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.teams:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.emoji.add")]
    pub async fn admin_emoji_add(
        &self,
        request: &AdminEmojiAddRequest,
    ) -> Result<AdminEmojiAddResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.emoji.addAlias`](https://docs.slack.dev/reference/methods/admin.emoji.addAlias): Add an emoji alias.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.teams:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.emoji.addAlias")]
    pub async fn admin_emoji_add_alias(
        &self,
        request: &AdminEmojiAddAliasRequest,
    ) -> Result<AdminEmojiAddAliasResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.emoji.list`](https://docs.slack.dev/reference/methods/admin.emoji.list): List emoji for an Enterprise organization.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.teams:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.emoji.list")]
    pub async fn admin_emoji_list(
        &self,
        request: &AdminEmojiListRequest,
    ) -> Result<AdminEmojiListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.emoji.remove`](https://docs.slack.dev/reference/methods/admin.emoji.remove): Remove an emoji across an Enterprise organization
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.teams:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.emoji.remove")]
    pub async fn admin_emoji_remove(
        &self,
        request: &AdminEmojiRemoveRequest,
    ) -> Result<AdminEmojiRemoveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.emoji.rename`](https://docs.slack.dev/reference/methods/admin.emoji.rename): Rename an emoji.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.teams:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.emoji.rename")]
    pub async fn admin_emoji_rename(
        &self,
        request: &AdminEmojiRenameRequest,
    ) -> Result<AdminEmojiRenameResponse, SlackError> {
        self.call(request).await
    }
}
