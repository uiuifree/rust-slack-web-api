// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`auth.revoke`](https://docs.slack.dev/reference/methods/auth.revoke): Revokes a token.
///
/// Send it with [`SlackClient::auth_revoke`].
#[doc(alias = "auth.revoke")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AuthRevokeRequest {
    /// Setting this parameter to `1` triggers a _testing mode_ where the specified token will not actually be revoked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
}

impl AuthRevokeRequest {
    pub fn new() -> Self {
        Self { test: None }
    }

    pub fn test(mut self, test: bool) -> Self {
        self.test = Some(test);
        self
    }
}

impl SlackApiMethod for AuthRevokeRequest {
    const METHOD: &'static str = "auth.revoke";
    type Response = AuthRevokeResponse;
}

/// Successful response of the Slack Web API method [`auth.revoke`](https://docs.slack.dev/reference/methods/auth.revoke).
#[doc(alias = "auth.revoke")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AuthRevokeResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub revoked: Option<bool>,
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

/// Arguments for the Slack Web API method [`auth.teams.list`](https://docs.slack.dev/reference/methods/auth.teams.list): Obtain a full list of workspaces your org-wide app has been approved for.
///
/// Send it with [`SlackClient::auth_teams_list`].
#[doc(alias = "auth.teams.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AuthTeamsListRequest {
    /// The maximum number of workspaces to return. Must be a positive integer no larger than 1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Whether to return icon paths for each workspace. An icon path represents a URI pointing to the image signifying the workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_icon: Option<bool>,
}

impl AuthTeamsListRequest {
    pub fn new() -> Self {
        Self {
            limit: None,
            cursor: None,
            include_icon: None,
        }
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn include_icon(mut self, include_icon: bool) -> Self {
        self.include_icon = Some(include_icon);
        self
    }
}

impl SlackApiMethod for AuthTeamsListRequest {
    const METHOD: &'static str = "auth.teams.list";
    type Response = AuthTeamsListResponse;
}

impl CursorPaginated for AuthTeamsListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AuthTeamsListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`auth.teams.list`](https://docs.slack.dev/reference/methods/auth.teams.list).
#[doc(alias = "auth.teams.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AuthTeamsListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub teams: Vec<AuthTeamsListResponseTeams>,
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
pub struct AuthTeamsListResponseTeams {
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
    pub id: Option<String>,
}

/// Arguments for the Slack Web API method [`auth.test`](https://docs.slack.dev/reference/methods/auth.test): Checks authentication & identity.
///
/// Send it with [`SlackClient::auth_test`].
#[doc(alias = "auth.test")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AuthTestRequest {}

impl AuthTestRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl SlackApiMethod for AuthTestRequest {
    const METHOD: &'static str = "auth.test";
    type Response = AuthTestResponse;
}

/// Successful response of the Slack Web API method [`auth.test`](https://docs.slack.dev/reference/methods/auth.test).
#[doc(alias = "auth.test")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AuthTestResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub team: Option<String>,
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
    pub team_id: Option<String>,
    #[serde(
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
    pub bot_id: Option<String>,
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
    /// Calls the Slack Web API method [`auth.revoke`](https://docs.slack.dev/reference/methods/auth.revoke): Revokes a token.
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "auth.revoke")]
    pub async fn auth_revoke(
        &self,
        request: &AuthRevokeRequest,
    ) -> Result<AuthRevokeResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`auth.teams.list`](https://docs.slack.dev/reference/methods/auth.teams.list): Obtain a full list of workspaces your org-wide app has been approved for.
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "auth.teams.list")]
    pub async fn auth_teams_list(
        &self,
        request: &AuthTeamsListRequest,
    ) -> Result<AuthTeamsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`auth.test`](https://docs.slack.dev/reference/methods/auth.test): Checks authentication & identity.
    ///
    /// Rate limit: Tier 5 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "auth.test")]
    pub async fn auth_test(
        &self,
        request: &AuthTestRequest,
    ) -> Result<AuthTestResponse, SlackError> {
        self.call(request).await
    }
}
