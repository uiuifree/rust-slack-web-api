// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`tooling.tokens.rotate`](https://docs.slack.dev/reference/methods/tooling.tokens.rotate): Exchanges a refresh token for a new app configuration token.
///
/// Send it with [`SlackClient::tooling_tokens_rotate`].
#[doc(alias = "tooling.tokens.rotate")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ToolingTokensRotateRequest {
    /// The `xoxe` refresh token that was issued along with the old app configuration token.
    pub refresh_token: String,
}

impl ToolingTokensRotateRequest {
    pub fn new(refresh_token: impl Into<String>) -> Self {
        Self {
            refresh_token: refresh_token.into(),
        }
    }
}

impl SlackApiMethod for ToolingTokensRotateRequest {
    const METHOD: &'static str = "tooling.tokens.rotate";
    type Response = ToolingTokensRotateResponse;
}

/// Successful response of the Slack Web API method [`tooling.tokens.rotate`](https://docs.slack.dev/reference/methods/tooling.tokens.rotate).
#[doc(alias = "tooling.tokens.rotate")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ToolingTokensRotateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub token: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_token: Option<String>,
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
    pub user_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub iat: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub exp: Option<i64>,
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
    /// Calls the Slack Web API method [`tooling.tokens.rotate`](https://docs.slack.dev/reference/methods/tooling.tokens.rotate): Exchanges a refresh token for a new app configuration token.
    ///
    /// Rate limit: Tier 5 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "tooling.tokens.rotate")]
    pub async fn tooling_tokens_rotate(
        &self,
        request: &ToolingTokensRotateRequest,
    ) -> Result<ToolingTokensRotateResponse, SlackError> {
        self.call(request).await
    }
}
