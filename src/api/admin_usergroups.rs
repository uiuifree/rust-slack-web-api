// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`admin.usergroups.addChannels`](https://docs.slack.dev/reference/methods/admin.usergroups.addChannels): Add up to one hundred default channels to an IDP group.
///
/// Send it with [`SlackClient::admin_usergroups_add_channels`].
#[doc(alias = "admin.usergroups.addChannels")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsergroupsAddChannelsRequest {
    /// ID of the IDP group to add default channels for.
    pub usergroup_id: String,
    /// Comma separated string of channel IDs.
    pub channel_ids: Vec<String>,
    /// The workspace to add default channels in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl AdminUsergroupsAddChannelsRequest {
    pub fn new(usergroup_id: impl Into<String>, channel_ids: Vec<String>) -> Self {
        Self {
            usergroup_id: usergroup_id.into(),
            channel_ids,
            team_id: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for AdminUsergroupsAddChannelsRequest {
    const METHOD: &'static str = "admin.usergroups.addChannels";
    type Response = AdminUsergroupsAddChannelsResponse;
}

/// Successful response of the Slack Web API method [`admin.usergroups.addChannels`](https://docs.slack.dev/reference/methods/admin.usergroups.addChannels).
#[doc(alias = "admin.usergroups.addChannels")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsergroupsAddChannelsResponse {
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

/// Arguments for the Slack Web API method [`admin.usergroups.addTeams`](https://docs.slack.dev/reference/methods/admin.usergroups.addTeams): Associate one or more default workspaces with an organization-wide IDP group.
///
/// Send it with [`SlackClient::admin_usergroups_add_teams`].
#[doc(alias = "admin.usergroups.addTeams")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsergroupsAddTeamsRequest {
    /// An encoded usergroup (IDP Group) ID.
    pub usergroup_id: String,
    /// A comma separated list of encoded team (workspace) IDs. Each workspace _MUST_ belong to the organization associated with the token.
    pub team_ids: Vec<String>,
    /// When `true`, this method automatically creates new workspace accounts for the IDP group members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_provision: Option<bool>,
}

impl AdminUsergroupsAddTeamsRequest {
    pub fn new(usergroup_id: impl Into<String>, team_ids: Vec<String>) -> Self {
        Self {
            usergroup_id: usergroup_id.into(),
            team_ids,
            auto_provision: None,
        }
    }

    pub fn auto_provision(mut self, auto_provision: bool) -> Self {
        self.auto_provision = Some(auto_provision);
        self
    }
}

impl SlackApiMethod for AdminUsergroupsAddTeamsRequest {
    const METHOD: &'static str = "admin.usergroups.addTeams";
    type Response = AdminUsergroupsAddTeamsResponse;
}

/// Successful response of the Slack Web API method [`admin.usergroups.addTeams`](https://docs.slack.dev/reference/methods/admin.usergroups.addTeams).
#[doc(alias = "admin.usergroups.addTeams")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsergroupsAddTeamsResponse {
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

/// Arguments for the Slack Web API method [`admin.usergroups.addUsers`](https://docs.slack.dev/reference/methods/admin.usergroups.addUsers): Add members to an existing organizational usergroup.
///
/// Send it with [`SlackClient::admin_usergroups_add_users`].
#[doc(alias = "admin.usergroups.addUsers")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsergroupsAddUsersRequest {
    /// ID of the usergroup to add users to
    pub id: String,
    /// The encoded user IDs to add to the usergroup, provided as a JSON array or a comma-separated string.
    pub users: Vec<String>,
}

impl AdminUsergroupsAddUsersRequest {
    pub fn new(id: impl Into<String>, users: Vec<String>) -> Self {
        Self {
            id: id.into(),
            users,
        }
    }
}

impl SlackApiMethod for AdminUsergroupsAddUsersRequest {
    const METHOD: &'static str = "admin.usergroups.addUsers";
    type Response = AdminUsergroupsAddUsersResponse;
}

/// Successful response of the Slack Web API method [`admin.usergroups.addUsers`](https://docs.slack.dev/reference/methods/admin.usergroups.addUsers).
#[doc(alias = "admin.usergroups.addUsers")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsergroupsAddUsersResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub invalid_users: Vec<AdminUsergroupsAddUsersResponseInvalidUsers>,
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
pub struct AdminUsergroupsAddUsersResponseInvalidUsers {
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
    pub reason: Option<String>,
}

/// Arguments for the Slack Web API method [`admin.usergroups.create`](https://docs.slack.dev/reference/methods/admin.usergroups.create): Create a new organizational usergroup.
///
/// Send it with [`SlackClient::admin_usergroups_create`].
#[doc(alias = "admin.usergroups.create")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsergroupsCreateRequest {
    /// Unique name for the usergroup
    pub name: String,
    /// Optional handle used to mention the usergroup in channel, must be unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// Optional purpose that describes what the usergroup is about
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// Configure whether or not this usergroup should be visible in the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_visible: Option<bool>,
}

impl AdminUsergroupsCreateRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            handle: None,
            purpose: None,
            is_visible: None,
        }
    }

    pub fn handle(mut self, handle: impl Into<String>) -> Self {
        self.handle = Some(handle.into());
        self
    }

    pub fn purpose(mut self, purpose: impl Into<String>) -> Self {
        self.purpose = Some(purpose.into());
        self
    }

    pub fn is_visible(mut self, is_visible: bool) -> Self {
        self.is_visible = Some(is_visible);
        self
    }
}

impl SlackApiMethod for AdminUsergroupsCreateRequest {
    const METHOD: &'static str = "admin.usergroups.create";
    type Response = AdminUsergroupsCreateResponse;
}

/// Successful response of the Slack Web API method [`admin.usergroups.create`](https://docs.slack.dev/reference/methods/admin.usergroups.create).
#[doc(alias = "admin.usergroups.create")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsergroupsCreateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub subteam: Option<Usergroup>,
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

/// Arguments for the Slack Web API method [`admin.usergroups.fetch`](https://docs.slack.dev/reference/methods/admin.usergroups.fetch): Fetch an organizational usergroup.
///
/// Send it with [`SlackClient::admin_usergroups_fetch`].
#[doc(alias = "admin.usergroups.fetch")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsergroupsFetchRequest {
    /// ID of the usergroup to fetch
    pub id: String,
}

impl AdminUsergroupsFetchRequest {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl SlackApiMethod for AdminUsergroupsFetchRequest {
    const METHOD: &'static str = "admin.usergroups.fetch";
    type Response = AdminUsergroupsFetchResponse;
}

/// Successful response of the Slack Web API method [`admin.usergroups.fetch`](https://docs.slack.dev/reference/methods/admin.usergroups.fetch).
#[doc(alias = "admin.usergroups.fetch")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsergroupsFetchResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub subteam: Option<Usergroup>,
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

/// Arguments for the Slack Web API method [`admin.usergroups.listChannels`](https://docs.slack.dev/reference/methods/admin.usergroups.listChannels): List the channels linked to an org-level IDP group (user group).
///
/// Send it with [`SlackClient::admin_usergroups_list_channels`].
#[doc(alias = "admin.usergroups.listChannels")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsergroupsListChannelsRequest {
    /// ID of the IDP group to list default channels for.
    pub usergroup_id: String,
    /// ID of the the workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Flag to include or exclude the count of members per channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_num_members: Option<bool>,
}

impl AdminUsergroupsListChannelsRequest {
    pub fn new(usergroup_id: impl Into<String>) -> Self {
        Self {
            usergroup_id: usergroup_id.into(),
            team_id: None,
            include_num_members: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn include_num_members(mut self, include_num_members: bool) -> Self {
        self.include_num_members = Some(include_num_members);
        self
    }
}

impl SlackApiMethod for AdminUsergroupsListChannelsRequest {
    const METHOD: &'static str = "admin.usergroups.listChannels";
    type Response = AdminUsergroupsListChannelsResponse;
}

/// Successful response of the Slack Web API method [`admin.usergroups.listChannels`](https://docs.slack.dev/reference/methods/admin.usergroups.listChannels).
#[doc(alias = "admin.usergroups.listChannels")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsergroupsListChannelsResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub channels: Vec<AdminUsergroupsListChannelsResponseChannels>,
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
pub struct AdminUsergroupsListChannelsResponseChannels {
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
    pub team_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub num_members: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_redacted: Option<bool>,
}

/// Arguments for the Slack Web API method [`admin.usergroups.removeChannels`](https://docs.slack.dev/reference/methods/admin.usergroups.removeChannels): Remove one or more default channels from an org-level IDP group (user group).
///
/// Send it with [`SlackClient::admin_usergroups_remove_channels`].
#[doc(alias = "admin.usergroups.removeChannels")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsergroupsRemoveChannelsRequest {
    /// ID of the IDP Group
    pub usergroup_id: String,
    /// Comma-separated string of channel IDs
    pub channel_ids: Vec<String>,
}

impl AdminUsergroupsRemoveChannelsRequest {
    pub fn new(usergroup_id: impl Into<String>, channel_ids: Vec<String>) -> Self {
        Self {
            usergroup_id: usergroup_id.into(),
            channel_ids,
        }
    }
}

impl SlackApiMethod for AdminUsergroupsRemoveChannelsRequest {
    const METHOD: &'static str = "admin.usergroups.removeChannels";
    type Response = AdminUsergroupsRemoveChannelsResponse;
}

/// Successful response of the Slack Web API method [`admin.usergroups.removeChannels`](https://docs.slack.dev/reference/methods/admin.usergroups.removeChannels).
#[doc(alias = "admin.usergroups.removeChannels")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsergroupsRemoveChannelsResponse {
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

/// Arguments for the Slack Web API method [`admin.usergroups.removeTeams`](https://docs.slack.dev/reference/methods/admin.usergroups.removeTeams): Remove one or more default workspaces from an organization-wide IDP Group or Admin Group
///
/// Send it with [`SlackClient::admin_usergroups_remove_teams`].
#[doc(alias = "admin.usergroups.removeTeams")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsergroupsRemoveTeamsRequest {
    /// An encoded usergroup (IDP Group) ID.
    pub usergroup_id: String,
    /// A comma separated list of encoded team (workspace) IDs. Each workspace _MUST_ belong to the organization associated with the token.
    pub team_ids: Vec<String>,
}

impl AdminUsergroupsRemoveTeamsRequest {
    pub fn new(usergroup_id: impl Into<String>, team_ids: Vec<String>) -> Self {
        Self {
            usergroup_id: usergroup_id.into(),
            team_ids,
        }
    }
}

impl SlackApiMethod for AdminUsergroupsRemoveTeamsRequest {
    const METHOD: &'static str = "admin.usergroups.removeTeams";
    type Response = AdminUsergroupsRemoveTeamsResponse;
}

/// Successful response of the Slack Web API method [`admin.usergroups.removeTeams`](https://docs.slack.dev/reference/methods/admin.usergroups.removeTeams).
#[doc(alias = "admin.usergroups.removeTeams")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsergroupsRemoveTeamsResponse {
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

/// Arguments for the Slack Web API method [`admin.usergroups.removeUsers`](https://docs.slack.dev/reference/methods/admin.usergroups.removeUsers): Remove members from an existing organizational usergroup.
///
/// Send it with [`SlackClient::admin_usergroups_remove_users`].
#[doc(alias = "admin.usergroups.removeUsers")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsergroupsRemoveUsersRequest {
    /// ID of the usergroup to remove users from
    pub id: String,
    /// The encoded user IDs to remove from the usergroup, provided as a JSON array or a comma-separated string.
    pub users: Vec<String>,
}

impl AdminUsergroupsRemoveUsersRequest {
    pub fn new(id: impl Into<String>, users: Vec<String>) -> Self {
        Self {
            id: id.into(),
            users,
        }
    }
}

impl SlackApiMethod for AdminUsergroupsRemoveUsersRequest {
    const METHOD: &'static str = "admin.usergroups.removeUsers";
    type Response = AdminUsergroupsRemoveUsersResponse;
}

/// Successful response of the Slack Web API method [`admin.usergroups.removeUsers`](https://docs.slack.dev/reference/methods/admin.usergroups.removeUsers).
#[doc(alias = "admin.usergroups.removeUsers")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsergroupsRemoveUsersResponse {
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

/// Arguments for the Slack Web API method [`admin.usergroups.update`](https://docs.slack.dev/reference/methods/admin.usergroups.update): Update one or more properties of an existing organizational usergroup.
///
/// Send it with [`SlackClient::admin_usergroups_update`].
#[doc(alias = "admin.usergroups.update")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsergroupsUpdateRequest {
    /// ID of the usergroup to update
    pub id: String,
    /// The name of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The handle used for mentioning the group in a channel, must be unique.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// The usergroup's purpose or description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Configure whether or not this usergroup should be visible in the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_visible: Option<bool>,
}

impl AdminUsergroupsUpdateRequest {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: None,
            handle: None,
            description: None,
            is_visible: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn handle(mut self, handle: impl Into<String>) -> Self {
        self.handle = Some(handle.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn is_visible(mut self, is_visible: bool) -> Self {
        self.is_visible = Some(is_visible);
        self
    }
}

impl SlackApiMethod for AdminUsergroupsUpdateRequest {
    const METHOD: &'static str = "admin.usergroups.update";
    type Response = AdminUsergroupsUpdateResponse;
}

/// Successful response of the Slack Web API method [`admin.usergroups.update`](https://docs.slack.dev/reference/methods/admin.usergroups.update).
#[doc(alias = "admin.usergroups.update")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsergroupsUpdateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub subteam: Option<Usergroup>,
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

/// Arguments for the Slack Web API method [`admin.usergroups.uploadUsers`](https://docs.slack.dev/reference/methods/admin.usergroups.uploadUsers): Add members to an existing organizational usergroup in bulk via CSV upload.
///
/// Send it with [`SlackClient::admin_usergroups_upload_users`].
#[doc(alias = "admin.usergroups.uploadUsers")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsergroupsUploadUsersRequest {
    /// ID of the usergroup to upload users to
    pub id: String,
    /// Csv of users to upload in format member id, email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

impl AdminUsergroupsUploadUsersRequest {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            file: None,
        }
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }
}

impl SlackApiMethod for AdminUsergroupsUploadUsersRequest {
    const METHOD: &'static str = "admin.usergroups.uploadUsers";
    type Response = AdminUsergroupsUploadUsersResponse;
}

/// Successful response of the Slack Web API method [`admin.usergroups.uploadUsers`](https://docs.slack.dev/reference/methods/admin.usergroups.uploadUsers).
#[doc(alias = "admin.usergroups.uploadUsers")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsergroupsUploadUsersResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub successful_user_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub invalid_users: Vec<AdminUsergroupsUploadUsersResponseInvalidUsers>,
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
pub struct AdminUsergroupsUploadUsersResponseInvalidUsers {
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
    pub reason: Option<String>,
}

impl SlackClient {
    /// Calls the Slack Web API method [`admin.usergroups.addChannels`](https://docs.slack.dev/reference/methods/admin.usergroups.addChannels): Add up to one hundred default channels to an IDP group.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.usergroups:write`
    ///
    /// Rate limit: Tier 1 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.usergroups.addChannels")]
    pub async fn admin_usergroups_add_channels(
        &self,
        request: &AdminUsergroupsAddChannelsRequest,
    ) -> Result<AdminUsergroupsAddChannelsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.usergroups.addTeams`](https://docs.slack.dev/reference/methods/admin.usergroups.addTeams): Associate one or more default workspaces with an organization-wide IDP group.
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
    #[doc(alias = "admin.usergroups.addTeams")]
    pub async fn admin_usergroups_add_teams(
        &self,
        request: &AdminUsergroupsAddTeamsRequest,
    ) -> Result<AdminUsergroupsAddTeamsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.usergroups.addUsers`](https://docs.slack.dev/reference/methods/admin.usergroups.addUsers): Add members to an existing organizational usergroup.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.usergroups:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.usergroups.addUsers")]
    pub async fn admin_usergroups_add_users(
        &self,
        request: &AdminUsergroupsAddUsersRequest,
    ) -> Result<AdminUsergroupsAddUsersResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.usergroups.create`](https://docs.slack.dev/reference/methods/admin.usergroups.create): Create a new organizational usergroup.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.usergroups:write`
    ///
    /// Rate limit: Tier 1 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.usergroups.create")]
    pub async fn admin_usergroups_create(
        &self,
        request: &AdminUsergroupsCreateRequest,
    ) -> Result<AdminUsergroupsCreateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.usergroups.fetch`](https://docs.slack.dev/reference/methods/admin.usergroups.fetch): Fetch an organizational usergroup.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.usergroups:read`
    ///
    /// Rate limit: Tier 1 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.usergroups.fetch")]
    pub async fn admin_usergroups_fetch(
        &self,
        request: &AdminUsergroupsFetchRequest,
    ) -> Result<AdminUsergroupsFetchResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.usergroups.listChannels`](https://docs.slack.dev/reference/methods/admin.usergroups.listChannels): List the channels linked to an org-level IDP group (user group).
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.usergroups:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.usergroups.listChannels")]
    pub async fn admin_usergroups_list_channels(
        &self,
        request: &AdminUsergroupsListChannelsRequest,
    ) -> Result<AdminUsergroupsListChannelsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.usergroups.removeChannels`](https://docs.slack.dev/reference/methods/admin.usergroups.removeChannels): Remove one or more default channels from an org-level IDP group (user group).
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.usergroups:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.usergroups.removeChannels")]
    pub async fn admin_usergroups_remove_channels(
        &self,
        request: &AdminUsergroupsRemoveChannelsRequest,
    ) -> Result<AdminUsergroupsRemoveChannelsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.usergroups.removeTeams`](https://docs.slack.dev/reference/methods/admin.usergroups.removeTeams): Remove one or more default workspaces from an organization-wide IDP Group or Admin Group
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
    #[doc(alias = "admin.usergroups.removeTeams")]
    pub async fn admin_usergroups_remove_teams(
        &self,
        request: &AdminUsergroupsRemoveTeamsRequest,
    ) -> Result<AdminUsergroupsRemoveTeamsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.usergroups.removeUsers`](https://docs.slack.dev/reference/methods/admin.usergroups.removeUsers): Remove members from an existing organizational usergroup.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.usergroups:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.usergroups.removeUsers")]
    pub async fn admin_usergroups_remove_users(
        &self,
        request: &AdminUsergroupsRemoveUsersRequest,
    ) -> Result<AdminUsergroupsRemoveUsersResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.usergroups.update`](https://docs.slack.dev/reference/methods/admin.usergroups.update): Update one or more properties of an existing organizational usergroup.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.usergroups:write`
    ///
    /// Rate limit: Tier 1 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.usergroups.update")]
    pub async fn admin_usergroups_update(
        &self,
        request: &AdminUsergroupsUpdateRequest,
    ) -> Result<AdminUsergroupsUpdateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.usergroups.uploadUsers`](https://docs.slack.dev/reference/methods/admin.usergroups.uploadUsers): Add members to an existing organizational usergroup in bulk via CSV upload.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.usergroups:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.usergroups.uploadUsers")]
    pub async fn admin_usergroups_upload_users(
        &self,
        request: &AdminUsergroupsUploadUsersRequest,
    ) -> Result<AdminUsergroupsUploadUsersResponse, SlackError> {
        self.call(request).await
    }
}
