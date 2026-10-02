// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`admin.functions.list`](https://docs.slack.dev/reference/methods/admin.functions.list): Look up functions by a set of apps.
///
/// Send it with [`SlackClient::admin_functions_list`].
#[doc(alias = "admin.functions.list")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminFunctionsListRequest {
    /// Comma-separated array of app IDs to get functions for; max 50.
    pub app_ids: Vec<String>,
    /// The team context to retrieve functions from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Whether to also include functions that are not yet distributed to any users in the function list. This is needed for admins that are approving an app request and will only work if the team owns the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_non_distributed_functions: Option<bool>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The number of results that will be returned by the API on each invocation. Must be between 1 and 1000, both inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl AdminFunctionsListRequest {
    pub fn new(app_ids: Vec<String>) -> Self {
        Self {
            app_ids,
            team_id: None,
            include_non_distributed_functions: None,
            cursor: None,
            limit: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn include_non_distributed_functions(
        mut self,
        include_non_distributed_functions: bool,
    ) -> Self {
        self.include_non_distributed_functions = Some(include_non_distributed_functions);
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

impl SlackApiMethod for AdminFunctionsListRequest {
    const METHOD: &'static str = "admin.functions.list";
    type Response = AdminFunctionsListResponse;
}

impl CursorPaginated for AdminFunctionsListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminFunctionsListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.functions.list`](https://docs.slack.dev/reference/methods/admin.functions.list).
#[doc(alias = "admin.functions.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminFunctionsListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub functions: Vec<AdminFunctionsListResponseFunctions>,
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
pub struct AdminFunctionsListResponseFunctions {
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
    pub callback_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
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
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub input_parameters: Vec<AdminFunctionsListResponseFunctionsInputParameters>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub output_parameters: Vec<AdminFunctionsListResponseFunctionsOutputParameters>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_updated: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_deleted: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminFunctionsListResponseFunctionsInputParameters {
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
    pub title: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_required: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminFunctionsListResponseFunctionsOutputParameters {
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
    pub title: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_required: Option<bool>,
}

/// Arguments for the Slack Web API method [`admin.functions.permissions.lookup`](https://docs.slack.dev/reference/methods/admin.functions.permissions.lookup): Lookup the visibility of multiple Slack functions and include the users if it is limited to particular named entities.
///
/// Send it with [`SlackClient::admin_functions_permissions_lookup`].
#[doc(alias = "admin.functions.permissions.lookup")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminFunctionsPermissionsLookupRequest {
    /// An array of function IDs to get permissions for
    pub function_ids: Vec<String>,
}

impl AdminFunctionsPermissionsLookupRequest {
    pub fn new(function_ids: Vec<String>) -> Self {
        Self { function_ids }
    }
}

impl SlackApiMethod for AdminFunctionsPermissionsLookupRequest {
    const METHOD: &'static str = "admin.functions.permissions.lookup";
    type Response = AdminFunctionsPermissionsLookupResponse;
}

/// Successful response of the Slack Web API method [`admin.functions.permissions.lookup`](https://docs.slack.dev/reference/methods/admin.functions.permissions.lookup).
#[doc(alias = "admin.functions.permissions.lookup")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminFunctionsPermissionsLookupResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub errors: Option<serde_json::Map<String, serde_json::Value>>,
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

/// Arguments for the Slack Web API method [`admin.functions.permissions.set`](https://docs.slack.dev/reference/methods/admin.functions.permissions.set): Set the visibility of a Slack function and define the users or workspaces if it is set to named_entities.
///
/// Send it with [`SlackClient::admin_functions_permissions_set`].
#[doc(alias = "admin.functions.permissions.set")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminFunctionsPermissionsSetRequest {
    /// The function ID to set permissions for.
    pub function_id: String,
    /// The function visibility.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    /// List of user IDs to allow for `named_entities` visibility.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub user_ids: Option<Vec<String>>,
    /// Array of permissions for the function.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub permissions: Option<Vec<serde_json::Value>>,
}

impl AdminFunctionsPermissionsSetRequest {
    pub fn new(function_id: impl Into<String>) -> Self {
        Self {
            function_id: function_id.into(),
            visibility: None,
            user_ids: None,
            permissions: None,
        }
    }

    pub fn visibility(mut self, visibility: impl Into<String>) -> Self {
        self.visibility = Some(visibility.into());
        self
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    pub fn permissions(mut self, permissions: Vec<serde_json::Value>) -> Self {
        self.permissions = Some(permissions);
        self
    }
}

impl SlackApiMethod for AdminFunctionsPermissionsSetRequest {
    const METHOD: &'static str = "admin.functions.permissions.set";
    type Response = AdminFunctionsPermissionsSetResponse;
}

/// Successful response of the Slack Web API method [`admin.functions.permissions.set`](https://docs.slack.dev/reference/methods/admin.functions.permissions.set).
#[doc(alias = "admin.functions.permissions.set")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminFunctionsPermissionsSetResponse {
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
    /// Calls the Slack Web API method [`admin.functions.list`](https://docs.slack.dev/reference/methods/admin.functions.list): Look up functions by a set of apps.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.workflows:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.functions.list")]
    pub async fn admin_functions_list(
        &self,
        request: &AdminFunctionsListRequest,
    ) -> Result<AdminFunctionsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.functions.permissions.lookup`](https://docs.slack.dev/reference/methods/admin.functions.permissions.lookup): Lookup the visibility of multiple Slack functions and include the users if it is limited to particular named entities.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.workflows:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.functions.permissions.lookup")]
    pub async fn admin_functions_permissions_lookup(
        &self,
        request: &AdminFunctionsPermissionsLookupRequest,
    ) -> Result<AdminFunctionsPermissionsLookupResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.functions.permissions.set`](https://docs.slack.dev/reference/methods/admin.functions.permissions.set): Set the visibility of a Slack function and define the users or workspaces if it is set to named_entities.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.workflows:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.functions.permissions.set")]
    pub async fn admin_functions_permissions_set(
        &self,
        request: &AdminFunctionsPermissionsSetRequest,
    ) -> Result<AdminFunctionsPermissionsSetResponse, SlackError> {
        self.call(request).await
    }
}
