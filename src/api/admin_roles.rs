// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`admin.roles.addAssignments`](https://docs.slack.dev/reference/methods/admin.roles.addAssignments): Adds members to the specified role with the specified scopes
///
/// Send it with [`SlackClient::admin_roles_add_assignments`].
#[doc(alias = "admin.roles.addAssignments")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminRolesAddAssignmentsRequest {
    /// ID of the role to which users will be assigned
    pub role_id: String,
    /// List of the entity IDs for which roles will be assigned. These can be Org IDs, Team IDs or Channel IDs
    #[serde(serialize_with = "crate::form::as_json")]
    pub entity_ids: Vec<String>,
    /// List of IDs from the users to be added to the given role
    #[serde(serialize_with = "crate::form::as_json")]
    pub user_ids: Vec<String>,
}

impl AdminRolesAddAssignmentsRequest {
    pub fn new(role_id: impl Into<String>, entity_ids: Vec<String>, user_ids: Vec<String>) -> Self {
        Self {
            role_id: role_id.into(),
            entity_ids,
            user_ids,
        }
    }
}

impl SlackApiMethod for AdminRolesAddAssignmentsRequest {
    const METHOD: &'static str = "admin.roles.addAssignments";
    type Response = AdminRolesAddAssignmentsResponse;
}

/// Successful response of the Slack Web API method [`admin.roles.addAssignments`](https://docs.slack.dev/reference/methods/admin.roles.addAssignments).
#[doc(alias = "admin.roles.addAssignments")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminRolesAddAssignmentsResponse {
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

/// Arguments for the Slack Web API method [`admin.roles.listAssignments`](https://docs.slack.dev/reference/methods/admin.roles.listAssignments): Lists assignments for all roles across entities. Options to scope results by any combination of roles or entities
///
/// Send it with [`SlackClient::admin_roles_list_assignments`].
#[doc(alias = "admin.roles.listAssignments")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminRolesListAssignmentsRequest {
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The maximum number of items to return. Must be between 1 - 200 both inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// collection of role ids to scope results by
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub role_ids: Option<Vec<String>>,
    /// The entities for which the roles apply
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub entity_ids: Option<Vec<String>>,
    /// Sort direction. Default is descending on date\_create, can be either ASC or DESC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_dir: Option<String>,
}

impl AdminRolesListAssignmentsRequest {
    pub fn new() -> Self {
        Self {
            cursor: None,
            limit: None,
            role_ids: None,
            entity_ids: None,
            sort_dir: None,
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

    pub fn role_ids(mut self, role_ids: Vec<String>) -> Self {
        self.role_ids = Some(role_ids);
        self
    }

    pub fn entity_ids(mut self, entity_ids: Vec<String>) -> Self {
        self.entity_ids = Some(entity_ids);
        self
    }

    pub fn sort_dir(mut self, sort_dir: impl Into<String>) -> Self {
        self.sort_dir = Some(sort_dir.into());
        self
    }
}

impl SlackApiMethod for AdminRolesListAssignmentsRequest {
    const METHOD: &'static str = "admin.roles.listAssignments";
    type Response = AdminRolesListAssignmentsResponse;
}

impl CursorPaginated for AdminRolesListAssignmentsRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminRolesListAssignmentsResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.roles.listAssignments`](https://docs.slack.dev/reference/methods/admin.roles.listAssignments).
#[doc(alias = "admin.roles.listAssignments")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminRolesListAssignmentsResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub role_assignments: Vec<AdminRolesListAssignmentsResponseRoleAssignments>,
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
pub struct AdminRolesListAssignmentsResponseRoleAssignments {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub role_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub entity_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_create: Option<i64>,
}

/// Arguments for the Slack Web API method [`admin.roles.removeAssignments`](https://docs.slack.dev/reference/methods/admin.roles.removeAssignments): Removes a set of users from a role for the given scopes and entities
///
/// Send it with [`SlackClient::admin_roles_remove_assignments`].
#[doc(alias = "admin.roles.removeAssignments")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminRolesRemoveAssignmentsRequest {
    /// ID of the role to which users will be assigned
    pub role_id: String,
    /// List of the entity IDs for which roles will be revoked. These can be Org IDs, Team IDs or Channel IDs
    #[serde(serialize_with = "crate::form::as_json")]
    pub entity_ids: Vec<String>,
    /// List of IDs of the users whose roles will be revoked
    #[serde(serialize_with = "crate::form::as_json")]
    pub user_ids: Vec<String>,
}

impl AdminRolesRemoveAssignmentsRequest {
    pub fn new(role_id: impl Into<String>, entity_ids: Vec<String>, user_ids: Vec<String>) -> Self {
        Self {
            role_id: role_id.into(),
            entity_ids,
            user_ids,
        }
    }
}

impl SlackApiMethod for AdminRolesRemoveAssignmentsRequest {
    const METHOD: &'static str = "admin.roles.removeAssignments";
    type Response = AdminRolesRemoveAssignmentsResponse;
}

/// Successful response of the Slack Web API method [`admin.roles.removeAssignments`](https://docs.slack.dev/reference/methods/admin.roles.removeAssignments).
#[doc(alias = "admin.roles.removeAssignments")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminRolesRemoveAssignmentsResponse {
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
    /// Calls the Slack Web API method [`admin.roles.addAssignments`](https://docs.slack.dev/reference/methods/admin.roles.addAssignments): Adds members to the specified role with the specified scopes
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.roles:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.roles.addAssignments")]
    pub async fn admin_roles_add_assignments(
        &self,
        request: &AdminRolesAddAssignmentsRequest,
    ) -> Result<AdminRolesAddAssignmentsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.roles.listAssignments`](https://docs.slack.dev/reference/methods/admin.roles.listAssignments): Lists assignments for all roles across entities. Options to scope results by any combination of roles or entities
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.roles:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.roles.listAssignments")]
    pub async fn admin_roles_list_assignments(
        &self,
        request: &AdminRolesListAssignmentsRequest,
    ) -> Result<AdminRolesListAssignmentsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.roles.removeAssignments`](https://docs.slack.dev/reference/methods/admin.roles.removeAssignments): Removes a set of users from a role for the given scopes and entities
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.roles:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.roles.removeAssignments")]
    pub async fn admin_roles_remove_assignments(
        &self,
        request: &AdminRolesRemoveAssignmentsRequest,
    ) -> Result<AdminRolesRemoveAssignmentsResponse, SlackError> {
        self.call(request).await
    }
}
