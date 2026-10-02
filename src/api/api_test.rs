// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`api.test`](https://docs.slack.dev/reference/methods/api.test): Checks API calling code.
///
/// Send it with [`SlackClient::api_test`].
#[doc(alias = "api.test")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct ApiTestRequest {
    /// Error response to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ApiTestRequest {
    pub fn new() -> Self {
        Self { error: None }
    }

    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }
}

impl SlackApiMethod for ApiTestRequest {
    const METHOD: &'static str = "api.test";
    type Response = ApiTestResponse;
}

/// Successful response of the Slack Web API method [`api.test`](https://docs.slack.dev/reference/methods/api.test).
#[doc(alias = "api.test")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ApiTestResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub args: Option<ApiTestResponseArgs>,
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
pub struct ApiTestResponseArgs {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub foo: Option<String>,
}

impl SlackClient {
    /// Calls the Slack Web API method [`api.test`](https://docs.slack.dev/reference/methods/api.test): Checks API calling code.
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "api.test")]
    pub async fn api_test(&self, request: &ApiTestRequest) -> Result<ApiTestResponse, SlackError> {
        self.call(request).await
    }
}
