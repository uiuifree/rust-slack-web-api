// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`dialog.open`](https://docs.slack.dev/reference/methods/dialog.open): Open a dialog with a user
///
/// Send it with [`SlackClient::dialog_open`].
#[doc(alias = "dialog.open")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct DialogOpenRequest {
    /// The dialog definition. This must be a JSON-encoded string.
    pub dialog: String,
    /// Exchange a trigger to post to the user.
    pub trigger_id: String,
}

impl DialogOpenRequest {
    pub fn new(dialog: impl Into<String>, trigger_id: impl Into<String>) -> Self {
        Self {
            dialog: dialog.into(),
            trigger_id: trigger_id.into(),
        }
    }
}

impl SlackApiMethod for DialogOpenRequest {
    const METHOD: &'static str = "dialog.open";
    type Response = DialogOpenResponse;
}

/// Successful response of the Slack Web API method [`dialog.open`](https://docs.slack.dev/reference/methods/dialog.open).
#[doc(alias = "dialog.open")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DialogOpenResponse {
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
    /// Calls the Slack Web API method [`dialog.open`](https://docs.slack.dev/reference/methods/dialog.open): Open a dialog with a user
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "dialog.open")]
    pub async fn dialog_open(
        &self,
        request: &DialogOpenRequest,
    ) -> Result<DialogOpenResponse, SlackError> {
        self.call(request).await
    }
}
