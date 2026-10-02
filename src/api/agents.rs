// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`agents.sessions.rename`](https://docs.slack.dev/reference/methods/agents.sessions.rename): Rename an agent session.
///
/// Send it with [`SlackClient::agents_sessions_rename`].
#[doc(alias = "agents.sessions.rename")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AgentsSessionsRenameRequest {
    /// New title for the agent session (1-200 characters). For a session channel, this also renames the channel.
    pub title: String,
    /// ID of the channel containing the agent session. Required for public channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// Timestamp of the thread root message the session is scoped to. Required for thread-based sessions in regular channels and DMs. Must be omitted for session channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
}

impl AgentsSessionsRenameRequest {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            channel_id: None,
            thread_ts: None,
        }
    }

    pub fn channel_id(mut self, channel_id: impl Into<String>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    pub fn thread_ts(mut self, thread_ts: impl Into<String>) -> Self {
        self.thread_ts = Some(thread_ts.into());
        self
    }
}

impl SlackApiMethod for AgentsSessionsRenameRequest {
    const METHOD: &'static str = "agents.sessions.rename";
    type Response = AgentsSessionsRenameResponse;
}

/// Successful response of the Slack Web API method [`agents.sessions.rename`](https://docs.slack.dev/reference/methods/agents.sessions.rename).
#[doc(alias = "agents.sessions.rename")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AgentsSessionsRenameResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
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

/// Arguments for the Slack Web API method [`agents.sessions.setStatus`](https://docs.slack.dev/reference/methods/agents.sessions.setStatus): Set an agent session's lifecycle status, creating the session if needed.
///
/// Send it with [`SlackClient::agents_sessions_set_status`].
#[doc(alias = "agents.sessions.setStatus")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AgentsSessionsSetStatusRequest {
    /// The lifecycle status to set.
    pub status: String,
    /// ID of the channel containing the agent session. Required for public channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// Timestamp of the thread root message the session is scoped to. Required for thread-based sessions in regular channels and DMs. Must be omitted for session channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
    /// Title for the agent session (max 200 characters). Only used when creating a new session; ignored if the session already exists. To rename an existing session, use `agents.sessions.rename`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The user who initiated the session. Only used when creating a new session; ignored if the session already exists. Must be a member of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_user_id: Option<String>,
    /// Emoji to use as the agent's icon. Takes priority over `icon_url`. Remains in effect until you clear it (pass `null`) or set a new value. Requires the `chat:write.customize` scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_emoji: Option<String>,
    /// URL to an image to use as the agent's icon. Remains in effect until you clear it (pass `null`) or set a new value. Requires the `chat:write.customize` scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Display name override for the agent (max 200 characters). Remains in effect until you clear it (pass `null`) or set a new value. Requires the `chat:write.customize` scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl AgentsSessionsSetStatusRequest {
    pub fn new(status: impl Into<String>) -> Self {
        Self {
            status: status.into(),
            channel_id: None,
            thread_ts: None,
            title: None,
            initiator_user_id: None,
            icon_emoji: None,
            icon_url: None,
            username: None,
        }
    }

    pub fn channel_id(mut self, channel_id: impl Into<String>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    pub fn thread_ts(mut self, thread_ts: impl Into<String>) -> Self {
        self.thread_ts = Some(thread_ts.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn initiator_user_id(mut self, initiator_user_id: impl Into<String>) -> Self {
        self.initiator_user_id = Some(initiator_user_id.into());
        self
    }

    pub fn icon_emoji(mut self, icon_emoji: impl Into<String>) -> Self {
        self.icon_emoji = Some(icon_emoji.into());
        self
    }

    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }
}

impl SlackApiMethod for AgentsSessionsSetStatusRequest {
    const METHOD: &'static str = "agents.sessions.setStatus";
    type Response = AgentsSessionsSetStatusResponse;
}

/// Successful response of the Slack Web API method [`agents.sessions.setStatus`](https://docs.slack.dev/reference/methods/agents.sessions.setStatus).
#[doc(alias = "agents.sessions.setStatus")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AgentsSessionsSetStatusResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub status: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_status: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
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
    /// Calls the Slack Web API method [`agents.sessions.rename`](https://docs.slack.dev/reference/methods/agents.sessions.rename): Rename an agent session.
    ///
    /// Required scopes:
    ///
    /// - bot token: `chat:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "agents.sessions.rename")]
    pub async fn agents_sessions_rename(
        &self,
        request: &AgentsSessionsRenameRequest,
    ) -> Result<AgentsSessionsRenameResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`agents.sessions.setStatus`](https://docs.slack.dev/reference/methods/agents.sessions.setStatus): Set an agent session's lifecycle status, creating the session if needed.
    ///
    /// Required scopes:
    ///
    /// - bot token: `chat:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "agents.sessions.setStatus")]
    pub async fn agents_sessions_set_status(
        &self,
        request: &AgentsSessionsSetStatusRequest,
    ) -> Result<AgentsSessionsSetStatusResponse, SlackError> {
        self.call(request).await
    }
}
