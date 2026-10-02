// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`admin.barriers.create`](https://docs.slack.dev/reference/methods/admin.barriers.create): Create an Information Barrier
///
/// Send it with [`SlackClient::admin_barriers_create`].
#[doc(alias = "admin.barriers.create")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminBarriersCreateRequest {
    /// The id of the primary [IDP Group](https://slack.com/help/articles/115001435788-Connect-identity-provider-groups-to-your-Enterprise-organization)
    pub primary_usergroup_id: String,
    /// A list of [IDP Groups](https://slack.com/help/articles/115001435788-Connect-identity-provider-groups-to-your-Enterprise-organization) ids that the primary usergroup is to be barriered from.
    pub barriered_from_usergroup_ids: Vec<String>,
    /// What kind of interactions are blocked by this barrier? For v1, we only support a list of all 3, eg `im, mpim, call`
    #[serde(serialize_with = "crate::form::as_json")]
    pub restricted_subjects: Vec<serde_json::Value>,
}

impl AdminBarriersCreateRequest {
    pub fn new(
        primary_usergroup_id: impl Into<String>,
        barriered_from_usergroup_ids: Vec<String>,
        restricted_subjects: Vec<serde_json::Value>,
    ) -> Self {
        Self {
            primary_usergroup_id: primary_usergroup_id.into(),
            barriered_from_usergroup_ids,
            restricted_subjects,
        }
    }
}

impl SlackApiMethod for AdminBarriersCreateRequest {
    const METHOD: &'static str = "admin.barriers.create";
    type Response = AdminBarriersCreateResponse;
}

/// Successful response of the Slack Web API method [`admin.barriers.create`](https://docs.slack.dev/reference/methods/admin.barriers.create).
#[doc(alias = "admin.barriers.create")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminBarriersCreateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub barrier: Option<AdminBarriersCreateResponseBarrier>,
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
pub struct AdminBarriersCreateResponseBarrier {
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
    pub enterprise_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_usergroup: Option<AdminBarriersCreateResponseBarrierPrimaryUsergroup>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub barriered_from_usergroups: Vec<AdminBarriersCreateResponseBarrierBarrieredFromUsergroups>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub restricted_subjects: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_update: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminBarriersCreateResponseBarrierPrimaryUsergroup {
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

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminBarriersCreateResponseBarrierBarrieredFromUsergroups {
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

/// Arguments for the Slack Web API method [`admin.barriers.delete`](https://docs.slack.dev/reference/methods/admin.barriers.delete): Delete an existing Information Barrier
///
/// Send it with [`SlackClient::admin_barriers_delete`].
#[doc(alias = "admin.barriers.delete")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminBarriersDeleteRequest {
    /// The ID of the barrier you're trying to delete
    pub barrier_id: String,
}

impl AdminBarriersDeleteRequest {
    pub fn new(barrier_id: impl Into<String>) -> Self {
        Self {
            barrier_id: barrier_id.into(),
        }
    }
}

impl SlackApiMethod for AdminBarriersDeleteRequest {
    const METHOD: &'static str = "admin.barriers.delete";
    type Response = AdminBarriersDeleteResponse;
}

/// Successful response of the Slack Web API method [`admin.barriers.delete`](https://docs.slack.dev/reference/methods/admin.barriers.delete).
#[doc(alias = "admin.barriers.delete")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminBarriersDeleteResponse {
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

/// Arguments for the Slack Web API method [`admin.barriers.list`](https://docs.slack.dev/reference/methods/admin.barriers.list): Get all Information Barriers for your organization
///
/// Send it with [`SlackClient::admin_barriers_list`].
#[doc(alias = "admin.barriers.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminBarriersListRequest {
    /// The maximum number of items to return. Must be between 1 - 1000 both inclusive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AdminBarriersListRequest {
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

impl SlackApiMethod for AdminBarriersListRequest {
    const METHOD: &'static str = "admin.barriers.list";
    type Response = AdminBarriersListResponse;
}

impl CursorPaginated for AdminBarriersListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminBarriersListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.barriers.list`](https://docs.slack.dev/reference/methods/admin.barriers.list).
#[doc(alias = "admin.barriers.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminBarriersListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub barriers: Vec<AdminBarriersListResponseBarriers>,
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
pub struct AdminBarriersListResponseBarriers {
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
    pub enterprise_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_usergroup: Option<AdminBarriersListResponseBarriersPrimaryUsergroup>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub barriered_from_usergroups: Vec<AdminBarriersListResponseBarriersBarrieredFromUsergroups>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub restricted_subjects: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_update: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminBarriersListResponseBarriersPrimaryUsergroup {
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

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminBarriersListResponseBarriersBarrieredFromUsergroups {
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

/// Arguments for the Slack Web API method [`admin.barriers.update`](https://docs.slack.dev/reference/methods/admin.barriers.update): Update an existing Information Barrier
///
/// Send it with [`SlackClient::admin_barriers_update`].
#[doc(alias = "admin.barriers.update")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminBarriersUpdateRequest {
    /// The ID of the barrier you're trying to modify
    pub barrier_id: String,
    /// The id of the primary [IDP Group](https://slack.com/help/articles/115001435788-Connect-identity-provider-groups-to-your-Enterprise-organization)
    pub primary_usergroup_id: String,
    /// A list of [IDP Groups](https://slack.com/help/articles/115001435788-Connect-identity-provider-groups-to-your-Enterprise-organization) ids that the primary usergroup is to be barriered from.
    pub barriered_from_usergroup_ids: Vec<String>,
    /// What kind of interactions are blocked by this barrier? For v1, we only support a list of all 3, eg `im, mpim, call`
    #[serde(serialize_with = "crate::form::as_json")]
    pub restricted_subjects: Vec<serde_json::Value>,
}

impl AdminBarriersUpdateRequest {
    pub fn new(
        barrier_id: impl Into<String>,
        primary_usergroup_id: impl Into<String>,
        barriered_from_usergroup_ids: Vec<String>,
        restricted_subjects: Vec<serde_json::Value>,
    ) -> Self {
        Self {
            barrier_id: barrier_id.into(),
            primary_usergroup_id: primary_usergroup_id.into(),
            barriered_from_usergroup_ids,
            restricted_subjects,
        }
    }
}

impl SlackApiMethod for AdminBarriersUpdateRequest {
    const METHOD: &'static str = "admin.barriers.update";
    type Response = AdminBarriersUpdateResponse;
}

/// Successful response of the Slack Web API method [`admin.barriers.update`](https://docs.slack.dev/reference/methods/admin.barriers.update).
#[doc(alias = "admin.barriers.update")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminBarriersUpdateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub barrier: Option<AdminBarriersUpdateResponseBarrier>,
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
pub struct AdminBarriersUpdateResponseBarrier {
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
    pub enterprise_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_usergroup: Option<AdminBarriersUpdateResponseBarrierPrimaryUsergroup>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub barriered_from_usergroups: Vec<AdminBarriersUpdateResponseBarrierBarrieredFromUsergroups>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub restricted_subjects: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_update: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminBarriersUpdateResponseBarrierPrimaryUsergroup {
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

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminBarriersUpdateResponseBarrierBarrieredFromUsergroups {
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

impl SlackClient {
    /// Calls the Slack Web API method [`admin.barriers.create`](https://docs.slack.dev/reference/methods/admin.barriers.create): Create an Information Barrier
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.barriers:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.barriers.create")]
    pub async fn admin_barriers_create(
        &self,
        request: &AdminBarriersCreateRequest,
    ) -> Result<AdminBarriersCreateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.barriers.delete`](https://docs.slack.dev/reference/methods/admin.barriers.delete): Delete an existing Information Barrier
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.barriers:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.barriers.delete")]
    pub async fn admin_barriers_delete(
        &self,
        request: &AdminBarriersDeleteRequest,
    ) -> Result<AdminBarriersDeleteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.barriers.list`](https://docs.slack.dev/reference/methods/admin.barriers.list): Get all Information Barriers for your organization
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.barriers:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.barriers.list")]
    pub async fn admin_barriers_list(
        &self,
        request: &AdminBarriersListRequest,
    ) -> Result<AdminBarriersListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.barriers.update`](https://docs.slack.dev/reference/methods/admin.barriers.update): Update an existing Information Barrier
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.barriers:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.barriers.update")]
    pub async fn admin_barriers_update(
        &self,
        request: &AdminBarriersUpdateRequest,
    ) -> Result<AdminBarriersUpdateResponse, SlackError> {
        self.call(request).await
    }
}
