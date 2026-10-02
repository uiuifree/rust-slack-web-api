// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`bots.info`](https://docs.slack.dev/reference/methods/bots.info): Gets information about a bot user.
///
/// Send it with [`SlackClient::bots_info`].
#[doc(alias = "bots.info")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct BotsInfoRequest {
    /// Bot user to get info on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<String>,
    /// encoded team id or enterprise id where the bot exists, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl BotsInfoRequest {
    pub fn new() -> Self {
        Self {
            bot: None,
            team_id: None,
        }
    }

    pub fn bot(mut self, bot: impl Into<String>) -> Self {
        self.bot = Some(bot.into());
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for BotsInfoRequest {
    const METHOD: &'static str = "bots.info";
    type Response = BotsInfoResponse;
}

/// Successful response of the Slack Web API method [`bots.info`](https://docs.slack.dev/reference/methods/bots.info).
#[doc(alias = "bots.info")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BotsInfoResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub bot: Option<Bot>,
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
    /// Calls the Slack Web API method [`bots.info`](https://docs.slack.dev/reference/methods/bots.info): Gets information about a bot user.
    ///
    /// Required scopes:
    ///
    /// - bot token: `users:read`
    /// - user token: `users:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "bots.info")]
    pub async fn bots_info(
        &self,
        request: &BotsInfoRequest,
    ) -> Result<BotsInfoResponse, SlackError> {
        self.call(request).await
    }
}
