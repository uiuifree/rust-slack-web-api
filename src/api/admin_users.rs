// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`admin.users.assign`](https://docs.slack.dev/reference/methods/admin.users.assign): Add an Enterprise user to a workspace.
///
/// Send it with [`SlackClient::admin_users_assign`].
#[doc(alias = "admin.users.assign")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsersAssignRequest {
    /// The ID (`T1234`) of the workspace.
    pub team_id: String,
    /// The ID of the user to add to the workspace.
    pub user_id: String,
    /// True if user should be added to the workspace as a guest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_restricted: Option<bool>,
    /// True if user should be added to the workspace as a single-channel guest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ultra_restricted: Option<bool>,
    /// Comma separated values of channel IDs to add user in the new workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<String>,
}

impl AdminUsersAssignRequest {
    pub fn new(team_id: impl Into<String>, user_id: impl Into<String>) -> Self {
        Self {
            team_id: team_id.into(),
            user_id: user_id.into(),
            is_restricted: None,
            is_ultra_restricted: None,
            channel_ids: None,
        }
    }

    pub fn is_restricted(mut self, is_restricted: bool) -> Self {
        self.is_restricted = Some(is_restricted);
        self
    }

    pub fn is_ultra_restricted(mut self, is_ultra_restricted: bool) -> Self {
        self.is_ultra_restricted = Some(is_ultra_restricted);
        self
    }

    pub fn channel_ids(mut self, channel_ids: impl Into<String>) -> Self {
        self.channel_ids = Some(channel_ids.into());
        self
    }
}

impl SlackApiMethod for AdminUsersAssignRequest {
    const METHOD: &'static str = "admin.users.assign";
    type Response = AdminUsersAssignResponse;
}

/// Successful response of the Slack Web API method [`admin.users.assign`](https://docs.slack.dev/reference/methods/admin.users.assign).
#[doc(alias = "admin.users.assign")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersAssignResponse {
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

/// Arguments for the Slack Web API method [`admin.users.getExpiration`](https://docs.slack.dev/reference/methods/admin.users.getExpiration): Fetches the expiration timestamp for a guest.
///
/// Send it with [`SlackClient::admin_users_get_expiration`].
#[doc(alias = "admin.users.getExpiration")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsersGetExpirationRequest {
    /// The ID of the guest user to get the expiration for.
    pub user_id: String,
    /// If an org token is passed in and this team is on the org, it will operate on the workspace level on the specified team. Otherwise it will operate on the org or team in context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_team: Option<String>,
}

impl AdminUsersGetExpirationRequest {
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            target_team: None,
        }
    }

    pub fn target_team(mut self, target_team: impl Into<String>) -> Self {
        self.target_team = Some(target_team.into());
        self
    }
}

impl SlackApiMethod for AdminUsersGetExpirationRequest {
    const METHOD: &'static str = "admin.users.getExpiration";
    type Response = AdminUsersGetExpirationResponse;
}

/// Successful response of the Slack Web API method [`admin.users.getExpiration`](https://docs.slack.dev/reference/methods/admin.users.getExpiration).
#[doc(alias = "admin.users.getExpiration")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersGetExpirationResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<AdminUsersGetExpirationResponseUser>,
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
pub struct AdminUsersGetExpirationResponseUser {
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
    pub email: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_restricted: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_ultra_restricted: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration_ts: Option<i64>,
}

/// Arguments for the Slack Web API method [`admin.users.invite`](https://docs.slack.dev/reference/methods/admin.users.invite): Invite a user to a workspace.
///
/// Send it with [`SlackClient::admin_users_invite`].
#[doc(alias = "admin.users.invite")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsersInviteRequest {
    /// The ID (`T1234`) of the workspace.
    pub team_id: String,
    /// The email address of the person to invite.
    pub email: String,
    /// A comma-separated list of `channel_id`s for this user to join. At least one channel is required.
    pub channel_ids: String,
    /// An optional message to send to the user in the invite email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
    /// Full name of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_name: Option<String>,
    /// Allow this invite to be resent in the future if a user has not signed up yet. Resending can only be done via the UI and has no expiration. (default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resend: Option<bool>,
    /// Is this user a multi-channel guest user? (default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_restricted: Option<bool>,
    /// Is this user a single channel guest user? (default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ultra_restricted: Option<bool>,
    /// Timestamp when guest account should be disabled. Only include this timestamp if you are inviting a guest user and you want their account to expire on a certain date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_expiration_ts: Option<String>,
    /// Allow invited user to sign in via email and password. Only available for Enterprise org teams via admin invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_password_policy_enabled: Option<bool>,
}

impl AdminUsersInviteRequest {
    pub fn new(
        team_id: impl Into<String>,
        email: impl Into<String>,
        channel_ids: impl Into<String>,
    ) -> Self {
        Self {
            team_id: team_id.into(),
            email: email.into(),
            channel_ids: channel_ids.into(),
            custom_message: None,
            real_name: None,
            resend: None,
            is_restricted: None,
            is_ultra_restricted: None,
            guest_expiration_ts: None,
            email_password_policy_enabled: None,
        }
    }

    pub fn custom_message(mut self, custom_message: impl Into<String>) -> Self {
        self.custom_message = Some(custom_message.into());
        self
    }

    pub fn real_name(mut self, real_name: impl Into<String>) -> Self {
        self.real_name = Some(real_name.into());
        self
    }

    pub fn resend(mut self, resend: bool) -> Self {
        self.resend = Some(resend);
        self
    }

    pub fn is_restricted(mut self, is_restricted: bool) -> Self {
        self.is_restricted = Some(is_restricted);
        self
    }

    pub fn is_ultra_restricted(mut self, is_ultra_restricted: bool) -> Self {
        self.is_ultra_restricted = Some(is_ultra_restricted);
        self
    }

    pub fn guest_expiration_ts(mut self, guest_expiration_ts: impl Into<String>) -> Self {
        self.guest_expiration_ts = Some(guest_expiration_ts.into());
        self
    }

    pub fn email_password_policy_enabled(mut self, email_password_policy_enabled: bool) -> Self {
        self.email_password_policy_enabled = Some(email_password_policy_enabled);
        self
    }
}

impl SlackApiMethod for AdminUsersInviteRequest {
    const METHOD: &'static str = "admin.users.invite";
    type Response = AdminUsersInviteResponse;
}

/// Successful response of the Slack Web API method [`admin.users.invite`](https://docs.slack.dev/reference/methods/admin.users.invite).
#[doc(alias = "admin.users.invite")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersInviteResponse {
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

/// Arguments for the Slack Web API method [`admin.users.list`](https://docs.slack.dev/reference/methods/admin.users.list): List users on a workspace
///
/// Send it with [`SlackClient::admin_users_list`].
#[doc(alias = "admin.users.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminUsersListRequest {
    /// The ID (T1234) of a workspace. Filters results to just the specified workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only active users will be returned. If false, only deactivated users will be returned. Default is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Only applies with org token and no team\_id. If true, return `workspaces` for a user even if they may be deactivated on them. If false, return `workspaces` for a user only when user is active on them. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deactivated_user_workspaces: Option<bool>,
    /// If true, returns only guests and their expiration dates that belong to the team\_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_guests: Option<bool>,
    /// If true, only admin users will be returned (excludes owners). Returns all admins and owners when combined with `include_owners`. Cannot be used together with `only_guests`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_admins: Option<bool>,
    /// If true, only owner users will be returned. Cannot be used together with `only_guests`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_owners: Option<bool>,
    /// Limit for how many users to be retrieved per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl AdminUsersListRequest {
    pub fn new() -> Self {
        Self {
            team_id: None,
            cursor: None,
            is_active: None,
            include_deactivated_user_workspaces: None,
            only_guests: None,
            include_admins: None,
            include_owners: None,
            limit: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }

    pub fn include_deactivated_user_workspaces(
        mut self,
        include_deactivated_user_workspaces: bool,
    ) -> Self {
        self.include_deactivated_user_workspaces = Some(include_deactivated_user_workspaces);
        self
    }

    pub fn only_guests(mut self, only_guests: bool) -> Self {
        self.only_guests = Some(only_guests);
        self
    }

    pub fn include_admins(mut self, include_admins: bool) -> Self {
        self.include_admins = Some(include_admins);
        self
    }

    pub fn include_owners(mut self, include_owners: bool) -> Self {
        self.include_owners = Some(include_owners);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl SlackApiMethod for AdminUsersListRequest {
    const METHOD: &'static str = "admin.users.list";
    type Response = AdminUsersListResponse;
}

impl CursorPaginated for AdminUsersListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminUsersListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.users.list`](https://docs.slack.dev/reference/methods/admin.users.list).
#[doc(alias = "admin.users.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub users: Vec<User>,
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

/// Arguments for the Slack Web API method [`admin.users.remove`](https://docs.slack.dev/reference/methods/admin.users.remove): Remove a user from a workspace.
///
/// Send it with [`SlackClient::admin_users_remove`].
#[doc(alias = "admin.users.remove")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsersRemoveRequest {
    /// The ID (`T1234`) of the workspace.
    pub team_id: String,
    /// The ID of the user to remove.
    pub user_id: String,
}

impl AdminUsersRemoveRequest {
    pub fn new(team_id: impl Into<String>, user_id: impl Into<String>) -> Self {
        Self {
            team_id: team_id.into(),
            user_id: user_id.into(),
        }
    }
}

impl SlackApiMethod for AdminUsersRemoveRequest {
    const METHOD: &'static str = "admin.users.remove";
    type Response = AdminUsersRemoveResponse;
}

/// Successful response of the Slack Web API method [`admin.users.remove`](https://docs.slack.dev/reference/methods/admin.users.remove).
#[doc(alias = "admin.users.remove")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersRemoveResponse {
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

/// Arguments for the Slack Web API method [`admin.users.session.clearSettings`](https://docs.slack.dev/reference/methods/admin.users.session.clearSettings): Clear user-specific session settings—the session duration and what happens when the client closes—for a list of users.
///
/// Send it with [`SlackClient::admin_users_session_clear_settings`].
#[doc(alias = "admin.users.session.clearSettings")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsersSessionClearSettingsRequest {
    /// The IDs of users you'd like to clear session settings for.
    #[serde(serialize_with = "crate::form::as_json")]
    pub user_ids: Vec<String>,
}

impl AdminUsersSessionClearSettingsRequest {
    pub fn new(user_ids: Vec<String>) -> Self {
        Self { user_ids }
    }
}

impl SlackApiMethod for AdminUsersSessionClearSettingsRequest {
    const METHOD: &'static str = "admin.users.session.clearSettings";
    type Response = AdminUsersSessionClearSettingsResponse;
}

/// Successful response of the Slack Web API method [`admin.users.session.clearSettings`](https://docs.slack.dev/reference/methods/admin.users.session.clearSettings).
#[doc(alias = "admin.users.session.clearSettings")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersSessionClearSettingsResponse {
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

/// Arguments for the Slack Web API method [`admin.users.session.getSettings`](https://docs.slack.dev/reference/methods/admin.users.session.getSettings): Get user-specific session settings—the session duration and what happens when the client closes—given a list of users.
///
/// Send it with [`SlackClient::admin_users_session_get_settings`].
#[doc(alias = "admin.users.session.getSettings")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsersSessionGetSettingsRequest {
    /// The IDs of users you'd like to fetch session settings for. Note: if a user does not have any active sessions, they will not be returned in the response.
    #[serde(serialize_with = "crate::form::as_json")]
    pub user_ids: Vec<String>,
}

impl AdminUsersSessionGetSettingsRequest {
    pub fn new(user_ids: Vec<String>) -> Self {
        Self { user_ids }
    }
}

impl SlackApiMethod for AdminUsersSessionGetSettingsRequest {
    const METHOD: &'static str = "admin.users.session.getSettings";
    type Response = AdminUsersSessionGetSettingsResponse;
}

/// Successful response of the Slack Web API method [`admin.users.session.getSettings`](https://docs.slack.dev/reference/methods/admin.users.session.getSettings).
#[doc(alias = "admin.users.session.getSettings")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersSessionGetSettingsResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub session_settings: Vec<AdminUsersSessionGetSettingsResponseSessionSettings>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub no_settings_applied: Vec<serde_json::Value>,
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
pub struct AdminUsersSessionGetSettingsResponseSessionSettings {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub desktop_app_browser_quit: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub duration: Option<i64>,
}

/// Arguments for the Slack Web API method [`admin.users.session.invalidate`](https://docs.slack.dev/reference/methods/admin.users.session.invalidate): Revoke a single session for a user. The user will be forced to login to Slack.
///
/// Send it with [`SlackClient::admin_users_session_invalidate`].
#[doc(alias = "admin.users.session.invalidate")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsersSessionInvalidateRequest {
    /// ID of the user that the session belongs to.
    pub user_id: String,
    /// ID of the session to invalidate.
    pub session_id: i64,
}

impl AdminUsersSessionInvalidateRequest {
    pub fn new(user_id: impl Into<String>, session_id: i64) -> Self {
        Self {
            user_id: user_id.into(),
            session_id,
        }
    }
}

impl SlackApiMethod for AdminUsersSessionInvalidateRequest {
    const METHOD: &'static str = "admin.users.session.invalidate";
    type Response = AdminUsersSessionInvalidateResponse;
}

/// Successful response of the Slack Web API method [`admin.users.session.invalidate`](https://docs.slack.dev/reference/methods/admin.users.session.invalidate).
#[doc(alias = "admin.users.session.invalidate")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersSessionInvalidateResponse {
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

/// Arguments for the Slack Web API method [`admin.users.session.list`](https://docs.slack.dev/reference/methods/admin.users.session.list): List active user sessions for an organization
///
/// Send it with [`SlackClient::admin_users_session_list`].
#[doc(alias = "admin.users.session.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminUsersSessionListRequest {
    /// The ID of the workspace you'd like active sessions for. If you pass a `team_id`, you'll need to pass a `user_id` as well.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// The ID of user you'd like active sessions for. If you pass a `user_id`, you'll need to pass a `team_id` as well.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The maximum number of items to return. Must be between 1 - 1000 both inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AdminUsersSessionListRequest {
    pub fn new() -> Self {
        Self {
            team_id: None,
            user_id: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
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

impl SlackApiMethod for AdminUsersSessionListRequest {
    const METHOD: &'static str = "admin.users.session.list";
    type Response = AdminUsersSessionListResponse;
}

impl CursorPaginated for AdminUsersSessionListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminUsersSessionListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.users.session.list`](https://docs.slack.dev/reference/methods/admin.users.session.list).
#[doc(alias = "admin.users.session.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersSessionListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub active_sessions: Vec<AdminUsersSessionListResponseActiveSessions>,
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
pub struct AdminUsersSessionListResponseActiveSessions {
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
    pub team_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub session_id: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub recent: Option<AdminUsersSessionListResponseActiveSessionsRecent>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub created: Option<AdminUsersSessionListResponseActiveSessionsCreated>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersSessionListResponseActiveSessionsRecent {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub device_hardware: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub os: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub os_version: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_client_version: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub ip: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersSessionListResponseActiveSessionsCreated {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub device_hardware: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub os: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub os_version: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_client_version: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub ip: Option<String>,
}

/// Arguments for the Slack Web API method [`admin.users.session.reset`](https://docs.slack.dev/reference/methods/admin.users.session.reset): Wipes all valid sessions on all devices for a given user
///
/// Send it with [`SlackClient::admin_users_session_reset`].
#[doc(alias = "admin.users.session.reset")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsersSessionResetRequest {
    /// The ID of the user to wipe sessions for
    pub user_id: String,
    /// Only expire mobile sessions (default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_only: Option<bool>,
    /// Only expire web sessions (default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_only: Option<bool>,
}

impl AdminUsersSessionResetRequest {
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            mobile_only: None,
            web_only: None,
        }
    }

    pub fn mobile_only(mut self, mobile_only: bool) -> Self {
        self.mobile_only = Some(mobile_only);
        self
    }

    pub fn web_only(mut self, web_only: bool) -> Self {
        self.web_only = Some(web_only);
        self
    }
}

impl SlackApiMethod for AdminUsersSessionResetRequest {
    const METHOD: &'static str = "admin.users.session.reset";
    type Response = AdminUsersSessionResetResponse;
}

/// Successful response of the Slack Web API method [`admin.users.session.reset`](https://docs.slack.dev/reference/methods/admin.users.session.reset).
#[doc(alias = "admin.users.session.reset")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersSessionResetResponse {
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

/// Arguments for the Slack Web API method [`admin.users.session.resetBulk`](https://docs.slack.dev/reference/methods/admin.users.session.resetBulk): Enqueues an asynchronous job to wipe all valid sessions on all devices for a given list of users
///
/// Send it with [`SlackClient::admin_users_session_reset_bulk`].
#[doc(alias = "admin.users.session.resetBulk")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsersSessionResetBulkRequest {
    /// The list of up to 1,000 user IDs to wipe sessions for
    #[serde(serialize_with = "crate::form::as_json")]
    pub user_ids: Vec<String>,
    /// Only expire mobile sessions (default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_only: Option<bool>,
    /// Only expire web sessions (default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_only: Option<bool>,
}

impl AdminUsersSessionResetBulkRequest {
    pub fn new(user_ids: Vec<String>) -> Self {
        Self {
            user_ids,
            mobile_only: None,
            web_only: None,
        }
    }

    pub fn mobile_only(mut self, mobile_only: bool) -> Self {
        self.mobile_only = Some(mobile_only);
        self
    }

    pub fn web_only(mut self, web_only: bool) -> Self {
        self.web_only = Some(web_only);
        self
    }
}

impl SlackApiMethod for AdminUsersSessionResetBulkRequest {
    const METHOD: &'static str = "admin.users.session.resetBulk";
    type Response = AdminUsersSessionResetBulkResponse;
}

/// Successful response of the Slack Web API method [`admin.users.session.resetBulk`](https://docs.slack.dev/reference/methods/admin.users.session.resetBulk).
#[doc(alias = "admin.users.session.resetBulk")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersSessionResetBulkResponse {
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

/// Arguments for the Slack Web API method [`admin.users.session.setSettings`](https://docs.slack.dev/reference/methods/admin.users.session.setSettings): Configure the user-level session settings—the session duration and what happens when the client closes—for one or more users.
///
/// Send it with [`SlackClient::admin_users_session_set_settings`].
#[doc(alias = "admin.users.session.setSettings")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsersSessionSetSettingsRequest {
    /// The list of up to 1,000 user IDs to apply the session settings for
    #[serde(serialize_with = "crate::form::as_json")]
    pub user_ids: Vec<String>,
    /// The session duration, in seconds. The minimum value is 28800, which represents 8 hours; the max value is 315569520 or 10 years (that's a long Slack session).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Terminate the session when the client—either the desktop app or a browser window—is closed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop_app_browser_quit: Option<bool>,
}

impl AdminUsersSessionSetSettingsRequest {
    pub fn new(user_ids: Vec<String>) -> Self {
        Self {
            user_ids,
            duration: None,
            desktop_app_browser_quit: None,
        }
    }

    pub fn duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn desktop_app_browser_quit(mut self, desktop_app_browser_quit: bool) -> Self {
        self.desktop_app_browser_quit = Some(desktop_app_browser_quit);
        self
    }
}

impl SlackApiMethod for AdminUsersSessionSetSettingsRequest {
    const METHOD: &'static str = "admin.users.session.setSettings";
    type Response = AdminUsersSessionSetSettingsResponse;
}

/// Successful response of the Slack Web API method [`admin.users.session.setSettings`](https://docs.slack.dev/reference/methods/admin.users.session.setSettings).
#[doc(alias = "admin.users.session.setSettings")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersSessionSetSettingsResponse {
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

/// Arguments for the Slack Web API method [`admin.users.setAdmin`](https://docs.slack.dev/reference/methods/admin.users.setAdmin): Set an existing regular user or owner to be a workspace or org admin.
///
/// Send it with [`SlackClient::admin_users_set_admin`].
#[doc(alias = "admin.users.setAdmin")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsersSetAdminRequest {
    /// The ID of the workspace or organization.
    pub team_id: String,
    /// The ID of the user to designate as an admin.
    pub user_id: String,
}

impl AdminUsersSetAdminRequest {
    pub fn new(team_id: impl Into<String>, user_id: impl Into<String>) -> Self {
        Self {
            team_id: team_id.into(),
            user_id: user_id.into(),
        }
    }
}

impl SlackApiMethod for AdminUsersSetAdminRequest {
    const METHOD: &'static str = "admin.users.setAdmin";
    type Response = AdminUsersSetAdminResponse;
}

/// Successful response of the Slack Web API method [`admin.users.setAdmin`](https://docs.slack.dev/reference/methods/admin.users.setAdmin).
#[doc(alias = "admin.users.setAdmin")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersSetAdminResponse {
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

/// Arguments for the Slack Web API method [`admin.users.setExpiration`](https://docs.slack.dev/reference/methods/admin.users.setExpiration): Set an expiration for a guest user
///
/// Send it with [`SlackClient::admin_users_set_expiration`].
#[doc(alias = "admin.users.setExpiration")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsersSetExpirationRequest {
    /// The ID of the user to set an expiration for.
    pub user_id: String,
    /// Epoch timestamp in seconds when guest account should be disabled.
    pub expiration_ts: i64,
    /// The ID (`T1234`) of the workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl AdminUsersSetExpirationRequest {
    pub fn new(user_id: impl Into<String>, expiration_ts: i64) -> Self {
        Self {
            user_id: user_id.into(),
            expiration_ts,
            team_id: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for AdminUsersSetExpirationRequest {
    const METHOD: &'static str = "admin.users.setExpiration";
    type Response = AdminUsersSetExpirationResponse;
}

/// Successful response of the Slack Web API method [`admin.users.setExpiration`](https://docs.slack.dev/reference/methods/admin.users.setExpiration).
#[doc(alias = "admin.users.setExpiration")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersSetExpirationResponse {
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

/// Arguments for the Slack Web API method [`admin.users.setOwner`](https://docs.slack.dev/reference/methods/admin.users.setOwner): Set an existing regular user or admin to be a workspace or org owner.
///
/// Send it with [`SlackClient::admin_users_set_owner`].
#[doc(alias = "admin.users.setOwner")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsersSetOwnerRequest {
    /// The ID of the workspace or organization.
    pub team_id: String,
    /// ID of the user to promote to owner.
    pub user_id: String,
}

impl AdminUsersSetOwnerRequest {
    pub fn new(team_id: impl Into<String>, user_id: impl Into<String>) -> Self {
        Self {
            team_id: team_id.into(),
            user_id: user_id.into(),
        }
    }
}

impl SlackApiMethod for AdminUsersSetOwnerRequest {
    const METHOD: &'static str = "admin.users.setOwner";
    type Response = AdminUsersSetOwnerResponse;
}

/// Successful response of the Slack Web API method [`admin.users.setOwner`](https://docs.slack.dev/reference/methods/admin.users.setOwner).
#[doc(alias = "admin.users.setOwner")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersSetOwnerResponse {
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

/// Arguments for the Slack Web API method [`admin.users.setRegular`](https://docs.slack.dev/reference/methods/admin.users.setRegular): Set an existing guest user, admin user, or owner to be a regular user.
///
/// Send it with [`SlackClient::admin_users_set_regular`].
#[doc(alias = "admin.users.setRegular")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminUsersSetRegularRequest {
    /// The ID of the workspace or organization.
    pub team_id: String,
    /// The ID of the user to designate as a regular user.
    pub user_id: String,
}

impl AdminUsersSetRegularRequest {
    pub fn new(team_id: impl Into<String>, user_id: impl Into<String>) -> Self {
        Self {
            team_id: team_id.into(),
            user_id: user_id.into(),
        }
    }
}

impl SlackApiMethod for AdminUsersSetRegularRequest {
    const METHOD: &'static str = "admin.users.setRegular";
    type Response = AdminUsersSetRegularResponse;
}

/// Successful response of the Slack Web API method [`admin.users.setRegular`](https://docs.slack.dev/reference/methods/admin.users.setRegular).
#[doc(alias = "admin.users.setRegular")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersSetRegularResponse {
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

/// Arguments for the Slack Web API method [`admin.users.unsupportedVersions.export`](https://docs.slack.dev/reference/methods/admin.users.unsupportedVersions.export): Ask Slack to send you an export listing all workspace members using unsupported software, presented as a zipped CSV file.
///
/// Send it with [`SlackClient::admin_users_unsupported_versions_export`].
#[doc(alias = "admin.users.unsupportedVersions.export")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminUsersUnsupportedVersionsExportRequest {
    /// Unix timestamp of a date to start looking for user sessions. If not provided will start six months ago.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_sessions_started: Option<i64>,
    /// Unix timestamp of the date of past or upcoming end of support cycles. If not provided will include all announced end of support cycles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_end_of_support: Option<i64>,
}

impl AdminUsersUnsupportedVersionsExportRequest {
    pub fn new() -> Self {
        Self {
            date_sessions_started: None,
            date_end_of_support: None,
        }
    }

    pub fn date_sessions_started(mut self, date_sessions_started: i64) -> Self {
        self.date_sessions_started = Some(date_sessions_started);
        self
    }

    pub fn date_end_of_support(mut self, date_end_of_support: i64) -> Self {
        self.date_end_of_support = Some(date_end_of_support);
        self
    }
}

impl SlackApiMethod for AdminUsersUnsupportedVersionsExportRequest {
    const METHOD: &'static str = "admin.users.unsupportedVersions.export";
    type Response = AdminUsersUnsupportedVersionsExportResponse;
}

/// Successful response of the Slack Web API method [`admin.users.unsupportedVersions.export`](https://docs.slack.dev/reference/methods/admin.users.unsupportedVersions.export).
#[doc(alias = "admin.users.unsupportedVersions.export")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminUsersUnsupportedVersionsExportResponse {
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
    /// Calls the Slack Web API method [`admin.users.assign`](https://docs.slack.dev/reference/methods/admin.users.assign): Add an Enterprise user to a workspace.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.users:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.users.assign")]
    pub async fn admin_users_assign(
        &self,
        request: &AdminUsersAssignRequest,
    ) -> Result<AdminUsersAssignResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.users.getExpiration`](https://docs.slack.dev/reference/methods/admin.users.getExpiration): Fetches the expiration timestamp for a guest.
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
    #[doc(alias = "admin.users.getExpiration")]
    pub async fn admin_users_get_expiration(
        &self,
        request: &AdminUsersGetExpirationRequest,
    ) -> Result<AdminUsersGetExpirationResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.users.invite`](https://docs.slack.dev/reference/methods/admin.users.invite): Invite a user to a workspace.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.users:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.users.invite")]
    pub async fn admin_users_invite(
        &self,
        request: &AdminUsersInviteRequest,
    ) -> Result<AdminUsersInviteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.users.list`](https://docs.slack.dev/reference/methods/admin.users.list): List users on a workspace
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
    #[doc(alias = "admin.users.list")]
    pub async fn admin_users_list(
        &self,
        request: &AdminUsersListRequest,
    ) -> Result<AdminUsersListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.users.remove`](https://docs.slack.dev/reference/methods/admin.users.remove): Remove a user from a workspace.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.users:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.users.remove")]
    pub async fn admin_users_remove(
        &self,
        request: &AdminUsersRemoveRequest,
    ) -> Result<AdminUsersRemoveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.users.session.clearSettings`](https://docs.slack.dev/reference/methods/admin.users.session.clearSettings): Clear user-specific session settings—the session duration and what happens when the client closes—for a list of users.
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
    #[doc(alias = "admin.users.session.clearSettings")]
    pub async fn admin_users_session_clear_settings(
        &self,
        request: &AdminUsersSessionClearSettingsRequest,
    ) -> Result<AdminUsersSessionClearSettingsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.users.session.getSettings`](https://docs.slack.dev/reference/methods/admin.users.session.getSettings): Get user-specific session settings—the session duration and what happens when the client closes—given a list of users.
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
    #[doc(alias = "admin.users.session.getSettings")]
    pub async fn admin_users_session_get_settings(
        &self,
        request: &AdminUsersSessionGetSettingsRequest,
    ) -> Result<AdminUsersSessionGetSettingsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.users.session.invalidate`](https://docs.slack.dev/reference/methods/admin.users.session.invalidate): Revoke a single session for a user. The user will be forced to login to Slack.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.users:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.users.session.invalidate")]
    pub async fn admin_users_session_invalidate(
        &self,
        request: &AdminUsersSessionInvalidateRequest,
    ) -> Result<AdminUsersSessionInvalidateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.users.session.list`](https://docs.slack.dev/reference/methods/admin.users.session.list): List active user sessions for an organization
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.users:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.users.session.list")]
    pub async fn admin_users_session_list(
        &self,
        request: &AdminUsersSessionListRequest,
    ) -> Result<AdminUsersSessionListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.users.session.reset`](https://docs.slack.dev/reference/methods/admin.users.session.reset): Wipes all valid sessions on all devices for a given user
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.users:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.users.session.reset")]
    pub async fn admin_users_session_reset(
        &self,
        request: &AdminUsersSessionResetRequest,
    ) -> Result<AdminUsersSessionResetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.users.session.resetBulk`](https://docs.slack.dev/reference/methods/admin.users.session.resetBulk): Enqueues an asynchronous job to wipe all valid sessions on all devices for a given list of users
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.users:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.users.session.resetBulk")]
    pub async fn admin_users_session_reset_bulk(
        &self,
        request: &AdminUsersSessionResetBulkRequest,
    ) -> Result<AdminUsersSessionResetBulkResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.users.session.setSettings`](https://docs.slack.dev/reference/methods/admin.users.session.setSettings): Configure the user-level session settings—the session duration and what happens when the client closes—for one or more users.
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
    #[doc(alias = "admin.users.session.setSettings")]
    pub async fn admin_users_session_set_settings(
        &self,
        request: &AdminUsersSessionSetSettingsRequest,
    ) -> Result<AdminUsersSessionSetSettingsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.users.setAdmin`](https://docs.slack.dev/reference/methods/admin.users.setAdmin): Set an existing regular user or owner to be a workspace or org admin.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.users:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.users.setAdmin")]
    pub async fn admin_users_set_admin(
        &self,
        request: &AdminUsersSetAdminRequest,
    ) -> Result<AdminUsersSetAdminResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.users.setExpiration`](https://docs.slack.dev/reference/methods/admin.users.setExpiration): Set an expiration for a guest user
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.users:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.users.setExpiration")]
    pub async fn admin_users_set_expiration(
        &self,
        request: &AdminUsersSetExpirationRequest,
    ) -> Result<AdminUsersSetExpirationResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.users.setOwner`](https://docs.slack.dev/reference/methods/admin.users.setOwner): Set an existing regular user or admin to be a workspace or org owner.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.users:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.users.setOwner")]
    pub async fn admin_users_set_owner(
        &self,
        request: &AdminUsersSetOwnerRequest,
    ) -> Result<AdminUsersSetOwnerResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.users.setRegular`](https://docs.slack.dev/reference/methods/admin.users.setRegular): Set an existing guest user, admin user, or owner to be a regular user.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.users:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.users.setRegular")]
    pub async fn admin_users_set_regular(
        &self,
        request: &AdminUsersSetRegularRequest,
    ) -> Result<AdminUsersSetRegularResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.users.unsupportedVersions.export`](https://docs.slack.dev/reference/methods/admin.users.unsupportedVersions.export): Ask Slack to send you an export listing all workspace members using unsupported software, presented as a zipped CSV file.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.users:read`
    ///
    /// Rate limit: Tier 1 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.users.unsupportedVersions.export")]
    pub async fn admin_users_unsupported_versions_export(
        &self,
        request: &AdminUsersUnsupportedVersionsExportRequest,
    ) -> Result<AdminUsersUnsupportedVersionsExportResponse, SlackError> {
        self.call(request).await
    }
}
