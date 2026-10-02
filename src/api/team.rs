// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`team.accessLogs`](https://docs.slack.dev/reference/methods/team.accessLogs): Gets the access logs for the current team.
///
/// Send it with [`SlackClient::team_access_logs`].
#[doc(alias = "team.accessLogs")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct TeamAccessLogsRequest {
    /// End of time range of logs to include in results (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// Parameter for pagination. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first "page" of the collection. See [pagination](https://docs.slack.dev/apis/web-api/pagination.md) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. If specified, result is returned using a cursor-based approach instead of a classic one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// encoded team id to get logs from, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl TeamAccessLogsRequest {
    pub fn new() -> Self {
        Self {
            before: None,
            count: None,
            page: None,
            cursor: None,
            limit: None,
            team_id: None,
        }
    }

    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    pub fn count(mut self, count: impl Into<String>) -> Self {
        self.count = Some(count.into());
        self
    }

    pub fn page(mut self, page: impl Into<String>) -> Self {
        self.page = Some(page.into());
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

impl SlackApiMethod for TeamAccessLogsRequest {
    const METHOD: &'static str = "team.accessLogs";
    type Response = TeamAccessLogsResponse;
}

impl CursorPaginated for TeamAccessLogsRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for TeamAccessLogsResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`team.accessLogs`](https://docs.slack.dev/reference/methods/team.accessLogs).
#[doc(alias = "team.accessLogs")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamAccessLogsResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub logins: Vec<TeamAccessLogsResponseLogins>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub paging: Option<TeamAccessLogsResponsePaging>,
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
pub struct TeamAccessLogsResponseLogins {
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
    pub username: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_first: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_last: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub ip: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_agent: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub isp: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub country: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamAccessLogsResponsePaging {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub count: Option<i64>,
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
    pub page: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub pages: Option<i64>,
}

/// Arguments for the Slack Web API method [`team.billableInfo`](https://docs.slack.dev/reference/methods/team.billableInfo): Gets billable users information for the current team.
///
/// Send it with [`SlackClient::team_billable_info`].
#[doc(alias = "team.billableInfo")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct TeamBillableInfoRequest {
    /// Set `cursor` to `next_cursor` returned by previous call, to indicate from where you want to list next page of users list. Default value fetches the first page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The maximum number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A user to retrieve the billable information for. Defaults to all users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// encoded team id to get the billable information from, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl TeamBillableInfoRequest {
    pub fn new() -> Self {
        Self {
            cursor: None,
            limit: None,
            user: None,
            team_id: None,
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

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for TeamBillableInfoRequest {
    const METHOD: &'static str = "team.billableInfo";
    type Response = TeamBillableInfoResponse;
}

impl CursorPaginated for TeamBillableInfoRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for TeamBillableInfoResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`team.billableInfo`](https://docs.slack.dev/reference/methods/team.billableInfo).
#[doc(alias = "team.billableInfo")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamBillableInfoResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub billable_info:
        Option<std::collections::HashMap<String, TeamBillableInfoResponseBillableInfoValue>>,
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
pub struct TeamBillableInfoResponseBillableInfoValue {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_active: Option<bool>,
}

/// Arguments for the Slack Web API method [`team.billing.info`](https://docs.slack.dev/reference/methods/team.billing.info): Reads a workspace's billing plan information.
///
/// Send it with [`SlackClient::team_billing_info`].
#[doc(alias = "team.billing.info")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct TeamBillingInfoRequest {}

impl TeamBillingInfoRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl SlackApiMethod for TeamBillingInfoRequest {
    const METHOD: &'static str = "team.billing.info";
    type Response = TeamBillingInfoResponse;
}

/// Successful response of the Slack Web API method [`team.billing.info`](https://docs.slack.dev/reference/methods/team.billing.info).
#[doc(alias = "team.billing.info")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamBillingInfoResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub plan: Option<String>,
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

/// Arguments for the Slack Web API method [`team.externalTeams.disconnect`](https://docs.slack.dev/reference/methods/team.externalTeams.disconnect): Disconnect an external organization.
///
/// Send it with [`SlackClient::team_external_teams_disconnect`].
#[doc(alias = "team.externalTeams.disconnect")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct TeamExternalTeamsDisconnectRequest {
    /// The team ID of the target team.
    pub target_team: String,
}

impl TeamExternalTeamsDisconnectRequest {
    pub fn new(target_team: impl Into<String>) -> Self {
        Self {
            target_team: target_team.into(),
        }
    }
}

impl SlackApiMethod for TeamExternalTeamsDisconnectRequest {
    const METHOD: &'static str = "team.externalTeams.disconnect";
    type Response = TeamExternalTeamsDisconnectResponse;
}

/// Successful response of the Slack Web API method [`team.externalTeams.disconnect`](https://docs.slack.dev/reference/methods/team.externalTeams.disconnect).
#[doc(alias = "team.externalTeams.disconnect")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamExternalTeamsDisconnectResponse {
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

/// Arguments for the Slack Web API method [`team.externalTeams.list`](https://docs.slack.dev/reference/methods/team.externalTeams.list): Returns a list of all the external teams connected and details about the connection.
///
/// Send it with [`SlackClient::team_external_teams_list`].
#[doc(alias = "team.externalTeams.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct TeamExternalTeamsListRequest {
    /// The maximum number of items to return per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Paginate through collections of data by setting parameter to the `team_id` attribute returned by a previous request's `response_metadata`. If not provided, the first page of the collection is returned. See [pagination](https://docs.slack.dev/apis/web-api/pagination.md#cursors) for more detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Name of the parameter that we are sorting by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,
    /// Direction to sort in asc or desc
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
    /// Filters connected orgs by Slack Connect pref override(s). Value can be: `approved_orgs_only` `allow_sc_file_uploads` `profile_visibility` `away_team_sc_invite_permissions` `accept_sc_invites` `sc_mpdm_to_private` `require_sc_channel_for_sc_dm` `external_awareness_context_bar`
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub slack_connect_pref_filter: Option<Vec<serde_json::Value>>,
    /// Shows connected orgs which are connected on a specified encoded workspace ID
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub workspace_filter: Option<Vec<serde_json::Value>>,
    /// Status of the connected team.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status_filter: Option<String>,
}

impl TeamExternalTeamsListRequest {
    pub fn new() -> Self {
        Self {
            limit: None,
            cursor: None,
            sort_field: None,
            sort_direction: None,
            slack_connect_pref_filter: None,
            workspace_filter: None,
            connection_status_filter: None,
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

    pub fn sort_field(mut self, sort_field: impl Into<String>) -> Self {
        self.sort_field = Some(sort_field.into());
        self
    }

    pub fn sort_direction(mut self, sort_direction: impl Into<String>) -> Self {
        self.sort_direction = Some(sort_direction.into());
        self
    }

    pub fn slack_connect_pref_filter(
        mut self,
        slack_connect_pref_filter: Vec<serde_json::Value>,
    ) -> Self {
        self.slack_connect_pref_filter = Some(slack_connect_pref_filter);
        self
    }

    pub fn workspace_filter(mut self, workspace_filter: Vec<serde_json::Value>) -> Self {
        self.workspace_filter = Some(workspace_filter);
        self
    }

    pub fn connection_status_filter(mut self, connection_status_filter: impl Into<String>) -> Self {
        self.connection_status_filter = Some(connection_status_filter.into());
        self
    }
}

impl SlackApiMethod for TeamExternalTeamsListRequest {
    const METHOD: &'static str = "team.externalTeams.list";
    type Response = TeamExternalTeamsListResponse;
}

impl CursorPaginated for TeamExternalTeamsListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for TeamExternalTeamsListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`team.externalTeams.list`](https://docs.slack.dev/reference/methods/team.externalTeams.list).
#[doc(alias = "team.externalTeams.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamExternalTeamsListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub organizations: Vec<TeamExternalTeamsListResponseOrganizations>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_count: Option<i64>,
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
pub struct TeamExternalTeamsListResponseOrganizations {
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
    pub team_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_domain: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub public_channel_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub private_channel_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub im_channel_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub mpim_channel_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub connected_workspaces: Option<TeamExternalTeamsListResponseOrganizationsConnectedWorkspaces>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_connect_prefs: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_status: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_active_timestamp: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_sponsored: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub canvas: Option<TeamExternalTeamsListResponseOrganizationsCanvas>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub lists: Option<TeamExternalTeamsListResponseOrganizationsLists>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamExternalTeamsListResponseOrganizationsConnectedWorkspaces {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub workspace_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub workspace_name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamExternalTeamsListResponseOrganizationsCanvas {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ownership_details: Vec<TeamExternalTeamsListResponseOrganizationsCanvasOwnershipDetails>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamExternalTeamsListResponseOrganizationsCanvasOwnershipDetails {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub count: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamExternalTeamsListResponseOrganizationsLists {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ownership_details: Vec<TeamExternalTeamsListResponseOrganizationsListsOwnershipDetails>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamExternalTeamsListResponseOrganizationsListsOwnershipDetails {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub count: Option<i64>,
}

/// Arguments for the Slack Web API method [`team.info`](https://docs.slack.dev/reference/methods/team.info): Gets information about the current team.
///
/// Send it with [`SlackClient::team_info`].
#[doc(alias = "team.info")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct TeamInfoRequest {
    /// Query by domain instead of team (only when team is null). This only works for domains in the same enterprise as the querying team token. This also expects the domain to belong to a team and not the enterprise itself. This is the value set up for the 'Joining This Workspace' workspace setting. If it contains more than one domain, the field will contain multiple comma-separated domain values. If no domain is set, the field is empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Team to get info about; if omitted, will return information about the current team.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
}

impl TeamInfoRequest {
    pub fn new() -> Self {
        Self {
            domain: None,
            team: None,
        }
    }

    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    pub fn team(mut self, team: impl Into<String>) -> Self {
        self.team = Some(team.into());
        self
    }
}

impl SlackApiMethod for TeamInfoRequest {
    const METHOD: &'static str = "team.info";
    type Response = TeamInfoResponse;
}

/// Successful response of the Slack Web API method [`team.info`](https://docs.slack.dev/reference/methods/team.info).
#[doc(alias = "team.info")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamInfoResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub team: Option<Team>,
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

/// Arguments for the Slack Web API method [`team.integrationLogs`](https://docs.slack.dev/reference/methods/team.integrationLogs): Gets the integration logs for the current team.
///
/// Send it with [`SlackClient::team_integration_logs`].
#[doc(alias = "team.integrationLogs")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct TeamIntegrationLogsRequest {
    /// Filter logs to this Slack app. Defaults to all logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// Filter logs with this change type. Possible values are `added`, `removed`, `enabled`, `disabled`, and `updated`. Defaults to all logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// Filter logs to this service. Defaults to all logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    /// encoded team id to get logs from, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Filter logs generated by this user’s actions. Defaults to all logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl TeamIntegrationLogsRequest {
    pub fn new() -> Self {
        Self {
            app_id: None,
            change_type: None,
            count: None,
            page: None,
            service_id: None,
            team_id: None,
            user: None,
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    pub fn change_type(mut self, change_type: impl Into<String>) -> Self {
        self.change_type = Some(change_type.into());
        self
    }

    pub fn count(mut self, count: impl Into<String>) -> Self {
        self.count = Some(count.into());
        self
    }

    pub fn page(mut self, page: impl Into<String>) -> Self {
        self.page = Some(page.into());
        self
    }

    pub fn service_id(mut self, service_id: impl Into<String>) -> Self {
        self.service_id = Some(service_id.into());
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

impl SlackApiMethod for TeamIntegrationLogsRequest {
    const METHOD: &'static str = "team.integrationLogs";
    type Response = TeamIntegrationLogsResponse;
}

/// Successful response of the Slack Web API method [`team.integrationLogs`](https://docs.slack.dev/reference/methods/team.integrationLogs).
#[doc(alias = "team.integrationLogs")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamIntegrationLogsResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub logs: Vec<TeamIntegrationLogsResponseLogs>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub paging: Option<TeamIntegrationLogsResponsePaging>,
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
pub struct TeamIntegrationLogsResponseLogs {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_id: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_type: Option<String>,
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
    pub user_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub date: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub change_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub scope: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamIntegrationLogsResponsePaging {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub count: Option<i64>,
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
    pub page: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub pages: Option<i64>,
}

/// Arguments for the Slack Web API method [`team.preferences.list`](https://docs.slack.dev/reference/methods/team.preferences.list): Retrieve a list of a workspace's team preferences.
///
/// Send it with [`SlackClient::team_preferences_list`].
#[doc(alias = "team.preferences.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct TeamPreferencesListRequest {}

impl TeamPreferencesListRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl SlackApiMethod for TeamPreferencesListRequest {
    const METHOD: &'static str = "team.preferences.list";
    type Response = TeamPreferencesListResponse;
}

/// Successful response of the Slack Web API method [`team.preferences.list`](https://docs.slack.dev/reference/methods/team.preferences.list).
#[doc(alias = "team.preferences.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamPreferencesListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_real_names: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_file_uploads: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub msg_edit_window_mins: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub who_can_post_general: Option<String>,
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

/// Arguments for the Slack Web API method [`team.profile.get`](https://docs.slack.dev/reference/methods/team.profile.get): Retrieve a team's profile.
///
/// Send it with [`SlackClient::team_profile_get`].
#[doc(alias = "team.profile.get")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct TeamProfileGetRequest {
    /// Filter by visibility.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

impl TeamProfileGetRequest {
    pub fn new() -> Self {
        Self { visibility: None }
    }

    pub fn visibility(mut self, visibility: impl Into<String>) -> Self {
        self.visibility = Some(visibility.into());
        self
    }
}

impl SlackApiMethod for TeamProfileGetRequest {
    const METHOD: &'static str = "team.profile.get";
    type Response = TeamProfileGetResponse;
}

/// Successful response of the Slack Web API method [`team.profile.get`](https://docs.slack.dev/reference/methods/team.profile.get).
#[doc(alias = "team.profile.get")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamProfileGetResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub profile: Option<TeamProfileGetResponseProfile>,
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
pub struct TeamProfileGetResponseProfile {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fields: Vec<TeamProfileGetResponseProfileFields>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sections: Vec<TeamProfileGetResponseProfileSections>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamProfileGetResponseProfileFields {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub ordering: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub label: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub hint: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub possible_values: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub options: Option<TeamProfileGetResponseProfileFieldsOptions>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_hidden: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub section_id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamProfileGetResponseProfileFieldsOptions {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_scim: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_protected: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamProfileGetResponseProfileSections {
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
    pub team_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub section_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub label: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub order: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_hidden: Option<bool>,
}

impl SlackClient {
    /// Calls the Slack Web API method [`team.accessLogs`](https://docs.slack.dev/reference/methods/team.accessLogs): Gets the access logs for the current team.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "team.accessLogs")]
    pub async fn team_access_logs(
        &self,
        request: &TeamAccessLogsRequest,
    ) -> Result<TeamAccessLogsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`team.billableInfo`](https://docs.slack.dev/reference/methods/team.billableInfo): Gets billable users information for the current team.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "team.billableInfo")]
    pub async fn team_billable_info(
        &self,
        request: &TeamBillableInfoRequest,
    ) -> Result<TeamBillableInfoResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`team.billing.info`](https://docs.slack.dev/reference/methods/team.billing.info): Reads a workspace's billing plan information.
    ///
    /// Required scopes:
    ///
    /// - bot token: `team.billing:read`
    /// - user token: `team.billing:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "team.billing.info")]
    pub async fn team_billing_info(
        &self,
        request: &TeamBillingInfoRequest,
    ) -> Result<TeamBillingInfoResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`team.externalTeams.disconnect`](https://docs.slack.dev/reference/methods/team.externalTeams.disconnect): Disconnect an external organization.
    ///
    /// Required scopes:
    ///
    /// - bot token: `conversations.connect:manage`
    ///
    /// Rate limit: Tier 1 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "team.externalTeams.disconnect")]
    pub async fn team_external_teams_disconnect(
        &self,
        request: &TeamExternalTeamsDisconnectRequest,
    ) -> Result<TeamExternalTeamsDisconnectResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`team.externalTeams.list`](https://docs.slack.dev/reference/methods/team.externalTeams.list): Returns a list of all the external teams connected and details about the connection.
    ///
    /// Required scopes:
    ///
    /// - bot token: `conversations.connect:manage`, `team:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "team.externalTeams.list")]
    pub async fn team_external_teams_list(
        &self,
        request: &TeamExternalTeamsListRequest,
    ) -> Result<TeamExternalTeamsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`team.info`](https://docs.slack.dev/reference/methods/team.info): Gets information about the current team.
    ///
    /// Required scopes:
    ///
    /// - bot token: `team:read`
    /// - user token: `team:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "team.info")]
    pub async fn team_info(
        &self,
        request: &TeamInfoRequest,
    ) -> Result<TeamInfoResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`team.integrationLogs`](https://docs.slack.dev/reference/methods/team.integrationLogs): Gets the integration logs for the current team.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "team.integrationLogs")]
    pub async fn team_integration_logs(
        &self,
        request: &TeamIntegrationLogsRequest,
    ) -> Result<TeamIntegrationLogsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`team.preferences.list`](https://docs.slack.dev/reference/methods/team.preferences.list): Retrieve a list of a workspace's team preferences.
    ///
    /// Required scopes:
    ///
    /// - bot token: `team.preferences:read`
    /// - user token: `team.preferences:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "team.preferences.list")]
    pub async fn team_preferences_list(
        &self,
        request: &TeamPreferencesListRequest,
    ) -> Result<TeamPreferencesListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`team.profile.get`](https://docs.slack.dev/reference/methods/team.profile.get): Retrieve a team's profile.
    ///
    /// Required scopes:
    ///
    /// - bot token: `users.profile:read`
    /// - user token: `users.profile:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "team.profile.get")]
    pub async fn team_profile_get(
        &self,
        request: &TeamProfileGetRequest,
    ) -> Result<TeamProfileGetResponse, SlackError> {
        self.call(request).await
    }
}
