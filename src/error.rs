use crate::ResponseMetadata;
use std::time::Duration;

/// Error returned by a Web API call.
#[derive(Debug, thiserror::Error)]
pub enum SlackError {
    /// Slack answered `"ok": false` (`invalid_auth`, `channel_not_found`, ...).
    #[error("slack api error: {}", .0.error)]
    Api(Box<SlackApiError>),
    /// Still rate limited (429) after the configured number of retries.
    #[error("rate limited by slack (retry after {retry_after:?})")]
    RateLimited { retry_after: Option<Duration> },
    /// A non-2xx status with a body that is not a Slack JSON response.
    #[error("unexpected http status {status}")]
    Http { status: u16, body: String },
    /// Connection or transfer failure.
    #[error("transport error: {0}")]
    Transport(#[from] reqwest::Error),
    /// The response body could not be decoded into the response type.
    #[error("failed to decode response: {source}")]
    Decode {
        source: serde_json::Error,
        body: String,
    },
    /// The request could not be encoded as a form body.
    #[error("failed to encode request: {0}")]
    Encode(String),
    /// A successful response lacked a field needed for the next step.
    #[error("response is missing {0}")]
    MissingField(&'static str),
    /// A background upload task stopped before finishing.
    #[error("background task failed: {0}")]
    Task(String),
}

/// Details of an `"ok": false` response.
#[derive(Debug, Clone, PartialEq)]
pub struct SlackApiError {
    /// The `error` code, such as `channel_not_found`.
    pub error: String,
    /// The `warning` value.
    pub warning: Option<String>,
    /// Details. The reasons for `invalid_blocks` and similar errors are in `messages`.
    pub response_metadata: Option<ResponseMetadata>,
}

impl SlackError {
    /// The `error` code returned by Slack, or `None` for non-API errors.
    pub fn api_error(&self) -> Option<&str> {
        match self {
            SlackError::Api(e) => Some(e.error.as_str()),
            _ => None,
        }
    }
}
