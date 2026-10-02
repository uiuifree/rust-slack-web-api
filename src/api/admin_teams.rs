// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`admin.teams.admins.list`](https://docs.slack.dev/reference/methods/admin.teams.admins.list): List all of the admins on a given workspace.
///
/// Send it with [`SlackClient::admin_teams_admins_list`].
#[doc(alias = "admin.teams.admins.list")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminTeamsAdminsListRequest {
    pub team_id: String,
    /// The maximum number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AdminTeamsAdminsListRequest {
    pub fn new(team_id: impl Into<String>) -> Self {
        Self {
            team_id: team_id.into(),
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

impl SlackApiMethod for AdminTeamsAdminsListRequest {
    const METHOD: &'static str = "admin.teams.admins.list";
    type Response = AdminTeamsAdminsListResponse;
}

impl CursorPaginated for AdminTeamsAdminsListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminTeamsAdminsListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.teams.admins.list`](https://docs.slack.dev/reference/methods/admin.teams.admins.list).
#[doc(alias = "admin.teams.admins.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminTeamsAdminsListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub admin_ids: Vec<String>,
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

/// Arguments for the Slack Web API method [`admin.teams.create`](https://docs.slack.dev/reference/methods/admin.teams.create): Create an Enterprise team.
///
/// Send it with [`SlackClient::admin_teams_create`].
#[doc(alias = "admin.teams.create")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminTeamsCreateRequest {
    /// Team domain (for example, slacksoftballteam). Domains are limited to 21 characters.
    pub team_domain: String,
    /// Team name (for example, Slack Softball Team).
    pub team_name: String,
    /// Description for the team.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_description: Option<String>,
    /// Who can join the team. A team's discoverability can be `open`, `closed`, `invite_only`, or `unlisted`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_discoverability: Option<String>,
}

impl AdminTeamsCreateRequest {
    pub fn new(team_domain: impl Into<String>, team_name: impl Into<String>) -> Self {
        Self {
            team_domain: team_domain.into(),
            team_name: team_name.into(),
            team_description: None,
            team_discoverability: None,
        }
    }

    pub fn team_description(mut self, team_description: impl Into<String>) -> Self {
        self.team_description = Some(team_description.into());
        self
    }

    pub fn team_discoverability(mut self, team_discoverability: impl Into<String>) -> Self {
        self.team_discoverability = Some(team_discoverability.into());
        self
    }
}

impl SlackApiMethod for AdminTeamsCreateRequest {
    const METHOD: &'static str = "admin.teams.create";
    type Response = AdminTeamsCreateResponse;
}

/// Successful response of the Slack Web API method [`admin.teams.create`](https://docs.slack.dev/reference/methods/admin.teams.create).
#[doc(alias = "admin.teams.create")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminTeamsCreateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub team: Option<String>,
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

/// Arguments for the Slack Web API method [`admin.teams.list`](https://docs.slack.dev/reference/methods/admin.teams.list): List all teams in an Enterprise organization
///
/// Send it with [`SlackClient::admin_teams_list`].
#[doc(alias = "admin.teams.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminTeamsListRequest {
    /// The maximum number of items to return. Must be a positive integer no larger than 1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AdminTeamsListRequest {
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

impl SlackApiMethod for AdminTeamsListRequest {
    const METHOD: &'static str = "admin.teams.list";
    type Response = AdminTeamsListResponse;
}

impl CursorPaginated for AdminTeamsListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminTeamsListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.teams.list`](https://docs.slack.dev/reference/methods/admin.teams.list).
#[doc(alias = "admin.teams.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminTeamsListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub teams: Vec<AdminTeamsListResponseTeams>,
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
pub struct AdminTeamsListResponseTeams {
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
    pub discoverability: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_owner: Option<AdminTeamsListResponseTeamsPrimaryOwner>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_url: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminTeamsListResponseTeamsPrimaryOwner {
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
    pub email: Option<String>,
}

/// Arguments for the Slack Web API method [`admin.teams.owners.list`](https://docs.slack.dev/reference/methods/admin.teams.owners.list): List all of the owners on a given workspace.
///
/// Send it with [`SlackClient::admin_teams_owners_list`].
#[doc(alias = "admin.teams.owners.list")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminTeamsOwnersListRequest {
    pub team_id: String,
    /// The maximum number of items to return. Must be between 1 - 1000 both inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AdminTeamsOwnersListRequest {
    pub fn new(team_id: impl Into<String>) -> Self {
        Self {
            team_id: team_id.into(),
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

impl SlackApiMethod for AdminTeamsOwnersListRequest {
    const METHOD: &'static str = "admin.teams.owners.list";
    type Response = AdminTeamsOwnersListResponse;
}

impl CursorPaginated for AdminTeamsOwnersListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminTeamsOwnersListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.teams.owners.list`](https://docs.slack.dev/reference/methods/admin.teams.owners.list).
#[doc(alias = "admin.teams.owners.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminTeamsOwnersListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub owner_ids: Vec<String>,
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

/// Arguments for the Slack Web API method [`admin.teams.settings.info`](https://docs.slack.dev/reference/methods/admin.teams.settings.info): Fetch information about settings in a workspace
///
/// Send it with [`SlackClient::admin_teams_settings_info`].
#[doc(alias = "admin.teams.settings.info")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminTeamsSettingsInfoRequest {
    pub team_id: String,
}

impl AdminTeamsSettingsInfoRequest {
    pub fn new(team_id: impl Into<String>) -> Self {
        Self {
            team_id: team_id.into(),
        }
    }
}

impl SlackApiMethod for AdminTeamsSettingsInfoRequest {
    const METHOD: &'static str = "admin.teams.settings.info";
    type Response = AdminTeamsSettingsInfoResponse;
}

/// Successful response of the Slack Web API method [`admin.teams.settings.info`](https://docs.slack.dev/reference/methods/admin.teams.settings.info).
#[doc(alias = "admin.teams.settings.info")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminTeamsSettingsInfoResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub team: Option<AdminTeamsSettingsInfoResponseTeam>,
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
pub struct AdminTeamsSettingsInfoResponseTeam {
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
    pub domain: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub email_domain: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub icon: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub enterprise_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub enterprise_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_channels: Option<String>,
}

/// Arguments for the Slack Web API method [`admin.teams.settings.setDefaultChannels`](https://docs.slack.dev/reference/methods/admin.teams.settings.setDefaultChannels): Set the default channels of a workspace.
///
/// Send it with [`SlackClient::admin_teams_settings_set_default_channels`].
#[doc(alias = "admin.teams.settings.setDefaultChannels")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminTeamsSettingsSetDefaultChannelsRequest {
    /// ID for the workspace to set the default channel for.
    pub team_id: String,
    /// An array of channel IDs.
    pub channel_ids: Vec<String>,
}

impl AdminTeamsSettingsSetDefaultChannelsRequest {
    pub fn new(team_id: impl Into<String>, channel_ids: Vec<String>) -> Self {
        Self {
            team_id: team_id.into(),
            channel_ids,
        }
    }
}

impl SlackApiMethod for AdminTeamsSettingsSetDefaultChannelsRequest {
    const METHOD: &'static str = "admin.teams.settings.setDefaultChannels";
    type Response = AdminTeamsSettingsSetDefaultChannelsResponse;
}

/// Successful response of the Slack Web API method [`admin.teams.settings.setDefaultChannels`](https://docs.slack.dev/reference/methods/admin.teams.settings.setDefaultChannels).
#[doc(alias = "admin.teams.settings.setDefaultChannels")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminTeamsSettingsSetDefaultChannelsResponse {
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

/// Arguments for the Slack Web API method [`admin.teams.settings.setDescription`](https://docs.slack.dev/reference/methods/admin.teams.settings.setDescription): Set the description of a given workspace.
///
/// Send it with [`SlackClient::admin_teams_settings_set_description`].
#[doc(alias = "admin.teams.settings.setDescription")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminTeamsSettingsSetDescriptionRequest {
    /// ID for the workspace to set the description for.
    pub team_id: String,
    /// The new description for the workspace.
    pub description: String,
}

impl AdminTeamsSettingsSetDescriptionRequest {
    pub fn new(team_id: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            team_id: team_id.into(),
            description: description.into(),
        }
    }
}

impl SlackApiMethod for AdminTeamsSettingsSetDescriptionRequest {
    const METHOD: &'static str = "admin.teams.settings.setDescription";
    type Response = AdminTeamsSettingsSetDescriptionResponse;
}

/// Successful response of the Slack Web API method [`admin.teams.settings.setDescription`](https://docs.slack.dev/reference/methods/admin.teams.settings.setDescription).
#[doc(alias = "admin.teams.settings.setDescription")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminTeamsSettingsSetDescriptionResponse {
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

/// Arguments for the Slack Web API method [`admin.teams.settings.setDiscoverability`](https://docs.slack.dev/reference/methods/admin.teams.settings.setDiscoverability): An API method that allows admins to set the discoverability of a given workspace
///
/// Send it with [`SlackClient::admin_teams_settings_set_discoverability`].
#[doc(alias = "admin.teams.settings.setDiscoverability")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminTeamsSettingsSetDiscoverabilityRequest {
    /// The ID of the workspace to set discoverability on.
    pub team_id: String,
    /// This workspace's discovery setting. It must be set to one of `open`, `invite_only`, `closed`, or `unlisted`.
    pub discoverability: String,
}

impl AdminTeamsSettingsSetDiscoverabilityRequest {
    pub fn new(team_id: impl Into<String>, discoverability: impl Into<String>) -> Self {
        Self {
            team_id: team_id.into(),
            discoverability: discoverability.into(),
        }
    }
}

impl SlackApiMethod for AdminTeamsSettingsSetDiscoverabilityRequest {
    const METHOD: &'static str = "admin.teams.settings.setDiscoverability";
    type Response = AdminTeamsSettingsSetDiscoverabilityResponse;
}

/// Successful response of the Slack Web API method [`admin.teams.settings.setDiscoverability`](https://docs.slack.dev/reference/methods/admin.teams.settings.setDiscoverability).
#[doc(alias = "admin.teams.settings.setDiscoverability")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminTeamsSettingsSetDiscoverabilityResponse {
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

/// Arguments for the Slack Web API method [`admin.teams.settings.setIcon`](https://docs.slack.dev/reference/methods/admin.teams.settings.setIcon): Sets the icon of a workspace.
///
/// Send it with [`SlackClient::admin_teams_settings_set_icon`].
#[doc(alias = "admin.teams.settings.setIcon")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminTeamsSettingsSetIconRequest {
    /// Image URL for the icon
    pub image_url: String,
    /// ID for the workspace to set the icon for.
    pub team_id: String,
}

impl AdminTeamsSettingsSetIconRequest {
    pub fn new(image_url: impl Into<String>, team_id: impl Into<String>) -> Self {
        Self {
            image_url: image_url.into(),
            team_id: team_id.into(),
        }
    }
}

impl SlackApiMethod for AdminTeamsSettingsSetIconRequest {
    const METHOD: &'static str = "admin.teams.settings.setIcon";
    type Response = AdminTeamsSettingsSetIconResponse;
}

/// Successful response of the Slack Web API method [`admin.teams.settings.setIcon`](https://docs.slack.dev/reference/methods/admin.teams.settings.setIcon).
#[doc(alias = "admin.teams.settings.setIcon")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminTeamsSettingsSetIconResponse {
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

/// Arguments for the Slack Web API method [`admin.teams.settings.setName`](https://docs.slack.dev/reference/methods/admin.teams.settings.setName): Set the name of a given workspace.
///
/// Send it with [`SlackClient::admin_teams_settings_set_name`].
#[doc(alias = "admin.teams.settings.setName")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminTeamsSettingsSetNameRequest {
    /// ID for the workspace to set the name for.
    pub team_id: String,
    /// The new name of the workspace.
    pub name: String,
}

impl AdminTeamsSettingsSetNameRequest {
    pub fn new(team_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            team_id: team_id.into(),
            name: name.into(),
        }
    }
}

impl SlackApiMethod for AdminTeamsSettingsSetNameRequest {
    const METHOD: &'static str = "admin.teams.settings.setName";
    type Response = AdminTeamsSettingsSetNameResponse;
}

/// Successful response of the Slack Web API method [`admin.teams.settings.setName`](https://docs.slack.dev/reference/methods/admin.teams.settings.setName).
#[doc(alias = "admin.teams.settings.setName")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminTeamsSettingsSetNameResponse {
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
    /// Calls the Slack Web API method [`admin.teams.admins.list`](https://docs.slack.dev/reference/methods/admin.teams.admins.list): List all of the admins on a given workspace.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.teams:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.teams.admins.list")]
    pub async fn admin_teams_admins_list(
        &self,
        request: &AdminTeamsAdminsListRequest,
    ) -> Result<AdminTeamsAdminsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.teams.create`](https://docs.slack.dev/reference/methods/admin.teams.create): Create an Enterprise team.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.teams:write`
    ///
    /// Rate limit: Tier 1 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.teams.create")]
    pub async fn admin_teams_create(
        &self,
        request: &AdminTeamsCreateRequest,
    ) -> Result<AdminTeamsCreateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.teams.list`](https://docs.slack.dev/reference/methods/admin.teams.list): List all teams in an Enterprise organization
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.teams:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.teams.list")]
    pub async fn admin_teams_list(
        &self,
        request: &AdminTeamsListRequest,
    ) -> Result<AdminTeamsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.teams.owners.list`](https://docs.slack.dev/reference/methods/admin.teams.owners.list): List all of the owners on a given workspace.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.teams:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.teams.owners.list")]
    pub async fn admin_teams_owners_list(
        &self,
        request: &AdminTeamsOwnersListRequest,
    ) -> Result<AdminTeamsOwnersListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.teams.settings.info`](https://docs.slack.dev/reference/methods/admin.teams.settings.info): Fetch information about settings in a workspace
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.teams:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.teams.settings.info")]
    pub async fn admin_teams_settings_info(
        &self,
        request: &AdminTeamsSettingsInfoRequest,
    ) -> Result<AdminTeamsSettingsInfoResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.teams.settings.setDefaultChannels`](https://docs.slack.dev/reference/methods/admin.teams.settings.setDefaultChannels): Set the default channels of a workspace.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.teams:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.teams.settings.setDefaultChannels")]
    pub async fn admin_teams_settings_set_default_channels(
        &self,
        request: &AdminTeamsSettingsSetDefaultChannelsRequest,
    ) -> Result<AdminTeamsSettingsSetDefaultChannelsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.teams.settings.setDescription`](https://docs.slack.dev/reference/methods/admin.teams.settings.setDescription): Set the description of a given workspace.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.teams:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.teams.settings.setDescription")]
    pub async fn admin_teams_settings_set_description(
        &self,
        request: &AdminTeamsSettingsSetDescriptionRequest,
    ) -> Result<AdminTeamsSettingsSetDescriptionResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.teams.settings.setDiscoverability`](https://docs.slack.dev/reference/methods/admin.teams.settings.setDiscoverability): An API method that allows admins to set the discoverability of a given workspace
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.teams:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.teams.settings.setDiscoverability")]
    pub async fn admin_teams_settings_set_discoverability(
        &self,
        request: &AdminTeamsSettingsSetDiscoverabilityRequest,
    ) -> Result<AdminTeamsSettingsSetDiscoverabilityResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.teams.settings.setIcon`](https://docs.slack.dev/reference/methods/admin.teams.settings.setIcon): Sets the icon of a workspace.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.teams:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.teams.settings.setIcon")]
    pub async fn admin_teams_settings_set_icon(
        &self,
        request: &AdminTeamsSettingsSetIconRequest,
    ) -> Result<AdminTeamsSettingsSetIconResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.teams.settings.setName`](https://docs.slack.dev/reference/methods/admin.teams.settings.setName): Set the name of a given workspace.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.teams:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.teams.settings.setName")]
    pub async fn admin_teams_settings_set_name(
        &self,
        request: &AdminTeamsSettingsSetNameRequest,
    ) -> Result<AdminTeamsSettingsSetNameResponse, SlackError> {
        self.call(request).await
    }
}
