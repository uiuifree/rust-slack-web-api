// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`conversations.acceptSharedInvite`](https://docs.slack.dev/reference/methods/conversations.acceptSharedInvite): Accepts an invitation to a Slack Connect channel.
///
/// Send it with [`SlackClient::conversations_accept_shared_invite`].
#[doc(alias = "conversations.acceptSharedInvite")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsAcceptSharedInviteRequest {
    /// Name of the channel. If the channel does not exist already in your workspace, this name is the one that the channel will take.
    pub channel_name: String,
    /// Whether the channel should be private.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    /// Whether you'd like to use your workspace's free trial to begin using Slack Connect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_trial_accepted: Option<bool>,
    /// ID of the invite that you’d like to accept. Must provide either `invite_id` or `channel_id`. See the [`shared_channel_invite_received`](https://docs.slack.dev/reference/events/shared_channel_invite_received.md) event payload for more details on how to retrieve the ID of the invitation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_id: Option<String>,
    /// ID of the channel that you'd like to accept. Must provide either `invite_id` or `channel_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// The ID of the workspace to accept the channel in. If an org-level token is used to call this method, the `team_id` argument is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl ConversationsAcceptSharedInviteRequest {
    pub fn new(channel_name: impl Into<String>) -> Self {
        Self {
            channel_name: channel_name.into(),
            is_private: None,
            free_trial_accepted: None,
            invite_id: None,
            channel_id: None,
            team_id: None,
        }
    }

    pub fn is_private(mut self, is_private: bool) -> Self {
        self.is_private = Some(is_private);
        self
    }

    pub fn free_trial_accepted(mut self, free_trial_accepted: bool) -> Self {
        self.free_trial_accepted = Some(free_trial_accepted);
        self
    }

    pub fn invite_id(mut self, invite_id: impl Into<String>) -> Self {
        self.invite_id = Some(invite_id.into());
        self
    }

    pub fn channel_id(mut self, channel_id: impl Into<String>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for ConversationsAcceptSharedInviteRequest {
    const METHOD: &'static str = "conversations.acceptSharedInvite";
    type Response = ConversationsAcceptSharedInviteResponse;
}

/// Successful response of the Slack Web API method [`conversations.acceptSharedInvite`](https://docs.slack.dev/reference/methods/conversations.acceptSharedInvite).
#[doc(alias = "conversations.acceptSharedInvite")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsAcceptSharedInviteResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub implicit_approval: Option<bool>,
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
    pub invite_id: Option<String>,
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

/// Arguments for the Slack Web API method [`conversations.approveSharedInvite`](https://docs.slack.dev/reference/methods/conversations.approveSharedInvite): Approves an invitation to a Slack Connect channel
///
/// Send it with [`SlackClient::conversations_approve_shared_invite`].
#[doc(alias = "conversations.approveSharedInvite")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsApproveSharedInviteRequest {
    /// ID of the shared channel invite to approve
    pub invite_id: String,
    /// The team or enterprise ID of the receiving party involved in the invitation you are approving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_team: Option<String>,
}

impl ConversationsApproveSharedInviteRequest {
    pub fn new(invite_id: impl Into<String>) -> Self {
        Self {
            invite_id: invite_id.into(),
            target_team: None,
        }
    }

    pub fn target_team(mut self, target_team: impl Into<String>) -> Self {
        self.target_team = Some(target_team.into());
        self
    }
}

impl SlackApiMethod for ConversationsApproveSharedInviteRequest {
    const METHOD: &'static str = "conversations.approveSharedInvite";
    type Response = ConversationsApproveSharedInviteResponse;
}

/// Successful response of the Slack Web API method [`conversations.approveSharedInvite`](https://docs.slack.dev/reference/methods/conversations.approveSharedInvite).
#[doc(alias = "conversations.approveSharedInvite")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsApproveSharedInviteResponse {
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

/// Arguments for the Slack Web API method [`conversations.archive`](https://docs.slack.dev/reference/methods/conversations.archive): Archives a conversation.
///
/// Send it with [`SlackClient::conversations_archive`].
#[doc(alias = "conversations.archive")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsArchiveRequest {
    /// ID of conversation to archive
    pub channel: String,
}

impl ConversationsArchiveRequest {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
        }
    }
}

impl SlackApiMethod for ConversationsArchiveRequest {
    const METHOD: &'static str = "conversations.archive";
    type Response = ConversationsArchiveResponse;
}

/// Successful response of the Slack Web API method [`conversations.archive`](https://docs.slack.dev/reference/methods/conversations.archive).
#[doc(alias = "conversations.archive")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsArchiveResponse {
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

/// Arguments for the Slack Web API method [`conversations.canvases.create`](https://docs.slack.dev/reference/methods/conversations.canvases.create): Create a channel canvas for a channel
///
/// Send it with [`SlackClient::conversations_canvases_create`].
#[doc(alias = "conversations.canvases.create")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsCanvasesCreateRequest {
    /// Channel ID of the channel the canvas will be tabbed in.
    pub channel_id: String,
    /// Structure describing the type and value of the content to create. The markdown content is limited to 1 MiB (1,048,576 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_content: Option<String>,
    /// Title of the newly created canvas
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ConversationsCanvasesCreateRequest {
    pub fn new(channel_id: impl Into<String>) -> Self {
        Self {
            channel_id: channel_id.into(),
            document_content: None,
            title: None,
        }
    }

    pub fn document_content(mut self, document_content: impl Into<String>) -> Self {
        self.document_content = Some(document_content.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl SlackApiMethod for ConversationsCanvasesCreateRequest {
    const METHOD: &'static str = "conversations.canvases.create";
    type Response = ConversationsCanvasesCreateResponse;
}

/// Successful response of the Slack Web API method [`conversations.canvases.create`](https://docs.slack.dev/reference/methods/conversations.canvases.create).
#[doc(alias = "conversations.canvases.create")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsCanvasesCreateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub canvas_id: Option<String>,
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

/// Arguments for the Slack Web API method [`conversations.close`](https://docs.slack.dev/reference/methods/conversations.close): Closes a direct message or multi-person direct message.
///
/// Send it with [`SlackClient::conversations_close`].
#[doc(alias = "conversations.close")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsCloseRequest {
    /// Conversation to close.
    pub channel: String,
}

impl ConversationsCloseRequest {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
        }
    }
}

impl SlackApiMethod for ConversationsCloseRequest {
    const METHOD: &'static str = "conversations.close";
    type Response = ConversationsCloseResponse;
}

/// Successful response of the Slack Web API method [`conversations.close`](https://docs.slack.dev/reference/methods/conversations.close).
#[doc(alias = "conversations.close")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsCloseResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub no_op: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub already_closed: Option<bool>,
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

/// Arguments for the Slack Web API method [`conversations.create`](https://docs.slack.dev/reference/methods/conversations.create): Initiates a public or private channel-based conversation
///
/// Send it with [`SlackClient::conversations_create`].
#[doc(alias = "conversations.create")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsCreateRequest {
    /// Name of the public or private channel to create
    pub name: String,
    /// Create a private channel instead of a public one
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    /// encoded team id to create the channel in, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl ConversationsCreateRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_private: None,
            team_id: None,
        }
    }

    pub fn is_private(mut self, is_private: bool) -> Self {
        self.is_private = Some(is_private);
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for ConversationsCreateRequest {
    const METHOD: &'static str = "conversations.create";
    type Response = ConversationsCreateResponse;
}

/// Successful response of the Slack Web API method [`conversations.create`](https://docs.slack.dev/reference/methods/conversations.create).
#[doc(alias = "conversations.create")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsCreateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<Conversation>,
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

/// Arguments for the Slack Web API method [`conversations.declineSharedInvite`](https://docs.slack.dev/reference/methods/conversations.declineSharedInvite): Declines a Slack Connect channel invite.
///
/// Send it with [`SlackClient::conversations_decline_shared_invite`].
#[doc(alias = "conversations.declineSharedInvite")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsDeclineSharedInviteRequest {
    /// ID of the Slack Connect invite to decline. Subscribe to the [`shared_channel_invite_accepted`](https://docs.slack.dev/reference/events/shared_channel_invite_accepted.md) event to receive IDs of Slack Connect channel invites that have been accepted and are awaiting approval.
    pub invite_id: String,
    /// The team or enterprise id of the other party involved in the invitation you are declining
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_team: Option<String>,
}

impl ConversationsDeclineSharedInviteRequest {
    pub fn new(invite_id: impl Into<String>) -> Self {
        Self {
            invite_id: invite_id.into(),
            target_team: None,
        }
    }

    pub fn target_team(mut self, target_team: impl Into<String>) -> Self {
        self.target_team = Some(target_team.into());
        self
    }
}

impl SlackApiMethod for ConversationsDeclineSharedInviteRequest {
    const METHOD: &'static str = "conversations.declineSharedInvite";
    type Response = ConversationsDeclineSharedInviteResponse;
}

/// Successful response of the Slack Web API method [`conversations.declineSharedInvite`](https://docs.slack.dev/reference/methods/conversations.declineSharedInvite).
#[doc(alias = "conversations.declineSharedInvite")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsDeclineSharedInviteResponse {
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

/// Arguments for the Slack Web API method [`conversations.externalInvitePermissions.set`](https://docs.slack.dev/reference/methods/conversations.externalInvitePermissions.set): Upgrade or downgrade Slack Connect channel permissions between 'can post only' and 'can post and invite'.
///
/// Send it with [`SlackClient::conversations_external_invite_permissions_set`].
#[doc(alias = "conversations.externalInvitePermissions.set")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsExternalInvitePermissionsSetRequest {
    /// The channel ID to change external invite permissions for
    pub channel: String,
    /// The encoded team ID of the target team. Must be in the specified channel.
    pub target_team: String,
    /// Type of action to be taken: upgrade or downgrade
    pub action: String,
}

impl ConversationsExternalInvitePermissionsSetRequest {
    pub fn new(
        channel: impl Into<String>,
        target_team: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            channel: channel.into(),
            target_team: target_team.into(),
            action: action.into(),
        }
    }
}

impl SlackApiMethod for ConversationsExternalInvitePermissionsSetRequest {
    const METHOD: &'static str = "conversations.externalInvitePermissions.set";
    type Response = ConversationsExternalInvitePermissionsSetResponse;
}

/// Successful response of the Slack Web API method [`conversations.externalInvitePermissions.set`](https://docs.slack.dev/reference/methods/conversations.externalInvitePermissions.set).
#[doc(alias = "conversations.externalInvitePermissions.set")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsExternalInvitePermissionsSetResponse {
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

/// Arguments for the Slack Web API method [`conversations.history`](https://docs.slack.dev/reference/methods/conversations.history): Fetches a conversation's history of messages and events.
///
/// Send it with [`SlackClient::conversations_history`].
#[doc(alias = "conversations.history")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsHistoryRequest {
    /// Conversation ID to fetch history for.
    pub channel: String,
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](https://docs.slack.dev/apis/web-api/pagination.md) for more detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Return all metadata associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all_metadata: Option<bool>,
    /// Include messages with `oldest` or `latest` timestamps in results. Ignored unless either timestamp is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive: Option<bool>,
    /// Only messages before this Unix timestamp will be included in results. Default is the current time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<String>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the conversation history hasn't been reached. Maximum of 999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Only messages after this Unix timestamp will be included in results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest: Option<String>,
}

impl ConversationsHistoryRequest {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            cursor: None,
            include_all_metadata: None,
            inclusive: None,
            latest: None,
            limit: None,
            oldest: None,
        }
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn include_all_metadata(mut self, include_all_metadata: bool) -> Self {
        self.include_all_metadata = Some(include_all_metadata);
        self
    }

    pub fn inclusive(mut self, inclusive: bool) -> Self {
        self.inclusive = Some(inclusive);
        self
    }

    pub fn latest(mut self, latest: impl Into<String>) -> Self {
        self.latest = Some(latest.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn oldest(mut self, oldest: impl Into<String>) -> Self {
        self.oldest = Some(oldest.into());
        self
    }
}

impl SlackApiMethod for ConversationsHistoryRequest {
    const METHOD: &'static str = "conversations.history";
    type Response = ConversationsHistoryResponse;
}

impl CursorPaginated for ConversationsHistoryRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for ConversationsHistoryResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`conversations.history`](https://docs.slack.dev/reference/methods/conversations.history).
#[doc(alias = "conversations.history")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsHistoryResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub messages: Vec<Message>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_more: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub pin_count: Option<i64>,
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
    pub latest: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
}

/// Arguments for the Slack Web API method [`conversations.info`](https://docs.slack.dev/reference/methods/conversations.info): Retrieve information about a conversation.
///
/// Send it with [`SlackClient::conversations_info`].
#[doc(alias = "conversations.info")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsInfoRequest {
    /// Conversation ID to learn more about
    pub channel: String,
    /// Set this to `true` to receive the locale for this conversation. Defaults to `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_locale: Option<bool>,
    /// Set to `true` to include the member count for the specified conversation. Defaults to `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_num_members: Option<bool>,
}

impl ConversationsInfoRequest {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            include_locale: None,
            include_num_members: None,
        }
    }

    pub fn include_locale(mut self, include_locale: bool) -> Self {
        self.include_locale = Some(include_locale);
        self
    }

    pub fn include_num_members(mut self, include_num_members: bool) -> Self {
        self.include_num_members = Some(include_num_members);
        self
    }
}

impl SlackApiMethod for ConversationsInfoRequest {
    const METHOD: &'static str = "conversations.info";
    type Response = ConversationsInfoResponse;
}

/// Successful response of the Slack Web API method [`conversations.info`](https://docs.slack.dev/reference/methods/conversations.info).
#[doc(alias = "conversations.info")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsInfoResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<Conversation>,
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

/// Arguments for the Slack Web API method [`conversations.invite`](https://docs.slack.dev/reference/methods/conversations.invite): Invites users to a channel.
///
/// Send it with [`SlackClient::conversations_invite`].
#[doc(alias = "conversations.invite")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsInviteRequest {
    /// The ID of the public or private channel to invite user(s) to.
    pub channel: String,
    /// A comma separated list of user IDs. Up to 1000 users may be listed.
    pub users: String,
    /// When set to `true` and multiple user IDs are provided, continue inviting the valid ones while disregarding invalid IDs. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl ConversationsInviteRequest {
    pub fn new(channel: impl Into<String>, users: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            users: users.into(),
            force: None,
        }
    }

    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }
}

impl SlackApiMethod for ConversationsInviteRequest {
    const METHOD: &'static str = "conversations.invite";
    type Response = ConversationsInviteResponse;
}

/// Successful response of the Slack Web API method [`conversations.invite`](https://docs.slack.dev/reference/methods/conversations.invite).
#[doc(alias = "conversations.invite")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsInviteResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<Conversation>,
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

/// Arguments for the Slack Web API method [`conversations.inviteShared`](https://docs.slack.dev/reference/methods/conversations.inviteShared): Sends an invitation to a Slack Connect channel
///
/// Send it with [`SlackClient::conversations_invite_shared`].
#[doc(alias = "conversations.inviteShared")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsInviteSharedRequest {
    /// ID of the channel on your team that you'd like to share
    pub channel: String,
    /// Optional email to receive this invite. Either `emails` or `user_ids` must be provided. Only one email or one user ID may be invited at a time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    /// Optional user\_id to receive this invite. Either `emails` or `user_ids` must be provided. Only one email or one user ID may be invited at a time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// Optional boolean on whether invite is to an external limited member. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_limited: Option<bool>,
}

impl ConversationsInviteSharedRequest {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            emails: None,
            user_ids: None,
            external_limited: None,
        }
    }

    pub fn emails(mut self, emails: Vec<String>) -> Self {
        self.emails = Some(emails);
        self
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    pub fn external_limited(mut self, external_limited: bool) -> Self {
        self.external_limited = Some(external_limited);
        self
    }
}

impl SlackApiMethod for ConversationsInviteSharedRequest {
    const METHOD: &'static str = "conversations.inviteShared";
    type Response = ConversationsInviteSharedResponse;
}

/// Successful response of the Slack Web API method [`conversations.inviteShared`](https://docs.slack.dev/reference/methods/conversations.inviteShared).
#[doc(alias = "conversations.inviteShared")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsInviteSharedResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub invite_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_legacy_shared_channel: Option<bool>,
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

/// Arguments for the Slack Web API method [`conversations.join`](https://docs.slack.dev/reference/methods/conversations.join): Joins an existing conversation.
///
/// Send it with [`SlackClient::conversations_join`].
#[doc(alias = "conversations.join")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsJoinRequest {
    /// ID of conversation to join
    pub channel: String,
}

impl ConversationsJoinRequest {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
        }
    }
}

impl SlackApiMethod for ConversationsJoinRequest {
    const METHOD: &'static str = "conversations.join";
    type Response = ConversationsJoinResponse;
}

/// Successful response of the Slack Web API method [`conversations.join`](https://docs.slack.dev/reference/methods/conversations.join).
#[doc(alias = "conversations.join")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsJoinResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<Conversation>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_metadata: Option<ResponseMetadata>,
}

/// Arguments for the Slack Web API method [`conversations.kick`](https://docs.slack.dev/reference/methods/conversations.kick): Removes a user from a conversation.
///
/// Send it with [`SlackClient::conversations_kick`].
#[doc(alias = "conversations.kick")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsKickRequest {
    /// ID of conversation to remove user from.
    pub channel: String,
    /// User ID to be removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ConversationsKickRequest {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            user: None,
        }
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

impl SlackApiMethod for ConversationsKickRequest {
    const METHOD: &'static str = "conversations.kick";
    type Response = ConversationsKickResponse;
}

/// Successful response of the Slack Web API method [`conversations.kick`](https://docs.slack.dev/reference/methods/conversations.kick).
#[doc(alias = "conversations.kick")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsKickResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub errors: Option<serde_json::Map<String, serde_json::Value>>,
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

/// Arguments for the Slack Web API method [`conversations.leave`](https://docs.slack.dev/reference/methods/conversations.leave): Leaves a conversation.
///
/// Send it with [`SlackClient::conversations_leave`].
#[doc(alias = "conversations.leave")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsLeaveRequest {
    /// Conversation to leave
    pub channel: String,
}

impl ConversationsLeaveRequest {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
        }
    }
}

impl SlackApiMethod for ConversationsLeaveRequest {
    const METHOD: &'static str = "conversations.leave";
    type Response = ConversationsLeaveResponse;
}

/// Successful response of the Slack Web API method [`conversations.leave`](https://docs.slack.dev/reference/methods/conversations.leave).
#[doc(alias = "conversations.leave")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsLeaveResponse {
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

/// Arguments for the Slack Web API method [`conversations.list`](https://docs.slack.dev/reference/methods/conversations.list): Lists all channels in a Slack team.
///
/// Send it with [`SlackClient::conversations_list`].
#[doc(alias = "conversations.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct ConversationsListRequest {
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](https://docs.slack.dev/apis/web-api/pagination.md) for more detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Set to `true` to exclude archived channels from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_archived: Option<bool>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. Must be an integer under 1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// encoded team id to list channels in, required if token belongs to org-wide app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Mix and match channel types by providing a comma-separated list of any combination of `public_channel`, `private_channel`, `mpim`, `im`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
}

impl ConversationsListRequest {
    pub fn new() -> Self {
        Self {
            cursor: None,
            exclude_archived: None,
            limit: None,
            team_id: None,
            types: None,
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
}

impl SlackApiMethod for ConversationsListRequest {
    const METHOD: &'static str = "conversations.list";
    type Response = ConversationsListResponse;
}

impl CursorPaginated for ConversationsListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for ConversationsListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`conversations.list`](https://docs.slack.dev/reference/methods/conversations.list).
#[doc(alias = "conversations.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsListResponse {
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

/// Arguments for the Slack Web API method [`conversations.listConnectInvites`](https://docs.slack.dev/reference/methods/conversations.listConnectInvites): Lists shared channel invites that have been generated or received but have not been approved by all parties
///
/// Send it with [`SlackClient::conversations_list_connect_invites`].
#[doc(alias = "conversations.listConnectInvites")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct ConversationsListConnectInvitesRequest {
    /// Encoded team id for the workspace to retrieve invites for, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Maximum number of invites to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Set to `next_cursor` returned by previous call to list items in subsequent page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ConversationsListConnectInvitesRequest {
    pub fn new() -> Self {
        Self {
            team_id: None,
            count: None,
            cursor: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

impl SlackApiMethod for ConversationsListConnectInvitesRequest {
    const METHOD: &'static str = "conversations.listConnectInvites";
    type Response = ConversationsListConnectInvitesResponse;
}

impl CursorPaginated for ConversationsListConnectInvitesRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for ConversationsListConnectInvitesResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`conversations.listConnectInvites`](https://docs.slack.dev/reference/methods/conversations.listConnectInvites).
#[doc(alias = "conversations.listConnectInvites")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsListConnectInvitesResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub invites: Vec<ConversationsListConnectInvitesResponseInvites>,
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
pub struct ConversationsListConnectInvitesResponseInvites {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub direction: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub status: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_last_updated: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub invite_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub invite: Option<ConversationsListConnectInvitesResponseInvitesInvite>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<Conversation>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub acceptances: Vec<ConversationsListConnectInvitesResponseInvitesAcceptances>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsListConnectInvitesResponseInvitesInvite {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_invalid: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub inviting_team: Option<Team>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub inviting_user: Option<User>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub link: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsListConnectInvitesResponseInvitesAcceptances {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub approval_status: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_accepted: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_invalid: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_last_updated: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub accepting_team: Option<Team>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub accepting_user: Option<User>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reviews: Vec<ConversationsListConnectInvitesResponseInvitesAcceptancesReviews>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsListConnectInvitesResponseInvitesAcceptancesReviews {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_review: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub reviewing_team: Option<Team>,
}

/// Arguments for the Slack Web API method [`conversations.mark`](https://docs.slack.dev/reference/methods/conversations.mark): Sets the read cursor in a channel.
///
/// Send it with [`SlackClient::conversations_mark`].
#[doc(alias = "conversations.mark")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsMarkRequest {
    /// Channel or conversation to set the read cursor for.
    pub channel: String,
    /// Unique identifier of message you want marked as most recently seen in this conversation.
    pub ts: String,
}

impl ConversationsMarkRequest {
    pub fn new(channel: impl Into<String>, ts: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            ts: ts.into(),
        }
    }
}

impl SlackApiMethod for ConversationsMarkRequest {
    const METHOD: &'static str = "conversations.mark";
    type Response = ConversationsMarkResponse;
}

/// Successful response of the Slack Web API method [`conversations.mark`](https://docs.slack.dev/reference/methods/conversations.mark).
#[doc(alias = "conversations.mark")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsMarkResponse {
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

/// Arguments for the Slack Web API method [`conversations.members`](https://docs.slack.dev/reference/methods/conversations.members): Retrieve members of a conversation.
///
/// Send it with [`SlackClient::conversations_members`].
#[doc(alias = "conversations.members")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsMembersRequest {
    /// ID of the conversation to retrieve members for
    pub channel: String,
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](https://docs.slack.dev/apis/web-api/pagination.md) for more detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ConversationsMembersRequest {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
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

impl SlackApiMethod for ConversationsMembersRequest {
    const METHOD: &'static str = "conversations.members";
    type Response = ConversationsMembersResponse;
}

impl CursorPaginated for ConversationsMembersRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for ConversationsMembersResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`conversations.members`](https://docs.slack.dev/reference/methods/conversations.members).
#[doc(alias = "conversations.members")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsMembersResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub members: Vec<String>,
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

/// Arguments for the Slack Web API method [`conversations.open`](https://docs.slack.dev/reference/methods/conversations.open): Opens or resumes a direct message or multi-person direct message.
///
/// Send it with [`SlackClient::conversations_open`].
#[doc(alias = "conversations.open")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct ConversationsOpenRequest {
    /// Resume a conversation by supplying an `im` or `mpim`'s ID. Or provide the `users` field instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// Boolean, indicates you want the full IM channel definition in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_im: Option<bool>,
    /// Comma separated lists of users. If only one user is included, this creates a 1:1 DM. The ordering of the users is preserved whenever a multi-person direct message is returned. Supply a `channel` when not supplying `users`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<String>,
    /// Do not create a direct message or multi-person direct message. This is used to see if there is an existing dm or mpdm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevent_creation: Option<bool>,
}

impl ConversationsOpenRequest {
    pub fn new() -> Self {
        Self {
            channel: None,
            return_im: None,
            users: None,
            prevent_creation: None,
        }
    }

    pub fn channel(mut self, channel: impl Into<String>) -> Self {
        self.channel = Some(channel.into());
        self
    }

    pub fn return_im(mut self, return_im: bool) -> Self {
        self.return_im = Some(return_im);
        self
    }

    pub fn users(mut self, users: impl Into<String>) -> Self {
        self.users = Some(users.into());
        self
    }

    pub fn prevent_creation(mut self, prevent_creation: bool) -> Self {
        self.prevent_creation = Some(prevent_creation);
        self
    }
}

impl SlackApiMethod for ConversationsOpenRequest {
    const METHOD: &'static str = "conversations.open";
    type Response = ConversationsOpenResponse;
}

/// Successful response of the Slack Web API method [`conversations.open`](https://docs.slack.dev/reference/methods/conversations.open).
#[doc(alias = "conversations.open")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsOpenResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<Conversation>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub no_op: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub already_open: Option<bool>,
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

/// Arguments for the Slack Web API method [`conversations.rename`](https://docs.slack.dev/reference/methods/conversations.rename): Renames a conversation.
///
/// Send it with [`SlackClient::conversations_rename`].
#[doc(alias = "conversations.rename")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsRenameRequest {
    /// ID of conversation to rename
    pub channel: String,
    /// New name for conversation.
    pub name: String,
}

impl ConversationsRenameRequest {
    pub fn new(channel: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            name: name.into(),
        }
    }
}

impl SlackApiMethod for ConversationsRenameRequest {
    const METHOD: &'static str = "conversations.rename";
    type Response = ConversationsRenameResponse;
}

/// Successful response of the Slack Web API method [`conversations.rename`](https://docs.slack.dev/reference/methods/conversations.rename).
#[doc(alias = "conversations.rename")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsRenameResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<Conversation>,
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

/// Arguments for the Slack Web API method [`conversations.replies`](https://docs.slack.dev/reference/methods/conversations.replies): Retrieve a thread of messages posted to a conversation
///
/// Send it with [`SlackClient::conversations_replies`].
#[doc(alias = "conversations.replies")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsRepliesRequest {
    /// Conversation ID to fetch thread from.
    pub channel: String,
    /// Unique identifier of either a thread’s parent message or a message in the thread. `ts` must be the timestamp of an existing message with 0 or more replies. If there are no replies then just the single message referenced by `ts` will return - it is just an ordinary, unthreaded message.
    pub ts: String,
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](https://docs.slack.dev/apis/web-api/pagination.md) for more detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Return all metadata associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all_metadata: Option<bool>,
    /// Include messages with `oldest` or `latest` timestamps in results. Ignored unless either timestamp is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive: Option<bool>,
    /// Only messages before this Unix timestamp will be included in results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<String>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Only messages after this Unix timestamp will be included in results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest: Option<String>,
}

impl ConversationsRepliesRequest {
    pub fn new(channel: impl Into<String>, ts: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            ts: ts.into(),
            cursor: None,
            include_all_metadata: None,
            inclusive: None,
            latest: None,
            limit: None,
            oldest: None,
        }
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn include_all_metadata(mut self, include_all_metadata: bool) -> Self {
        self.include_all_metadata = Some(include_all_metadata);
        self
    }

    pub fn inclusive(mut self, inclusive: bool) -> Self {
        self.inclusive = Some(inclusive);
        self
    }

    pub fn latest(mut self, latest: impl Into<String>) -> Self {
        self.latest = Some(latest.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn oldest(mut self, oldest: impl Into<String>) -> Self {
        self.oldest = Some(oldest.into());
        self
    }
}

impl SlackApiMethod for ConversationsRepliesRequest {
    const METHOD: &'static str = "conversations.replies";
    type Response = ConversationsRepliesResponse;
}

impl CursorPaginated for ConversationsRepliesRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for ConversationsRepliesResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`conversations.replies`](https://docs.slack.dev/reference/methods/conversations.replies).
#[doc(alias = "conversations.replies")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsRepliesResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub messages: Vec<Message>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_more: Option<bool>,
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

/// Arguments for the Slack Web API method [`conversations.requestSharedInvite.approve`](https://docs.slack.dev/reference/methods/conversations.requestSharedInvite.approve): Approves a request to add an external user to a channel and sends them a Slack Connect invite
///
/// Send it with [`SlackClient::conversations_request_shared_invite_approve`].
#[doc(alias = "conversations.requestSharedInvite.approve")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsRequestSharedInviteApproveRequest {
    /// ID of the requested shared channel invite to approve.
    pub invite_id: String,
    /// Optional boolean on whether the invited team will have post-only permissions in the channel. Will override the value on the requested invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_external_limited: Option<bool>,
    /// Optional channel\_id to which external user will be invited to. Will override the value on the requested invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// Object describing the text to send along with the invite. If this object is specified, both `text` and `is_override` are required properties. If `is_override` is set to `true`, `text` will override the original invitation message. Otherwise, `text` will be appended to the original invitation message. The total length of the message cannot exceed 560 characters. If `is_override` is set to `false`, the length of `text` and the user specified message on the invite request in total must be less than 560 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<serde_json::Value>,
}

impl ConversationsRequestSharedInviteApproveRequest {
    pub fn new(invite_id: impl Into<String>) -> Self {
        Self {
            invite_id: invite_id.into(),
            is_external_limited: None,
            channel_id: None,
            message: None,
        }
    }

    pub fn is_external_limited(mut self, is_external_limited: bool) -> Self {
        self.is_external_limited = Some(is_external_limited);
        self
    }

    pub fn channel_id(mut self, channel_id: impl Into<String>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    pub fn message(mut self, message: serde_json::Value) -> Self {
        self.message = Some(message);
        self
    }
}

impl SlackApiMethod for ConversationsRequestSharedInviteApproveRequest {
    const METHOD: &'static str = "conversations.requestSharedInvite.approve";
    type Response = ConversationsRequestSharedInviteApproveResponse;
}

/// Successful response of the Slack Web API method [`conversations.requestSharedInvite.approve`](https://docs.slack.dev/reference/methods/conversations.requestSharedInvite.approve).
#[doc(alias = "conversations.requestSharedInvite.approve")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsRequestSharedInviteApproveResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub invite_id: Option<String>,
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

/// Arguments for the Slack Web API method [`conversations.requestSharedInvite.deny`](https://docs.slack.dev/reference/methods/conversations.requestSharedInvite.deny): Denies a request to invite an external user to a channel
///
/// Send it with [`SlackClient::conversations_request_shared_invite_deny`].
#[doc(alias = "conversations.requestSharedInvite.deny")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsRequestSharedInviteDenyRequest {
    /// ID of the requested shared channel invite to deny.
    pub invite_id: String,
    /// Optional message explaining why the request to invite was denied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ConversationsRequestSharedInviteDenyRequest {
    pub fn new(invite_id: impl Into<String>) -> Self {
        Self {
            invite_id: invite_id.into(),
            message: None,
        }
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

impl SlackApiMethod for ConversationsRequestSharedInviteDenyRequest {
    const METHOD: &'static str = "conversations.requestSharedInvite.deny";
    type Response = ConversationsRequestSharedInviteDenyResponse;
}

/// Successful response of the Slack Web API method [`conversations.requestSharedInvite.deny`](https://docs.slack.dev/reference/methods/conversations.requestSharedInvite.deny).
#[doc(alias = "conversations.requestSharedInvite.deny")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsRequestSharedInviteDenyResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub invite_id: Option<String>,
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

/// Arguments for the Slack Web API method [`conversations.requestSharedInvite.list`](https://docs.slack.dev/reference/methods/conversations.requestSharedInvite.list): Lists requests to add external users to channels with ability to filter.
///
/// Send it with [`SlackClient::conversations_request_shared_invite_list`].
#[doc(alias = "conversations.requestSharedInvite.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct ConversationsRequestSharedInviteListRequest {
    /// Optional filter to return invitation requests for the inviting user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// When true expired invitation requests will be returned, otherwise they will be excluded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_expired: Option<bool>,
    /// When true approved invitation requests will be returned, otherwise they will be excluded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_approved: Option<bool>,
    /// When true denied invitation requests will be returned, otherwise they will be excluded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_denied: Option<bool>,
    /// An optional list of invitation ids to look up
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_ids: Option<Vec<String>>,
    /// The number of items to return. Must be between 1 - 1000 (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. See [pagination](https://docs.slack.dev/apis/web-api/pagination.md) for more detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ConversationsRequestSharedInviteListRequest {
    pub fn new() -> Self {
        Self {
            user_id: None,
            include_expired: None,
            include_approved: None,
            include_denied: None,
            invite_ids: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn include_expired(mut self, include_expired: bool) -> Self {
        self.include_expired = Some(include_expired);
        self
    }

    pub fn include_approved(mut self, include_approved: bool) -> Self {
        self.include_approved = Some(include_approved);
        self
    }

    pub fn include_denied(mut self, include_denied: bool) -> Self {
        self.include_denied = Some(include_denied);
        self
    }

    pub fn invite_ids(mut self, invite_ids: Vec<String>) -> Self {
        self.invite_ids = Some(invite_ids);
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

impl SlackApiMethod for ConversationsRequestSharedInviteListRequest {
    const METHOD: &'static str = "conversations.requestSharedInvite.list";
    type Response = ConversationsRequestSharedInviteListResponse;
}

impl CursorPaginated for ConversationsRequestSharedInviteListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for ConversationsRequestSharedInviteListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`conversations.requestSharedInvite.list`](https://docs.slack.dev/reference/methods/conversations.requestSharedInvite.list).
#[doc(alias = "conversations.requestSharedInvite.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsRequestSharedInviteListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub invite_requests: Vec<ConversationsRequestSharedInviteListResponseInviteRequests>,
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
pub struct ConversationsRequestSharedInviteListResponseInviteRequests {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_last_updated: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires_at: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub inviting_team: Option<Team>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub inviting_user: Option<User>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_external_limited: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_sponsored: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub recipient_email: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_user: Option<ConversationsRequestSharedInviteListResponseInviteRequestsTargetUser>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_denied: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<Conversation>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub reviewing_user:
        Option<ConversationsRequestSharedInviteListResponseInviteRequestsReviewingUser>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsRequestSharedInviteListResponseInviteRequestsTargetUser {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub recipient_email: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub recipient_user_id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsRequestSharedInviteListResponseInviteRequestsReviewingUser {
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
    pub team_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub who_can_share_contact_card: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub profile: Option<UserProfile>,
}

/// Arguments for the Slack Web API method [`conversations.setPurpose`](https://docs.slack.dev/reference/methods/conversations.setPurpose): Sets the channel description.
///
/// Send it with [`SlackClient::conversations_set_purpose`].
#[doc(alias = "conversations.setPurpose")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsSetPurposeRequest {
    /// Channel to set the description of
    pub channel: String,
    /// The description
    pub purpose: String,
}

impl ConversationsSetPurposeRequest {
    pub fn new(channel: impl Into<String>, purpose: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            purpose: purpose.into(),
        }
    }
}

impl SlackApiMethod for ConversationsSetPurposeRequest {
    const METHOD: &'static str = "conversations.setPurpose";
    type Response = ConversationsSetPurposeResponse;
}

/// Successful response of the Slack Web API method [`conversations.setPurpose`](https://docs.slack.dev/reference/methods/conversations.setPurpose).
#[doc(alias = "conversations.setPurpose")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsSetPurposeResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub purpose: Option<String>,
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

/// Arguments for the Slack Web API method [`conversations.setTopic`](https://docs.slack.dev/reference/methods/conversations.setTopic): Sets the topic for a conversation.
///
/// Send it with [`SlackClient::conversations_set_topic`].
#[doc(alias = "conversations.setTopic")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsSetTopicRequest {
    /// Conversation to set the topic of
    pub channel: String,
    /// The new topic string. Does not support formatting or linkification.
    pub topic: String,
}

impl ConversationsSetTopicRequest {
    pub fn new(channel: impl Into<String>, topic: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            topic: topic.into(),
        }
    }
}

impl SlackApiMethod for ConversationsSetTopicRequest {
    const METHOD: &'static str = "conversations.setTopic";
    type Response = ConversationsSetTopicResponse;
}

/// Successful response of the Slack Web API method [`conversations.setTopic`](https://docs.slack.dev/reference/methods/conversations.setTopic).
#[doc(alias = "conversations.setTopic")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsSetTopicResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel: Option<Conversation>,
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

/// Arguments for the Slack Web API method [`conversations.unarchive`](https://docs.slack.dev/reference/methods/conversations.unarchive): Reverses conversation archival.
///
/// Send it with [`SlackClient::conversations_unarchive`].
#[doc(alias = "conversations.unarchive")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConversationsUnarchiveRequest {
    /// ID of conversation to unarchive
    pub channel: String,
}

impl ConversationsUnarchiveRequest {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
        }
    }
}

impl SlackApiMethod for ConversationsUnarchiveRequest {
    const METHOD: &'static str = "conversations.unarchive";
    type Response = ConversationsUnarchiveResponse;
}

/// Successful response of the Slack Web API method [`conversations.unarchive`](https://docs.slack.dev/reference/methods/conversations.unarchive).
#[doc(alias = "conversations.unarchive")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsUnarchiveResponse {
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
    /// Calls the Slack Web API method [`conversations.acceptSharedInvite`](https://docs.slack.dev/reference/methods/conversations.acceptSharedInvite): Accepts an invitation to a Slack Connect channel.
    ///
    /// Required scopes:
    ///
    /// - bot token: `conversations.connect:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.acceptSharedInvite")]
    pub async fn conversations_accept_shared_invite(
        &self,
        request: &ConversationsAcceptSharedInviteRequest,
    ) -> Result<ConversationsAcceptSharedInviteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.approveSharedInvite`](https://docs.slack.dev/reference/methods/conversations.approveSharedInvite): Approves an invitation to a Slack Connect channel
    ///
    /// Required scopes:
    ///
    /// - bot token: `conversations.connect:manage`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.approveSharedInvite")]
    pub async fn conversations_approve_shared_invite(
        &self,
        request: &ConversationsApproveSharedInviteRequest,
    ) -> Result<ConversationsApproveSharedInviteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.archive`](https://docs.slack.dev/reference/methods/conversations.archive): Archives a conversation.
    ///
    /// Required scopes:
    ///
    /// - bot token: `groups:write`, `im:write`, `mpim:write`, `channels:write`, `channels:manage`
    /// - user token: `groups:write`, `im:write`, `mpim:write`, `channels:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.archive")]
    pub async fn conversations_archive(
        &self,
        request: &ConversationsArchiveRequest,
    ) -> Result<ConversationsArchiveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.canvases.create`](https://docs.slack.dev/reference/methods/conversations.canvases.create): Create a channel canvas for a channel
    ///
    /// Required scopes:
    ///
    /// - bot token: `canvases:write`
    /// - user token: `canvases:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.canvases.create")]
    pub async fn conversations_canvases_create(
        &self,
        request: &ConversationsCanvasesCreateRequest,
    ) -> Result<ConversationsCanvasesCreateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.close`](https://docs.slack.dev/reference/methods/conversations.close): Closes a direct message or multi-person direct message.
    ///
    /// Required scopes:
    ///
    /// - bot token: `groups:write`, `im:write`, `mpim:write`, `channels:write`, `channels:manage`
    /// - user token: `groups:write`, `im:write`, `mpim:write`, `channels:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.close")]
    pub async fn conversations_close(
        &self,
        request: &ConversationsCloseRequest,
    ) -> Result<ConversationsCloseResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.create`](https://docs.slack.dev/reference/methods/conversations.create): Initiates a public or private channel-based conversation
    ///
    /// Required scopes:
    ///
    /// - bot token: `groups:write`, `im:write`, `mpim:write`, `channels:write`, `channels:manage`
    /// - user token: `groups:write`, `im:write`, `mpim:write`, `channels:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.create")]
    pub async fn conversations_create(
        &self,
        request: &ConversationsCreateRequest,
    ) -> Result<ConversationsCreateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.declineSharedInvite`](https://docs.slack.dev/reference/methods/conversations.declineSharedInvite): Declines a Slack Connect channel invite.
    ///
    /// Required scopes:
    ///
    /// - bot token: `conversations.connect:manage`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.declineSharedInvite")]
    pub async fn conversations_decline_shared_invite(
        &self,
        request: &ConversationsDeclineSharedInviteRequest,
    ) -> Result<ConversationsDeclineSharedInviteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.externalInvitePermissions.set`](https://docs.slack.dev/reference/methods/conversations.externalInvitePermissions.set): Upgrade or downgrade Slack Connect channel permissions between 'can post only' and 'can post and invite'.
    ///
    /// Required scopes:
    ///
    /// - bot token: `conversations.connect:manage`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.externalInvitePermissions.set")]
    pub async fn conversations_external_invite_permissions_set(
        &self,
        request: &ConversationsExternalInvitePermissionsSetRequest,
    ) -> Result<ConversationsExternalInvitePermissionsSetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.history`](https://docs.slack.dev/reference/methods/conversations.history): Fetches a conversation's history of messages and events.
    ///
    /// Required scopes:
    ///
    /// - bot token: `groups:history`, `im:history`, `mpim:history`, `channels:history`
    /// - user token: `groups:history`, `im:history`, `mpim:history`, `channels:history`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.history")]
    pub async fn conversations_history(
        &self,
        request: &ConversationsHistoryRequest,
    ) -> Result<ConversationsHistoryResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.info`](https://docs.slack.dev/reference/methods/conversations.info): Retrieve information about a conversation.
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
    #[doc(alias = "conversations.info")]
    pub async fn conversations_info(
        &self,
        request: &ConversationsInfoRequest,
    ) -> Result<ConversationsInfoResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.invite`](https://docs.slack.dev/reference/methods/conversations.invite): Invites users to a channel.
    ///
    /// Required scopes:
    ///
    /// - bot token: `channels:write.invites`, `groups:write.invites`, `channels:manage`, `groups:write`, `im:write`, `mpim:write`
    /// - user token: `channels:write.invites`, `groups:write.invites`, `channels:write`, `groups:write`, `im:write`, `mpim:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.invite")]
    pub async fn conversations_invite(
        &self,
        request: &ConversationsInviteRequest,
    ) -> Result<ConversationsInviteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.inviteShared`](https://docs.slack.dev/reference/methods/conversations.inviteShared): Sends an invitation to a Slack Connect channel
    ///
    /// Required scopes:
    ///
    /// - bot token: `conversations.connect:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.inviteShared")]
    pub async fn conversations_invite_shared(
        &self,
        request: &ConversationsInviteSharedRequest,
    ) -> Result<ConversationsInviteSharedResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.join`](https://docs.slack.dev/reference/methods/conversations.join): Joins an existing conversation.
    ///
    /// Required scopes:
    ///
    /// - bot token: `channels:join`
    /// - user token: `channels:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.join")]
    pub async fn conversations_join(
        &self,
        request: &ConversationsJoinRequest,
    ) -> Result<ConversationsJoinResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.kick`](https://docs.slack.dev/reference/methods/conversations.kick): Removes a user from a conversation.
    ///
    /// Required scopes:
    ///
    /// - bot token: `groups:write`, `channels:manage`
    /// - user token: `groups:write`, `channels:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.kick")]
    pub async fn conversations_kick(
        &self,
        request: &ConversationsKickRequest,
    ) -> Result<ConversationsKickResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.leave`](https://docs.slack.dev/reference/methods/conversations.leave): Leaves a conversation.
    ///
    /// Required scopes:
    ///
    /// - bot token: `groups:write`, `im:write`, `mpim:write`, `channels:write`, `channels:manage`
    /// - user token: `groups:write`, `im:write`, `mpim:write`, `channels:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.leave")]
    pub async fn conversations_leave(
        &self,
        request: &ConversationsLeaveRequest,
    ) -> Result<ConversationsLeaveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.list`](https://docs.slack.dev/reference/methods/conversations.list): Lists all channels in a Slack team.
    ///
    /// Required scopes:
    ///
    /// - bot token: `groups:read`, `im:read`, `mpim:read`, `channels:read`
    /// - user token: `groups:read`, `im:read`, `mpim:read`, `channels:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.list")]
    pub async fn conversations_list(
        &self,
        request: &ConversationsListRequest,
    ) -> Result<ConversationsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.listConnectInvites`](https://docs.slack.dev/reference/methods/conversations.listConnectInvites): Lists shared channel invites that have been generated or received but have not been approved by all parties
    ///
    /// Required scopes:
    ///
    /// - bot token: `conversations.connect:manage`
    ///
    /// Rate limit: Tier 1 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.listConnectInvites")]
    pub async fn conversations_list_connect_invites(
        &self,
        request: &ConversationsListConnectInvitesRequest,
    ) -> Result<ConversationsListConnectInvitesResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.mark`](https://docs.slack.dev/reference/methods/conversations.mark): Sets the read cursor in a channel.
    ///
    /// Required scopes:
    ///
    /// - bot token: `groups:write`, `im:write`, `mpim:write`, `channels:write`, `channels:manage`
    /// - user token: `groups:write`, `im:write`, `mpim:write`, `channels:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.mark")]
    pub async fn conversations_mark(
        &self,
        request: &ConversationsMarkRequest,
    ) -> Result<ConversationsMarkResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.members`](https://docs.slack.dev/reference/methods/conversations.members): Retrieve members of a conversation.
    ///
    /// Required scopes:
    ///
    /// - bot token: `groups:read`, `im:read`, `mpim:read`, `channels:read`
    /// - user token: `groups:read`, `im:read`, `mpim:read`, `channels:read`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.members")]
    pub async fn conversations_members(
        &self,
        request: &ConversationsMembersRequest,
    ) -> Result<ConversationsMembersResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.open`](https://docs.slack.dev/reference/methods/conversations.open): Opens or resumes a direct message or multi-person direct message.
    ///
    /// Required scopes:
    ///
    /// - bot token: `channels:manage`, `groups:write`, `im:write`, `mpim:write`
    /// - user token: `channels:write`, `groups:write`, `im:write`, `mpim:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.open")]
    pub async fn conversations_open(
        &self,
        request: &ConversationsOpenRequest,
    ) -> Result<ConversationsOpenResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.rename`](https://docs.slack.dev/reference/methods/conversations.rename): Renames a conversation.
    ///
    /// Required scopes:
    ///
    /// - bot token: `groups:write`, `im:write`, `mpim:write`, `channels:write`, `channels:manage`
    /// - user token: `groups:write`, `im:write`, `mpim:write`, `channels:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.rename")]
    pub async fn conversations_rename(
        &self,
        request: &ConversationsRenameRequest,
    ) -> Result<ConversationsRenameResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.replies`](https://docs.slack.dev/reference/methods/conversations.replies): Retrieve a thread of messages posted to a conversation
    ///
    /// Required scopes:
    ///
    /// - bot token: `groups:history`, `im:history`, `mpim:history`, `channels:history`
    /// - user token: `groups:history`, `im:history`, `mpim:history`, `channels:history`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.replies")]
    pub async fn conversations_replies(
        &self,
        request: &ConversationsRepliesRequest,
    ) -> Result<ConversationsRepliesResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.requestSharedInvite.approve`](https://docs.slack.dev/reference/methods/conversations.requestSharedInvite.approve): Approves a request to add an external user to a channel and sends them a Slack Connect invite
    ///
    /// Required scopes:
    ///
    /// - bot token: `conversations.connect:manage`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.requestSharedInvite.approve")]
    pub async fn conversations_request_shared_invite_approve(
        &self,
        request: &ConversationsRequestSharedInviteApproveRequest,
    ) -> Result<ConversationsRequestSharedInviteApproveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.requestSharedInvite.deny`](https://docs.slack.dev/reference/methods/conversations.requestSharedInvite.deny): Denies a request to invite an external user to a channel
    ///
    /// Required scopes:
    ///
    /// - bot token: `conversations.connect:manage`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.requestSharedInvite.deny")]
    pub async fn conversations_request_shared_invite_deny(
        &self,
        request: &ConversationsRequestSharedInviteDenyRequest,
    ) -> Result<ConversationsRequestSharedInviteDenyResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.requestSharedInvite.list`](https://docs.slack.dev/reference/methods/conversations.requestSharedInvite.list): Lists requests to add external users to channels with ability to filter.
    ///
    /// Required scopes:
    ///
    /// - bot token: `conversations.connect:manage`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.requestSharedInvite.list")]
    pub async fn conversations_request_shared_invite_list(
        &self,
        request: &ConversationsRequestSharedInviteListRequest,
    ) -> Result<ConversationsRequestSharedInviteListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.setPurpose`](https://docs.slack.dev/reference/methods/conversations.setPurpose): Sets the channel description.
    ///
    /// Required scopes:
    ///
    /// - bot token: `channels:write.topic`, `groups:write.topic`, `mpim:write.topic`, `im:write.topic`, `channels:manage`, `groups:write`, `im:write`, `mpim:write`
    /// - user token: `channels:write.topic`, `groups:write.topic`, `mpim:write.topic`, `im:write.topic`, `channels:write`, `groups:write`, `im:write`, `mpim:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.setPurpose")]
    pub async fn conversations_set_purpose(
        &self,
        request: &ConversationsSetPurposeRequest,
    ) -> Result<ConversationsSetPurposeResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.setTopic`](https://docs.slack.dev/reference/methods/conversations.setTopic): Sets the topic for a conversation.
    ///
    /// Required scopes:
    ///
    /// - bot token: `channels:write.topic`, `groups:write.topic`, `mpim:write.topic`, `im:write.topic`, `channels:manage`, `groups:write`, `im:write`, `mpim:write`
    /// - user token: `channels:write.topic`, `groups:write.topic`, `mpim:write.topic`, `im:write.topic`, `channels:write`, `groups:write`, `im:write`, `mpim:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.setTopic")]
    pub async fn conversations_set_topic(
        &self,
        request: &ConversationsSetTopicRequest,
    ) -> Result<ConversationsSetTopicResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`conversations.unarchive`](https://docs.slack.dev/reference/methods/conversations.unarchive): Reverses conversation archival.
    ///
    /// Required scopes:
    ///
    /// - bot token: `groups:write`, `im:write`, `mpim:write`, `channels:write`, `channels:manage`
    /// - user token: `groups:write`, `im:write`, `mpim:write`, `channels:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "conversations.unarchive")]
    pub async fn conversations_unarchive(
        &self,
        request: &ConversationsUnarchiveRequest,
    ) -> Result<ConversationsUnarchiveResponse, SlackError> {
        self.call(request).await
    }
}
