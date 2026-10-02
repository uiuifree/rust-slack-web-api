// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`workflows.featured.add`](https://docs.slack.dev/reference/methods/workflows.featured.add): Add featured workflows to a channel.
///
/// Send it with [`SlackClient::workflows_featured_add`].
#[doc(alias = "workflows.featured.add")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct WorkflowsFeaturedAddRequest {
    /// Channel to add featured workflow in.
    pub channel_id: String,
    /// Comma-separated array of trigger IDs to add; max 15
    #[serde(serialize_with = "crate::form::as_json")]
    pub trigger_ids: Vec<String>,
}

impl WorkflowsFeaturedAddRequest {
    pub fn new(channel_id: impl Into<String>, trigger_ids: Vec<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
            trigger_ids,
        }
    }
}

impl SlackApiMethod for WorkflowsFeaturedAddRequest {
    const METHOD: &'static str = "workflows.featured.add";
    type Response = WorkflowsFeaturedAddResponse;
}

/// Successful response of the Slack Web API method [`workflows.featured.add`](https://docs.slack.dev/reference/methods/workflows.featured.add).
#[doc(alias = "workflows.featured.add")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkflowsFeaturedAddResponse {
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

/// Arguments for the Slack Web API method [`workflows.featured.list`](https://docs.slack.dev/reference/methods/workflows.featured.list): List the featured workflows for specified channels.
///
/// Send it with [`SlackClient::workflows_featured_list`].
#[doc(alias = "workflows.featured.list")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct WorkflowsFeaturedListRequest {
    /// Comma-separated array of channel IDs to list featured workflows for.
    #[serde(serialize_with = "crate::form::as_json")]
    pub channel_ids: Vec<String>,
}

impl WorkflowsFeaturedListRequest {
    pub fn new(channel_ids: Vec<String>) -> Self {
        Self { channel_ids }
    }
}

impl SlackApiMethod for WorkflowsFeaturedListRequest {
    const METHOD: &'static str = "workflows.featured.list";
    type Response = WorkflowsFeaturedListResponse;
}

/// Successful response of the Slack Web API method [`workflows.featured.list`](https://docs.slack.dev/reference/methods/workflows.featured.list).
#[doc(alias = "workflows.featured.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkflowsFeaturedListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub featured_workflows: Vec<WorkflowsFeaturedListResponseFeaturedWorkflows>,
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
pub struct WorkflowsFeaturedListResponseFeaturedWorkflows {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub triggers: Vec<WorkflowsFeaturedListResponseFeaturedWorkflowsTriggers>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkflowsFeaturedListResponseFeaturedWorkflowsTriggers {
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
    pub title: Option<String>,
}

/// Arguments for the Slack Web API method [`workflows.featured.remove`](https://docs.slack.dev/reference/methods/workflows.featured.remove): Remove featured workflows from a channel.
///
/// Send it with [`SlackClient::workflows_featured_remove`].
#[doc(alias = "workflows.featured.remove")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct WorkflowsFeaturedRemoveRequest {
    /// Channel to remove featured workflow from.
    pub channel_id: String,
    /// Comma-separated array of trigger IDs to remove; max 15
    #[serde(serialize_with = "crate::form::as_json")]
    pub trigger_ids: Vec<String>,
}

impl WorkflowsFeaturedRemoveRequest {
    pub fn new(channel_id: impl Into<String>, trigger_ids: Vec<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
            trigger_ids,
        }
    }
}

impl SlackApiMethod for WorkflowsFeaturedRemoveRequest {
    const METHOD: &'static str = "workflows.featured.remove";
    type Response = WorkflowsFeaturedRemoveResponse;
}

/// Successful response of the Slack Web API method [`workflows.featured.remove`](https://docs.slack.dev/reference/methods/workflows.featured.remove).
#[doc(alias = "workflows.featured.remove")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkflowsFeaturedRemoveResponse {
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

/// Arguments for the Slack Web API method [`workflows.featured.set`](https://docs.slack.dev/reference/methods/workflows.featured.set): Set featured workflows for a channel.
///
/// Send it with [`SlackClient::workflows_featured_set`].
#[doc(alias = "workflows.featured.set")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct WorkflowsFeaturedSetRequest {
    /// Channel to set featured workflows in.
    pub channel_id: String,
    /// Comma-separated array of trigger IDs that will replace any existing featured workflows in the channel; max 15
    #[serde(serialize_with = "crate::form::as_json")]
    pub trigger_ids: Vec<String>,
}

impl WorkflowsFeaturedSetRequest {
    pub fn new(channel_id: impl Into<String>, trigger_ids: Vec<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
            trigger_ids,
        }
    }
}

impl SlackApiMethod for WorkflowsFeaturedSetRequest {
    const METHOD: &'static str = "workflows.featured.set";
    type Response = WorkflowsFeaturedSetResponse;
}

/// Successful response of the Slack Web API method [`workflows.featured.set`](https://docs.slack.dev/reference/methods/workflows.featured.set).
#[doc(alias = "workflows.featured.set")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkflowsFeaturedSetResponse {
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

/// Arguments for the Slack Web API method [`workflows.triggers.permissions.add`](https://docs.slack.dev/reference/methods/workflows.triggers.permissions.add): Allows users to run a trigger that has its permission type set to named_entities
///
/// Send it with [`SlackClient::workflows_triggers_permissions_add`].
#[doc(alias = "workflows.triggers.permissions.add")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct WorkflowsTriggersPermissionsAddRequest {
    /// Encoded ID of the trigger
    pub trigger_id: String,
    /// List of encoded user IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// List of encoded channel IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
    /// List of encoded workspace IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<String>>,
    /// List of encoded organization IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_ids: Option<Vec<String>>,
}

impl WorkflowsTriggersPermissionsAddRequest {
    pub fn new(trigger_id: impl Into<String>) -> Self {
        Self {
            trigger_id: trigger_id.into(),
            user_ids: None,
            channel_ids: None,
            team_ids: None,
            org_ids: None,
        }
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    pub fn channel_ids(mut self, channel_ids: Vec<String>) -> Self {
        self.channel_ids = Some(channel_ids);
        self
    }

    pub fn team_ids(mut self, team_ids: Vec<String>) -> Self {
        self.team_ids = Some(team_ids);
        self
    }

    pub fn org_ids(mut self, org_ids: Vec<String>) -> Self {
        self.org_ids = Some(org_ids);
        self
    }
}

impl SlackApiMethod for WorkflowsTriggersPermissionsAddRequest {
    const METHOD: &'static str = "workflows.triggers.permissions.add";
    type Response = WorkflowsTriggersPermissionsAddResponse;
}

/// Successful response of the Slack Web API method [`workflows.triggers.permissions.add`](https://docs.slack.dev/reference/methods/workflows.triggers.permissions.add).
#[doc(alias = "workflows.triggers.permissions.add")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkflowsTriggersPermissionsAddResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_ids: Vec<String>,
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

/// Arguments for the Slack Web API method [`workflows.triggers.permissions.list`](https://docs.slack.dev/reference/methods/workflows.triggers.permissions.list): Returns the permission type of a trigger and if applicable, includes the entities that have been granted access
///
/// Send it with [`SlackClient::workflows_triggers_permissions_list`].
#[doc(alias = "workflows.triggers.permissions.list")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct WorkflowsTriggersPermissionsListRequest {
    /// Encoded ID of the trigger
    pub trigger_id: String,
}

impl WorkflowsTriggersPermissionsListRequest {
    pub fn new(trigger_id: impl Into<String>) -> Self {
        Self {
            trigger_id: trigger_id.into(),
        }
    }
}

impl SlackApiMethod for WorkflowsTriggersPermissionsListRequest {
    const METHOD: &'static str = "workflows.triggers.permissions.list";
    type Response = WorkflowsTriggersPermissionsListResponse;
}

/// Successful response of the Slack Web API method [`workflows.triggers.permissions.list`](https://docs.slack.dev/reference/methods/workflows.triggers.permissions.list).
#[doc(alias = "workflows.triggers.permissions.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkflowsTriggersPermissionsListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_ids: Vec<String>,
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

/// Arguments for the Slack Web API method [`workflows.triggers.permissions.remove`](https://docs.slack.dev/reference/methods/workflows.triggers.permissions.remove): Revoke an entity's access to a trigger that has its permission type set to named_entities
///
/// Send it with [`SlackClient::workflows_triggers_permissions_remove`].
#[doc(alias = "workflows.triggers.permissions.remove")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct WorkflowsTriggersPermissionsRemoveRequest {
    /// Encoded ID of the trigger
    pub trigger_id: String,
    /// List of encoded user IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// List of encoded channel IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
    /// List of encoded workspace IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<String>>,
    /// List of encoded organization IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_ids: Option<Vec<String>>,
}

impl WorkflowsTriggersPermissionsRemoveRequest {
    pub fn new(trigger_id: impl Into<String>) -> Self {
        Self {
            trigger_id: trigger_id.into(),
            user_ids: None,
            channel_ids: None,
            team_ids: None,
            org_ids: None,
        }
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    pub fn channel_ids(mut self, channel_ids: Vec<String>) -> Self {
        self.channel_ids = Some(channel_ids);
        self
    }

    pub fn team_ids(mut self, team_ids: Vec<String>) -> Self {
        self.team_ids = Some(team_ids);
        self
    }

    pub fn org_ids(mut self, org_ids: Vec<String>) -> Self {
        self.org_ids = Some(org_ids);
        self
    }
}

impl SlackApiMethod for WorkflowsTriggersPermissionsRemoveRequest {
    const METHOD: &'static str = "workflows.triggers.permissions.remove";
    type Response = WorkflowsTriggersPermissionsRemoveResponse;
}

/// Successful response of the Slack Web API method [`workflows.triggers.permissions.remove`](https://docs.slack.dev/reference/methods/workflows.triggers.permissions.remove).
#[doc(alias = "workflows.triggers.permissions.remove")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkflowsTriggersPermissionsRemoveResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_ids: Vec<String>,
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

/// Arguments for the Slack Web API method [`workflows.triggers.permissions.set`](https://docs.slack.dev/reference/methods/workflows.triggers.permissions.set): Set the permission type for who can run a trigger
///
/// Send it with [`SlackClient::workflows_triggers_permissions_set`].
#[doc(alias = "workflows.triggers.permissions.set")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct WorkflowsTriggersPermissionsSetRequest {
    /// Encoded ID of the trigger
    pub trigger_id: String,
    /// The type of permission that defines who can run a trigger
    pub permission_type: String,
    /// List of encoded user IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// List of encoded channel IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
    /// List of encoded workspace IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<String>>,
    /// List of encoded organization IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_ids: Option<Vec<String>>,
}

impl WorkflowsTriggersPermissionsSetRequest {
    pub fn new(trigger_id: impl Into<String>, permission_type: impl Into<String>) -> Self {
        Self {
            trigger_id: trigger_id.into(),
            permission_type: permission_type.into(),
            user_ids: None,
            channel_ids: None,
            team_ids: None,
            org_ids: None,
        }
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    pub fn channel_ids(mut self, channel_ids: Vec<String>) -> Self {
        self.channel_ids = Some(channel_ids);
        self
    }

    pub fn team_ids(mut self, team_ids: Vec<String>) -> Self {
        self.team_ids = Some(team_ids);
        self
    }

    pub fn org_ids(mut self, org_ids: Vec<String>) -> Self {
        self.org_ids = Some(org_ids);
        self
    }
}

impl SlackApiMethod for WorkflowsTriggersPermissionsSetRequest {
    const METHOD: &'static str = "workflows.triggers.permissions.set";
    type Response = WorkflowsTriggersPermissionsSetResponse;
}

/// Successful response of the Slack Web API method [`workflows.triggers.permissions.set`](https://docs.slack.dev/reference/methods/workflows.triggers.permissions.set).
#[doc(alias = "workflows.triggers.permissions.set")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkflowsTriggersPermissionsSetResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_ids: Vec<String>,
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
    /// Calls the Slack Web API method [`workflows.featured.add`](https://docs.slack.dev/reference/methods/workflows.featured.add): Add featured workflows to a channel.
    ///
    /// Required scopes:
    ///
    /// - bot token: `bookmarks:write`
    /// - user token: `bookmarks:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "workflows.featured.add")]
    pub async fn workflows_featured_add(
        &self,
        request: &WorkflowsFeaturedAddRequest,
    ) -> Result<WorkflowsFeaturedAddResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`workflows.featured.list`](https://docs.slack.dev/reference/methods/workflows.featured.list): List the featured workflows for specified channels.
    ///
    /// Required scopes:
    ///
    /// - bot token: `bookmarks:read`
    /// - user token: `bookmarks:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "workflows.featured.list")]
    pub async fn workflows_featured_list(
        &self,
        request: &WorkflowsFeaturedListRequest,
    ) -> Result<WorkflowsFeaturedListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`workflows.featured.remove`](https://docs.slack.dev/reference/methods/workflows.featured.remove): Remove featured workflows from a channel.
    ///
    /// Required scopes:
    ///
    /// - bot token: `bookmarks:write`
    /// - user token: `bookmarks:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "workflows.featured.remove")]
    pub async fn workflows_featured_remove(
        &self,
        request: &WorkflowsFeaturedRemoveRequest,
    ) -> Result<WorkflowsFeaturedRemoveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`workflows.featured.set`](https://docs.slack.dev/reference/methods/workflows.featured.set): Set featured workflows for a channel.
    ///
    /// Required scopes:
    ///
    /// - bot token: `bookmarks:write`
    /// - user token: `bookmarks:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "workflows.featured.set")]
    pub async fn workflows_featured_set(
        &self,
        request: &WorkflowsFeaturedSetRequest,
    ) -> Result<WorkflowsFeaturedSetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`workflows.triggers.permissions.add`](https://docs.slack.dev/reference/methods/workflows.triggers.permissions.add): Allows users to run a trigger that has its permission type set to named_entities
    ///
    /// Required scopes:
    ///
    /// - bot token: `triggers:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "workflows.triggers.permissions.add")]
    pub async fn workflows_triggers_permissions_add(
        &self,
        request: &WorkflowsTriggersPermissionsAddRequest,
    ) -> Result<WorkflowsTriggersPermissionsAddResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`workflows.triggers.permissions.list`](https://docs.slack.dev/reference/methods/workflows.triggers.permissions.list): Returns the permission type of a trigger and if applicable, includes the entities that have been granted access
    ///
    /// Required scopes:
    ///
    /// - bot token: `triggers:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "workflows.triggers.permissions.list")]
    pub async fn workflows_triggers_permissions_list(
        &self,
        request: &WorkflowsTriggersPermissionsListRequest,
    ) -> Result<WorkflowsTriggersPermissionsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`workflows.triggers.permissions.remove`](https://docs.slack.dev/reference/methods/workflows.triggers.permissions.remove): Revoke an entity's access to a trigger that has its permission type set to named_entities
    ///
    /// Required scopes:
    ///
    /// - bot token: `triggers:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "workflows.triggers.permissions.remove")]
    pub async fn workflows_triggers_permissions_remove(
        &self,
        request: &WorkflowsTriggersPermissionsRemoveRequest,
    ) -> Result<WorkflowsTriggersPermissionsRemoveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`workflows.triggers.permissions.set`](https://docs.slack.dev/reference/methods/workflows.triggers.permissions.set): Set the permission type for who can run a trigger
    ///
    /// Required scopes:
    ///
    /// - bot token: `triggers:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "workflows.triggers.permissions.set")]
    pub async fn workflows_triggers_permissions_set(
        &self,
        request: &WorkflowsTriggersPermissionsSetRequest,
    ) -> Result<WorkflowsTriggersPermissionsSetResponse, SlackError> {
        self.call(request).await
    }
}
