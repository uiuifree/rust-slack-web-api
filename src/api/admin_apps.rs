// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`admin.apps.activities.list`](https://docs.slack.dev/reference/methods/admin.apps.activities.list): Get logs for a specified team/org
///
/// Send it with [`SlackClient::admin_apps_activities_list`].
#[doc(alias = "admin.apps.activities.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminAppsActivitiesListRequest {
    /// The ID of the app to get activities from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// The team who owns this log.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. See [pagination](https://docs.slack.dev/apis/web-api/pagination.md) for more detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The maximum number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The minimum log level of the log events to be returned. Defaults to `info`. Acceptable values (in order of relative importance from smallest to largest) are `trace`, `debug`, `info`, `warn`, `error` and `fatal`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_log_level: Option<String>,
    /// The event type of log events to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_event_type: Option<String>,
    /// The source of log events to be returned. Acceptable values are `slack` and `developer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// The component type of log events to be returned. Acceptable values are `events_api`, `workflows`, `functions` and `tables`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// The component ID of log events to be returned. Will be `FnXXXXXX` for functions, and `WfXXXXXX` for workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// The trace ID of log events to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// The earliest timestamp of the log to retrieve (epoch microseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_date_created: Option<i64>,
    /// The latest timestamp of the log to retrieve (epoch microseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_date_created: Option<i64>,
    /// The direction you want the data sorted by (always by timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
}

impl AdminAppsActivitiesListRequest {
    pub fn new() -> Self {
        Self {
            app_id: None,
            team_id: None,
            cursor: None,
            limit: None,
            min_log_level: None,
            log_event_type: None,
            source: None,
            component_type: None,
            component_id: None,
            trace_id: None,
            min_date_created: None,
            max_date_created: None,
            sort_direction: None,
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
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

    pub fn min_log_level(mut self, min_log_level: impl Into<String>) -> Self {
        self.min_log_level = Some(min_log_level.into());
        self
    }

    pub fn log_event_type(mut self, log_event_type: impl Into<String>) -> Self {
        self.log_event_type = Some(log_event_type.into());
        self
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn component_type(mut self, component_type: impl Into<String>) -> Self {
        self.component_type = Some(component_type.into());
        self
    }

    pub fn component_id(mut self, component_id: impl Into<String>) -> Self {
        self.component_id = Some(component_id.into());
        self
    }

    pub fn trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    pub fn min_date_created(mut self, min_date_created: i64) -> Self {
        self.min_date_created = Some(min_date_created);
        self
    }

    pub fn max_date_created(mut self, max_date_created: i64) -> Self {
        self.max_date_created = Some(max_date_created);
        self
    }

    pub fn sort_direction(mut self, sort_direction: impl Into<String>) -> Self {
        self.sort_direction = Some(sort_direction.into());
        self
    }
}

impl SlackApiMethod for AdminAppsActivitiesListRequest {
    const METHOD: &'static str = "admin.apps.activities.list";
    type Response = AdminAppsActivitiesListResponse;
}

impl CursorPaginated for AdminAppsActivitiesListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminAppsActivitiesListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.apps.activities.list`](https://docs.slack.dev/reference/methods/admin.apps.activities.list).
#[doc(alias = "admin.apps.activities.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsActivitiesListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub activities: Vec<AdminAppsActivitiesListResponseActivities>,
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
pub struct AdminAppsActivitiesListResponseActivities {
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
    pub level: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub event_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub source: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub component_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub component_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub payload: Option<AdminAppsActivitiesListResponseActivitiesPayload>,
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
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsActivitiesListResponseActivitiesPayload {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub function_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub function_type: Option<String>,
}

/// Arguments for the Slack Web API method [`admin.apps.approve`](https://docs.slack.dev/reference/methods/admin.apps.approve): Approve an app for installation on a workspace.
///
/// Send it with [`SlackClient::admin_apps_approve`].
#[doc(alias = "admin.apps.approve")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminAppsApproveRequest {
    /// Auto-create an Admin-Approved App automation rule that pre-approves future child app installs from this manager app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_child_auto_install: Option<bool>,
    /// The id of the app to approve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// The id of the request to approve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The ID of the workspace to approve the app on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// The ID of the enterprise to approve the app on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_id: Option<String>,
    /// User scopes to approve for the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_scopes: Option<String>,
    /// Bot scopes to approve for the app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_scopes: Option<String>,
}

impl AdminAppsApproveRequest {
    pub fn new() -> Self {
        Self {
            allow_child_auto_install: None,
            app_id: None,
            request_id: None,
            team_id: None,
            enterprise_id: None,
            user_scopes: None,
            bot_scopes: None,
        }
    }

    pub fn allow_child_auto_install(mut self, allow_child_auto_install: bool) -> Self {
        self.allow_child_auto_install = Some(allow_child_auto_install);
        self
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    pub fn request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn enterprise_id(mut self, enterprise_id: impl Into<String>) -> Self {
        self.enterprise_id = Some(enterprise_id.into());
        self
    }

    pub fn user_scopes(mut self, user_scopes: impl Into<String>) -> Self {
        self.user_scopes = Some(user_scopes.into());
        self
    }

    pub fn bot_scopes(mut self, bot_scopes: impl Into<String>) -> Self {
        self.bot_scopes = Some(bot_scopes.into());
        self
    }
}

impl SlackApiMethod for AdminAppsApproveRequest {
    const METHOD: &'static str = "admin.apps.approve";
    type Response = AdminAppsApproveResponse;
}

/// Successful response of the Slack Web API method [`admin.apps.approve`](https://docs.slack.dev/reference/methods/admin.apps.approve).
#[doc(alias = "admin.apps.approve")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsApproveResponse {
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

/// Arguments for the Slack Web API method [`admin.apps.approved.list`](https://docs.slack.dev/reference/methods/admin.apps.approved.list): List approved apps for an org or workspace.
///
/// Send it with [`SlackClient::admin_apps_approved_list`].
#[doc(alias = "admin.apps.approved.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminAppsApprovedListRequest {
    /// The maximum number of items to return. Must be between 1 - 1000 both inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_id: Option<String>,
    /// Limit the results to only include certified apps. When false, no certified apps will appear in the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certified: Option<bool>,
}

impl AdminAppsApprovedListRequest {
    pub fn new() -> Self {
        Self {
            limit: None,
            cursor: None,
            team_id: None,
            enterprise_id: None,
            certified: None,
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

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn enterprise_id(mut self, enterprise_id: impl Into<String>) -> Self {
        self.enterprise_id = Some(enterprise_id.into());
        self
    }

    pub fn certified(mut self, certified: bool) -> Self {
        self.certified = Some(certified);
        self
    }
}

impl SlackApiMethod for AdminAppsApprovedListRequest {
    const METHOD: &'static str = "admin.apps.approved.list";
    type Response = AdminAppsApprovedListResponse;
}

impl CursorPaginated for AdminAppsApprovedListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminAppsApprovedListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.apps.approved.list`](https://docs.slack.dev/reference/methods/admin.apps.approved.list).
#[doc(alias = "admin.apps.approved.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsApprovedListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub approved_apps: Vec<AdminAppsApprovedListResponseApprovedApps>,
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
pub struct AdminAppsApprovedListResponseApprovedApps {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub app: Option<AdminAppsApprovedListResponseApprovedAppsApp>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<AdminAppsApprovedListResponseApprovedAppsScopes>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_updated: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_resolved_by: Option<AdminAppsApprovedListResponseApprovedAppsLastResolvedBy>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsApprovedListResponseApprovedAppsApp {
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
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub help_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub privacy_policy_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_homepage_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_directory_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_app_directory_approved: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_internal: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub developer_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub socket_mode_enabled: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub icons: Option<Icons>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_info: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsApprovedListResponseApprovedAppsScopes {
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
    pub description: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_sensitive: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub token_type: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsApprovedListResponseApprovedAppsLastResolvedBy {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_type: Option<String>,
}

/// Arguments for the Slack Web API method [`admin.apps.clearResolution`](https://docs.slack.dev/reference/methods/admin.apps.clearResolution): Clear an app resolution
///
/// Send it with [`SlackClient::admin_apps_clear_resolution`].
#[doc(alias = "admin.apps.clearResolution")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminAppsClearResolutionRequest {
    /// The id of the app whose resolution you want to clear/undo.
    pub app_id: String,
    /// The workspace to clear the app resolution from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// The enterprise to clear the app resolution from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_id: Option<String>,
}

impl AdminAppsClearResolutionRequest {
    pub fn new(app_id: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            team_id: None,
            enterprise_id: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn enterprise_id(mut self, enterprise_id: impl Into<String>) -> Self {
        self.enterprise_id = Some(enterprise_id.into());
        self
    }
}

impl SlackApiMethod for AdminAppsClearResolutionRequest {
    const METHOD: &'static str = "admin.apps.clearResolution";
    type Response = AdminAppsClearResolutionResponse;
}

/// Successful response of the Slack Web API method [`admin.apps.clearResolution`](https://docs.slack.dev/reference/methods/admin.apps.clearResolution).
#[doc(alias = "admin.apps.clearResolution")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsClearResolutionResponse {
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

/// Arguments for the Slack Web API method [`admin.apps.config.lookup`](https://docs.slack.dev/reference/methods/admin.apps.config.lookup): Look up the app config for connectors by their IDs
///
/// Send it with [`SlackClient::admin_apps_config_lookup`].
#[doc(alias = "admin.apps.config.lookup")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminAppsConfigLookupRequest {
    /// An array of app IDs to get app configs for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_ids: Option<Vec<String>>,
    /// return apps with the corresponding rich link preview layouts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_link_preview_types: Option<Vec<String>>,
}

impl AdminAppsConfigLookupRequest {
    pub fn new() -> Self {
        Self {
            app_ids: None,
            rich_link_preview_types: None,
        }
    }

    pub fn app_ids(mut self, app_ids: Vec<String>) -> Self {
        self.app_ids = Some(app_ids);
        self
    }

    pub fn rich_link_preview_types(mut self, rich_link_preview_types: Vec<String>) -> Self {
        self.rich_link_preview_types = Some(rich_link_preview_types);
        self
    }
}

impl SlackApiMethod for AdminAppsConfigLookupRequest {
    const METHOD: &'static str = "admin.apps.config.lookup";
    type Response = AdminAppsConfigLookupResponse;
}

/// Successful response of the Slack Web API method [`admin.apps.config.lookup`](https://docs.slack.dev/reference/methods/admin.apps.config.lookup).
#[doc(alias = "admin.apps.config.lookup")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsConfigLookupResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub configs: Vec<AdminAppsConfigLookupResponseConfigs>,
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
pub struct AdminAppsConfigLookupResponseConfigs {
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
    pub workflow_auth_strategy: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub rich_link_preview_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub domain_restrictions: Option<AdminAppsConfigLookupResponseConfigsDomainRestrictions>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsConfigLookupResponseConfigsDomainRestrictions {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub emails: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub urls: Vec<String>,
}

/// Arguments for the Slack Web API method [`admin.apps.config.set`](https://docs.slack.dev/reference/methods/admin.apps.config.set): Set the app config for a connector
///
/// Send it with [`SlackClient::admin_apps_config_set`].
#[doc(alias = "admin.apps.config.set")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminAppsConfigSetRequest {
    /// The encoded app ID to set the app config for
    pub app_id: String,
    /// The workflow auth permission. Can be one of `builder_choice` or `end_user_only`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_auth_strategy: Option<String>,
    /// Indicates the app-level override for rich link preview. Unsupported for free teams.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_link_preview_type: Option<String>,
    /// Domain restrictions for the app. Should be an object with two properties: `urls` and `emails`. Each is an array of strings, and each sets the allowed URLs and emails for connector authorization, respectively.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_restrictions: Option<serde_json::Value>,
}

impl AdminAppsConfigSetRequest {
    pub fn new(app_id: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            workflow_auth_strategy: None,
            rich_link_preview_type: None,
            domain_restrictions: None,
        }
    }

    pub fn workflow_auth_strategy(mut self, workflow_auth_strategy: impl Into<String>) -> Self {
        self.workflow_auth_strategy = Some(workflow_auth_strategy.into());
        self
    }

    pub fn rich_link_preview_type(mut self, rich_link_preview_type: impl Into<String>) -> Self {
        self.rich_link_preview_type = Some(rich_link_preview_type.into());
        self
    }

    pub fn domain_restrictions(mut self, domain_restrictions: serde_json::Value) -> Self {
        self.domain_restrictions = Some(domain_restrictions);
        self
    }
}

impl SlackApiMethod for AdminAppsConfigSetRequest {
    const METHOD: &'static str = "admin.apps.config.set";
    type Response = AdminAppsConfigSetResponse;
}

/// Successful response of the Slack Web API method [`admin.apps.config.set`](https://docs.slack.dev/reference/methods/admin.apps.config.set).
#[doc(alias = "admin.apps.config.set")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsConfigSetResponse {
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

/// Arguments for the Slack Web API method [`admin.apps.mcp.servers.list`](https://docs.slack.dev/reference/methods/admin.apps.mcp.servers.list): List third-party app MCP servers approved for an organization.
///
/// Send it with [`SlackClient::admin_apps_mcp_servers_list`].
#[doc(alias = "admin.apps.mcp.servers.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminAppsMcpServersListRequest {
    /// The maximum number of items to return. Must be between 1 - 1000 both inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AdminAppsMcpServersListRequest {
    pub fn new() -> Self {
        Self {
            limit: None,
            cursor: None,
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
}

impl SlackApiMethod for AdminAppsMcpServersListRequest {
    const METHOD: &'static str = "admin.apps.mcp.servers.list";
    type Response = AdminAppsMcpServersListResponse;
}

impl CursorPaginated for AdminAppsMcpServersListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminAppsMcpServersListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.apps.mcp.servers.list`](https://docs.slack.dev/reference/methods/admin.apps.mcp.servers.list).
#[doc(alias = "admin.apps.mcp.servers.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsMcpServersListResponse {
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

/// Arguments for the Slack Web API method [`admin.apps.mcp.servers.permissions.list`](https://docs.slack.dev/reference/methods/admin.apps.mcp.servers.permissions.list): List MCP servers for an app with their access control permissions.
///
/// Send it with [`SlackClient::admin_apps_mcp_servers_permissions_list`].
#[doc(alias = "admin.apps.mcp.servers.permissions.list")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminAppsMcpServersPermissionsListRequest {
    /// Encoded ID of the app
    pub app_id: String,
}

impl AdminAppsMcpServersPermissionsListRequest {
    pub fn new(app_id: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
        }
    }
}

impl SlackApiMethod for AdminAppsMcpServersPermissionsListRequest {
    const METHOD: &'static str = "admin.apps.mcp.servers.permissions.list";
    type Response = AdminAppsMcpServersPermissionsListResponse;
}

/// Successful response of the Slack Web API method [`admin.apps.mcp.servers.permissions.list`](https://docs.slack.dev/reference/methods/admin.apps.mcp.servers.permissions.list).
#[doc(alias = "admin.apps.mcp.servers.permissions.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsMcpServersPermissionsListResponse {
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

/// Arguments for the Slack Web API method [`admin.apps.mcp.servers.permissions.set`](https://docs.slack.dev/reference/methods/admin.apps.mcp.servers.permissions.set): Set the access control permission for who can use an MCP server.
///
/// Send it with [`SlackClient::admin_apps_mcp_servers_permissions_set`].
#[doc(alias = "admin.apps.mcp.servers.permissions.set")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminAppsMcpServersPermissionsSetRequest {
    /// Encoded ID of the app
    pub app_id: String,
    /// Encoded ID of the MCP server
    pub server_id: String,
    /// The type of permission that defines who can use this MCP server
    pub permission_type: String,
    /// List of user IDs to set for named\_entities or named\_entities\_exclude
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub user_ids: Option<Vec<String>>,
    /// List of encoded usergroup IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usergroup_ids: Option<Vec<String>>,
}

impl AdminAppsMcpServersPermissionsSetRequest {
    pub fn new(
        app_id: impl Into<String>,
        server_id: impl Into<String>,
        permission_type: impl Into<String>,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            server_id: server_id.into(),
            permission_type: permission_type.into(),
            user_ids: None,
            usergroup_ids: None,
        }
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    pub fn usergroup_ids(mut self, usergroup_ids: Vec<String>) -> Self {
        self.usergroup_ids = Some(usergroup_ids);
        self
    }
}

impl SlackApiMethod for AdminAppsMcpServersPermissionsSetRequest {
    const METHOD: &'static str = "admin.apps.mcp.servers.permissions.set";
    type Response = AdminAppsMcpServersPermissionsSetResponse;
}

/// Successful response of the Slack Web API method [`admin.apps.mcp.servers.permissions.set`](https://docs.slack.dev/reference/methods/admin.apps.mcp.servers.permissions.set).
#[doc(alias = "admin.apps.mcp.servers.permissions.set")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsMcpServersPermissionsSetResponse {
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

/// Arguments for the Slack Web API method [`admin.apps.permissions.add`](https://docs.slack.dev/reference/methods/admin.apps.permissions.add): Grant permission for entities to access an app that has its permission type set to named_entities.
///
/// Send it with [`SlackClient::admin_apps_permissions_add`].
#[doc(alias = "admin.apps.permissions.add")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminAppsPermissionsAddRequest {
    /// Encoded ID of the app
    pub app_id: String,
    /// List of user IDs to allow for named\_entities visibility
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub user_ids: Option<Vec<String>>,
    /// List of encoded usergroup IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usergroup_ids: Option<Vec<String>>,
    /// List of encoded channel IDs to add to the channel restriction list. Interpretation depends on the app's `channel_restriction_mode`, which is configured via the `admin.apps.permissions.set` method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
}

impl AdminAppsPermissionsAddRequest {
    pub fn new(app_id: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            user_ids: None,
            usergroup_ids: None,
            channel_ids: None,
        }
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    pub fn usergroup_ids(mut self, usergroup_ids: Vec<String>) -> Self {
        self.usergroup_ids = Some(usergroup_ids);
        self
    }

    pub fn channel_ids(mut self, channel_ids: Vec<String>) -> Self {
        self.channel_ids = Some(channel_ids);
        self
    }
}

impl SlackApiMethod for AdminAppsPermissionsAddRequest {
    const METHOD: &'static str = "admin.apps.permissions.add";
    type Response = AdminAppsPermissionsAddResponse;
}

/// Successful response of the Slack Web API method [`admin.apps.permissions.add`](https://docs.slack.dev/reference/methods/admin.apps.permissions.add).
#[doc(alias = "admin.apps.permissions.add")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsPermissionsAddResponse {
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

/// Arguments for the Slack Web API method [`admin.apps.permissions.list`](https://docs.slack.dev/reference/methods/admin.apps.permissions.list): Returns the permission type of an app and the entities that have been granted access.
///
/// Send it with [`SlackClient::admin_apps_permissions_list`].
#[doc(alias = "admin.apps.permissions.list")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminAppsPermissionsListRequest {
    /// Encoded ID of the app
    pub app_id: String,
}

impl AdminAppsPermissionsListRequest {
    pub fn new(app_id: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
        }
    }
}

impl SlackApiMethod for AdminAppsPermissionsListRequest {
    const METHOD: &'static str = "admin.apps.permissions.list";
    type Response = AdminAppsPermissionsListResponse;
}

/// Successful response of the Slack Web API method [`admin.apps.permissions.list`](https://docs.slack.dev/reference/methods/admin.apps.permissions.list).
#[doc(alias = "admin.apps.permissions.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsPermissionsListResponse {
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

/// Arguments for the Slack Web API method [`admin.apps.permissions.remove`](https://docs.slack.dev/reference/methods/admin.apps.permissions.remove): Revoke an entity's access to an app that has its permission type set to named_entities.
///
/// Send it with [`SlackClient::admin_apps_permissions_remove`].
#[doc(alias = "admin.apps.permissions.remove")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminAppsPermissionsRemoveRequest {
    /// Encoded ID of the app
    pub app_id: String,
    /// List of user IDs whose named\_entities access will be revoked
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub user_ids: Option<Vec<String>>,
    /// List of encoded usergroup IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usergroup_ids: Option<Vec<String>>,
    /// List of encoded channel IDs to remove from the channel restriction list. Interpretation depends on the app's `channel_restriction_mode`, which is configured via the `admin.apps.permissions.set` method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
}

impl AdminAppsPermissionsRemoveRequest {
    pub fn new(app_id: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            user_ids: None,
            usergroup_ids: None,
            channel_ids: None,
        }
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    pub fn usergroup_ids(mut self, usergroup_ids: Vec<String>) -> Self {
        self.usergroup_ids = Some(usergroup_ids);
        self
    }

    pub fn channel_ids(mut self, channel_ids: Vec<String>) -> Self {
        self.channel_ids = Some(channel_ids);
        self
    }
}

impl SlackApiMethod for AdminAppsPermissionsRemoveRequest {
    const METHOD: &'static str = "admin.apps.permissions.remove";
    type Response = AdminAppsPermissionsRemoveResponse;
}

/// Successful response of the Slack Web API method [`admin.apps.permissions.remove`](https://docs.slack.dev/reference/methods/admin.apps.permissions.remove).
#[doc(alias = "admin.apps.permissions.remove")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsPermissionsRemoveResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_restriction_mode: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub channel_ids: Vec<String>,
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

/// Arguments for the Slack Web API method [`admin.apps.permissions.set`](https://docs.slack.dev/reference/methods/admin.apps.permissions.set): Set the permission type for who can access an app.
///
/// Send it with [`SlackClient::admin_apps_permissions_set`].
#[doc(alias = "admin.apps.permissions.set")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminAppsPermissionsSetRequest {
    /// Encoded ID of the app
    pub app_id: String,
    /// The type of permission that defines who can access the app
    pub permission_type: String,
    /// List of user IDs to allow for named\_entities visibility
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub user_ids: Option<Vec<String>>,
    /// List of encoded usergroup IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usergroup_ids: Option<Vec<String>>,
    /// The mode that defines where the app can be used in channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_restriction_mode: Option<String>,
    /// List of encoded channel IDs for channel restrictions. Semantics depend on channel\_restriction\_mode: allowlist for specific\_channels, exclusion list for all\_channels\_except
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
}

impl AdminAppsPermissionsSetRequest {
    pub fn new(app_id: impl Into<String>, permission_type: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            permission_type: permission_type.into(),
            user_ids: None,
            usergroup_ids: None,
            channel_restriction_mode: None,
            channel_ids: None,
        }
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    pub fn usergroup_ids(mut self, usergroup_ids: Vec<String>) -> Self {
        self.usergroup_ids = Some(usergroup_ids);
        self
    }

    pub fn channel_restriction_mode(mut self, channel_restriction_mode: impl Into<String>) -> Self {
        self.channel_restriction_mode = Some(channel_restriction_mode.into());
        self
    }

    pub fn channel_ids(mut self, channel_ids: Vec<String>) -> Self {
        self.channel_ids = Some(channel_ids);
        self
    }
}

impl SlackApiMethod for AdminAppsPermissionsSetRequest {
    const METHOD: &'static str = "admin.apps.permissions.set";
    type Response = AdminAppsPermissionsSetResponse;
}

/// Successful response of the Slack Web API method [`admin.apps.permissions.set`](https://docs.slack.dev/reference/methods/admin.apps.permissions.set).
#[doc(alias = "admin.apps.permissions.set")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsPermissionsSetResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_restriction_mode: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub channel_ids: Vec<String>,
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

/// Arguments for the Slack Web API method [`admin.apps.requests.cancel`](https://docs.slack.dev/reference/methods/admin.apps.requests.cancel): Cancel app request for team
///
/// Send it with [`SlackClient::admin_apps_requests_cancel`].
#[doc(alias = "admin.apps.requests.cancel")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminAppsRequestsCancelRequest {
    /// The id of the request to cancel.
    pub request_id: String,
    /// The ID of the workspace where this request belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// The ID of the enterprise where this request belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_id: Option<String>,
}

impl AdminAppsRequestsCancelRequest {
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            team_id: None,
            enterprise_id: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn enterprise_id(mut self, enterprise_id: impl Into<String>) -> Self {
        self.enterprise_id = Some(enterprise_id.into());
        self
    }
}

impl SlackApiMethod for AdminAppsRequestsCancelRequest {
    const METHOD: &'static str = "admin.apps.requests.cancel";
    type Response = AdminAppsRequestsCancelResponse;
}

/// Successful response of the Slack Web API method [`admin.apps.requests.cancel`](https://docs.slack.dev/reference/methods/admin.apps.requests.cancel).
#[doc(alias = "admin.apps.requests.cancel")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsRequestsCancelResponse {
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

/// Arguments for the Slack Web API method [`admin.apps.requests.list`](https://docs.slack.dev/reference/methods/admin.apps.requests.list): List app requests for a team/workspace.
///
/// Send it with [`SlackClient::admin_apps_requests_list`].
#[doc(alias = "admin.apps.requests.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminAppsRequestsListRequest {
    /// The maximum number of items to return. Must be between 1 - 1000 both inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_id: Option<String>,
    /// Include requests for certified apps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certified: Option<bool>,
}

impl AdminAppsRequestsListRequest {
    pub fn new() -> Self {
        Self {
            limit: None,
            cursor: None,
            team_id: None,
            enterprise_id: None,
            certified: None,
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

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn enterprise_id(mut self, enterprise_id: impl Into<String>) -> Self {
        self.enterprise_id = Some(enterprise_id.into());
        self
    }

    pub fn certified(mut self, certified: bool) -> Self {
        self.certified = Some(certified);
        self
    }
}

impl SlackApiMethod for AdminAppsRequestsListRequest {
    const METHOD: &'static str = "admin.apps.requests.list";
    type Response = AdminAppsRequestsListResponse;
}

impl CursorPaginated for AdminAppsRequestsListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminAppsRequestsListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.apps.requests.list`](https://docs.slack.dev/reference/methods/admin.apps.requests.list).
#[doc(alias = "admin.apps.requests.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsRequestsListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub app_requests: Vec<AdminAppsRequestsListResponseAppRequests>,
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
pub struct AdminAppsRequestsListResponseAppRequests {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub app: Option<AdminAppsRequestsListResponseAppRequestsApp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_resolution: Option<serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<AdminAppsRequestsListResponseAppRequestsUser>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub team: Option<Team>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<AdminAppsRequestsListResponseAppRequestsScopes>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub message: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_user_app_collaborator: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsRequestsListResponseAppRequestsApp {
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
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub help_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub privacy_policy_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_homepage_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_directory_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_app_directory_approved: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_internal: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub developer_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub socket_mode_enabled: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub icons: Option<Icons>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_info: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsRequestsListResponseAppRequestsUser {
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
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsRequestsListResponseAppRequestsScopes {
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
    pub description: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_sensitive: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub token_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_optional: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_approved: Option<bool>,
}

/// Arguments for the Slack Web API method [`admin.apps.restrict`](https://docs.slack.dev/reference/methods/admin.apps.restrict): Restrict an app for installation on a workspace.
///
/// Send it with [`SlackClient::admin_apps_restrict`].
#[doc(alias = "admin.apps.restrict")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminAppsRestrictRequest {
    /// The id of the app to restrict.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// The id of the request to restrict.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The ID of the workspace to approve the app on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// The ID of the enterprise to approve the app on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_id: Option<String>,
}

impl AdminAppsRestrictRequest {
    pub fn new() -> Self {
        Self {
            app_id: None,
            request_id: None,
            team_id: None,
            enterprise_id: None,
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    pub fn request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn enterprise_id(mut self, enterprise_id: impl Into<String>) -> Self {
        self.enterprise_id = Some(enterprise_id.into());
        self
    }
}

impl SlackApiMethod for AdminAppsRestrictRequest {
    const METHOD: &'static str = "admin.apps.restrict";
    type Response = AdminAppsRestrictResponse;
}

/// Successful response of the Slack Web API method [`admin.apps.restrict`](https://docs.slack.dev/reference/methods/admin.apps.restrict).
#[doc(alias = "admin.apps.restrict")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsRestrictResponse {
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

/// Arguments for the Slack Web API method [`admin.apps.restricted.list`](https://docs.slack.dev/reference/methods/admin.apps.restricted.list): List restricted apps for an org or workspace.
///
/// Send it with [`SlackClient::admin_apps_restricted_list`].
#[doc(alias = "admin.apps.restricted.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminAppsRestrictedListRequest {
    /// The maximum number of items to return. Must be between 1 - 1000 both inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_id: Option<String>,
    /// Limit the results to only include certified apps. When false, no certified apps will appear in the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certified: Option<bool>,
}

impl AdminAppsRestrictedListRequest {
    pub fn new() -> Self {
        Self {
            limit: None,
            cursor: None,
            team_id: None,
            enterprise_id: None,
            certified: None,
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

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn enterprise_id(mut self, enterprise_id: impl Into<String>) -> Self {
        self.enterprise_id = Some(enterprise_id.into());
        self
    }

    pub fn certified(mut self, certified: bool) -> Self {
        self.certified = Some(certified);
        self
    }
}

impl SlackApiMethod for AdminAppsRestrictedListRequest {
    const METHOD: &'static str = "admin.apps.restricted.list";
    type Response = AdminAppsRestrictedListResponse;
}

impl CursorPaginated for AdminAppsRestrictedListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminAppsRestrictedListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.apps.restricted.list`](https://docs.slack.dev/reference/methods/admin.apps.restricted.list).
#[doc(alias = "admin.apps.restricted.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsRestrictedListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub restricted_apps: Vec<AdminAppsRestrictedListResponseRestrictedApps>,
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
pub struct AdminAppsRestrictedListResponseRestrictedApps {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub app: Option<AdminAppsRestrictedListResponseRestrictedAppsApp>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<AdminAppsRestrictedListResponseRestrictedAppsScopes>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_updated: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_resolved_by: Option<AdminAppsRestrictedListResponseRestrictedAppsLastResolvedBy>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsRestrictedListResponseRestrictedAppsApp {
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
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub help_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub privacy_policy_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_homepage_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_directory_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_app_directory_approved: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_internal: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub developer_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub socket_mode_enabled: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub icons: Option<Icons>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_info: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsRestrictedListResponseRestrictedAppsScopes {
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
    pub description: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_sensitive: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub token_type: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsRestrictedListResponseRestrictedAppsLastResolvedBy {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_type: Option<String>,
}

/// Arguments for the Slack Web API method [`admin.apps.uninstall`](https://docs.slack.dev/reference/methods/admin.apps.uninstall): Uninstall an app from one or many workspaces, or an entire enterprise organization.
///
/// Send it with [`SlackClient::admin_apps_uninstall`].
#[doc(alias = "admin.apps.uninstall")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminAppsUninstallRequest {
    /// The ID of the app to uninstall.
    pub app_id: String,
    /// IDs of the teams to uninstall from (max 100). With an org-level token, this or `enterprise_id` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<String>,
    /// The enterprise to completely uninstall the application from (across all workspaces). With an org-level token, this or `team_ids` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_id: Option<String>,
}

impl AdminAppsUninstallRequest {
    pub fn new(app_id: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            team_ids: None,
            enterprise_id: None,
        }
    }

    pub fn team_ids(mut self, team_ids: impl Into<String>) -> Self {
        self.team_ids = Some(team_ids.into());
        self
    }

    pub fn enterprise_id(mut self, enterprise_id: impl Into<String>) -> Self {
        self.enterprise_id = Some(enterprise_id.into());
        self
    }
}

impl SlackApiMethod for AdminAppsUninstallRequest {
    const METHOD: &'static str = "admin.apps.uninstall";
    type Response = AdminAppsUninstallResponse;
}

/// Successful response of the Slack Web API method [`admin.apps.uninstall`](https://docs.slack.dev/reference/methods/admin.apps.uninstall).
#[doc(alias = "admin.apps.uninstall")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAppsUninstallResponse {
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
    /// Calls the Slack Web API method [`admin.apps.activities.list`](https://docs.slack.dev/reference/methods/admin.apps.activities.list): Get logs for a specified team/org
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.app_activities:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.activities.list")]
    pub async fn admin_apps_activities_list(
        &self,
        request: &AdminAppsActivitiesListRequest,
    ) -> Result<AdminAppsActivitiesListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.apps.approve`](https://docs.slack.dev/reference/methods/admin.apps.approve): Approve an app for installation on a workspace.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.apps:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.approve")]
    pub async fn admin_apps_approve(
        &self,
        request: &AdminAppsApproveRequest,
    ) -> Result<AdminAppsApproveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.apps.approved.list`](https://docs.slack.dev/reference/methods/admin.apps.approved.list): List approved apps for an org or workspace.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.apps:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.approved.list")]
    pub async fn admin_apps_approved_list(
        &self,
        request: &AdminAppsApprovedListRequest,
    ) -> Result<AdminAppsApprovedListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.apps.clearResolution`](https://docs.slack.dev/reference/methods/admin.apps.clearResolution): Clear an app resolution
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.apps:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.clearResolution")]
    pub async fn admin_apps_clear_resolution(
        &self,
        request: &AdminAppsClearResolutionRequest,
    ) -> Result<AdminAppsClearResolutionResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.apps.config.lookup`](https://docs.slack.dev/reference/methods/admin.apps.config.lookup): Look up the app config for connectors by their IDs
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.apps:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.config.lookup")]
    pub async fn admin_apps_config_lookup(
        &self,
        request: &AdminAppsConfigLookupRequest,
    ) -> Result<AdminAppsConfigLookupResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.apps.config.set`](https://docs.slack.dev/reference/methods/admin.apps.config.set): Set the app config for a connector
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.apps:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.config.set")]
    pub async fn admin_apps_config_set(
        &self,
        request: &AdminAppsConfigSetRequest,
    ) -> Result<AdminAppsConfigSetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.apps.mcp.servers.list`](https://docs.slack.dev/reference/methods/admin.apps.mcp.servers.list): List third-party app MCP servers approved for an organization.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.apps:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.mcp.servers.list")]
    pub async fn admin_apps_mcp_servers_list(
        &self,
        request: &AdminAppsMcpServersListRequest,
    ) -> Result<AdminAppsMcpServersListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.apps.mcp.servers.permissions.list`](https://docs.slack.dev/reference/methods/admin.apps.mcp.servers.permissions.list): List MCP servers for an app with their access control permissions.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.apps:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.mcp.servers.permissions.list")]
    pub async fn admin_apps_mcp_servers_permissions_list(
        &self,
        request: &AdminAppsMcpServersPermissionsListRequest,
    ) -> Result<AdminAppsMcpServersPermissionsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.apps.mcp.servers.permissions.set`](https://docs.slack.dev/reference/methods/admin.apps.mcp.servers.permissions.set): Set the access control permission for who can use an MCP server.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.apps:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.mcp.servers.permissions.set")]
    pub async fn admin_apps_mcp_servers_permissions_set(
        &self,
        request: &AdminAppsMcpServersPermissionsSetRequest,
    ) -> Result<AdminAppsMcpServersPermissionsSetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.apps.permissions.add`](https://docs.slack.dev/reference/methods/admin.apps.permissions.add): Grant permission for entities to access an app that has its permission type set to named_entities.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.apps:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.permissions.add")]
    pub async fn admin_apps_permissions_add(
        &self,
        request: &AdminAppsPermissionsAddRequest,
    ) -> Result<AdminAppsPermissionsAddResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.apps.permissions.list`](https://docs.slack.dev/reference/methods/admin.apps.permissions.list): Returns the permission type of an app and the entities that have been granted access.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.apps:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.permissions.list")]
    pub async fn admin_apps_permissions_list(
        &self,
        request: &AdminAppsPermissionsListRequest,
    ) -> Result<AdminAppsPermissionsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.apps.permissions.remove`](https://docs.slack.dev/reference/methods/admin.apps.permissions.remove): Revoke an entity's access to an app that has its permission type set to named_entities.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.apps:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.permissions.remove")]
    pub async fn admin_apps_permissions_remove(
        &self,
        request: &AdminAppsPermissionsRemoveRequest,
    ) -> Result<AdminAppsPermissionsRemoveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.apps.permissions.set`](https://docs.slack.dev/reference/methods/admin.apps.permissions.set): Set the permission type for who can access an app.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.apps:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.permissions.set")]
    pub async fn admin_apps_permissions_set(
        &self,
        request: &AdminAppsPermissionsSetRequest,
    ) -> Result<AdminAppsPermissionsSetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.apps.requests.cancel`](https://docs.slack.dev/reference/methods/admin.apps.requests.cancel): Cancel app request for team
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.apps:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.requests.cancel")]
    pub async fn admin_apps_requests_cancel(
        &self,
        request: &AdminAppsRequestsCancelRequest,
    ) -> Result<AdminAppsRequestsCancelResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.apps.requests.list`](https://docs.slack.dev/reference/methods/admin.apps.requests.list): List app requests for a team/workspace.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.apps:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.requests.list")]
    pub async fn admin_apps_requests_list(
        &self,
        request: &AdminAppsRequestsListRequest,
    ) -> Result<AdminAppsRequestsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.apps.restrict`](https://docs.slack.dev/reference/methods/admin.apps.restrict): Restrict an app for installation on a workspace.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.apps:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.restrict")]
    pub async fn admin_apps_restrict(
        &self,
        request: &AdminAppsRestrictRequest,
    ) -> Result<AdminAppsRestrictResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.apps.restricted.list`](https://docs.slack.dev/reference/methods/admin.apps.restricted.list): List restricted apps for an org or workspace.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.apps:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.restricted.list")]
    pub async fn admin_apps_restricted_list(
        &self,
        request: &AdminAppsRestrictedListRequest,
    ) -> Result<AdminAppsRestrictedListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.apps.uninstall`](https://docs.slack.dev/reference/methods/admin.apps.uninstall): Uninstall an app from one or many workspaces, or an entire enterprise organization.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.apps:write`
    ///
    /// Rate limit: Tier 1 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.apps.uninstall")]
    pub async fn admin_apps_uninstall(
        &self,
        request: &AdminAppsUninstallRequest,
    ) -> Result<AdminAppsUninstallResponse, SlackError> {
        self.call(request).await
    }
}
