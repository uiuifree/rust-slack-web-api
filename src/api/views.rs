// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`views.open`](https://docs.slack.dev/reference/methods/views.open): Open a view for a user.
///
/// Send it with [`SlackClient::views_open`].
#[doc(alias = "views.open")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ViewsOpenRequest {
    /// A [view payload](https://docs.slack.dev/reference/views.md). This must be a JSON-encoded string.
    pub view: crate::blocks::View,
    /// Exchange a trigger to post to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_id: Option<String>,
    /// Exchange an interactivity pointer to post to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactivity_pointer: Option<String>,
}

impl ViewsOpenRequest {
    pub fn new(view: crate::blocks::View) -> Self {
        Self {
            view,
            trigger_id: None,
            interactivity_pointer: None,
        }
    }

    pub fn trigger_id(mut self, trigger_id: impl Into<String>) -> Self {
        self.trigger_id = Some(trigger_id.into());
        self
    }

    pub fn interactivity_pointer(mut self, interactivity_pointer: impl Into<String>) -> Self {
        self.interactivity_pointer = Some(interactivity_pointer.into());
        self
    }
}

impl SlackApiMethod for ViewsOpenRequest {
    const METHOD: &'static str = "views.open";
    type Response = ViewsOpenResponse;
}

/// Successful response of the Slack Web API method [`views.open`](https://docs.slack.dev/reference/methods/views.open).
#[doc(alias = "views.open")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ViewsOpenResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub view: Option<ViewInfo>,
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

/// Arguments for the Slack Web API method [`views.publish`](https://docs.slack.dev/reference/methods/views.publish): Publish a static view for a User.
///
/// Send it with [`SlackClient::views_publish`].
#[doc(alias = "views.publish")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ViewsPublishRequest {
    /// `id` of the user you want publish a view to.
    pub user_id: String,
    /// A [view payload](https://docs.slack.dev/reference/views.md). This must be a JSON-encoded string.
    pub view: crate::blocks::View,
    /// A string that represents view state to protect against possible race conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactivity_pointer: Option<String>,
}

impl ViewsPublishRequest {
    pub fn new(user_id: impl Into<String>, view: crate::blocks::View) -> Self {
        Self {
            user_id: user_id.into(),
            view,
            hash: None,
            interactivity_pointer: None,
        }
    }

    pub fn hash(mut self, hash: impl Into<String>) -> Self {
        self.hash = Some(hash.into());
        self
    }

    pub fn interactivity_pointer(mut self, interactivity_pointer: impl Into<String>) -> Self {
        self.interactivity_pointer = Some(interactivity_pointer.into());
        self
    }
}

impl SlackApiMethod for ViewsPublishRequest {
    const METHOD: &'static str = "views.publish";
    type Response = ViewsPublishResponse;
}

/// Successful response of the Slack Web API method [`views.publish`](https://docs.slack.dev/reference/methods/views.publish).
#[doc(alias = "views.publish")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ViewsPublishResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub view: Option<ViewInfo>,
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

/// Arguments for the Slack Web API method [`views.push`](https://docs.slack.dev/reference/methods/views.push): Push a view onto the stack of a root view.
///
/// Send it with [`SlackClient::views_push`].
#[doc(alias = "views.push")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ViewsPushRequest {
    /// A [view payload](https://docs.slack.dev/reference/views.md). This must be a JSON-encoded string.
    pub view: crate::blocks::View,
    /// Exchange a trigger to post to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_id: Option<String>,
    /// Exchange an interactivity pointer to post to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactivity_pointer: Option<String>,
}

impl ViewsPushRequest {
    pub fn new(view: crate::blocks::View) -> Self {
        Self {
            view,
            trigger_id: None,
            interactivity_pointer: None,
        }
    }

    pub fn trigger_id(mut self, trigger_id: impl Into<String>) -> Self {
        self.trigger_id = Some(trigger_id.into());
        self
    }

    pub fn interactivity_pointer(mut self, interactivity_pointer: impl Into<String>) -> Self {
        self.interactivity_pointer = Some(interactivity_pointer.into());
        self
    }
}

impl SlackApiMethod for ViewsPushRequest {
    const METHOD: &'static str = "views.push";
    type Response = ViewsPushResponse;
}

/// Successful response of the Slack Web API method [`views.push`](https://docs.slack.dev/reference/methods/views.push).
#[doc(alias = "views.push")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ViewsPushResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub view: Option<ViewInfo>,
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

/// Arguments for the Slack Web API method [`views.update`](https://docs.slack.dev/reference/methods/views.update): Update an existing view.
///
/// Send it with [`SlackClient::views_update`].
#[doc(alias = "views.update")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ViewsUpdateRequest {
    /// A [view object](https://docs.slack.dev/reference/views.md). This must be a JSON-encoded string.
    pub view: crate::blocks::View,
    /// A unique identifier of the view to be updated. Either `view_id` or `external_id` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
    /// A unique identifier of the view set by the developer. Must be unique for all views on a team. Max length of 255 characters. Either `view_id` or `external_id` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// A string that represents view state to protect against possible race conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

impl ViewsUpdateRequest {
    pub fn new(view: crate::blocks::View) -> Self {
        Self {
            view,
            view_id: None,
            external_id: None,
            hash: None,
        }
    }

    pub fn view_id(mut self, view_id: impl Into<String>) -> Self {
        self.view_id = Some(view_id.into());
        self
    }

    pub fn external_id(mut self, external_id: impl Into<String>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }

    pub fn hash(mut self, hash: impl Into<String>) -> Self {
        self.hash = Some(hash.into());
        self
    }
}

impl SlackApiMethod for ViewsUpdateRequest {
    const METHOD: &'static str = "views.update";
    type Response = ViewsUpdateResponse;
}

/// Successful response of the Slack Web API method [`views.update`](https://docs.slack.dev/reference/methods/views.update).
#[doc(alias = "views.update")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ViewsUpdateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub view: Option<ViewInfo>,
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
    /// Calls the Slack Web API method [`views.open`](https://docs.slack.dev/reference/methods/views.open): Open a view for a user.
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "views.open")]
    pub async fn views_open(
        &self,
        request: &ViewsOpenRequest,
    ) -> Result<ViewsOpenResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`views.publish`](https://docs.slack.dev/reference/methods/views.publish): Publish a static view for a User.
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "views.publish")]
    pub async fn views_publish(
        &self,
        request: &ViewsPublishRequest,
    ) -> Result<ViewsPublishResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`views.push`](https://docs.slack.dev/reference/methods/views.push): Push a view onto the stack of a root view.
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "views.push")]
    pub async fn views_push(
        &self,
        request: &ViewsPushRequest,
    ) -> Result<ViewsPushResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`views.update`](https://docs.slack.dev/reference/methods/views.update): Update an existing view.
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "views.update")]
    pub async fn views_update(
        &self,
        request: &ViewsUpdateRequest,
    ) -> Result<ViewsUpdateResponse, SlackError> {
        self.call(request).await
    }
}
