// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`admin.conversations.archive`](https://docs.slack.dev/reference/methods/admin.conversations.archive): Archive a public or private channel.
///
/// Send it with [`SlackClient::admin_conversations_archive`].
#[doc(alias = "admin.conversations.archive")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsArchiveRequest {
    /// The channel to archive.
    pub channel_id: String,
}

impl AdminConversationsArchiveRequest {
    pub fn new(channel_id: impl Into<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
        }
    }
}

impl SlackApiMethod for AdminConversationsArchiveRequest {
    const METHOD: &'static str = "admin.conversations.archive";
    type Response = AdminConversationsArchiveResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.archive`](https://docs.slack.dev/reference/methods/admin.conversations.archive).
#[doc(alias = "admin.conversations.archive")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsArchiveResponse {
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

/// Arguments for the Slack Web API method [`admin.conversations.bulkArchive`](https://docs.slack.dev/reference/methods/admin.conversations.bulkArchive): Archive public or private channels in bulk.
///
/// Send it with [`SlackClient::admin_conversations_bulk_archive`].
#[doc(alias = "admin.conversations.bulkArchive")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsBulkArchiveRequest {
    /// An array of channel IDs to archive. No more than 100 items are allowed.
    pub channel_ids: Vec<String>,
}

impl AdminConversationsBulkArchiveRequest {
    pub fn new(channel_ids: Vec<String>) -> Self {
        Self { channel_ids }
    }
}

impl SlackApiMethod for AdminConversationsBulkArchiveRequest {
    const METHOD: &'static str = "admin.conversations.bulkArchive";
    type Response = AdminConversationsBulkArchiveResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.bulkArchive`](https://docs.slack.dev/reference/methods/admin.conversations.bulkArchive).
#[doc(alias = "admin.conversations.bulkArchive")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsBulkArchiveResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub bulk_action_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub not_added: Vec<AdminConversationsBulkArchiveResponseNotAdded>,
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
pub struct AdminConversationsBulkArchiveResponseNotAdded {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub error: Option<String>,
}

/// Arguments for the Slack Web API method [`admin.conversations.bulkDelete`](https://docs.slack.dev/reference/methods/admin.conversations.bulkDelete): Delete public or private channels in bulk
///
/// Send it with [`SlackClient::admin_conversations_bulk_delete`].
#[doc(alias = "admin.conversations.bulkDelete")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsBulkDeleteRequest {
    /// An array of channel IDs.
    pub channel_ids: Vec<String>,
}

impl AdminConversationsBulkDeleteRequest {
    pub fn new(channel_ids: Vec<String>) -> Self {
        Self { channel_ids }
    }
}

impl SlackApiMethod for AdminConversationsBulkDeleteRequest {
    const METHOD: &'static str = "admin.conversations.bulkDelete";
    type Response = AdminConversationsBulkDeleteResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.bulkDelete`](https://docs.slack.dev/reference/methods/admin.conversations.bulkDelete).
#[doc(alias = "admin.conversations.bulkDelete")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsBulkDeleteResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub bulk_action_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub not_added: Vec<AdminConversationsBulkDeleteResponseNotAdded>,
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
pub struct AdminConversationsBulkDeleteResponseNotAdded {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub error: Option<String>,
}

/// Arguments for the Slack Web API method [`admin.conversations.bulkMove`](https://docs.slack.dev/reference/methods/admin.conversations.bulkMove): Move public or private channels in bulk.
///
/// Send it with [`SlackClient::admin_conversations_bulk_move`].
#[doc(alias = "admin.conversations.bulkMove")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsBulkMoveRequest {
    /// Target team ID
    pub target_team_id: String,
    /// An array of channel IDs.
    pub channel_ids: Vec<String>,
}

impl AdminConversationsBulkMoveRequest {
    pub fn new(target_team_id: impl Into<String>, channel_ids: Vec<String>) -> Self {
        Self {
            target_team_id: target_team_id.into(),
            channel_ids,
        }
    }
}

impl SlackApiMethod for AdminConversationsBulkMoveRequest {
    const METHOD: &'static str = "admin.conversations.bulkMove";
    type Response = AdminConversationsBulkMoveResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.bulkMove`](https://docs.slack.dev/reference/methods/admin.conversations.bulkMove).
#[doc(alias = "admin.conversations.bulkMove")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsBulkMoveResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub bulk_action_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub not_added: Vec<AdminConversationsBulkMoveResponseNotAdded>,
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
pub struct AdminConversationsBulkMoveResponseNotAdded {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub error: Option<String>,
}

/// Arguments for the Slack Web API method [`admin.conversations.bulkSetExcludeFromSlackAi`](https://docs.slack.dev/reference/methods/admin.conversations.bulkSetExcludeFromSlackAi): Exclude channels from Slack AI in bulk
///
/// Send it with [`SlackClient::admin_conversations_bulk_set_exclude_from_slack_ai`].
#[doc(alias = "admin.conversations.bulkSetExcludeFromSlackAi")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsBulkSetExcludeFromSlackAiRequest {
    /// An array of channel IDs to exclude from Slack AI.
    #[serde(serialize_with = "crate::form::as_json")]
    pub channel_ids: Vec<String>,
    /// Whether the channels should be excluded from Slack AI.
    pub exclude: bool,
}

impl AdminConversationsBulkSetExcludeFromSlackAiRequest {
    pub fn new(channel_ids: Vec<String>, exclude: bool) -> Self {
        Self {
            channel_ids,
            exclude,
        }
    }
}

impl SlackApiMethod for AdminConversationsBulkSetExcludeFromSlackAiRequest {
    const METHOD: &'static str = "admin.conversations.bulkSetExcludeFromSlackAi";
    type Response = AdminConversationsBulkSetExcludeFromSlackAiResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.bulkSetExcludeFromSlackAi`](https://docs.slack.dev/reference/methods/admin.conversations.bulkSetExcludeFromSlackAi).
#[doc(alias = "admin.conversations.bulkSetExcludeFromSlackAi")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsBulkSetExcludeFromSlackAiResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub bulk_action_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub not_added: Vec<AdminConversationsBulkSetExcludeFromSlackAiResponseNotAdded>,
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
pub struct AdminConversationsBulkSetExcludeFromSlackAiResponseNotAdded {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub error: Option<String>,
}

/// Arguments for the Slack Web API method [`admin.conversations.bulkSetProperties`](https://docs.slack.dev/reference/methods/admin.conversations.bulkSetProperties): Set properties on channels in bulk
///
/// Send it with [`SlackClient::admin_conversations_bulk_set_properties`].
#[doc(alias = "admin.conversations.bulkSetProperties")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsBulkSetPropertiesRequest {
    /// An array of channel IDs on which to set the property.
    pub channel_ids: Vec<String>,
    /// The property for this channel in a key value format. Only one property can be updated at a time
    pub property: String,
}

impl AdminConversationsBulkSetPropertiesRequest {
    pub fn new(channel_ids: Vec<String>, property: impl Into<String>) -> Self {
        Self {
            channel_ids,
            property: property.into(),
        }
    }
}

impl SlackApiMethod for AdminConversationsBulkSetPropertiesRequest {
    const METHOD: &'static str = "admin.conversations.bulkSetProperties";
    type Response = AdminConversationsBulkSetPropertiesResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.bulkSetProperties`](https://docs.slack.dev/reference/methods/admin.conversations.bulkSetProperties).
#[doc(alias = "admin.conversations.bulkSetProperties")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsBulkSetPropertiesResponse {
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

/// Arguments for the Slack Web API method [`admin.conversations.convertToPrivate`](https://docs.slack.dev/reference/methods/admin.conversations.convertToPrivate): Convert a public channel to a private channel.
///
/// Send it with [`SlackClient::admin_conversations_convert_to_private`].
#[doc(alias = "admin.conversations.convertToPrivate")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsConvertToPrivateRequest {
    /// The channel to convert to private.
    pub channel_id: String,
    /// Name of private channel to create. Only respected when converting an MPIM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AdminConversationsConvertToPrivateRequest {
    pub fn new(channel_id: impl Into<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
            name: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

impl SlackApiMethod for AdminConversationsConvertToPrivateRequest {
    const METHOD: &'static str = "admin.conversations.convertToPrivate";
    type Response = AdminConversationsConvertToPrivateResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.convertToPrivate`](https://docs.slack.dev/reference/methods/admin.conversations.convertToPrivate).
#[doc(alias = "admin.conversations.convertToPrivate")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsConvertToPrivateResponse {
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

/// Arguments for the Slack Web API method [`admin.conversations.convertToPublic`](https://docs.slack.dev/reference/methods/admin.conversations.convertToPublic): Convert a private channel to a public channel.
///
/// Send it with [`SlackClient::admin_conversations_convert_to_public`].
#[doc(alias = "admin.conversations.convertToPublic")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsConvertToPublicRequest {
    /// The channel to convert to public.
    pub channel_id: String,
}

impl AdminConversationsConvertToPublicRequest {
    pub fn new(channel_id: impl Into<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
        }
    }
}

impl SlackApiMethod for AdminConversationsConvertToPublicRequest {
    const METHOD: &'static str = "admin.conversations.convertToPublic";
    type Response = AdminConversationsConvertToPublicResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.convertToPublic`](https://docs.slack.dev/reference/methods/admin.conversations.convertToPublic).
#[doc(alias = "admin.conversations.convertToPublic")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsConvertToPublicResponse {
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

/// Arguments for the Slack Web API method [`admin.conversations.create`](https://docs.slack.dev/reference/methods/admin.conversations.create): Create a public or private channel-based conversation.
///
/// Send it with [`SlackClient::admin_conversations_create`].
#[doc(alias = "admin.conversations.create")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsCreateRequest {
    /// Name of the public or private channel to create.
    pub name: String,
    /// When `true`, creates a private channel instead of a public channel
    pub is_private: bool,
    /// Description of the public or private channel to create.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When `true`, the channel will be available org-wide. Note: if the channel is not `org_wide=true`, you must specify a `team_id` for this channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_wide: Option<bool>,
    /// The workspace to create the channel in. Note: this argument is required unless you set `org_wide=true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl AdminConversationsCreateRequest {
    pub fn new(name: impl Into<String>, is_private: bool) -> Self {
        Self {
            name: name.into(),
            is_private,
            description: None,
            org_wide: None,
            team_id: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn org_wide(mut self, org_wide: bool) -> Self {
        self.org_wide = Some(org_wide);
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for AdminConversationsCreateRequest {
    const METHOD: &'static str = "admin.conversations.create";
    type Response = AdminConversationsCreateResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.create`](https://docs.slack.dev/reference/methods/admin.conversations.create).
#[doc(alias = "admin.conversations.create")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsCreateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_id: Option<String>,
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

/// Arguments for the Slack Web API method [`admin.conversations.createForObjects`](https://docs.slack.dev/reference/methods/admin.conversations.createForObjects): Create a Salesforce channel for the corresponding object provided.
///
/// Send it with [`SlackClient::admin_conversations_create_for_objects`].
#[doc(alias = "admin.conversations.createForObjects")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsCreateForObjectsRequest {
    /// Object / Record ID (15 or 18 digit accepted). See [here](https://help.salesforce.com/s/articleView?id=000385008&type=1) for how to look up an ID.
    pub object_id: String,
    /// Salesforce org ID (15 or 18 digit accepted). See [here](https://help.salesforce.com/s/articleView?id=000385215&type=1) for how to look up Salesforce org ID.
    pub salesforce_org_id: String,
    /// Optional flag to add all team members related to the object to the newly created Salesforce channel. When true, adds a maximum of 100 team members to the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_object_team: Option<bool>,
}

impl AdminConversationsCreateForObjectsRequest {
    pub fn new(object_id: impl Into<String>, salesforce_org_id: impl Into<String>) -> Self {
        Self {
            object_id: object_id.into(),
            salesforce_org_id: salesforce_org_id.into(),
            invite_object_team: None,
        }
    }

    pub fn invite_object_team(mut self, invite_object_team: bool) -> Self {
        self.invite_object_team = Some(invite_object_team);
        self
    }
}

impl SlackApiMethod for AdminConversationsCreateForObjectsRequest {
    const METHOD: &'static str = "admin.conversations.createForObjects";
    type Response = AdminConversationsCreateForObjectsResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.createForObjects`](https://docs.slack.dev/reference/methods/admin.conversations.createForObjects).
#[doc(alias = "admin.conversations.createForObjects")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsCreateForObjectsResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_id: Option<String>,
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

/// Arguments for the Slack Web API method [`admin.conversations.delete`](https://docs.slack.dev/reference/methods/admin.conversations.delete): Delete a public or private channel.
///
/// Send it with [`SlackClient::admin_conversations_delete`].
#[doc(alias = "admin.conversations.delete")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsDeleteRequest {
    /// The channel to delete.
    pub channel_id: String,
}

impl AdminConversationsDeleteRequest {
    pub fn new(channel_id: impl Into<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
        }
    }
}

impl SlackApiMethod for AdminConversationsDeleteRequest {
    const METHOD: &'static str = "admin.conversations.delete";
    type Response = AdminConversationsDeleteResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.delete`](https://docs.slack.dev/reference/methods/admin.conversations.delete).
#[doc(alias = "admin.conversations.delete")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsDeleteResponse {
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

/// Arguments for the Slack Web API method [`admin.conversations.disconnectShared`](https://docs.slack.dev/reference/methods/admin.conversations.disconnectShared): Disconnect a connected channel from one or more workspaces.
///
/// Send it with [`SlackClient::admin_conversations_disconnect_shared`].
#[doc(alias = "admin.conversations.disconnectShared")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsDisconnectSharedRequest {
    /// The channel to be disconnected from some workspaces.
    pub channel_id: String,
    /// Used for disconnecting a team from a shared channel. Only one team ID may be passed at a time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaving_team_ids: Option<Vec<String>>,
}

impl AdminConversationsDisconnectSharedRequest {
    pub fn new(channel_id: impl Into<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
            leaving_team_ids: None,
        }
    }

    pub fn leaving_team_ids(mut self, leaving_team_ids: Vec<String>) -> Self {
        self.leaving_team_ids = Some(leaving_team_ids);
        self
    }
}

impl SlackApiMethod for AdminConversationsDisconnectSharedRequest {
    const METHOD: &'static str = "admin.conversations.disconnectShared";
    type Response = AdminConversationsDisconnectSharedResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.disconnectShared`](https://docs.slack.dev/reference/methods/admin.conversations.disconnectShared).
#[doc(alias = "admin.conversations.disconnectShared")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsDisconnectSharedResponse {
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

/// Arguments for the Slack Web API method [`admin.conversations.ekm.listOriginalConnectedChannelInfo`](https://docs.slack.dev/reference/methods/admin.conversations.ekm.listOriginalConnectedChannelInfo): List all disconnected channels—i.e., channels that were once connected to other workspaces and then disconnected—and the corresponding original channel IDs for key revocation with EKM.
///
/// Send it with [`SlackClient::admin_conversations_ekm_list_original_connected_channel_info`].
#[doc(alias = "admin.conversations.ekm.listOriginalConnectedChannelInfo")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminConversationsEkmListOriginalConnectedChannelInfoRequest {
    /// A comma-separated list of channels to filter to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<String>,
    /// A comma-separated list of the workspaces to which the channels you would like returned belong.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<String>,
    /// The maximum number of items to return. Must be between 1 - 1000 both inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AdminConversationsEkmListOriginalConnectedChannelInfoRequest {
    pub fn new() -> Self {
        Self {
            channel_ids: None,
            team_ids: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn channel_ids(mut self, channel_ids: impl Into<String>) -> Self {
        self.channel_ids = Some(channel_ids.into());
        self
    }

    pub fn team_ids(mut self, team_ids: impl Into<String>) -> Self {
        self.team_ids = Some(team_ids.into());
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

impl SlackApiMethod for AdminConversationsEkmListOriginalConnectedChannelInfoRequest {
    const METHOD: &'static str = "admin.conversations.ekm.listOriginalConnectedChannelInfo";
    type Response = AdminConversationsEkmListOriginalConnectedChannelInfoResponse;
}

impl CursorPaginated for AdminConversationsEkmListOriginalConnectedChannelInfoRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminConversationsEkmListOriginalConnectedChannelInfoResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.conversations.ekm.listOriginalConnectedChannelInfo`](https://docs.slack.dev/reference/methods/admin.conversations.ekm.listOriginalConnectedChannelInfo).
#[doc(alias = "admin.conversations.ekm.listOriginalConnectedChannelInfo")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsEkmListOriginalConnectedChannelInfoResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub channels: Vec<AdminConversationsEkmListOriginalConnectedChannelInfoResponseChannels>,
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
pub struct AdminConversationsEkmListOriginalConnectedChannelInfoResponseChannels {
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
    pub internal_team_ids: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_connected_host_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_connected_channel_id: Option<String>,
}

/// Arguments for the Slack Web API method [`admin.conversations.getConversationPrefs`](https://docs.slack.dev/reference/methods/admin.conversations.getConversationPrefs): Get conversation preferences for a public or private channel.
///
/// Send it with [`SlackClient::admin_conversations_get_conversation_prefs`].
#[doc(alias = "admin.conversations.getConversationPrefs")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsGetConversationPrefsRequest {
    /// The channel to get preferences for.
    pub channel_id: String,
}

impl AdminConversationsGetConversationPrefsRequest {
    pub fn new(channel_id: impl Into<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
        }
    }
}

impl SlackApiMethod for AdminConversationsGetConversationPrefsRequest {
    const METHOD: &'static str = "admin.conversations.getConversationPrefs";
    type Response = AdminConversationsGetConversationPrefsResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.getConversationPrefs`](https://docs.slack.dev/reference/methods/admin.conversations.getConversationPrefs).
#[doc(alias = "admin.conversations.getConversationPrefs")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsGetConversationPrefsResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub prefs: Option<AdminConversationsGetConversationPrefsResponsePrefs>,
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
pub struct AdminConversationsGetConversationPrefsResponsePrefs {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub who_can_post: Option<AdminConversationsGetConversationPrefsResponsePrefsWhoCanPost>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_thread: Option<AdminConversationsGetConversationPrefsResponsePrefsCanThread>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_at_channel:
        Option<AdminConversationsGetConversationPrefsResponsePrefsEnableAtChannel>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_at_here: Option<AdminConversationsGetConversationPrefsResponsePrefsEnableAtHere>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsGetConversationPrefsResponsePrefsWhoCanPost {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsGetConversationPrefsResponsePrefsCanThread {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsGetConversationPrefsResponsePrefsEnableAtChannel {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsGetConversationPrefsResponsePrefsEnableAtHere {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled: Option<bool>,
}

/// Arguments for the Slack Web API method [`admin.conversations.getCustomRetention`](https://docs.slack.dev/reference/methods/admin.conversations.getCustomRetention): This API endpoint can be used by any admin to get a conversation's retention policy.
///
/// Send it with [`SlackClient::admin_conversations_get_custom_retention`].
#[doc(alias = "admin.conversations.getCustomRetention")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsGetCustomRetentionRequest {
    /// The conversation to get the retention policy for.
    pub channel_id: String,
}

impl AdminConversationsGetCustomRetentionRequest {
    pub fn new(channel_id: impl Into<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
        }
    }
}

impl SlackApiMethod for AdminConversationsGetCustomRetentionRequest {
    const METHOD: &'static str = "admin.conversations.getCustomRetention";
    type Response = AdminConversationsGetCustomRetentionResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.getCustomRetention`](https://docs.slack.dev/reference/methods/admin.conversations.getCustomRetention).
#[doc(alias = "admin.conversations.getCustomRetention")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsGetCustomRetentionResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_policy_enabled: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_days: Option<i64>,
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

/// Arguments for the Slack Web API method [`admin.conversations.getTeams`](https://docs.slack.dev/reference/methods/admin.conversations.getTeams): Get all the workspaces a given public or private channel is connected to within this Enterprise org.
///
/// Send it with [`SlackClient::admin_conversations_get_teams`].
#[doc(alias = "admin.conversations.getTeams")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsGetTeamsRequest {
    /// The channel to determine connected workspaces within the organization for.
    pub channel_id: String,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The maximum number of items to return. Must be between 1 - 1000 both inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl AdminConversationsGetTeamsRequest {
    pub fn new(channel_id: impl Into<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
            cursor: None,
            limit: None,
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
}

impl SlackApiMethod for AdminConversationsGetTeamsRequest {
    const METHOD: &'static str = "admin.conversations.getTeams";
    type Response = AdminConversationsGetTeamsResponse;
}

impl CursorPaginated for AdminConversationsGetTeamsRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminConversationsGetTeamsResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.conversations.getTeams`](https://docs.slack.dev/reference/methods/admin.conversations.getTeams).
#[doc(alias = "admin.conversations.getTeams")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsGetTeamsResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub team_ids: Vec<String>,
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

/// Arguments for the Slack Web API method [`admin.conversations.invite`](https://docs.slack.dev/reference/methods/admin.conversations.invite): Invite a user to a public or private channel.
///
/// Send it with [`SlackClient::admin_conversations_invite`].
#[doc(alias = "admin.conversations.invite")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsInviteRequest {
    /// The users to invite.
    pub user_ids: Vec<String>,
    /// The channel that the users will be invited to.
    pub channel_id: String,
}

impl AdminConversationsInviteRequest {
    pub fn new(user_ids: Vec<String>, channel_id: impl Into<String>) -> Self {
        Self {
            user_ids,
            channel_id: channel_id.into(),
        }
    }
}

impl SlackApiMethod for AdminConversationsInviteRequest {
    const METHOD: &'static str = "admin.conversations.invite";
    type Response = AdminConversationsInviteResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.invite`](https://docs.slack.dev/reference/methods/admin.conversations.invite).
#[doc(alias = "admin.conversations.invite")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsInviteResponse {
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

/// Arguments for the Slack Web API method [`admin.conversations.linkObjects`](https://docs.slack.dev/reference/methods/admin.conversations.linkObjects): Link a Salesforce record to a channel
///
/// Send it with [`SlackClient::admin_conversations_link_objects`].
#[doc(alias = "admin.conversations.linkObjects")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsLinkObjectsRequest {
    /// Channel ID for Slack channel that will be linked to a Salesforce record.
    pub channel: String,
    /// Salesforce record ID (15 or 18 digit accepted). See [here](https://help.salesforce.com/s/articleView?id=000385008&type=1) for how to look up record ID.
    pub record_id: String,
    /// Salesforce org ID (15 or 18 digit accepted). See [here](https://help.salesforce.com/s/articleView?id=000385215&type=1) for how to look up Salesforce org ID.
    pub salesforce_org_id: String,
}

impl AdminConversationsLinkObjectsRequest {
    pub fn new(
        channel: impl Into<String>,
        record_id: impl Into<String>,
        salesforce_org_id: impl Into<String>,
    ) -> Self {
        Self {
            channel: channel.into(),
            record_id: record_id.into(),
            salesforce_org_id: salesforce_org_id.into(),
        }
    }
}

impl SlackApiMethod for AdminConversationsLinkObjectsRequest {
    const METHOD: &'static str = "admin.conversations.linkObjects";
    type Response = AdminConversationsLinkObjectsResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.linkObjects`](https://docs.slack.dev/reference/methods/admin.conversations.linkObjects).
#[doc(alias = "admin.conversations.linkObjects")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsLinkObjectsResponse {
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

/// Arguments for the Slack Web API method [`admin.conversations.lookup`](https://docs.slack.dev/reference/methods/admin.conversations.lookup): Returns channels on the given team using the filters.
///
/// Send it with [`SlackClient::admin_conversations_lookup`].
#[doc(alias = "admin.conversations.lookup")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsLookupRequest {
    /// Array of team IDs to filter by
    pub team_ids: Vec<String>,
    /// Filter by _public_ channels where the most recent message was sent _before_ last\_message\_activity
    pub last_message_activity_before: i64,
    /// Filter by _public_ channels with member count _equal to or less than_ the specified number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_member_count: Option<i64>,
    /// Set `cursor` to `next_cursor` returned in the previous call, to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Maximum number of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl AdminConversationsLookupRequest {
    pub fn new(team_ids: Vec<String>, last_message_activity_before: i64) -> Self {
        Self {
            team_ids,
            last_message_activity_before,
            max_member_count: None,
            cursor: None,
            limit: None,
        }
    }

    pub fn max_member_count(mut self, max_member_count: i64) -> Self {
        self.max_member_count = Some(max_member_count);
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

impl SlackApiMethod for AdminConversationsLookupRequest {
    const METHOD: &'static str = "admin.conversations.lookup";
    type Response = AdminConversationsLookupResponse;
}

impl CursorPaginated for AdminConversationsLookupRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminConversationsLookupResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.conversations.lookup`](https://docs.slack.dev/reference/methods/admin.conversations.lookup).
#[doc(alias = "admin.conversations.lookup")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsLookupResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub channels: Vec<String>,
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

/// Arguments for the Slack Web API method [`admin.conversations.removeCustomRetention`](https://docs.slack.dev/reference/methods/admin.conversations.removeCustomRetention): This API endpoint can be used by any admin to remove a conversation's retention policy.
///
/// Send it with [`SlackClient::admin_conversations_remove_custom_retention`].
#[doc(alias = "admin.conversations.removeCustomRetention")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsRemoveCustomRetentionRequest {
    /// The conversation to set the retention policy for.
    pub channel_id: String,
}

impl AdminConversationsRemoveCustomRetentionRequest {
    pub fn new(channel_id: impl Into<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
        }
    }
}

impl SlackApiMethod for AdminConversationsRemoveCustomRetentionRequest {
    const METHOD: &'static str = "admin.conversations.removeCustomRetention";
    type Response = AdminConversationsRemoveCustomRetentionResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.removeCustomRetention`](https://docs.slack.dev/reference/methods/admin.conversations.removeCustomRetention).
#[doc(alias = "admin.conversations.removeCustomRetention")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsRemoveCustomRetentionResponse {
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

/// Arguments for the Slack Web API method [`admin.conversations.rename`](https://docs.slack.dev/reference/methods/admin.conversations.rename): Rename a public or private channel.
///
/// Send it with [`SlackClient::admin_conversations_rename`].
#[doc(alias = "admin.conversations.rename")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsRenameRequest {
    /// The channel to rename.
    pub channel_id: String,
    pub name: String,
}

impl AdminConversationsRenameRequest {
    pub fn new(channel_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
            name: name.into(),
        }
    }
}

impl SlackApiMethod for AdminConversationsRenameRequest {
    const METHOD: &'static str = "admin.conversations.rename";
    type Response = AdminConversationsRenameResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.rename`](https://docs.slack.dev/reference/methods/admin.conversations.rename).
#[doc(alias = "admin.conversations.rename")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsRenameResponse {
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

/// Arguments for the Slack Web API method [`admin.conversations.restrictAccess.addGroup`](https://docs.slack.dev/reference/methods/admin.conversations.restrictAccess.addGroup): Add an allowlist of IDP groups for accessing a channel
///
/// Send it with [`SlackClient::admin_conversations_restrict_access_add_group`].
#[doc(alias = "admin.conversations.restrictAccess.addGroup")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsRestrictAccessAddGroupRequest {
    /// The [IDP Group](https://slack.com/help/articles/115001435788-Connect-identity-provider-groups-to-your-Enterprise-organization) ID to be an allowlist for the private channel.
    pub group_id: String,
    /// The channel to link this group to.
    pub channel_id: String,
    /// The workspace where the channel exists. This argument is required for channels only tied to one workspace, and optional for channels that are shared across an organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl AdminConversationsRestrictAccessAddGroupRequest {
    pub fn new(group_id: impl Into<String>, channel_id: impl Into<String>) -> Self {
        Self {
            group_id: group_id.into(),
            channel_id: channel_id.into(),
            team_id: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for AdminConversationsRestrictAccessAddGroupRequest {
    const METHOD: &'static str = "admin.conversations.restrictAccess.addGroup";
    type Response = AdminConversationsRestrictAccessAddGroupResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.restrictAccess.addGroup`](https://docs.slack.dev/reference/methods/admin.conversations.restrictAccess.addGroup).
#[doc(alias = "admin.conversations.restrictAccess.addGroup")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsRestrictAccessAddGroupResponse {
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

/// Arguments for the Slack Web API method [`admin.conversations.restrictAccess.listGroups`](https://docs.slack.dev/reference/methods/admin.conversations.restrictAccess.listGroups): List all IDP Groups linked to a channel
///
/// Send it with [`SlackClient::admin_conversations_restrict_access_list_groups`].
#[doc(alias = "admin.conversations.restrictAccess.listGroups")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsRestrictAccessListGroupsRequest {
    pub channel_id: String,
    /// The workspace where the channel exists. This argument is required for channels only tied to one workspace, and optional for channels that are shared across an organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl AdminConversationsRestrictAccessListGroupsRequest {
    pub fn new(channel_id: impl Into<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
            team_id: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for AdminConversationsRestrictAccessListGroupsRequest {
    const METHOD: &'static str = "admin.conversations.restrictAccess.listGroups";
    type Response = AdminConversationsRestrictAccessListGroupsResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.restrictAccess.listGroups`](https://docs.slack.dev/reference/methods/admin.conversations.restrictAccess.listGroups).
#[doc(alias = "admin.conversations.restrictAccess.listGroups")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsRestrictAccessListGroupsResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_ids: Vec<String>,
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

/// Arguments for the Slack Web API method [`admin.conversations.restrictAccess.removeGroup`](https://docs.slack.dev/reference/methods/admin.conversations.restrictAccess.removeGroup): Remove a linked IDP group linked from a private channel
///
/// Send it with [`SlackClient::admin_conversations_restrict_access_remove_group`].
#[doc(alias = "admin.conversations.restrictAccess.removeGroup")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsRestrictAccessRemoveGroupRequest {
    /// The workspace where the channel exists. This argument is required for channels only tied to one workspace, and optional for channels that are shared across an organization.
    pub team_id: String,
    /// The [IDP Group](https://slack.com/help/articles/115001435788-Connect-identity-provider-groups-to-your-Enterprise-organization) ID to remove from the private channel.
    pub group_id: String,
    /// The channel to remove the linked group from.
    pub channel_id: String,
}

impl AdminConversationsRestrictAccessRemoveGroupRequest {
    pub fn new(
        team_id: impl Into<String>,
        group_id: impl Into<String>,
        channel_id: impl Into<String>,
    ) -> Self {
        Self {
            team_id: team_id.into(),
            group_id: group_id.into(),
            channel_id: channel_id.into(),
        }
    }
}

impl SlackApiMethod for AdminConversationsRestrictAccessRemoveGroupRequest {
    const METHOD: &'static str = "admin.conversations.restrictAccess.removeGroup";
    type Response = AdminConversationsRestrictAccessRemoveGroupResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.restrictAccess.removeGroup`](https://docs.slack.dev/reference/methods/admin.conversations.restrictAccess.removeGroup).
#[doc(alias = "admin.conversations.restrictAccess.removeGroup")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsRestrictAccessRemoveGroupResponse {
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

/// Arguments for the Slack Web API method [`admin.conversations.search`](https://docs.slack.dev/reference/methods/admin.conversations.search): Search for public or private channels in an Enterprise organization.
///
/// Send it with [`SlackClient::admin_conversations_search`].
#[doc(alias = "admin.conversations.search")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminConversationsSearchRequest {
    /// Comma separated string of team IDs, signifying the internal workspaces to search through.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<String>>,
    /// Array of encoded team IDs, signifying the external orgs to search through.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub connected_team_ids: Option<Vec<String>>,
    /// Name of the the channel to query by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Maximum number of items to be returned. Must be between 1 - 20 both inclusive. Default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The type of channel to include or exclude in the search. For example `private` will search private channels, while `private_exclude` will exclude them. For a full list of types, check the [Types section](#types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_channel_types: Option<Vec<String>>,
    /// Possible values are `relevant` (search ranking based on what we think is closest), `name` (alphabetical), `member_count` (number of users in the channel), and `created` (date channel was created). You can optionally pair this with the `sort_dir` arg to change how it is sorted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Sort direction. Possible values are `asc` for ascending order like (1, 2, 3) or (a, b, c), and `desc` for descending order like (3, 2, 1) or (c, b, a)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_dir: Option<String>,
    /// Only return the total\_count of channels. Omits channel data and allows access for admins without channel manager permissions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count_only: Option<bool>,
}

impl AdminConversationsSearchRequest {
    pub fn new() -> Self {
        Self {
            team_ids: None,
            connected_team_ids: None,
            query: None,
            limit: None,
            cursor: None,
            search_channel_types: None,
            sort: None,
            sort_dir: None,
            total_count_only: None,
        }
    }

    pub fn team_ids(mut self, team_ids: Vec<String>) -> Self {
        self.team_ids = Some(team_ids);
        self
    }

    pub fn connected_team_ids(mut self, connected_team_ids: Vec<String>) -> Self {
        self.connected_team_ids = Some(connected_team_ids);
        self
    }

    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
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

    pub fn search_channel_types(mut self, search_channel_types: Vec<String>) -> Self {
        self.search_channel_types = Some(search_channel_types);
        self
    }

    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    pub fn sort_dir(mut self, sort_dir: impl Into<String>) -> Self {
        self.sort_dir = Some(sort_dir.into());
        self
    }

    pub fn total_count_only(mut self, total_count_only: bool) -> Self {
        self.total_count_only = Some(total_count_only);
        self
    }
}

impl SlackApiMethod for AdminConversationsSearchRequest {
    const METHOD: &'static str = "admin.conversations.search";
    type Response = AdminConversationsSearchResponse;
}

impl CursorPaginated for AdminConversationsSearchRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminConversationsSearchResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.next_cursor
            .as_deref()
            .filter(|c| !c.is_empty())
            .or_else(|| self.response_metadata.as_ref()?.next_cursor.as_deref())
    }
}

/// Successful response of the Slack Web API method [`admin.conversations.search`](https://docs.slack.dev/reference/methods/admin.conversations.search).
#[doc(alias = "admin.conversations.search")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsSearchResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conversations: Vec<AdminConversationsSearchResponseConversations>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_cursor: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_count: Option<i64>,
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
pub struct AdminConversationsSearchResponseConversations {
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
    pub purpose: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub created: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub creator_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_private: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_archived: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_general: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_activity_ts: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_ext_shared: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_global_shared: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_org_default: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_org_mandatory: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_org_shared: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_frozen: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub connected_team_ids: Vec<serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_team_ids_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_team_ids_sample_team: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pending_connected_team_ids: Vec<serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_pending_ext_shared: Option<bool>,
}

/// Arguments for the Slack Web API method [`admin.conversations.setConversationPrefs`](https://docs.slack.dev/reference/methods/admin.conversations.setConversationPrefs): Set the posting permissions for a public or private channel.
///
/// Send it with [`SlackClient::admin_conversations_set_conversation_prefs`].
#[doc(alias = "admin.conversations.setConversationPrefs")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsSetConversationPrefsRequest {
    /// The channel to set the prefs for
    pub channel_id: String,
    /// The prefs for this channel in a stringified JSON format.
    pub prefs: String,
}

impl AdminConversationsSetConversationPrefsRequest {
    pub fn new(channel_id: impl Into<String>, prefs: impl Into<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
            prefs: prefs.into(),
        }
    }
}

impl SlackApiMethod for AdminConversationsSetConversationPrefsRequest {
    const METHOD: &'static str = "admin.conversations.setConversationPrefs";
    type Response = AdminConversationsSetConversationPrefsResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.setConversationPrefs`](https://docs.slack.dev/reference/methods/admin.conversations.setConversationPrefs).
#[doc(alias = "admin.conversations.setConversationPrefs")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsSetConversationPrefsResponse {
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

/// Arguments for the Slack Web API method [`admin.conversations.setCustomRetention`](https://docs.slack.dev/reference/methods/admin.conversations.setCustomRetention): This API endpoint can be used by any admin to set a conversation's retention policy.
///
/// Send it with [`SlackClient::admin_conversations_set_custom_retention`].
#[doc(alias = "admin.conversations.setCustomRetention")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsSetCustomRetentionRequest {
    /// The conversation to set the retention policy for.
    pub channel_id: String,
    /// The message retention duration in days to set for this conversation
    pub duration_days: i64,
}

impl AdminConversationsSetCustomRetentionRequest {
    pub fn new(channel_id: impl Into<String>, duration_days: i64) -> Self {
        Self {
            channel_id: channel_id.into(),
            duration_days,
        }
    }
}

impl SlackApiMethod for AdminConversationsSetCustomRetentionRequest {
    const METHOD: &'static str = "admin.conversations.setCustomRetention";
    type Response = AdminConversationsSetCustomRetentionResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.setCustomRetention`](https://docs.slack.dev/reference/methods/admin.conversations.setCustomRetention).
#[doc(alias = "admin.conversations.setCustomRetention")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsSetCustomRetentionResponse {
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

/// Arguments for the Slack Web API method [`admin.conversations.setTeams`](https://docs.slack.dev/reference/methods/admin.conversations.setTeams): Set the workspaces in an Enterprise org that connect to a public or private channel.
///
/// Send it with [`SlackClient::admin_conversations_set_teams`].
#[doc(alias = "admin.conversations.setTeams")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsSetTeamsRequest {
    /// The encoded `channel_id` to add or remove to workspaces.
    pub channel_id: String,
    /// The workspace to which the channel belongs if the channel is a local workspace channel. Omit this argument if the channel is a cross-workspace or org-wide shared channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// A comma-separated list of workspaces to which the channel should be shared. Not required if the channel is being shared org-wide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_team_ids: Option<Vec<String>>,
    /// True if channel has to be converted to an org channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_channel: Option<bool>,
}

impl AdminConversationsSetTeamsRequest {
    pub fn new(channel_id: impl Into<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
            team_id: None,
            target_team_ids: None,
            org_channel: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn target_team_ids(mut self, target_team_ids: Vec<String>) -> Self {
        self.target_team_ids = Some(target_team_ids);
        self
    }

    pub fn org_channel(mut self, org_channel: bool) -> Self {
        self.org_channel = Some(org_channel);
        self
    }
}

impl SlackApiMethod for AdminConversationsSetTeamsRequest {
    const METHOD: &'static str = "admin.conversations.setTeams";
    type Response = AdminConversationsSetTeamsResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.setTeams`](https://docs.slack.dev/reference/methods/admin.conversations.setTeams).
#[doc(alias = "admin.conversations.setTeams")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsSetTeamsResponse {
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

/// Arguments for the Slack Web API method [`admin.conversations.unarchive`](https://docs.slack.dev/reference/methods/admin.conversations.unarchive): Unarchive a public or private channel.
///
/// Send it with [`SlackClient::admin_conversations_unarchive`].
#[doc(alias = "admin.conversations.unarchive")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsUnarchiveRequest {
    /// The channel to unarchive.
    pub channel_id: String,
}

impl AdminConversationsUnarchiveRequest {
    pub fn new(channel_id: impl Into<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
        }
    }
}

impl SlackApiMethod for AdminConversationsUnarchiveRequest {
    const METHOD: &'static str = "admin.conversations.unarchive";
    type Response = AdminConversationsUnarchiveResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.unarchive`](https://docs.slack.dev/reference/methods/admin.conversations.unarchive).
#[doc(alias = "admin.conversations.unarchive")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsUnarchiveResponse {
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

/// Arguments for the Slack Web API method [`admin.conversations.unlinkObjects`](https://docs.slack.dev/reference/methods/admin.conversations.unlinkObjects): Unlink a Salesforce record from a channel
///
/// Send it with [`SlackClient::admin_conversations_unlink_objects`].
#[doc(alias = "admin.conversations.unlinkObjects")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminConversationsUnlinkObjectsRequest {
    /// Channel ID for Slack channel that will be unlinked from the Salesforce record.
    pub channel: String,
    /// Channel name you would like to give to the channel that is being unlinked from the Salesforce record.
    pub new_name: String,
}

impl AdminConversationsUnlinkObjectsRequest {
    pub fn new(channel: impl Into<String>, new_name: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            new_name: new_name.into(),
        }
    }
}

impl SlackApiMethod for AdminConversationsUnlinkObjectsRequest {
    const METHOD: &'static str = "admin.conversations.unlinkObjects";
    type Response = AdminConversationsUnlinkObjectsResponse;
}

/// Successful response of the Slack Web API method [`admin.conversations.unlinkObjects`](https://docs.slack.dev/reference/methods/admin.conversations.unlinkObjects).
#[doc(alias = "admin.conversations.unlinkObjects")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminConversationsUnlinkObjectsResponse {
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
    /// Calls the Slack Web API method [`admin.conversations.archive`](https://docs.slack.dev/reference/methods/admin.conversations.archive): Archive a public or private channel.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.archive")]
    pub async fn admin_conversations_archive(
        &self,
        request: &AdminConversationsArchiveRequest,
    ) -> Result<AdminConversationsArchiveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.bulkArchive`](https://docs.slack.dev/reference/methods/admin.conversations.bulkArchive): Archive public or private channels in bulk.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.bulkArchive")]
    pub async fn admin_conversations_bulk_archive(
        &self,
        request: &AdminConversationsBulkArchiveRequest,
    ) -> Result<AdminConversationsBulkArchiveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.bulkDelete`](https://docs.slack.dev/reference/methods/admin.conversations.bulkDelete): Delete public or private channels in bulk
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.bulkDelete")]
    pub async fn admin_conversations_bulk_delete(
        &self,
        request: &AdminConversationsBulkDeleteRequest,
    ) -> Result<AdminConversationsBulkDeleteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.bulkMove`](https://docs.slack.dev/reference/methods/admin.conversations.bulkMove): Move public or private channels in bulk.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.bulkMove")]
    pub async fn admin_conversations_bulk_move(
        &self,
        request: &AdminConversationsBulkMoveRequest,
    ) -> Result<AdminConversationsBulkMoveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.bulkSetExcludeFromSlackAi`](https://docs.slack.dev/reference/methods/admin.conversations.bulkSetExcludeFromSlackAi): Exclude channels from Slack AI in bulk
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.bulkSetExcludeFromSlackAi")]
    pub async fn admin_conversations_bulk_set_exclude_from_slack_ai(
        &self,
        request: &AdminConversationsBulkSetExcludeFromSlackAiRequest,
    ) -> Result<AdminConversationsBulkSetExcludeFromSlackAiResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.bulkSetProperties`](https://docs.slack.dev/reference/methods/admin.conversations.bulkSetProperties): Set properties on channels in bulk
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.bulkSetProperties")]
    pub async fn admin_conversations_bulk_set_properties(
        &self,
        request: &AdminConversationsBulkSetPropertiesRequest,
    ) -> Result<AdminConversationsBulkSetPropertiesResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.convertToPrivate`](https://docs.slack.dev/reference/methods/admin.conversations.convertToPrivate): Convert a public channel to a private channel.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.convertToPrivate")]
    pub async fn admin_conversations_convert_to_private(
        &self,
        request: &AdminConversationsConvertToPrivateRequest,
    ) -> Result<AdminConversationsConvertToPrivateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.convertToPublic`](https://docs.slack.dev/reference/methods/admin.conversations.convertToPublic): Convert a private channel to a public channel.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.convertToPublic")]
    pub async fn admin_conversations_convert_to_public(
        &self,
        request: &AdminConversationsConvertToPublicRequest,
    ) -> Result<AdminConversationsConvertToPublicResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.create`](https://docs.slack.dev/reference/methods/admin.conversations.create): Create a public or private channel-based conversation.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.create")]
    pub async fn admin_conversations_create(
        &self,
        request: &AdminConversationsCreateRequest,
    ) -> Result<AdminConversationsCreateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.createForObjects`](https://docs.slack.dev/reference/methods/admin.conversations.createForObjects): Create a Salesforce channel for the corresponding object provided.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:manage_objects`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.createForObjects")]
    pub async fn admin_conversations_create_for_objects(
        &self,
        request: &AdminConversationsCreateForObjectsRequest,
    ) -> Result<AdminConversationsCreateForObjectsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.delete`](https://docs.slack.dev/reference/methods/admin.conversations.delete): Delete a public or private channel.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.delete")]
    pub async fn admin_conversations_delete(
        &self,
        request: &AdminConversationsDeleteRequest,
    ) -> Result<AdminConversationsDeleteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.disconnectShared`](https://docs.slack.dev/reference/methods/admin.conversations.disconnectShared): Disconnect a connected channel from one or more workspaces.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.disconnectShared")]
    pub async fn admin_conversations_disconnect_shared(
        &self,
        request: &AdminConversationsDisconnectSharedRequest,
    ) -> Result<AdminConversationsDisconnectSharedResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.ekm.listOriginalConnectedChannelInfo`](https://docs.slack.dev/reference/methods/admin.conversations.ekm.listOriginalConnectedChannelInfo): List all disconnected channels—i.e., channels that were once connected to other workspaces and then disconnected—and the corresponding original channel IDs for key revocation with EKM.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.ekm.listOriginalConnectedChannelInfo")]
    pub async fn admin_conversations_ekm_list_original_connected_channel_info(
        &self,
        request: &AdminConversationsEkmListOriginalConnectedChannelInfoRequest,
    ) -> Result<AdminConversationsEkmListOriginalConnectedChannelInfoResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.getConversationPrefs`](https://docs.slack.dev/reference/methods/admin.conversations.getConversationPrefs): Get conversation preferences for a public or private channel.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.getConversationPrefs")]
    pub async fn admin_conversations_get_conversation_prefs(
        &self,
        request: &AdminConversationsGetConversationPrefsRequest,
    ) -> Result<AdminConversationsGetConversationPrefsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.getCustomRetention`](https://docs.slack.dev/reference/methods/admin.conversations.getCustomRetention): This API endpoint can be used by any admin to get a conversation's retention policy.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.getCustomRetention")]
    pub async fn admin_conversations_get_custom_retention(
        &self,
        request: &AdminConversationsGetCustomRetentionRequest,
    ) -> Result<AdminConversationsGetCustomRetentionResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.getTeams`](https://docs.slack.dev/reference/methods/admin.conversations.getTeams): Get all the workspaces a given public or private channel is connected to within this Enterprise org.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.getTeams")]
    pub async fn admin_conversations_get_teams(
        &self,
        request: &AdminConversationsGetTeamsRequest,
    ) -> Result<AdminConversationsGetTeamsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.invite`](https://docs.slack.dev/reference/methods/admin.conversations.invite): Invite a user to a public or private channel.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.invite")]
    pub async fn admin_conversations_invite(
        &self,
        request: &AdminConversationsInviteRequest,
    ) -> Result<AdminConversationsInviteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.linkObjects`](https://docs.slack.dev/reference/methods/admin.conversations.linkObjects): Link a Salesforce record to a channel
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:manage_objects`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.linkObjects")]
    pub async fn admin_conversations_link_objects(
        &self,
        request: &AdminConversationsLinkObjectsRequest,
    ) -> Result<AdminConversationsLinkObjectsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.lookup`](https://docs.slack.dev/reference/methods/admin.conversations.lookup): Returns channels on the given team using the filters.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.lookup")]
    pub async fn admin_conversations_lookup(
        &self,
        request: &AdminConversationsLookupRequest,
    ) -> Result<AdminConversationsLookupResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.removeCustomRetention`](https://docs.slack.dev/reference/methods/admin.conversations.removeCustomRetention): This API endpoint can be used by any admin to remove a conversation's retention policy.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.removeCustomRetention")]
    pub async fn admin_conversations_remove_custom_retention(
        &self,
        request: &AdminConversationsRemoveCustomRetentionRequest,
    ) -> Result<AdminConversationsRemoveCustomRetentionResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.rename`](https://docs.slack.dev/reference/methods/admin.conversations.rename): Rename a public or private channel.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.rename")]
    pub async fn admin_conversations_rename(
        &self,
        request: &AdminConversationsRenameRequest,
    ) -> Result<AdminConversationsRenameResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.restrictAccess.addGroup`](https://docs.slack.dev/reference/methods/admin.conversations.restrictAccess.addGroup): Add an allowlist of IDP groups for accessing a channel
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.restrictAccess.addGroup")]
    pub async fn admin_conversations_restrict_access_add_group(
        &self,
        request: &AdminConversationsRestrictAccessAddGroupRequest,
    ) -> Result<AdminConversationsRestrictAccessAddGroupResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.restrictAccess.listGroups`](https://docs.slack.dev/reference/methods/admin.conversations.restrictAccess.listGroups): List all IDP Groups linked to a channel
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.restrictAccess.listGroups")]
    pub async fn admin_conversations_restrict_access_list_groups(
        &self,
        request: &AdminConversationsRestrictAccessListGroupsRequest,
    ) -> Result<AdminConversationsRestrictAccessListGroupsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.restrictAccess.removeGroup`](https://docs.slack.dev/reference/methods/admin.conversations.restrictAccess.removeGroup): Remove a linked IDP group linked from a private channel
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 1 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.restrictAccess.removeGroup")]
    pub async fn admin_conversations_restrict_access_remove_group(
        &self,
        request: &AdminConversationsRestrictAccessRemoveGroupRequest,
    ) -> Result<AdminConversationsRestrictAccessRemoveGroupResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.search`](https://docs.slack.dev/reference/methods/admin.conversations.search): Search for public or private channels in an Enterprise organization.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.search")]
    pub async fn admin_conversations_search(
        &self,
        request: &AdminConversationsSearchRequest,
    ) -> Result<AdminConversationsSearchResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.setConversationPrefs`](https://docs.slack.dev/reference/methods/admin.conversations.setConversationPrefs): Set the posting permissions for a public or private channel.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.setConversationPrefs")]
    pub async fn admin_conversations_set_conversation_prefs(
        &self,
        request: &AdminConversationsSetConversationPrefsRequest,
    ) -> Result<AdminConversationsSetConversationPrefsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.setCustomRetention`](https://docs.slack.dev/reference/methods/admin.conversations.setCustomRetention): This API endpoint can be used by any admin to set a conversation's retention policy.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.setCustomRetention")]
    pub async fn admin_conversations_set_custom_retention(
        &self,
        request: &AdminConversationsSetCustomRetentionRequest,
    ) -> Result<AdminConversationsSetCustomRetentionResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.setTeams`](https://docs.slack.dev/reference/methods/admin.conversations.setTeams): Set the workspaces in an Enterprise org that connect to a public or private channel.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.setTeams")]
    pub async fn admin_conversations_set_teams(
        &self,
        request: &AdminConversationsSetTeamsRequest,
    ) -> Result<AdminConversationsSetTeamsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.unarchive`](https://docs.slack.dev/reference/methods/admin.conversations.unarchive): Unarchive a public or private channel.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.unarchive")]
    pub async fn admin_conversations_unarchive(
        &self,
        request: &AdminConversationsUnarchiveRequest,
    ) -> Result<AdminConversationsUnarchiveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.conversations.unlinkObjects`](https://docs.slack.dev/reference/methods/admin.conversations.unlinkObjects): Unlink a Salesforce record from a channel
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.conversations:manage_objects`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.conversations.unlinkObjects")]
    pub async fn admin_conversations_unlink_objects(
        &self,
        request: &AdminConversationsUnlinkObjectsRequest,
    ) -> Result<AdminConversationsUnlinkObjectsResponse, SlackError> {
        self.call(request).await
    }
}
