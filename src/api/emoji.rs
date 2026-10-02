// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`emoji.list`](https://docs.slack.dev/reference/methods/emoji.list): Lists custom emoji for a team.
///
/// Send it with [`SlackClient::emoji_list`].
#[doc(alias = "emoji.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct EmojiListRequest {
    /// Include a list of categories for Unicode emoji and the emoji in each category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_categories: Option<bool>,
}

impl EmojiListRequest {
    pub fn new() -> Self {
        Self {
            include_categories: None,
        }
    }

    pub fn include_categories(mut self, include_categories: bool) -> Self {
        self.include_categories = Some(include_categories);
        self
    }
}

impl SlackApiMethod for EmojiListRequest {
    const METHOD: &'static str = "emoji.list";
    type Response = EmojiListResponse;
}

/// Successful response of the Slack Web API method [`emoji.list`](https://docs.slack.dev/reference/methods/emoji.list).
#[doc(alias = "emoji.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EmojiListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub emoji: Option<std::collections::HashMap<String, String>>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub cache_ts: Option<String>,
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
    /// Calls the Slack Web API method [`emoji.list`](https://docs.slack.dev/reference/methods/emoji.list): Lists custom emoji for a team.
    ///
    /// Required scopes:
    ///
    /// - bot token: `emoji:read`
    /// - user token: `emoji:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "emoji.list")]
    pub async fn emoji_list(
        &self,
        request: &EmojiListRequest,
    ) -> Result<EmojiListResponse, SlackError> {
        self.call(request).await
    }
}
