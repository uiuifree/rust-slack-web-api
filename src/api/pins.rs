// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`pins.add`](https://docs.slack.dev/reference/methods/pins.add): Pins an item to a channel.
///
/// Send it with [`SlackClient::pins_add`].
#[doc(alias = "pins.add")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct PinsAddRequest {
    /// Channel to pin the messsage to. You must also include a `timestamp` when pinning messages.
    pub channel: String,
    /// Timestamp of the message to pin. You must also include the `channel`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl PinsAddRequest {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            timestamp: None,
        }
    }

    pub fn timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
}

impl SlackApiMethod for PinsAddRequest {
    const METHOD: &'static str = "pins.add";
    type Response = PinsAddResponse;
}

/// Successful response of the Slack Web API method [`pins.add`](https://docs.slack.dev/reference/methods/pins.add).
#[doc(alias = "pins.add")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PinsAddResponse {
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

/// Arguments for the Slack Web API method [`pins.list`](https://docs.slack.dev/reference/methods/pins.list): Lists items pinned to a channel.
///
/// Send it with [`SlackClient::pins_list`].
#[doc(alias = "pins.list")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct PinsListRequest {
    /// Channel to get pinned items for.
    pub channel: String,
}

impl PinsListRequest {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
        }
    }
}

impl SlackApiMethod for PinsListRequest {
    const METHOD: &'static str = "pins.list";
    type Response = PinsListResponse;
}

/// Successful response of the Slack Web API method [`pins.list`](https://docs.slack.dev/reference/methods/pins.list).
#[doc(alias = "pins.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PinsListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<PinsListResponseItems>,
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
pub struct PinsListResponseItems {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<String>,
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
    pub created_by: Option<String>,
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
    pub r#type: Option<String>,
}

/// Arguments for the Slack Web API method [`pins.remove`](https://docs.slack.dev/reference/methods/pins.remove): Un-pins an item from a channel.
///
/// Send it with [`SlackClient::pins_remove`].
#[doc(alias = "pins.remove")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct PinsRemoveRequest {
    /// Channel where the item is pinned to.
    pub channel: String,
    /// Timestamp of the message to un-pin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl PinsRemoveRequest {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            timestamp: None,
        }
    }

    pub fn timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
}

impl SlackApiMethod for PinsRemoveRequest {
    const METHOD: &'static str = "pins.remove";
    type Response = PinsRemoveResponse;
}

/// Successful response of the Slack Web API method [`pins.remove`](https://docs.slack.dev/reference/methods/pins.remove).
#[doc(alias = "pins.remove")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PinsRemoveResponse {
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
    /// Calls the Slack Web API method [`pins.add`](https://docs.slack.dev/reference/methods/pins.add): Pins an item to a channel.
    ///
    /// Required scopes:
    ///
    /// - bot token: `pins:write`
    /// - user token: `pins:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "pins.add")]
    pub async fn pins_add(&self, request: &PinsAddRequest) -> Result<PinsAddResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`pins.list`](https://docs.slack.dev/reference/methods/pins.list): Lists items pinned to a channel.
    ///
    /// Required scopes:
    ///
    /// - bot token: `pins:read`
    /// - user token: `pins:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "pins.list")]
    pub async fn pins_list(
        &self,
        request: &PinsListRequest,
    ) -> Result<PinsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`pins.remove`](https://docs.slack.dev/reference/methods/pins.remove): Un-pins an item from a channel.
    ///
    /// Required scopes:
    ///
    /// - bot token: `pins:write`
    /// - user token: `pins:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "pins.remove")]
    pub async fn pins_remove(
        &self,
        request: &PinsRemoveRequest,
    ) -> Result<PinsRemoveResponse, SlackError> {
        self.call(request).await
    }
}
