// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`blocks.validate`](https://docs.slack.dev/reference/methods/blocks.validate): Validates blocks, messages, and views Block Kit JSON payloads.
///
/// Send it with [`SlackClient::blocks_validate`].
#[doc(alias = "blocks.validate")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct BlocksValidateRequest {
    /// A JSON-encoded array of [structured blocks](https://docs.slack.dev/reference/block-kit/blocks.md). Provide exactly one of `blocks`, `view`, or `message`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub blocks: Option<Vec<crate::blocks::Block>>,
    /// A JSON-encoded [message payload](https://docs.slack.dev/messaging/creating-interactive-messages.md) to validate. Provide exactly one of `blocks`, `view`, or `message`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// A JSON-encoded [view payload](https://docs.slack.dev/reference/views.md) to validate. Provide exactly one of `blocks`, `view`, or `message`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<crate::blocks::View>,
}

impl BlocksValidateRequest {
    pub fn new() -> Self {
        Self {
            blocks: None,
            message: None,
            view: None,
        }
    }

    pub fn blocks(mut self, blocks: Vec<crate::blocks::Block>) -> Self {
        self.blocks = Some(blocks);
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn view(mut self, view: crate::blocks::View) -> Self {
        self.view = Some(view);
        self
    }
}

impl SlackApiMethod for BlocksValidateRequest {
    const METHOD: &'static str = "blocks.validate";
    type Response = BlocksValidateResponse;
}

/// Successful response of the Slack Web API method [`blocks.validate`](https://docs.slack.dev/reference/methods/blocks.validate).
#[doc(alias = "blocks.validate")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BlocksValidateResponse {
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
    /// Calls the Slack Web API method [`blocks.validate`](https://docs.slack.dev/reference/methods/blocks.validate): Validates blocks, messages, and views Block Kit JSON payloads.
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "blocks.validate")]
    pub async fn blocks_validate(
        &self,
        request: &BlocksValidateRequest,
    ) -> Result<BlocksValidateResponse, SlackError> {
        self.call(request).await
    }
}
