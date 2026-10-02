// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`rtm.connect`](https://docs.slack.dev/reference/methods/rtm.connect): Starts a Real Time Messaging session.
///
/// Send it with [`SlackClient::rtm_connect`].
#[doc(alias = "rtm.connect")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct RtmConnectRequest {
    /// Batch presence deliveries via subscription. Enabling changes the shape of `presence_change` events. See [batch presence](https://docs.slack.dev/apis/web-api/user-presence-and-status.md#batching).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_presence_aware: Option<bool>,
    /// Only deliver presence events when requested by subscription. See [presence subscriptions](https://docs.slack.dev/apis/web-api/user-presence-and-status.md#subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_sub: Option<bool>,
}

impl RtmConnectRequest {
    pub fn new() -> Self {
        Self {
            batch_presence_aware: None,
            presence_sub: None,
        }
    }

    pub fn batch_presence_aware(mut self, batch_presence_aware: bool) -> Self {
        self.batch_presence_aware = Some(batch_presence_aware);
        self
    }

    pub fn presence_sub(mut self, presence_sub: bool) -> Self {
        self.presence_sub = Some(presence_sub);
        self
    }
}

impl SlackApiMethod for RtmConnectRequest {
    const METHOD: &'static str = "rtm.connect";
    type Response = RtmConnectResponse;
}

/// Successful response of the Slack Web API method [`rtm.connect`](https://docs.slack.dev/reference/methods/rtm.connect).
#[doc(alias = "rtm.connect")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RtmConnectResponse {
    #[serde(
        rename = "self",
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub self_: Option<RtmConnectResponseSelf>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub team: Option<Team>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
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
pub struct RtmConnectResponseSelf {
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
}

/// Arguments for the Slack Web API method [`rtm.start`](https://docs.slack.dev/reference/methods/rtm.start): Deprecated: Starts a Real Time Messaging session. Use rtm.connect instead.
///
/// Send it with [`SlackClient::rtm_start`].
#[doc(alias = "rtm.start")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct RtmStartRequest {
    /// Return timestamp only for latest message object of each channel (improves performance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_latest: Option<bool>,
    /// Skip unread counts for each channel (improves performance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_unreads: Option<bool>,
    /// Returns MPIMs to the client in the API response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpim_aware: Option<bool>,
    /// Only deliver presence events when requested by subscription. See [presence subscriptions](https://docs.slack.dev/apis/web-api/user-presence-and-status.md#subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_sub: Option<bool>,
    /// Batch presence deliveries via subscription. Enabling changes the shape of `presence_change` events. See [batch presence](https://docs.slack.dev/apis/web-api/user-presence-and-status.md#batching).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_presence_aware: Option<bool>,
    /// Exclude latest timestamps for channels, groups, mpims, and ims. Automatically sets `no_unreads` to `1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_latest: Option<bool>,
    /// Set this to `true` to receive the locale for users and channels. Defaults to `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_locale: Option<bool>,
}

impl RtmStartRequest {
    pub fn new() -> Self {
        Self {
            simple_latest: None,
            no_unreads: None,
            mpim_aware: None,
            presence_sub: None,
            batch_presence_aware: None,
            no_latest: None,
            include_locale: None,
        }
    }

    pub fn simple_latest(mut self, simple_latest: bool) -> Self {
        self.simple_latest = Some(simple_latest);
        self
    }

    pub fn no_unreads(mut self, no_unreads: bool) -> Self {
        self.no_unreads = Some(no_unreads);
        self
    }

    pub fn mpim_aware(mut self, mpim_aware: bool) -> Self {
        self.mpim_aware = Some(mpim_aware);
        self
    }

    pub fn presence_sub(mut self, presence_sub: bool) -> Self {
        self.presence_sub = Some(presence_sub);
        self
    }

    pub fn batch_presence_aware(mut self, batch_presence_aware: bool) -> Self {
        self.batch_presence_aware = Some(batch_presence_aware);
        self
    }

    pub fn no_latest(mut self, no_latest: bool) -> Self {
        self.no_latest = Some(no_latest);
        self
    }

    pub fn include_locale(mut self, include_locale: bool) -> Self {
        self.include_locale = Some(include_locale);
        self
    }
}

impl SlackApiMethod for RtmStartRequest {
    const METHOD: &'static str = "rtm.start";
    type Response = RtmStartResponse;
}

/// Successful response of the Slack Web API method [`rtm.start`](https://docs.slack.dev/reference/methods/rtm.start).
#[doc(alias = "rtm.start")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RtmStartResponse {
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
    /// Calls the Slack Web API method [`rtm.connect`](https://docs.slack.dev/reference/methods/rtm.connect): Starts a Real Time Messaging session.
    ///
    /// Rate limit: Tier 1 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "rtm.connect")]
    pub async fn rtm_connect(
        &self,
        request: &RtmConnectRequest,
    ) -> Result<RtmConnectResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`rtm.start`](https://docs.slack.dev/reference/methods/rtm.start): Deprecated: Starts a Real Time Messaging session. Use rtm.connect instead.
    ///
    /// Rate limit: Tier 1 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "rtm.start")]
    pub async fn rtm_start(
        &self,
        request: &RtmStartRequest,
    ) -> Result<RtmStartResponse, SlackError> {
        self.call(request).await
    }
}
