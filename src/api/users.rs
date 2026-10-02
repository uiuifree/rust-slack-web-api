// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`users.conversations`](https://docs.slack.dev/reference/methods/users.conversations): List conversations the calling user is a member of.
///
/// Send it with [`SlackClient::users_conversations`].
#[doc(alias = "users.conversations")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct UsersConversationsRequest {
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](https://docs.slack.dev/apis/web-api/pagination.md) for more detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Set to `true` to exclude archived channels from the list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_archived: Option<bool>,
    /// Set to `true` to exclude muted channels from the list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_muted: Option<bool>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. Must be an integer with a max value of 999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// encoded team id to list conversations in, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Mix and match channel types by providing a comma-separated list of any combination of `public_channel`, `private_channel`, `mpim`, `im`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    /// Browse conversations by a specific user ID's membership. Non-public channels are restricted to those where the calling user shares membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl UsersConversationsRequest {
    pub fn new() -> Self {
        Self {
            cursor: None,
            exclude_archived: None,
            exclude_muted: None,
            limit: None,
            team_id: None,
            types: None,
            user: None,
        }
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn exclude_archived(mut self, exclude_archived: bool) -> Self {
        self.exclude_archived = Some(exclude_archived);
        self
    }

    pub fn exclude_muted(mut self, exclude_muted: bool) -> Self {
        self.exclude_muted = Some(exclude_muted);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn types(mut self, types: impl Into<String>) -> Self {
        self.types = Some(types.into());
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

impl SlackApiMethod for UsersConversationsRequest {
    const METHOD: &'static str = "users.conversations";
    type Response = UsersConversationsResponse;
}

impl CursorPaginated for UsersConversationsRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for UsersConversationsResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`users.conversations`](https://docs.slack.dev/reference/methods/users.conversations).
#[doc(alias = "users.conversations")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersConversationsResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub channels: Vec<Conversation>,
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

/// Arguments for the Slack Web API method [`users.deletePhoto`](https://docs.slack.dev/reference/methods/users.deletePhoto): Delete the user profile photo
///
/// Send it with [`SlackClient::users_delete_photo`].
#[doc(alias = "users.deletePhoto")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct UsersDeletePhotoRequest {}

impl UsersDeletePhotoRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl SlackApiMethod for UsersDeletePhotoRequest {
    const METHOD: &'static str = "users.deletePhoto";
    type Response = UsersDeletePhotoResponse;
}

/// Successful response of the Slack Web API method [`users.deletePhoto`](https://docs.slack.dev/reference/methods/users.deletePhoto).
#[doc(alias = "users.deletePhoto")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersDeletePhotoResponse {
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

/// Arguments for the Slack Web API method [`users.discoverableContacts.lookup`](https://docs.slack.dev/reference/methods/users.discoverableContacts.lookup): Look up an email address to see if someone is discoverable on Slack
///
/// Send it with [`SlackClient::users_discoverable_contacts_lookup`].
#[doc(alias = "users.discoverableContacts.lookup")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct UsersDiscoverableContactsLookupRequest {
    pub email: String,
}

impl UsersDiscoverableContactsLookupRequest {
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
        }
    }
}

impl SlackApiMethod for UsersDiscoverableContactsLookupRequest {
    const METHOD: &'static str = "users.discoverableContacts.lookup";
    type Response = UsersDiscoverableContactsLookupResponse;
}

/// Successful response of the Slack Web API method [`users.discoverableContacts.lookup`](https://docs.slack.dev/reference/methods/users.discoverableContacts.lookup).
#[doc(alias = "users.discoverableContacts.lookup")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersDiscoverableContactsLookupResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_discoverable: Option<bool>,
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

/// Arguments for the Slack Web API method [`users.getPresence`](https://docs.slack.dev/reference/methods/users.getPresence): Gets user presence information.
///
/// Send it with [`SlackClient::users_get_presence`].
#[doc(alias = "users.getPresence")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct UsersGetPresenceRequest {
    /// User to get presence info on. Defaults to the authed user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl UsersGetPresenceRequest {
    pub fn new() -> Self {
        Self { user: None }
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

impl SlackApiMethod for UsersGetPresenceRequest {
    const METHOD: &'static str = "users.getPresence";
    type Response = UsersGetPresenceResponse;
}

/// Successful response of the Slack Web API method [`users.getPresence`](https://docs.slack.dev/reference/methods/users.getPresence).
#[doc(alias = "users.getPresence")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersGetPresenceResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub presence: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub online: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_away: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub manual_away: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_activity: Option<i64>,
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

/// Arguments for the Slack Web API method [`users.identity`](https://docs.slack.dev/reference/methods/users.identity): Get a user's identity.
///
/// Send it with [`SlackClient::users_identity`].
#[doc(alias = "users.identity")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct UsersIdentityRequest {}

impl UsersIdentityRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl SlackApiMethod for UsersIdentityRequest {
    const METHOD: &'static str = "users.identity";
    type Response = UsersIdentityResponse;
}

/// Successful response of the Slack Web API method [`users.identity`](https://docs.slack.dev/reference/methods/users.identity).
#[doc(alias = "users.identity")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersIdentityResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<UsersIdentityResponseUser>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub team: Option<UsersIdentityResponseTeam>,
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
pub struct UsersIdentityResponseUser {
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
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub email: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_24: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_32: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_48: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_72: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_192: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersIdentityResponseTeam {
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

/// Arguments for the Slack Web API method [`users.info`](https://docs.slack.dev/reference/methods/users.info): Gets information about a user.
///
/// Send it with [`SlackClient::users_info`].
#[doc(alias = "users.info")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct UsersInfoRequest {
    /// Set this to `true` to receive the locale for this user. Defaults to `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_locale: Option<bool>,
    /// User to get info on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl UsersInfoRequest {
    pub fn new() -> Self {
        Self {
            include_locale: None,
            user: None,
        }
    }

    pub fn include_locale(mut self, include_locale: bool) -> Self {
        self.include_locale = Some(include_locale);
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

impl SlackApiMethod for UsersInfoRequest {
    const METHOD: &'static str = "users.info";
    type Response = UsersInfoResponse;
}

/// Successful response of the Slack Web API method [`users.info`](https://docs.slack.dev/reference/methods/users.info).
#[doc(alias = "users.info")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersInfoResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<User>,
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

/// Arguments for the Slack Web API method [`users.list`](https://docs.slack.dev/reference/methods/users.list): Lists all users in a Slack team.
///
/// Send it with [`SlackClient::users_list`].
#[doc(alias = "users.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct UsersListRequest {
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](https://docs.slack.dev/apis/web-api/pagination.md) for more detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Set this to `true` to receive the locale for users. Defaults to `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_locale: Option<bool>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. Providing no `limit` value will result in Slack attempting to deliver you the entire result set. If the collection is too large you may experience `limit_required` or HTTP 500 errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// encoded team id to list users in, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl UsersListRequest {
    pub fn new() -> Self {
        Self {
            cursor: None,
            include_locale: None,
            limit: None,
            team_id: None,
        }
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn include_locale(mut self, include_locale: bool) -> Self {
        self.include_locale = Some(include_locale);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for UsersListRequest {
    const METHOD: &'static str = "users.list";
    type Response = UsersListResponse;
}

impl CursorPaginated for UsersListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for UsersListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`users.list`](https://docs.slack.dev/reference/methods/users.list).
#[doc(alias = "users.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub members: Vec<User>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub cache_ts: Option<i64>,
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

/// Arguments for the Slack Web API method [`users.lookupByEmail`](https://docs.slack.dev/reference/methods/users.lookupByEmail): Find a user with an email address.
///
/// Send it with [`SlackClient::users_lookup_by_email`].
#[doc(alias = "users.lookupByEmail")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct UsersLookupByEmailRequest {
    /// An email address belonging to a user in the workspace
    pub email: String,
}

impl UsersLookupByEmailRequest {
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
        }
    }
}

impl SlackApiMethod for UsersLookupByEmailRequest {
    const METHOD: &'static str = "users.lookupByEmail";
    type Response = UsersLookupByEmailResponse;
}

/// Successful response of the Slack Web API method [`users.lookupByEmail`](https://docs.slack.dev/reference/methods/users.lookupByEmail).
#[doc(alias = "users.lookupByEmail")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersLookupByEmailResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<User>,
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

/// Arguments for the Slack Web API method [`users.profile.get`](https://docs.slack.dev/reference/methods/users.profile.get): Retrieve a user's profile information, including their custom status.
///
/// Send it with [`SlackClient::users_profile_get`].
#[doc(alias = "users.profile.get")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct UsersProfileGetRequest {
    /// Include labels for each ID in custom profile fields. Using this parameter will heavily rate-limit your requests and is not recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_labels: Option<bool>,
    /// User to retrieve profile info for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl UsersProfileGetRequest {
    pub fn new() -> Self {
        Self {
            include_labels: None,
            user: None,
        }
    }

    pub fn include_labels(mut self, include_labels: bool) -> Self {
        self.include_labels = Some(include_labels);
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

impl SlackApiMethod for UsersProfileGetRequest {
    const METHOD: &'static str = "users.profile.get";
    type Response = UsersProfileGetResponse;
}

/// Successful response of the Slack Web API method [`users.profile.get`](https://docs.slack.dev/reference/methods/users.profile.get).
#[doc(alias = "users.profile.get")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersProfileGetResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub profile: Option<UserProfile>,
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

/// Arguments for the Slack Web API method [`users.profile.set`](https://docs.slack.dev/reference/methods/users.profile.set): Set a user's profile information, including custom status.
///
/// Send it with [`SlackClient::users_profile_set`].
#[doc(alias = "users.profile.set")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct UsersProfileSetRequest {
    /// Name of a single key to set. Usable only if `profile` is not passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Collection of key:value pairs presented as a URL-encoded JSON hash. At most 50 fields may be set. Each field name is limited to 255 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// ID of user to change. This argument may only be specified by admins on paid teams.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Value to set a single key to. Usable only if `profile` is not passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl UsersProfileSetRequest {
    pub fn new() -> Self {
        Self {
            name: None,
            profile: None,
            user: None,
            value: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn profile(mut self, profile: impl Into<String>) -> Self {
        self.profile = Some(profile.into());
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
}

impl SlackApiMethod for UsersProfileSetRequest {
    const METHOD: &'static str = "users.profile.set";
    type Response = UsersProfileSetResponse;
}

/// Successful response of the Slack Web API method [`users.profile.set`](https://docs.slack.dev/reference/methods/users.profile.set).
#[doc(alias = "users.profile.set")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersProfileSetResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub profile: Option<UserProfile>,
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

/// Arguments for the Slack Web API method [`users.setActive`](https://docs.slack.dev/reference/methods/users.setActive): Marked a user as active. Deprecated and non-functional.
///
/// Send it with [`SlackClient::users_set_active`].
#[doc(alias = "users.setActive")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct UsersSetActiveRequest {}

impl UsersSetActiveRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl SlackApiMethod for UsersSetActiveRequest {
    const METHOD: &'static str = "users.setActive";
    type Response = UsersSetActiveResponse;
}

/// Successful response of the Slack Web API method [`users.setActive`](https://docs.slack.dev/reference/methods/users.setActive).
#[doc(alias = "users.setActive")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersSetActiveResponse {
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

/// Arguments for the Slack Web API method [`users.setPhoto`](https://docs.slack.dev/reference/methods/users.setPhoto): Set the user profile photo
///
/// Send it with [`SlackClient::users_set_photo`].
#[doc(alias = "users.setPhoto")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct UsersSetPhotoRequest {
    /// Width/height of crop box (always square)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop_w: Option<String>,
    /// X coordinate of top-left corner of crop box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop_x: Option<String>,
    /// Y coordinate of top-left corner of crop box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop_y: Option<String>,
    /// File contents via `multipart/form-data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

impl UsersSetPhotoRequest {
    pub fn new() -> Self {
        Self {
            crop_w: None,
            crop_x: None,
            crop_y: None,
            image: None,
        }
    }

    pub fn crop_w(mut self, crop_w: impl Into<String>) -> Self {
        self.crop_w = Some(crop_w.into());
        self
    }

    pub fn crop_x(mut self, crop_x: impl Into<String>) -> Self {
        self.crop_x = Some(crop_x.into());
        self
    }

    pub fn crop_y(mut self, crop_y: impl Into<String>) -> Self {
        self.crop_y = Some(crop_y.into());
        self
    }

    pub fn image(mut self, image: impl Into<String>) -> Self {
        self.image = Some(image.into());
        self
    }
}

impl SlackApiMethod for UsersSetPhotoRequest {
    const METHOD: &'static str = "users.setPhoto";
    type Response = UsersSetPhotoResponse;
}

/// Successful response of the Slack Web API method [`users.setPhoto`](https://docs.slack.dev/reference/methods/users.setPhoto).
#[doc(alias = "users.setPhoto")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersSetPhotoResponse {
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

/// Arguments for the Slack Web API method [`users.setPresence`](https://docs.slack.dev/reference/methods/users.setPresence): Manually sets user presence.
///
/// Send it with [`SlackClient::users_set_presence`].
#[doc(alias = "users.setPresence")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct UsersSetPresenceRequest {
    /// Either `auto` or `away`
    pub presence: String,
}

impl UsersSetPresenceRequest {
    pub fn new(presence: impl Into<String>) -> Self {
        Self {
            presence: presence.into(),
        }
    }
}

impl SlackApiMethod for UsersSetPresenceRequest {
    const METHOD: &'static str = "users.setPresence";
    type Response = UsersSetPresenceResponse;
}

/// Successful response of the Slack Web API method [`users.setPresence`](https://docs.slack.dev/reference/methods/users.setPresence).
#[doc(alias = "users.setPresence")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersSetPresenceResponse {
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
    /// Calls the Slack Web API method [`users.conversations`](https://docs.slack.dev/reference/methods/users.conversations): List conversations the calling user is a member of.
    ///
    /// Required scopes:
    ///
    /// - bot token: `groups:read`, `im:read`, `mpim:read`, `channels:read`
    /// - user token: `groups:read`, `im:read`, `mpim:read`, `channels:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "users.conversations")]
    pub async fn users_conversations(
        &self,
        request: &UsersConversationsRequest,
    ) -> Result<UsersConversationsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`users.deletePhoto`](https://docs.slack.dev/reference/methods/users.deletePhoto): Delete the user profile photo
    ///
    /// Required scopes:
    ///
    /// - user token: `users.profile:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "users.deletePhoto")]
    pub async fn users_delete_photo(
        &self,
        request: &UsersDeletePhotoRequest,
    ) -> Result<UsersDeletePhotoResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`users.discoverableContacts.lookup`](https://docs.slack.dev/reference/methods/users.discoverableContacts.lookup): Look up an email address to see if someone is discoverable on Slack
    ///
    /// Required scopes:
    ///
    /// - bot token: `conversations.connect:manage`, `team:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "users.discoverableContacts.lookup")]
    pub async fn users_discoverable_contacts_lookup(
        &self,
        request: &UsersDiscoverableContactsLookupRequest,
    ) -> Result<UsersDiscoverableContactsLookupResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`users.getPresence`](https://docs.slack.dev/reference/methods/users.getPresence): Gets user presence information.
    ///
    /// Required scopes:
    ///
    /// - bot token: `users:read`
    /// - user token: `users:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "users.getPresence")]
    pub async fn users_get_presence(
        &self,
        request: &UsersGetPresenceRequest,
    ) -> Result<UsersGetPresenceResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`users.identity`](https://docs.slack.dev/reference/methods/users.identity): Get a user's identity.
    ///
    /// Required scopes:
    ///
    /// - user token: `identity:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "users.identity")]
    pub async fn users_identity(
        &self,
        request: &UsersIdentityRequest,
    ) -> Result<UsersIdentityResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`users.info`](https://docs.slack.dev/reference/methods/users.info): Gets information about a user.
    ///
    /// Required scopes:
    ///
    /// - bot token: `users:read`
    /// - user token: `users:read`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "users.info")]
    pub async fn users_info(
        &self,
        request: &UsersInfoRequest,
    ) -> Result<UsersInfoResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`users.list`](https://docs.slack.dev/reference/methods/users.list): Lists all users in a Slack team.
    ///
    /// Required scopes:
    ///
    /// - bot token: `users:read`
    /// - user token: `users:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "users.list")]
    pub async fn users_list(
        &self,
        request: &UsersListRequest,
    ) -> Result<UsersListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`users.lookupByEmail`](https://docs.slack.dev/reference/methods/users.lookupByEmail): Find a user with an email address.
    ///
    /// Required scopes:
    ///
    /// - bot token: `users:read.email`
    /// - user token: `users:read.email`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "users.lookupByEmail")]
    pub async fn users_lookup_by_email(
        &self,
        request: &UsersLookupByEmailRequest,
    ) -> Result<UsersLookupByEmailResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`users.profile.get`](https://docs.slack.dev/reference/methods/users.profile.get): Retrieve a user's profile information, including their custom status.
    ///
    /// Required scopes:
    ///
    /// - bot token: `users.profile:read`
    /// - user token: `users.profile:read`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "users.profile.get")]
    pub async fn users_profile_get(
        &self,
        request: &UsersProfileGetRequest,
    ) -> Result<UsersProfileGetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`users.profile.set`](https://docs.slack.dev/reference/methods/users.profile.set): Set a user's profile information, including custom status.
    ///
    /// Required scopes:
    ///
    /// - user token: `users.profile:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "users.profile.set")]
    pub async fn users_profile_set(
        &self,
        request: &UsersProfileSetRequest,
    ) -> Result<UsersProfileSetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`users.setActive`](https://docs.slack.dev/reference/methods/users.setActive): Marked a user as active. Deprecated and non-functional.
    ///
    /// Required scopes:
    ///
    /// - bot token: `users:write`
    /// - user token: `users:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "users.setActive")]
    pub async fn users_set_active(
        &self,
        request: &UsersSetActiveRequest,
    ) -> Result<UsersSetActiveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`users.setPhoto`](https://docs.slack.dev/reference/methods/users.setPhoto): Set the user profile photo
    ///
    /// Required scopes:
    ///
    /// - user token: `users.profile:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "users.setPhoto")]
    pub async fn users_set_photo(
        &self,
        request: &UsersSetPhotoRequest,
    ) -> Result<UsersSetPhotoResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`users.setPresence`](https://docs.slack.dev/reference/methods/users.setPresence): Manually sets user presence.
    ///
    /// Required scopes:
    ///
    /// - bot token: `users:write`
    /// - user token: `users:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "users.setPresence")]
    pub async fn users_set_presence(
        &self,
        request: &UsersSetPresenceRequest,
    ) -> Result<UsersSetPresenceResponse, SlackError> {
        self.call(request).await
    }
}
