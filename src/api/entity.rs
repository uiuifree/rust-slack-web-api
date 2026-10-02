// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`entity.acknowledgeCommentAction`](https://docs.slack.dev/reference/methods/entity.acknowledgeCommentAction): Acknowledge a comment post, edit, or delete on a Work Object. Apps call this method to confirm they have processed a comment action.
///
/// Send it with [`SlackClient::entity_acknowledge_comment_action`].
#[doc(alias = "entity.acknowledgeCommentAction")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct EntityAcknowledgeCommentActionRequest {
    /// A reference to the original user action that initiated the comment mutation
    pub trigger_id: String,
    /// The full comment data. Required for edit and post actions. See the [comment schema](https://docs.slack.dev/messaging/work-objects-comments.md#entity-present-comments-method) for the full list of properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<serde_json::Value>,
    /// Error message if the action failed in the app. When present, signals that the mutation could not be completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl EntityAcknowledgeCommentActionRequest {
    pub fn new(trigger_id: impl Into<String>) -> Self {
        Self {
            trigger_id: trigger_id.into(),
            comment: None,
            error: None,
        }
    }

    pub fn comment(mut self, comment: serde_json::Value) -> Self {
        self.comment = Some(comment);
        self
    }

    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }
}

impl SlackApiMethod for EntityAcknowledgeCommentActionRequest {
    const METHOD: &'static str = "entity.acknowledgeCommentAction";
    type Response = EntityAcknowledgeCommentActionResponse;
}

/// Successful response of the Slack Web API method [`entity.acknowledgeCommentAction`](https://docs.slack.dev/reference/methods/entity.acknowledgeCommentAction).
#[doc(alias = "entity.acknowledgeCommentAction")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EntityAcknowledgeCommentActionResponse {
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

/// Arguments for the Slack Web API method [`entity.presentComments`](https://docs.slack.dev/reference/methods/entity.presentComments): Provide comments for Work Objects. Apps call this method to send per-user flexpane comment data to the client.
///
/// Send it with [`SlackClient::entity_present_comments`].
#[doc(alias = "entity.presentComments")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct EntityPresentCommentsRequest {
    /// Array of comments to present to the user. See the [comment schema](https://docs.slack.dev/messaging/work-objects-comments.md#entity-present-comments-method) for the full list of properties.
    #[serde(serialize_with = "crate::form::as_json")]
    pub comments: Vec<serde_json::Value>,
    /// A reference to the original user action that initiated the request
    pub trigger_id: String,
    /// App supplied cursor used for pagination, will be sent in the next request for comments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Indicates whether the user has permissions to post comments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_comment: Option<bool>,
    /// The block action id that will be sent when a delete request is initiated for a comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_action_id: Option<String>,
    /// Set to true (or 1) to indicate that the user must authenticate to see the comments data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_auth_required: Option<bool>,
    /// A custom URL to which users are directed for authentication if required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_auth_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl EntityPresentCommentsRequest {
    pub fn new(comments: Vec<serde_json::Value>, trigger_id: impl Into<String>) -> Self {
        Self {
            comments,
            trigger_id: trigger_id.into(),
            cursor: None,
            can_post_comment: None,
            delete_action_id: None,
            user_auth_required: None,
            user_auth_url: None,
            error: None,
        }
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn can_post_comment(mut self, can_post_comment: bool) -> Self {
        self.can_post_comment = Some(can_post_comment);
        self
    }

    pub fn delete_action_id(mut self, delete_action_id: impl Into<String>) -> Self {
        self.delete_action_id = Some(delete_action_id.into());
        self
    }

    pub fn user_auth_required(mut self, user_auth_required: bool) -> Self {
        self.user_auth_required = Some(user_auth_required);
        self
    }

    pub fn user_auth_url(mut self, user_auth_url: impl Into<String>) -> Self {
        self.user_auth_url = Some(user_auth_url.into());
        self
    }

    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }
}

impl SlackApiMethod for EntityPresentCommentsRequest {
    const METHOD: &'static str = "entity.presentComments";
    type Response = EntityPresentCommentsResponse;
}

impl CursorPaginated for EntityPresentCommentsRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for EntityPresentCommentsResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`entity.presentComments`](https://docs.slack.dev/reference/methods/entity.presentComments).
#[doc(alias = "entity.presentComments")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EntityPresentCommentsResponse {
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

/// Arguments for the Slack Web API method [`entity.presentDetails`](https://docs.slack.dev/reference/methods/entity.presentDetails): Provide custom flexpane behavior for Work Objects. Apps call this method to send per-user flexpane metadata to the client.
///
/// Send it with [`SlackClient::entity_present_details`].
#[doc(alias = "entity.presentDetails")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct EntityPresentDetailsRequest {
    /// A reference to the original user action that initiated the request.
    pub trigger_id: String,
    /// URL-encoded JSON object containing flexpane metadata from the app that will be conformed to a Work Object metadata schema, keyed by entity ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Set to true (or 1) to indicate that the user must authenticate to view full flexpane data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_auth_required: Option<bool>,
    /// A custom URL to which users are directed for authentication if required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_auth_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl EntityPresentDetailsRequest {
    pub fn new(trigger_id: impl Into<String>) -> Self {
        Self {
            trigger_id: trigger_id.into(),
            metadata: None,
            user_auth_required: None,
            user_auth_url: None,
            error: None,
        }
    }

    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn user_auth_required(mut self, user_auth_required: bool) -> Self {
        self.user_auth_required = Some(user_auth_required);
        self
    }

    pub fn user_auth_url(mut self, user_auth_url: impl Into<String>) -> Self {
        self.user_auth_url = Some(user_auth_url.into());
        self
    }

    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }
}

impl SlackApiMethod for EntityPresentDetailsRequest {
    const METHOD: &'static str = "entity.presentDetails";
    type Response = EntityPresentDetailsResponse;
}

/// Successful response of the Slack Web API method [`entity.presentDetails`](https://docs.slack.dev/reference/methods/entity.presentDetails).
#[doc(alias = "entity.presentDetails")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EntityPresentDetailsResponse {
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
    /// Calls the Slack Web API method [`entity.acknowledgeCommentAction`](https://docs.slack.dev/reference/methods/entity.acknowledgeCommentAction): Acknowledge a comment post, edit, or delete on a Work Object. Apps call this method to confirm they have processed a comment action.
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "entity.acknowledgeCommentAction")]
    pub async fn entity_acknowledge_comment_action(
        &self,
        request: &EntityAcknowledgeCommentActionRequest,
    ) -> Result<EntityAcknowledgeCommentActionResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`entity.presentComments`](https://docs.slack.dev/reference/methods/entity.presentComments): Provide comments for Work Objects. Apps call this method to send per-user flexpane comment data to the client.
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "entity.presentComments")]
    pub async fn entity_present_comments(
        &self,
        request: &EntityPresentCommentsRequest,
    ) -> Result<EntityPresentCommentsResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`entity.presentDetails`](https://docs.slack.dev/reference/methods/entity.presentDetails): Provide custom flexpane behavior for Work Objects. Apps call this method to send per-user flexpane metadata to the client.
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "entity.presentDetails")]
    pub async fn entity_present_details(
        &self,
        request: &EntityPresentDetailsRequest,
    ) -> Result<EntityPresentDetailsResponse, SlackError> {
        self.call(request).await
    }
}
