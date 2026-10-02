// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`reminders.add`](https://docs.slack.dev/reference/methods/reminders.add): Creates a reminder.
///
/// Send it with [`SlackClient::reminders_add`].
#[doc(alias = "reminders.add")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct RemindersAddRequest {
    /// The content of the reminder
    pub text: String,
    /// Can also take a type of integer. When this reminder should happen: the Unix timestamp (up to five years from now), the number of seconds until the reminder (if within 24 hours), or a natural language description (Ex. "in 15 minutes," or "every Thursday")
    pub time: String,
    /// No longer supported - reminders cannot be set for other users. Previously, was the user who would receive the reminder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Encoded team id, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Specify the repeating behavior of a reminder. Available options: `daily`, `weekly`, `monthly`, or `yearly`. If `weekly`, may further specify the days of the week.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<serde_json::Value>,
}

impl RemindersAddRequest {
    pub fn new(text: impl Into<String>, time: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            time: time.into(),
            user: None,
            team_id: None,
            recurrence: None,
        }
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn recurrence(mut self, recurrence: serde_json::Value) -> Self {
        self.recurrence = Some(recurrence);
        self
    }
}

impl SlackApiMethod for RemindersAddRequest {
    const METHOD: &'static str = "reminders.add";
    type Response = RemindersAddResponse;
}

/// Successful response of the Slack Web API method [`reminders.add`](https://docs.slack.dev/reference/methods/reminders.add).
#[doc(alias = "reminders.add")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RemindersAddResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub reminder: Option<Reminder>,
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

/// Arguments for the Slack Web API method [`reminders.complete`](https://docs.slack.dev/reference/methods/reminders.complete): Marks a reminder as complete.
///
/// Send it with [`SlackClient::reminders_complete`].
#[doc(alias = "reminders.complete")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct RemindersCompleteRequest {
    /// The ID of the reminder to be marked as complete
    pub reminder: String,
    /// Encoded team id, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl RemindersCompleteRequest {
    pub fn new(reminder: impl Into<String>) -> Self {
        Self {
            reminder: reminder.into(),
            team_id: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for RemindersCompleteRequest {
    const METHOD: &'static str = "reminders.complete";
    type Response = RemindersCompleteResponse;
}

/// Successful response of the Slack Web API method [`reminders.complete`](https://docs.slack.dev/reference/methods/reminders.complete).
#[doc(alias = "reminders.complete")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RemindersCompleteResponse {
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

/// Arguments for the Slack Web API method [`reminders.delete`](https://docs.slack.dev/reference/methods/reminders.delete): Deletes a reminder.
///
/// Send it with [`SlackClient::reminders_delete`].
#[doc(alias = "reminders.delete")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct RemindersDeleteRequest {
    /// The ID of the reminder
    pub reminder: String,
    /// Encoded team id, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl RemindersDeleteRequest {
    pub fn new(reminder: impl Into<String>) -> Self {
        Self {
            reminder: reminder.into(),
            team_id: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for RemindersDeleteRequest {
    const METHOD: &'static str = "reminders.delete";
    type Response = RemindersDeleteResponse;
}

/// Successful response of the Slack Web API method [`reminders.delete`](https://docs.slack.dev/reference/methods/reminders.delete).
#[doc(alias = "reminders.delete")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RemindersDeleteResponse {
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

/// Arguments for the Slack Web API method [`reminders.info`](https://docs.slack.dev/reference/methods/reminders.info): Gets information about a reminder.
///
/// Send it with [`SlackClient::reminders_info`].
#[doc(alias = "reminders.info")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct RemindersInfoRequest {
    /// The ID of the reminder
    pub reminder: String,
    /// Encoded team id, required if org token is passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl RemindersInfoRequest {
    pub fn new(reminder: impl Into<String>) -> Self {
        Self {
            reminder: reminder.into(),
            team_id: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for RemindersInfoRequest {
    const METHOD: &'static str = "reminders.info";
    type Response = RemindersInfoResponse;
}

/// Successful response of the Slack Web API method [`reminders.info`](https://docs.slack.dev/reference/methods/reminders.info).
#[doc(alias = "reminders.info")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RemindersInfoResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub reminder: Option<Reminder>,
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

/// Arguments for the Slack Web API method [`reminders.list`](https://docs.slack.dev/reference/methods/reminders.list): Lists all reminders created by or for a given user.
///
/// Send it with [`SlackClient::reminders_list`].
#[doc(alias = "reminders.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct RemindersListRequest {
    /// Encoded team id, required if org token is passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl RemindersListRequest {
    pub fn new() -> Self {
        Self { team_id: None }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for RemindersListRequest {
    const METHOD: &'static str = "reminders.list";
    type Response = RemindersListResponse;
}

/// Successful response of the Slack Web API method [`reminders.list`](https://docs.slack.dev/reference/methods/reminders.list).
#[doc(alias = "reminders.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RemindersListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reminders: Vec<Reminder>,
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
    /// Calls the Slack Web API method [`reminders.add`](https://docs.slack.dev/reference/methods/reminders.add): Creates a reminder.
    ///
    /// Required scopes:
    ///
    /// - user token: `reminders:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "reminders.add")]
    pub async fn reminders_add(
        &self,
        request: &RemindersAddRequest,
    ) -> Result<RemindersAddResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`reminders.complete`](https://docs.slack.dev/reference/methods/reminders.complete): Marks a reminder as complete.
    ///
    /// Required scopes:
    ///
    /// - user token: `reminders:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "reminders.complete")]
    pub async fn reminders_complete(
        &self,
        request: &RemindersCompleteRequest,
    ) -> Result<RemindersCompleteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`reminders.delete`](https://docs.slack.dev/reference/methods/reminders.delete): Deletes a reminder.
    ///
    /// Required scopes:
    ///
    /// - user token: `reminders:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "reminders.delete")]
    pub async fn reminders_delete(
        &self,
        request: &RemindersDeleteRequest,
    ) -> Result<RemindersDeleteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`reminders.info`](https://docs.slack.dev/reference/methods/reminders.info): Gets information about a reminder.
    ///
    /// Required scopes:
    ///
    /// - user token: `reminders:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "reminders.info")]
    pub async fn reminders_info(
        &self,
        request: &RemindersInfoRequest,
    ) -> Result<RemindersInfoResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`reminders.list`](https://docs.slack.dev/reference/methods/reminders.list): Lists all reminders created by or for a given user.
    ///
    /// Required scopes:
    ///
    /// - user token: `reminders:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "reminders.list")]
    pub async fn reminders_list(
        &self,
        request: &RemindersListRequest,
    ) -> Result<RemindersListResponse, SlackError> {
        self.call(request).await
    }
}
