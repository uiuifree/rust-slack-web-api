// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`admin.inviteRequests.approve`](https://docs.slack.dev/reference/methods/admin.inviteRequests.approve): Approve a workspace invite request.
///
/// Send it with [`SlackClient::admin_invite_requests_approve`].
#[doc(alias = "admin.inviteRequests.approve")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminInviteRequestsApproveRequest {
    /// ID of the request to invite.
    pub invite_request_id: String,
    /// ID for the workspace where the invite request was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl AdminInviteRequestsApproveRequest {
    pub fn new(invite_request_id: impl Into<String>) -> Self {
        Self {
            invite_request_id: invite_request_id.into(),
            team_id: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for AdminInviteRequestsApproveRequest {
    const METHOD: &'static str = "admin.inviteRequests.approve";
    type Response = AdminInviteRequestsApproveResponse;
}

/// Successful response of the Slack Web API method [`admin.inviteRequests.approve`](https://docs.slack.dev/reference/methods/admin.inviteRequests.approve).
#[doc(alias = "admin.inviteRequests.approve")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminInviteRequestsApproveResponse {
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

/// Arguments for the Slack Web API method [`admin.inviteRequests.approved.list`](https://docs.slack.dev/reference/methods/admin.inviteRequests.approved.list): List all approved workspace invite requests.
///
/// Send it with [`SlackClient::admin_invite_requests_approved_list`].
#[doc(alias = "admin.inviteRequests.approved.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminInviteRequestsApprovedListRequest {
    /// ID for the workspace where the invite requests were made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Value of the `next_cursor` field sent as part of the previous API response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The number of results that will be returned by the API on each invocation. Must be between 1 - 1000, both inclusive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl AdminInviteRequestsApprovedListRequest {
    pub fn new() -> Self {
        Self {
            team_id: None,
            cursor: None,
            limit: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
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
}

impl SlackApiMethod for AdminInviteRequestsApprovedListRequest {
    const METHOD: &'static str = "admin.inviteRequests.approved.list";
    type Response = AdminInviteRequestsApprovedListResponse;
}

impl CursorPaginated for AdminInviteRequestsApprovedListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminInviteRequestsApprovedListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.inviteRequests.approved.list`](https://docs.slack.dev/reference/methods/admin.inviteRequests.approved.list).
#[doc(alias = "admin.inviteRequests.approved.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminInviteRequestsApprovedListResponse {
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

/// Arguments for the Slack Web API method [`admin.inviteRequests.denied.list`](https://docs.slack.dev/reference/methods/admin.inviteRequests.denied.list): List all denied workspace invite requests.
///
/// Send it with [`SlackClient::admin_invite_requests_denied_list`].
#[doc(alias = "admin.inviteRequests.denied.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminInviteRequestsDeniedListRequest {
    /// ID for the workspace where the invite requests were made
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Value of the `next_cursor` field sent as part of the previous api response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The number of results that will be returned by the API on each invocation. Must be between 1 - 1000 both inclusive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl AdminInviteRequestsDeniedListRequest {
    pub fn new() -> Self {
        Self {
            team_id: None,
            cursor: None,
            limit: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
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
}

impl SlackApiMethod for AdminInviteRequestsDeniedListRequest {
    const METHOD: &'static str = "admin.inviteRequests.denied.list";
    type Response = AdminInviteRequestsDeniedListResponse;
}

impl CursorPaginated for AdminInviteRequestsDeniedListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminInviteRequestsDeniedListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.inviteRequests.denied.list`](https://docs.slack.dev/reference/methods/admin.inviteRequests.denied.list).
#[doc(alias = "admin.inviteRequests.denied.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminInviteRequestsDeniedListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub denied_requests: Vec<AdminInviteRequestsDeniedListResponseDeniedRequests>,
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
pub struct AdminInviteRequestsDeniedListResponseDeniedRequests {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub invite_request: Option<AdminInviteRequestsDeniedListResponseDeniedRequestsInviteRequest>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub denied_by: Option<AdminInviteRequestsDeniedListResponseDeniedRequestsDeniedBy>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminInviteRequestsDeniedListResponseDeniedRequestsInviteRequest {
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
    pub email: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub requester_ids: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub channel_ids: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_restricted: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_ultra_restricted: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub real_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_expire: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_reason: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminInviteRequestsDeniedListResponseDeniedRequestsDeniedBy {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_id: Option<String>,
}

/// Arguments for the Slack Web API method [`admin.inviteRequests.deny`](https://docs.slack.dev/reference/methods/admin.inviteRequests.deny): Deny a workspace invite request.
///
/// Send it with [`SlackClient::admin_invite_requests_deny`].
#[doc(alias = "admin.inviteRequests.deny")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminInviteRequestsDenyRequest {
    /// ID of the request to invite.
    pub invite_request_id: String,
    /// ID for the workspace where the invite request was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl AdminInviteRequestsDenyRequest {
    pub fn new(invite_request_id: impl Into<String>) -> Self {
        Self {
            invite_request_id: invite_request_id.into(),
            team_id: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for AdminInviteRequestsDenyRequest {
    const METHOD: &'static str = "admin.inviteRequests.deny";
    type Response = AdminInviteRequestsDenyResponse;
}

/// Successful response of the Slack Web API method [`admin.inviteRequests.deny`](https://docs.slack.dev/reference/methods/admin.inviteRequests.deny).
#[doc(alias = "admin.inviteRequests.deny")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminInviteRequestsDenyResponse {
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

/// Arguments for the Slack Web API method [`admin.inviteRequests.list`](https://docs.slack.dev/reference/methods/admin.inviteRequests.list): List all pending workspace invite requests.
///
/// Send it with [`SlackClient::admin_invite_requests_list`].
#[doc(alias = "admin.inviteRequests.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminInviteRequestsListRequest {
    /// ID for the workspace where the invite requests were made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Value of the `next_cursor` field sent as part of the previous API response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The number of results that will be returned by the API on each invocation. Must be between 1 - 1000, both inclusive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl AdminInviteRequestsListRequest {
    pub fn new() -> Self {
        Self {
            team_id: None,
            cursor: None,
            limit: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
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
}

impl SlackApiMethod for AdminInviteRequestsListRequest {
    const METHOD: &'static str = "admin.inviteRequests.list";
    type Response = AdminInviteRequestsListResponse;
}

impl CursorPaginated for AdminInviteRequestsListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminInviteRequestsListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.inviteRequests.list`](https://docs.slack.dev/reference/methods/admin.inviteRequests.list).
#[doc(alias = "admin.inviteRequests.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminInviteRequestsListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub invite_requests: Vec<AdminInviteRequestsListResponseInviteRequests>,
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
pub struct AdminInviteRequestsListResponseInviteRequests {
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
    pub email: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub requester_ids: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub channel_ids: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub invite_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub real_name: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_expire: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_reason: Option<serde_json::Value>,
}

impl SlackClient {
    /// Calls the Slack Web API method [`admin.inviteRequests.approve`](https://docs.slack.dev/reference/methods/admin.inviteRequests.approve): Approve a workspace invite request.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.invites:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.inviteRequests.approve")]
    pub async fn admin_invite_requests_approve(
        &self,
        request: &AdminInviteRequestsApproveRequest,
    ) -> Result<AdminInviteRequestsApproveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.inviteRequests.approved.list`](https://docs.slack.dev/reference/methods/admin.inviteRequests.approved.list): List all approved workspace invite requests.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.invites:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.inviteRequests.approved.list")]
    pub async fn admin_invite_requests_approved_list(
        &self,
        request: &AdminInviteRequestsApprovedListRequest,
    ) -> Result<AdminInviteRequestsApprovedListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.inviteRequests.denied.list`](https://docs.slack.dev/reference/methods/admin.inviteRequests.denied.list): List all denied workspace invite requests.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.invites:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.inviteRequests.denied.list")]
    pub async fn admin_invite_requests_denied_list(
        &self,
        request: &AdminInviteRequestsDeniedListRequest,
    ) -> Result<AdminInviteRequestsDeniedListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.inviteRequests.deny`](https://docs.slack.dev/reference/methods/admin.inviteRequests.deny): Deny a workspace invite request.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.invites:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.inviteRequests.deny")]
    pub async fn admin_invite_requests_deny(
        &self,
        request: &AdminInviteRequestsDenyRequest,
    ) -> Result<AdminInviteRequestsDenyResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.inviteRequests.list`](https://docs.slack.dev/reference/methods/admin.inviteRequests.list): List all pending workspace invite requests.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.invites:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.inviteRequests.list")]
    pub async fn admin_invite_requests_list(
        &self,
        request: &AdminInviteRequestsListRequest,
    ) -> Result<AdminInviteRequestsListResponse, SlackError> {
        self.call(request).await
    }
}
