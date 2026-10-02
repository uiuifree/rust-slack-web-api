// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`admin.auth.policy.assignEntities`](https://docs.slack.dev/reference/methods/admin.auth.policy.assignEntities): Assign entities to a particular authentication policy.
///
/// Send it with [`SlackClient::admin_auth_policy_assign_entities`].
#[doc(alias = "admin.auth.policy.assignEntities")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminAuthPolicyAssignEntitiesRequest {
    /// The name of the authentication policy to assign the entities to. Currently, `email_password` is the only policy that may be used with this method.
    pub policy_name: String,
    /// The type of entity to assign to the policy. Currently, `USER` is supported.
    pub entity_type: String,
    /// Array of IDs to assign to the policy.
    #[serde(serialize_with = "crate::form::as_json")]
    pub entity_ids: Vec<String>,
}

impl AdminAuthPolicyAssignEntitiesRequest {
    pub fn new(
        policy_name: impl Into<String>,
        entity_type: impl Into<String>,
        entity_ids: Vec<String>,
    ) -> Self {
        Self {
            policy_name: policy_name.into(),
            entity_type: entity_type.into(),
            entity_ids,
        }
    }
}

impl SlackApiMethod for AdminAuthPolicyAssignEntitiesRequest {
    const METHOD: &'static str = "admin.auth.policy.assignEntities";
    type Response = AdminAuthPolicyAssignEntitiesResponse;
}

/// Successful response of the Slack Web API method [`admin.auth.policy.assignEntities`](https://docs.slack.dev/reference/methods/admin.auth.policy.assignEntities).
#[doc(alias = "admin.auth.policy.assignEntities")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAuthPolicyAssignEntitiesResponse {
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

/// Arguments for the Slack Web API method [`admin.auth.policy.getEntities`](https://docs.slack.dev/reference/methods/admin.auth.policy.getEntities): Fetch all the entities assigned to a particular authentication policy by name.
///
/// Send it with [`SlackClient::admin_auth_policy_get_entities`].
#[doc(alias = "admin.auth.policy.getEntities")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminAuthPolicyGetEntitiesRequest {
    /// The name of the policy to fetch entities for. Currently, `email_password` is the only policy that may be used with this method.
    pub policy_name: String,
    /// The type of entity to assign to the policy. Currently, `USER` is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// The maximum number of items to return. Must be between 1 and 1000, both inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AdminAuthPolicyGetEntitiesRequest {
    pub fn new(policy_name: impl Into<String>) -> Self {
        Self {
            policy_name: policy_name.into(),
            entity_type: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn entity_type(mut self, entity_type: impl Into<String>) -> Self {
        self.entity_type = Some(entity_type.into());
        self
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

impl SlackApiMethod for AdminAuthPolicyGetEntitiesRequest {
    const METHOD: &'static str = "admin.auth.policy.getEntities";
    type Response = AdminAuthPolicyGetEntitiesResponse;
}

impl CursorPaginated for AdminAuthPolicyGetEntitiesRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminAuthPolicyGetEntitiesResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.auth.policy.getEntities`](https://docs.slack.dev/reference/methods/admin.auth.policy.getEntities).
#[doc(alias = "admin.auth.policy.getEntities")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAuthPolicyGetEntitiesResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub entities: Vec<AdminAuthPolicyGetEntitiesResponseEntities>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub entity_total_count: Option<i64>,
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
pub struct AdminAuthPolicyGetEntitiesResponseEntities {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub entity_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub entity_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_added: Option<i64>,
}

/// Arguments for the Slack Web API method [`admin.auth.policy.removeEntities`](https://docs.slack.dev/reference/methods/admin.auth.policy.removeEntities): Remove specified entities from a specified authentication policy.
///
/// Send it with [`SlackClient::admin_auth_policy_remove_entities`].
#[doc(alias = "admin.auth.policy.removeEntities")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminAuthPolicyRemoveEntitiesRequest {
    /// The name of the policy to remove entities from. Currently, `email_password` is the only policy that may be used with this method.
    pub policy_name: String,
    /// The type of entity to assign to the policy. Currently, `USER` is supported.
    pub entity_type: String,
    /// Encoded IDs of the entities you'd like to remove from the policy.
    #[serde(serialize_with = "crate::form::as_json")]
    pub entity_ids: Vec<String>,
}

impl AdminAuthPolicyRemoveEntitiesRequest {
    pub fn new(
        policy_name: impl Into<String>,
        entity_type: impl Into<String>,
        entity_ids: Vec<String>,
    ) -> Self {
        Self {
            policy_name: policy_name.into(),
            entity_type: entity_type.into(),
            entity_ids,
        }
    }
}

impl SlackApiMethod for AdminAuthPolicyRemoveEntitiesRequest {
    const METHOD: &'static str = "admin.auth.policy.removeEntities";
    type Response = AdminAuthPolicyRemoveEntitiesResponse;
}

/// Successful response of the Slack Web API method [`admin.auth.policy.removeEntities`](https://docs.slack.dev/reference/methods/admin.auth.policy.removeEntities).
#[doc(alias = "admin.auth.policy.removeEntities")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAuthPolicyRemoveEntitiesResponse {
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
    /// Calls the Slack Web API method [`admin.auth.policy.assignEntities`](https://docs.slack.dev/reference/methods/admin.auth.policy.assignEntities): Assign entities to a particular authentication policy.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.users:write`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.auth.policy.assignEntities")]
    pub async fn admin_auth_policy_assign_entities(
        &self,
        request: &AdminAuthPolicyAssignEntitiesRequest,
    ) -> Result<AdminAuthPolicyAssignEntitiesResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.auth.policy.getEntities`](https://docs.slack.dev/reference/methods/admin.auth.policy.getEntities): Fetch all the entities assigned to a particular authentication policy by name.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.users:read`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.auth.policy.getEntities")]
    pub async fn admin_auth_policy_get_entities(
        &self,
        request: &AdminAuthPolicyGetEntitiesRequest,
    ) -> Result<AdminAuthPolicyGetEntitiesResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.auth.policy.removeEntities`](https://docs.slack.dev/reference/methods/admin.auth.policy.removeEntities): Remove specified entities from a specified authentication policy.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.users:write`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.auth.policy.removeEntities")]
    pub async fn admin_auth_policy_remove_entities(
        &self,
        request: &AdminAuthPolicyRemoveEntitiesRequest,
    ) -> Result<AdminAuthPolicyRemoveEntitiesResponse, SlackError> {
        self.call(request).await
    }
}
