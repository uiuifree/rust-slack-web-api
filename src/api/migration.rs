// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`migration.exchange`](https://docs.slack.dev/reference/methods/migration.exchange): For Enterprise organization workspaces, map local user IDs to global user IDs
///
/// Send it with [`SlackClient::migration_exchange`].
#[doc(alias = "migration.exchange")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct MigrationExchangeRequest {
    /// A comma-separated list of user ids, up to 400 per request
    #[serde(serialize_with = "crate::form::as_json")]
    pub users: Vec<serde_json::Value>,
    /// Specify team\_id starts with `T` in case of Org Token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Specify `true` to convert `W` global user IDs to workspace-specific `U` IDs. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_old: Option<bool>,
}

impl MigrationExchangeRequest {
    pub fn new(users: Vec<serde_json::Value>) -> Self {
        Self {
            users,
            team_id: None,
            to_old: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn to_old(mut self, to_old: bool) -> Self {
        self.to_old = Some(to_old);
        self
    }
}

impl SlackApiMethod for MigrationExchangeRequest {
    const METHOD: &'static str = "migration.exchange";
    type Response = MigrationExchangeResponse;
}

/// Successful response of the Slack Web API method [`migration.exchange`](https://docs.slack.dev/reference/methods/migration.exchange).
#[doc(alias = "migration.exchange")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MigrationExchangeResponse {
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
    pub enterprise_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id_map: Option<std::collections::HashMap<String, String>>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub invalid_user_ids: Vec<String>,
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
    /// Calls the Slack Web API method [`migration.exchange`](https://docs.slack.dev/reference/methods/migration.exchange): For Enterprise organization workspaces, map local user IDs to global user IDs
    ///
    /// Required scopes:
    ///
    /// - bot token: `tokens.basic`
    /// - user token: `tokens.basic`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "migration.exchange")]
    pub async fn migration_exchange(
        &self,
        request: &MigrationExchangeRequest,
    ) -> Result<MigrationExchangeResponse, SlackError> {
        self.call(request).await
    }
}
