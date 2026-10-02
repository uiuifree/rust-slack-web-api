// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`calls.add`](https://docs.slack.dev/reference/methods/calls.add): Registers a new Call.
///
/// Send it with [`SlackClient::calls_add`].
#[doc(alias = "calls.add")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CallsAddRequest {
    /// An ID supplied by the 3rd-party Call provider. It must be unique across all Calls from that service.
    pub external_unique_id: String,
    /// The URL required for a client to join the Call.
    pub join_url: String,
    /// An optional, human-readable ID supplied by the 3rd-party Call provider. If supplied, this ID will be displayed in the Call object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_display_id: Option<String>,
    /// When supplied, available Slack clients will attempt to directly launch the 3rd-party Call with this URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop_app_join_url: Option<String>,
    /// Unix timestamp of the call start time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_start: Option<i64>,
    /// The name of the Call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The valid Slack user ID of the user who created this Call. When this method is called with a user token, the `created_by` field is optional and defaults to the authed user of the token. Otherwise, the field is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// The list of users to register as participants in the Call. [Read more on how to specify users here](https://docs.slack.dev/apis/web-api/using-the-calls-api.md#users).
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub users: Option<Vec<serde_json::Value>>,
}

impl CallsAddRequest {
    pub fn new(external_unique_id: impl Into<String>, join_url: impl Into<String>) -> Self {
        Self {
            external_unique_id: external_unique_id.into(),
            join_url: join_url.into(),
            external_display_id: None,
            desktop_app_join_url: None,
            date_start: None,
            title: None,
            created_by: None,
            users: None,
        }
    }

    pub fn external_display_id(mut self, external_display_id: impl Into<String>) -> Self {
        self.external_display_id = Some(external_display_id.into());
        self
    }

    pub fn desktop_app_join_url(mut self, desktop_app_join_url: impl Into<String>) -> Self {
        self.desktop_app_join_url = Some(desktop_app_join_url.into());
        self
    }

    pub fn date_start(mut self, date_start: i64) -> Self {
        self.date_start = Some(date_start);
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn created_by(mut self, created_by: impl Into<String>) -> Self {
        self.created_by = Some(created_by.into());
        self
    }

    pub fn users(mut self, users: Vec<serde_json::Value>) -> Self {
        self.users = Some(users);
        self
    }
}

impl SlackApiMethod for CallsAddRequest {
    const METHOD: &'static str = "calls.add";
    type Response = CallsAddResponse;
}

/// Successful response of the Slack Web API method [`calls.add`](https://docs.slack.dev/reference/methods/calls.add).
#[doc(alias = "calls.add")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CallsAddResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub call: Option<CallsAddResponseCall>,
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
pub struct CallsAddResponseCall {
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
    pub date_start: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_unique_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub join_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub desktop_app_join_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_display_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub users: Vec<CallsAddResponseCallUsers>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CallsAddResponseCallUsers {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub avatar_url: Option<String>,
}

/// Arguments for the Slack Web API method [`calls.end`](https://docs.slack.dev/reference/methods/calls.end): Ends a Call.
///
/// Send it with [`SlackClient::calls_end`].
#[doc(alias = "calls.end")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CallsEndRequest {
    /// `id` returned when registering the call using the [`calls.add`](https://docs.slack.dev/reference/methods/calls.add.md) method.
    pub id: String,
    /// Call duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}

impl CallsEndRequest {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            duration: None,
        }
    }

    pub fn duration(mut self, duration: i64) -> Self {
        self.duration = Some(duration);
        self
    }
}

impl SlackApiMethod for CallsEndRequest {
    const METHOD: &'static str = "calls.end";
    type Response = CallsEndResponse;
}

/// Successful response of the Slack Web API method [`calls.end`](https://docs.slack.dev/reference/methods/calls.end).
#[doc(alias = "calls.end")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CallsEndResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub error: Option<String>,
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

/// Arguments for the Slack Web API method [`calls.info`](https://docs.slack.dev/reference/methods/calls.info): Returns information about a Call.
///
/// Send it with [`SlackClient::calls_info`].
#[doc(alias = "calls.info")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CallsInfoRequest {
    /// `id` of the Call returned by the [`calls.add`](https://docs.slack.dev/reference/methods/calls.add.md) method.
    pub id: String,
}

impl CallsInfoRequest {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl SlackApiMethod for CallsInfoRequest {
    const METHOD: &'static str = "calls.info";
    type Response = CallsInfoResponse;
}

/// Successful response of the Slack Web API method [`calls.info`](https://docs.slack.dev/reference/methods/calls.info).
#[doc(alias = "calls.info")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CallsInfoResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub call: Option<CallsInfoResponseCall>,
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
pub struct CallsInfoResponseCall {
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
    pub date_start: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_unique_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub join_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub desktop_app_join_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_display_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub users: Vec<CallsInfoResponseCallUsers>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CallsInfoResponseCallUsers {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub avatar_url: Option<String>,
}

/// Arguments for the Slack Web API method [`calls.participants.add`](https://docs.slack.dev/reference/methods/calls.participants.add): Registers new participants added to a Call.
///
/// Send it with [`SlackClient::calls_participants_add`].
#[doc(alias = "calls.participants.add")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CallsParticipantsAddRequest {
    /// `id` returned by the [`calls.add`](https://docs.slack.dev/reference/methods/calls.add.md) method.
    pub id: String,
    /// The list of users to add as participants in the Call. [Read more on how to specify users here](https://docs.slack.dev/apis/web-api/using-the-calls-api.md#users).
    #[serde(serialize_with = "crate::form::as_json")]
    pub users: Vec<serde_json::Value>,
}

impl CallsParticipantsAddRequest {
    pub fn new(id: impl Into<String>, users: Vec<serde_json::Value>) -> Self {
        Self {
            id: id.into(),
            users,
        }
    }
}

impl SlackApiMethod for CallsParticipantsAddRequest {
    const METHOD: &'static str = "calls.participants.add";
    type Response = CallsParticipantsAddResponse;
}

/// Successful response of the Slack Web API method [`calls.participants.add`](https://docs.slack.dev/reference/methods/calls.participants.add).
#[doc(alias = "calls.participants.add")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CallsParticipantsAddResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub error: Option<String>,
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

/// Arguments for the Slack Web API method [`calls.participants.remove`](https://docs.slack.dev/reference/methods/calls.participants.remove): Registers participants removed from a Call.
///
/// Send it with [`SlackClient::calls_participants_remove`].
#[doc(alias = "calls.participants.remove")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CallsParticipantsRemoveRequest {
    /// `id` returned by the [`calls.add`](https://docs.slack.dev/reference/methods/calls.add.md) method.
    pub id: String,
    /// The list of users to remove as participants in the Call. [Read more on how to specify users here](https://docs.slack.dev/apis/web-api/using-the-calls-api.md#users).
    #[serde(serialize_with = "crate::form::as_json")]
    pub users: Vec<serde_json::Value>,
}

impl CallsParticipantsRemoveRequest {
    pub fn new(id: impl Into<String>, users: Vec<serde_json::Value>) -> Self {
        Self {
            id: id.into(),
            users,
        }
    }
}

impl SlackApiMethod for CallsParticipantsRemoveRequest {
    const METHOD: &'static str = "calls.participants.remove";
    type Response = CallsParticipantsRemoveResponse;
}

/// Successful response of the Slack Web API method [`calls.participants.remove`](https://docs.slack.dev/reference/methods/calls.participants.remove).
#[doc(alias = "calls.participants.remove")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CallsParticipantsRemoveResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub error: Option<String>,
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

/// Arguments for the Slack Web API method [`calls.update`](https://docs.slack.dev/reference/methods/calls.update): Updates information about a Call.
///
/// Send it with [`SlackClient::calls_update`].
#[doc(alias = "calls.update")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CallsUpdateRequest {
    /// `id` returned by the [`calls.add`](https://docs.slack.dev/reference/methods/calls.add.md) method.
    pub id: String,
    /// The name of the Call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The URL required for a client to join the Call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_url: Option<String>,
    /// When supplied, available Slack clients will attempt to directly launch the 3rd-party Call with this URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop_app_join_url: Option<String>,
}

impl CallsUpdateRequest {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: None,
            join_url: None,
            desktop_app_join_url: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn join_url(mut self, join_url: impl Into<String>) -> Self {
        self.join_url = Some(join_url.into());
        self
    }

    pub fn desktop_app_join_url(mut self, desktop_app_join_url: impl Into<String>) -> Self {
        self.desktop_app_join_url = Some(desktop_app_join_url.into());
        self
    }
}

impl SlackApiMethod for CallsUpdateRequest {
    const METHOD: &'static str = "calls.update";
    type Response = CallsUpdateResponse;
}

/// Successful response of the Slack Web API method [`calls.update`](https://docs.slack.dev/reference/methods/calls.update).
#[doc(alias = "calls.update")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CallsUpdateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub call: Option<CallsUpdateResponseCall>,
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
pub struct CallsUpdateResponseCall {
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
    pub date_start: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_unique_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub join_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub desktop_app_join_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_display_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub users: Vec<CallsUpdateResponseCallUsers>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CallsUpdateResponseCallUsers {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub avatar_url: Option<String>,
}

impl SlackClient {
    /// Calls the Slack Web API method [`calls.add`](https://docs.slack.dev/reference/methods/calls.add): Registers a new Call.
    ///
    /// Required scopes:
    ///
    /// - bot token: `calls:write`
    /// - user token: `calls:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "calls.add")]
    pub async fn calls_add(
        &self,
        request: &CallsAddRequest,
    ) -> Result<CallsAddResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`calls.end`](https://docs.slack.dev/reference/methods/calls.end): Ends a Call.
    ///
    /// Required scopes:
    ///
    /// - bot token: `calls:write`
    /// - user token: `calls:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "calls.end")]
    pub async fn calls_end(
        &self,
        request: &CallsEndRequest,
    ) -> Result<CallsEndResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`calls.info`](https://docs.slack.dev/reference/methods/calls.info): Returns information about a Call.
    ///
    /// Required scopes:
    ///
    /// - bot token: `calls:read`
    /// - user token: `calls:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "calls.info")]
    pub async fn calls_info(
        &self,
        request: &CallsInfoRequest,
    ) -> Result<CallsInfoResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`calls.participants.add`](https://docs.slack.dev/reference/methods/calls.participants.add): Registers new participants added to a Call.
    ///
    /// Required scopes:
    ///
    /// - bot token: `calls:write`
    /// - user token: `calls:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "calls.participants.add")]
    pub async fn calls_participants_add(
        &self,
        request: &CallsParticipantsAddRequest,
    ) -> Result<CallsParticipantsAddResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`calls.participants.remove`](https://docs.slack.dev/reference/methods/calls.participants.remove): Registers participants removed from a Call.
    ///
    /// Required scopes:
    ///
    /// - bot token: `calls:write`
    /// - user token: `calls:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "calls.participants.remove")]
    pub async fn calls_participants_remove(
        &self,
        request: &CallsParticipantsRemoveRequest,
    ) -> Result<CallsParticipantsRemoveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`calls.update`](https://docs.slack.dev/reference/methods/calls.update): Updates information about a Call.
    ///
    /// Required scopes:
    ///
    /// - bot token: `calls:write`
    /// - user token: `calls:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "calls.update")]
    pub async fn calls_update(
        &self,
        request: &CallsUpdateRequest,
    ) -> Result<CallsUpdateResponse, SlackError> {
        self.call(request).await
    }
}
