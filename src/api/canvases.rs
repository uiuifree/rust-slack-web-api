// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`canvases.access.delete`](https://docs.slack.dev/reference/methods/canvases.access.delete): Remove access to a canvas for specified entities
///
/// Send it with [`SlackClient::canvases_access_delete`].
#[doc(alias = "canvases.access.delete")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CanvasesAccessDeleteRequest {
    /// Encoded ID of the canvas
    pub canvas_id: String,
    /// List of channels you wish to update access for
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub channel_ids: Option<Vec<String>>,
    /// List of users you wish to update access for
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub user_ids: Option<Vec<String>>,
}

impl CanvasesAccessDeleteRequest {
    pub fn new(canvas_id: impl Into<String>) -> Self {
        Self {
            canvas_id: canvas_id.into(),
            channel_ids: None,
            user_ids: None,
        }
    }

    pub fn channel_ids(mut self, channel_ids: Vec<String>) -> Self {
        self.channel_ids = Some(channel_ids);
        self
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }
}

impl SlackApiMethod for CanvasesAccessDeleteRequest {
    const METHOD: &'static str = "canvases.access.delete";
    type Response = CanvasesAccessDeleteResponse;
}

/// Successful response of the Slack Web API method [`canvases.access.delete`](https://docs.slack.dev/reference/methods/canvases.access.delete).
#[doc(alias = "canvases.access.delete")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CanvasesAccessDeleteResponse {
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

/// Arguments for the Slack Web API method [`canvases.access.set`](https://docs.slack.dev/reference/methods/canvases.access.set): Sets the access level to a canvas for specified entities
///
/// Send it with [`SlackClient::canvases_access_set`].
#[doc(alias = "canvases.access.set")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CanvasesAccessSetRequest {
    /// Encoded ID of the canvas
    pub canvas_id: String,
    /// Desired level of access
    pub access_level: String,
    /// List of channels you wish to update access for. Can only be used if user\_ids is not provided.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub channel_ids: Option<Vec<String>>,
    /// List of users you wish to update access for. Can only be used if channel\_ids is not provided.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub user_ids: Option<Vec<String>>,
}

impl CanvasesAccessSetRequest {
    pub fn new(canvas_id: impl Into<String>, access_level: impl Into<String>) -> Self {
        Self {
            canvas_id: canvas_id.into(),
            access_level: access_level.into(),
            channel_ids: None,
            user_ids: None,
        }
    }

    pub fn channel_ids(mut self, channel_ids: Vec<String>) -> Self {
        self.channel_ids = Some(channel_ids);
        self
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }
}

impl SlackApiMethod for CanvasesAccessSetRequest {
    const METHOD: &'static str = "canvases.access.set";
    type Response = CanvasesAccessSetResponse;
}

/// Successful response of the Slack Web API method [`canvases.access.set`](https://docs.slack.dev/reference/methods/canvases.access.set).
#[doc(alias = "canvases.access.set")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CanvasesAccessSetResponse {
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

/// Arguments for the Slack Web API method [`canvases.create`](https://docs.slack.dev/reference/methods/canvases.create): Create canvas for a user
///
/// Send it with [`SlackClient::canvases_create`].
#[doc(alias = "canvases.create")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct CanvasesCreateRequest {
    /// Title of the newly created canvas
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Structure describing the type and value of the content to create. The markdown content is limited to 1 MiB (1,048,576 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_content: Option<String>,
    /// Channel ID of the channel the canvas will be tabbed in. This is a required field for free teams.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
}

impl CanvasesCreateRequest {
    pub fn new() -> Self {
        Self {
            title: None,
            document_content: None,
            channel_id: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn document_content(mut self, document_content: impl Into<String>) -> Self {
        self.document_content = Some(document_content.into());
        self
    }

    pub fn channel_id(mut self, channel_id: impl Into<String>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }
}

impl SlackApiMethod for CanvasesCreateRequest {
    const METHOD: &'static str = "canvases.create";
    type Response = CanvasesCreateResponse;
}

/// Successful response of the Slack Web API method [`canvases.create`](https://docs.slack.dev/reference/methods/canvases.create).
#[doc(alias = "canvases.create")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CanvasesCreateResponse {
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

/// Arguments for the Slack Web API method [`canvases.delete`](https://docs.slack.dev/reference/methods/canvases.delete): Deletes a canvas
///
/// Send it with [`SlackClient::canvases_delete`].
#[doc(alias = "canvases.delete")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CanvasesDeleteRequest {
    /// Encoded ID of the canvas
    pub canvas_id: String,
}

impl CanvasesDeleteRequest {
    pub fn new(canvas_id: impl Into<String>) -> Self {
        Self {
            canvas_id: canvas_id.into(),
        }
    }
}

impl SlackApiMethod for CanvasesDeleteRequest {
    const METHOD: &'static str = "canvases.delete";
    type Response = CanvasesDeleteResponse;
}

/// Successful response of the Slack Web API method [`canvases.delete`](https://docs.slack.dev/reference/methods/canvases.delete).
#[doc(alias = "canvases.delete")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CanvasesDeleteResponse {
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

/// Arguments for the Slack Web API method [`canvases.edit`](https://docs.slack.dev/reference/methods/canvases.edit): Update an existing canvas
///
/// Send it with [`SlackClient::canvases_edit`].
#[doc(alias = "canvases.edit")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CanvasesEditRequest {
    /// Encoded ID of the canvas
    pub canvas_id: String,
    /// List of changes to apply on the specified canvas. The markdown content of each change is limited to 1 MiB (1,048,576 characters).
    #[serde(serialize_with = "crate::form::as_json")]
    pub changes: Vec<serde_json::Value>,
}

impl CanvasesEditRequest {
    pub fn new(canvas_id: impl Into<String>, changes: Vec<serde_json::Value>) -> Self {
        Self {
            canvas_id: canvas_id.into(),
            changes,
        }
    }
}

impl SlackApiMethod for CanvasesEditRequest {
    const METHOD: &'static str = "canvases.edit";
    type Response = CanvasesEditResponse;
}

/// Successful response of the Slack Web API method [`canvases.edit`](https://docs.slack.dev/reference/methods/canvases.edit).
#[doc(alias = "canvases.edit")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CanvasesEditResponse {
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

/// Arguments for the Slack Web API method [`canvases.getContent`](https://docs.slack.dev/reference/methods/canvases.getContent): Get the content of a canvas as markdown (default) or HTML.
///
/// Send it with [`SlackClient::canvases_get_content`].
#[doc(alias = "canvases.getContent")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CanvasesGetContentRequest {
    /// Encoded ID of the canvas
    pub canvas_id: String,
    /// Format in which to return the canvas content. Defaults to markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl CanvasesGetContentRequest {
    pub fn new(canvas_id: impl Into<String>) -> Self {
        Self {
            canvas_id: canvas_id.into(),
            content_type: None,
        }
    }

    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }
}

impl SlackApiMethod for CanvasesGetContentRequest {
    const METHOD: &'static str = "canvases.getContent";
    type Response = CanvasesGetContentResponse;
}

/// Successful response of the Slack Web API method [`canvases.getContent`](https://docs.slack.dev/reference/methods/canvases.getContent).
#[doc(alias = "canvases.getContent")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CanvasesGetContentResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub content: Option<String>,
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

/// Arguments for the Slack Web API method [`canvases.sections.lookup`](https://docs.slack.dev/reference/methods/canvases.sections.lookup): Find sections matching the provided criteria
///
/// Send it with [`SlackClient::canvases_sections_lookup`].
#[doc(alias = "canvases.sections.lookup")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CanvasesSectionsLookupRequest {
    /// Encoded ID of the canvas
    pub canvas_id: String,
    /// Filtering criteria
    pub criteria: String,
}

impl CanvasesSectionsLookupRequest {
    pub fn new(canvas_id: impl Into<String>, criteria: impl Into<String>) -> Self {
        Self {
            canvas_id: canvas_id.into(),
            criteria: criteria.into(),
        }
    }
}

impl SlackApiMethod for CanvasesSectionsLookupRequest {
    const METHOD: &'static str = "canvases.sections.lookup";
    type Response = CanvasesSectionsLookupResponse;
}

/// Successful response of the Slack Web API method [`canvases.sections.lookup`](https://docs.slack.dev/reference/methods/canvases.sections.lookup).
#[doc(alias = "canvases.sections.lookup")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CanvasesSectionsLookupResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sections: Vec<CanvasesSectionsLookupResponseSections>,
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
pub struct CanvasesSectionsLookupResponseSections {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
}

impl SlackClient {
    /// Calls the Slack Web API method [`canvases.access.delete`](https://docs.slack.dev/reference/methods/canvases.access.delete): Remove access to a canvas for specified entities
    ///
    /// Required scopes:
    ///
    /// - bot token: `canvases:write`
    /// - user token: `canvases:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "canvases.access.delete")]
    pub async fn canvases_access_delete(
        &self,
        request: &CanvasesAccessDeleteRequest,
    ) -> Result<CanvasesAccessDeleteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`canvases.access.set`](https://docs.slack.dev/reference/methods/canvases.access.set): Sets the access level to a canvas for specified entities
    ///
    /// Required scopes:
    ///
    /// - bot token: `canvases:write`
    /// - user token: `canvases:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "canvases.access.set")]
    pub async fn canvases_access_set(
        &self,
        request: &CanvasesAccessSetRequest,
    ) -> Result<CanvasesAccessSetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`canvases.create`](https://docs.slack.dev/reference/methods/canvases.create): Create canvas for a user
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
    #[doc(alias = "canvases.create")]
    pub async fn canvases_create(
        &self,
        request: &CanvasesCreateRequest,
    ) -> Result<CanvasesCreateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`canvases.delete`](https://docs.slack.dev/reference/methods/canvases.delete): Deletes a canvas
    ///
    /// Required scopes:
    ///
    /// - bot token: `canvases:write`
    /// - user token: `canvases:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "canvases.delete")]
    pub async fn canvases_delete(
        &self,
        request: &CanvasesDeleteRequest,
    ) -> Result<CanvasesDeleteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`canvases.edit`](https://docs.slack.dev/reference/methods/canvases.edit): Update an existing canvas
    ///
    /// Required scopes:
    ///
    /// - bot token: `canvases:write`
    /// - user token: `canvases:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "canvases.edit")]
    pub async fn canvases_edit(
        &self,
        request: &CanvasesEditRequest,
    ) -> Result<CanvasesEditResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`canvases.getContent`](https://docs.slack.dev/reference/methods/canvases.getContent): Get the content of a canvas as markdown (default) or HTML.
    ///
    /// Required scopes:
    ///
    /// - bot token: `canvases:read`
    /// - user token: `canvases:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "canvases.getContent")]
    pub async fn canvases_get_content(
        &self,
        request: &CanvasesGetContentRequest,
    ) -> Result<CanvasesGetContentResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`canvases.sections.lookup`](https://docs.slack.dev/reference/methods/canvases.sections.lookup): Find sections matching the provided criteria
    ///
    /// Required scopes:
    ///
    /// - bot token: `canvases:read`
    /// - user token: `canvases:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "canvases.sections.lookup")]
    pub async fn canvases_sections_lookup(
        &self,
        request: &CanvasesSectionsLookupRequest,
    ) -> Result<CanvasesSectionsLookupResponse, SlackError> {
        self.call(request).await
    }
}
