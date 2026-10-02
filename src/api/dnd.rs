// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`dnd.endDnd`](https://docs.slack.dev/reference/methods/dnd.endDnd): Ends the current user's Do Not Disturb session immediately.
///
/// Send it with [`SlackClient::dnd_end_dnd`].
#[doc(alias = "dnd.endDnd")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct DndEndDndRequest {}

impl DndEndDndRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl SlackApiMethod for DndEndDndRequest {
    const METHOD: &'static str = "dnd.endDnd";
    type Response = DndEndDndResponse;
}

/// Successful response of the Slack Web API method [`dnd.endDnd`](https://docs.slack.dev/reference/methods/dnd.endDnd).
#[doc(alias = "dnd.endDnd")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DndEndDndResponse {
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

/// Arguments for the Slack Web API method [`dnd.endSnooze`](https://docs.slack.dev/reference/methods/dnd.endSnooze): Ends the current user's snooze mode immediately.
///
/// Send it with [`SlackClient::dnd_end_snooze`].
#[doc(alias = "dnd.endSnooze")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct DndEndSnoozeRequest {}

impl DndEndSnoozeRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl SlackApiMethod for DndEndSnoozeRequest {
    const METHOD: &'static str = "dnd.endSnooze";
    type Response = DndEndSnoozeResponse;
}

/// Successful response of the Slack Web API method [`dnd.endSnooze`](https://docs.slack.dev/reference/methods/dnd.endSnooze).
#[doc(alias = "dnd.endSnooze")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DndEndSnoozeResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub dnd_enabled: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_dnd_start_ts: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_dnd_end_ts: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub snooze_enabled: Option<bool>,
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

/// Arguments for the Slack Web API method [`dnd.info`](https://docs.slack.dev/reference/methods/dnd.info): Retrieves a user's current Do Not Disturb status.
///
/// Send it with [`SlackClient::dnd_info`].
#[doc(alias = "dnd.info")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct DndInfoRequest {
    /// User to fetch status for (defaults to current user)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Encoded team id where passed in user param belongs, required if org token is used. If no user param is passed, then a team which has access to the app should be passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl DndInfoRequest {
    pub fn new() -> Self {
        Self {
            user: None,
            team_id: None,
        }
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for DndInfoRequest {
    const METHOD: &'static str = "dnd.info";
    type Response = DndInfoResponse;
}

/// Successful response of the Slack Web API method [`dnd.info`](https://docs.slack.dev/reference/methods/dnd.info).
#[doc(alias = "dnd.info")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DndInfoResponse {
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

/// Arguments for the Slack Web API method [`dnd.setSnooze`](https://docs.slack.dev/reference/methods/dnd.setSnooze): Turns on Do Not Disturb mode for the current user, or changes its duration.
///
/// Send it with [`SlackClient::dnd_set_snooze`].
#[doc(alias = "dnd.setSnooze")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct DndSetSnoozeRequest {
    /// This argument is required. Number of minutes, from now, to snooze until.
    pub num_minutes: String,
}

impl DndSetSnoozeRequest {
    pub fn new(num_minutes: impl Into<String>) -> Self {
        Self {
            num_minutes: num_minutes.into(),
        }
    }
}

impl SlackApiMethod for DndSetSnoozeRequest {
    const METHOD: &'static str = "dnd.setSnooze";
    type Response = DndSetSnoozeResponse;
}

/// Successful response of the Slack Web API method [`dnd.setSnooze`](https://docs.slack.dev/reference/methods/dnd.setSnooze).
#[doc(alias = "dnd.setSnooze")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DndSetSnoozeResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub snooze_enabled: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub snooze_endtime: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub snooze_remaining: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub snooze_is_indefinite: Option<bool>,
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

/// Arguments for the Slack Web API method [`dnd.teamInfo`](https://docs.slack.dev/reference/methods/dnd.teamInfo): Retrieves the Do Not Disturb status for up to 50 users on a team.
///
/// Send it with [`SlackClient::dnd_team_info`].
#[doc(alias = "dnd.teamInfo")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct DndTeamInfoRequest {
    /// Comma-separated list of users to fetch Do Not Disturb status for
    pub users: String,
    /// Encoded team id where passed in users belong, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl DndTeamInfoRequest {
    pub fn new(users: impl Into<String>) -> Self {
        Self {
            users: users.into(),
            team_id: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for DndTeamInfoRequest {
    const METHOD: &'static str = "dnd.teamInfo";
    type Response = DndTeamInfoResponse;
}

/// Successful response of the Slack Web API method [`dnd.teamInfo`](https://docs.slack.dev/reference/methods/dnd.teamInfo).
#[doc(alias = "dnd.teamInfo")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DndTeamInfoResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub users: Option<std::collections::HashMap<String, DndTeamInfoResponseUsersValue>>,
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
pub struct DndTeamInfoResponseUsersValue {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub dnd_enabled: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_dnd_start_ts: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_dnd_end_ts: Option<i64>,
}

impl SlackClient {
    /// Calls the Slack Web API method [`dnd.endDnd`](https://docs.slack.dev/reference/methods/dnd.endDnd): Ends the current user's Do Not Disturb session immediately.
    ///
    /// Required scopes:
    ///
    /// - user token: `dnd:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "dnd.endDnd")]
    pub async fn dnd_end_dnd(
        &self,
        request: &DndEndDndRequest,
    ) -> Result<DndEndDndResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`dnd.endSnooze`](https://docs.slack.dev/reference/methods/dnd.endSnooze): Ends the current user's snooze mode immediately.
    ///
    /// Required scopes:
    ///
    /// - user token: `dnd:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "dnd.endSnooze")]
    pub async fn dnd_end_snooze(
        &self,
        request: &DndEndSnoozeRequest,
    ) -> Result<DndEndSnoozeResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`dnd.info`](https://docs.slack.dev/reference/methods/dnd.info): Retrieves a user's current Do Not Disturb status.
    ///
    /// Required scopes:
    ///
    /// - bot token: `dnd:read`
    /// - user token: `dnd:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "dnd.info")]
    pub async fn dnd_info(&self, request: &DndInfoRequest) -> Result<DndInfoResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`dnd.setSnooze`](https://docs.slack.dev/reference/methods/dnd.setSnooze): Turns on Do Not Disturb mode for the current user, or changes its duration.
    ///
    /// Required scopes:
    ///
    /// - user token: `dnd:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "dnd.setSnooze")]
    pub async fn dnd_set_snooze(
        &self,
        request: &DndSetSnoozeRequest,
    ) -> Result<DndSetSnoozeResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`dnd.teamInfo`](https://docs.slack.dev/reference/methods/dnd.teamInfo): Retrieves the Do Not Disturb status for up to 50 users on a team.
    ///
    /// Required scopes:
    ///
    /// - bot token: `dnd:read`
    /// - user token: `dnd:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "dnd.teamInfo")]
    pub async fn dnd_team_info(
        &self,
        request: &DndTeamInfoRequest,
    ) -> Result<DndTeamInfoResponse, SlackError> {
        self.call(request).await
    }
}
