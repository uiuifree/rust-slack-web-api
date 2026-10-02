// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`usergroups.create`](https://docs.slack.dev/reference/methods/usergroups.create): Create a User Group.
///
/// Send it with [`SlackClient::usergroups_create`].
#[doc(alias = "usergroups.create")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct UsergroupsCreateRequest {
    /// A name for the User Group. Must be unique among User Groups.
    pub name: String,
    /// A comma separated string of encoded channel IDs for which the User Group uses as a default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    /// A comma separated string of encoded channel IDs for which the User Group can custom add usergroup members too.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_channels: Option<Vec<String>>,
    /// A short description of the User Group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A mention handle. Must be unique among channels, users and User Groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// Include the number of users in each User Group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_count: Option<bool>,
    /// Encoded team id where the user group has to be created, required if org token is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Configure this user group to show as a sidebar section for all group members. Note: Only relevant if group has 1 or more default channels added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_section: Option<bool>,
}

impl UsergroupsCreateRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            channels: None,
            additional_channels: None,
            description: None,
            handle: None,
            include_count: None,
            team_id: None,
            enable_section: None,
        }
    }

    pub fn channels(mut self, channels: Vec<String>) -> Self {
        self.channels = Some(channels);
        self
    }

    pub fn additional_channels(mut self, additional_channels: Vec<String>) -> Self {
        self.additional_channels = Some(additional_channels);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn handle(mut self, handle: impl Into<String>) -> Self {
        self.handle = Some(handle.into());
        self
    }

    pub fn include_count(mut self, include_count: bool) -> Self {
        self.include_count = Some(include_count);
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn enable_section(mut self, enable_section: bool) -> Self {
        self.enable_section = Some(enable_section);
        self
    }
}

impl SlackApiMethod for UsergroupsCreateRequest {
    const METHOD: &'static str = "usergroups.create";
    type Response = UsergroupsCreateResponse;
}

/// Successful response of the Slack Web API method [`usergroups.create`](https://docs.slack.dev/reference/methods/usergroups.create).
#[doc(alias = "usergroups.create")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsergroupsCreateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub usergroup: Option<Usergroup>,
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

/// Arguments for the Slack Web API method [`usergroups.disable`](https://docs.slack.dev/reference/methods/usergroups.disable): Disable an existing User Group.
///
/// Send it with [`SlackClient::usergroups_disable`].
#[doc(alias = "usergroups.disable")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct UsergroupsDisableRequest {
    /// The encoded ID of the User Group to disable.
    pub usergroup: String,
    /// Include the number of users in the User Group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_count: Option<bool>,
    /// Encoded target team id where the user group is, required if org token is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl UsergroupsDisableRequest {
    pub fn new(usergroup: impl Into<String>) -> Self {
        Self {
            usergroup: usergroup.into(),
            include_count: None,
            team_id: None,
        }
    }

    pub fn include_count(mut self, include_count: bool) -> Self {
        self.include_count = Some(include_count);
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for UsergroupsDisableRequest {
    const METHOD: &'static str = "usergroups.disable";
    type Response = UsergroupsDisableResponse;
}

/// Successful response of the Slack Web API method [`usergroups.disable`](https://docs.slack.dev/reference/methods/usergroups.disable).
#[doc(alias = "usergroups.disable")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsergroupsDisableResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub usergroup: Option<Usergroup>,
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

/// Arguments for the Slack Web API method [`usergroups.enable`](https://docs.slack.dev/reference/methods/usergroups.enable): Enable a User Group.
///
/// Send it with [`SlackClient::usergroups_enable`].
#[doc(alias = "usergroups.enable")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct UsergroupsEnableRequest {
    /// The encoded ID of the User Group to enable.
    pub usergroup: String,
    /// Include the number of users in the User Group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_count: Option<bool>,
    /// Encoded team id where the user group is, required if org token is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl UsergroupsEnableRequest {
    pub fn new(usergroup: impl Into<String>) -> Self {
        Self {
            usergroup: usergroup.into(),
            include_count: None,
            team_id: None,
        }
    }

    pub fn include_count(mut self, include_count: bool) -> Self {
        self.include_count = Some(include_count);
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for UsergroupsEnableRequest {
    const METHOD: &'static str = "usergroups.enable";
    type Response = UsergroupsEnableResponse;
}

/// Successful response of the Slack Web API method [`usergroups.enable`](https://docs.slack.dev/reference/methods/usergroups.enable).
#[doc(alias = "usergroups.enable")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsergroupsEnableResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub usergroup: Option<Usergroup>,
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

/// Arguments for the Slack Web API method [`usergroups.list`](https://docs.slack.dev/reference/methods/usergroups.list): List all User Groups for a team.
///
/// Send it with [`SlackClient::usergroups_list`].
#[doc(alias = "usergroups.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct UsergroupsListRequest {
    /// Include the number of users in each User Group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_count: Option<bool>,
    /// Include results for disabled User Groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_disabled: Option<bool>,
    /// Include the list of users for each User Group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_users: Option<bool>,
    /// The user group's encoded team ID. Required if org token is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl UsergroupsListRequest {
    pub fn new() -> Self {
        Self {
            include_count: None,
            include_disabled: None,
            include_users: None,
            team_id: None,
        }
    }

    pub fn include_count(mut self, include_count: bool) -> Self {
        self.include_count = Some(include_count);
        self
    }

    pub fn include_disabled(mut self, include_disabled: bool) -> Self {
        self.include_disabled = Some(include_disabled);
        self
    }

    pub fn include_users(mut self, include_users: bool) -> Self {
        self.include_users = Some(include_users);
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for UsergroupsListRequest {
    const METHOD: &'static str = "usergroups.list";
    type Response = UsergroupsListResponse;
}

/// Successful response of the Slack Web API method [`usergroups.list`](https://docs.slack.dev/reference/methods/usergroups.list).
#[doc(alias = "usergroups.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsergroupsListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub usergroups: Vec<Usergroup>,
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

/// Arguments for the Slack Web API method [`usergroups.update`](https://docs.slack.dev/reference/methods/usergroups.update): Update an existing User Group.
///
/// Send it with [`SlackClient::usergroups_update`].
#[doc(alias = "usergroups.update")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct UsergroupsUpdateRequest {
    /// The encoded ID of the User Group to update.
    pub usergroup: String,
    /// A comma separated string of encoded channel IDs for which the User Group uses as a default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    /// A comma separated string of encoded channel IDs for which the User Group can custom add usergroup members too.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_channels: Option<Vec<String>>,
    /// A short description of the User Group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A mention handle. Must be unique among channels, users and User Groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// Include the number of users in the User Group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_count: Option<bool>,
    /// A name for the User Group. Must be unique among User Groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// encoded team id where the user group exists, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Configure this user group to show as a sidebar section for all group members. Note: Only relevant if group has 1 or more default channels added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_section: Option<bool>,
}

impl UsergroupsUpdateRequest {
    pub fn new(usergroup: impl Into<String>) -> Self {
        Self {
            usergroup: usergroup.into(),
            channels: None,
            additional_channels: None,
            description: None,
            handle: None,
            include_count: None,
            name: None,
            team_id: None,
            enable_section: None,
        }
    }

    pub fn channels(mut self, channels: Vec<String>) -> Self {
        self.channels = Some(channels);
        self
    }

    pub fn additional_channels(mut self, additional_channels: Vec<String>) -> Self {
        self.additional_channels = Some(additional_channels);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn handle(mut self, handle: impl Into<String>) -> Self {
        self.handle = Some(handle.into());
        self
    }

    pub fn include_count(mut self, include_count: bool) -> Self {
        self.include_count = Some(include_count);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn enable_section(mut self, enable_section: bool) -> Self {
        self.enable_section = Some(enable_section);
        self
    }
}

impl SlackApiMethod for UsergroupsUpdateRequest {
    const METHOD: &'static str = "usergroups.update";
    type Response = UsergroupsUpdateResponse;
}

/// Successful response of the Slack Web API method [`usergroups.update`](https://docs.slack.dev/reference/methods/usergroups.update).
#[doc(alias = "usergroups.update")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsergroupsUpdateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub usergroup: Option<Usergroup>,
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

/// Arguments for the Slack Web API method [`usergroups.users.list`](https://docs.slack.dev/reference/methods/usergroups.users.list): List all users in a User Group.
///
/// Send it with [`SlackClient::usergroups_users_list`].
#[doc(alias = "usergroups.users.list")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct UsergroupsUsersListRequest {
    /// The encoded ID of the User Group.
    pub usergroup: String,
    /// Include results for disabled User Groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_disabled: Option<bool>,
    /// The user group's encoded team ID. Required if org token is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl UsergroupsUsersListRequest {
    pub fn new(usergroup: impl Into<String>) -> Self {
        Self {
            usergroup: usergroup.into(),
            include_disabled: None,
            team_id: None,
        }
    }

    pub fn include_disabled(mut self, include_disabled: bool) -> Self {
        self.include_disabled = Some(include_disabled);
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for UsergroupsUsersListRequest {
    const METHOD: &'static str = "usergroups.users.list";
    type Response = UsergroupsUsersListResponse;
}

/// Successful response of the Slack Web API method [`usergroups.users.list`](https://docs.slack.dev/reference/methods/usergroups.users.list).
#[doc(alias = "usergroups.users.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsergroupsUsersListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub users: Vec<String>,
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

/// Arguments for the Slack Web API method [`usergroups.users.update`](https://docs.slack.dev/reference/methods/usergroups.users.update): Update the list of users for a user group.
///
/// Send it with [`SlackClient::usergroups_users_update`].
#[doc(alias = "usergroups.users.update")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct UsergroupsUsersUpdateRequest {
    /// The encoded ID of the user group to update.
    pub usergroup: String,
    /// A comma separated string of encoded user IDs that represent the entire list of users for the user group.
    pub users: Vec<String>,
    /// Include the number of users in the user group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_count: Option<bool>,
    /// encoded team id where the user group exists, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// A comma separated string of encoded channel IDs for which the User Group can custom add usergroup members too.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_channels: Option<Vec<String>>,
    /// Boolean to identify if the API is getting called when a shared section is getting shared
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_shared: Option<bool>,
}

impl UsergroupsUsersUpdateRequest {
    pub fn new(usergroup: impl Into<String>, users: Vec<String>) -> Self {
        Self {
            usergroup: usergroup.into(),
            users,
            include_count: None,
            team_id: None,
            additional_channels: None,
            is_shared: None,
        }
    }

    pub fn include_count(mut self, include_count: bool) -> Self {
        self.include_count = Some(include_count);
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn additional_channels(mut self, additional_channels: Vec<String>) -> Self {
        self.additional_channels = Some(additional_channels);
        self
    }

    pub fn is_shared(mut self, is_shared: bool) -> Self {
        self.is_shared = Some(is_shared);
        self
    }
}

impl SlackApiMethod for UsergroupsUsersUpdateRequest {
    const METHOD: &'static str = "usergroups.users.update";
    type Response = UsergroupsUsersUpdateResponse;
}

/// Successful response of the Slack Web API method [`usergroups.users.update`](https://docs.slack.dev/reference/methods/usergroups.users.update).
#[doc(alias = "usergroups.users.update")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsergroupsUsersUpdateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub usergroup: Option<Usergroup>,
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
    /// Calls the Slack Web API method [`usergroups.create`](https://docs.slack.dev/reference/methods/usergroups.create): Create a User Group.
    ///
    /// Required scopes:
    ///
    /// - bot token: `usergroups:write`
    /// - user token: `usergroups:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "usergroups.create")]
    pub async fn usergroups_create(
        &self,
        request: &UsergroupsCreateRequest,
    ) -> Result<UsergroupsCreateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`usergroups.disable`](https://docs.slack.dev/reference/methods/usergroups.disable): Disable an existing User Group.
    ///
    /// Required scopes:
    ///
    /// - bot token: `usergroups:write`
    /// - user token: `usergroups:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "usergroups.disable")]
    pub async fn usergroups_disable(
        &self,
        request: &UsergroupsDisableRequest,
    ) -> Result<UsergroupsDisableResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`usergroups.enable`](https://docs.slack.dev/reference/methods/usergroups.enable): Enable a User Group.
    ///
    /// Required scopes:
    ///
    /// - bot token: `usergroups:write`
    /// - user token: `usergroups:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "usergroups.enable")]
    pub async fn usergroups_enable(
        &self,
        request: &UsergroupsEnableRequest,
    ) -> Result<UsergroupsEnableResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`usergroups.list`](https://docs.slack.dev/reference/methods/usergroups.list): List all User Groups for a team.
    ///
    /// Required scopes:
    ///
    /// - bot token: `usergroups:read`
    /// - user token: `usergroups:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "usergroups.list")]
    pub async fn usergroups_list(
        &self,
        request: &UsergroupsListRequest,
    ) -> Result<UsergroupsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`usergroups.update`](https://docs.slack.dev/reference/methods/usergroups.update): Update an existing User Group.
    ///
    /// Required scopes:
    ///
    /// - bot token: `usergroups:write`
    /// - user token: `usergroups:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "usergroups.update")]
    pub async fn usergroups_update(
        &self,
        request: &UsergroupsUpdateRequest,
    ) -> Result<UsergroupsUpdateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`usergroups.users.list`](https://docs.slack.dev/reference/methods/usergroups.users.list): List all users in a User Group.
    ///
    /// Required scopes:
    ///
    /// - bot token: `usergroups:read`
    /// - user token: `usergroups:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "usergroups.users.list")]
    pub async fn usergroups_users_list(
        &self,
        request: &UsergroupsUsersListRequest,
    ) -> Result<UsergroupsUsersListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`usergroups.users.update`](https://docs.slack.dev/reference/methods/usergroups.users.update): Update the list of users for a user group.
    ///
    /// Required scopes:
    ///
    /// - bot token: `usergroups:write`
    /// - user token: `usergroups:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "usergroups.users.update")]
    pub async fn usergroups_users_update(
        &self,
        request: &UsergroupsUsersUpdateRequest,
    ) -> Result<UsergroupsUsersUpdateResponse, SlackError> {
        self.call(request).await
    }
}
