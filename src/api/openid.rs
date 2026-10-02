// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`openid.connect.token`](https://docs.slack.dev/reference/methods/openid.connect.token): Exchanges a temporary OAuth verifier code for an access token for Sign in with Slack.
///
/// Send it with [`SlackClient::openid_connect_token`].
#[doc(alias = "openid.connect.token")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct OpenidConnectTokenRequest {
    /// Issued when you created your application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Issued when you created your application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// The `code` param returned via the OAuth callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// This must match the originally submitted URI (if one was sent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    /// The `grant_type` param as described in the OAuth spec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<String>,
    /// The `refresh_token` param as described in the OAuth spec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// PKCE code verifier (RFC 7636). Required when the authorization request included a `code_challenge`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verifier: Option<String>,
}

impl OpenidConnectTokenRequest {
    pub fn new() -> Self {
        Self {
            client_id: None,
            client_secret: None,
            code: None,
            redirect_uri: None,
            grant_type: None,
            refresh_token: None,
            code_verifier: None,
        }
    }

    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.client_secret = Some(client_secret.into());
        self
    }

    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn redirect_uri(mut self, redirect_uri: impl Into<String>) -> Self {
        self.redirect_uri = Some(redirect_uri.into());
        self
    }

    pub fn grant_type(mut self, grant_type: impl Into<String>) -> Self {
        self.grant_type = Some(grant_type.into());
        self
    }

    pub fn refresh_token(mut self, refresh_token: impl Into<String>) -> Self {
        self.refresh_token = Some(refresh_token.into());
        self
    }

    pub fn code_verifier(mut self, code_verifier: impl Into<String>) -> Self {
        self.code_verifier = Some(code_verifier.into());
        self
    }
}

impl SlackApiMethod for OpenidConnectTokenRequest {
    const METHOD: &'static str = "openid.connect.token";
    type Response = OpenidConnectTokenResponse;
}

/// Successful response of the Slack Web API method [`openid.connect.token`](https://docs.slack.dev/reference/methods/openid.connect.token).
#[doc(alias = "openid.connect.token")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OpenidConnectTokenResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub token_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub id_token: Option<String>,
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

/// Arguments for the Slack Web API method [`openid.connect.userInfo`](https://docs.slack.dev/reference/methods/openid.connect.userInfo): Get the identity of a user who has authorized Sign in with Slack.
///
/// Send it with [`SlackClient::openid_connect_user_info`].
#[doc(alias = "openid.connect.userInfo")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct OpenidConnectUserInfoRequest {}

impl OpenidConnectUserInfoRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl SlackApiMethod for OpenidConnectUserInfoRequest {
    const METHOD: &'static str = "openid.connect.userInfo";
    type Response = OpenidConnectUserInfoResponse;
}

/// Successful response of the Slack Web API method [`openid.connect.userInfo`](https://docs.slack.dev/reference/methods/openid.connect.userInfo).
#[doc(alias = "openid.connect.userInfo")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OpenidConnectUserInfoResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub sub: Option<String>,
    #[serde(
        rename = "https://slack.com/user_id",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_user_id: Option<String>,
    #[serde(
        rename = "https://slack.com/team_id",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_team_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub email: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub email_verified: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_email_verified: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub picture: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub given_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub family_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub locale: Option<String>,
    #[serde(
        rename = "https://slack.com/team_name",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_team_name: Option<String>,
    #[serde(
        rename = "https://slack.com/team_domain",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_team_domain: Option<String>,
    #[serde(
        rename = "https://slack.com/user_image_24",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_user_image_24: Option<String>,
    #[serde(
        rename = "https://slack.com/user_image_32",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_user_image_32: Option<String>,
    #[serde(
        rename = "https://slack.com/user_image_48",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_user_image_48: Option<String>,
    #[serde(
        rename = "https://slack.com/user_image_72",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_user_image_72: Option<String>,
    #[serde(
        rename = "https://slack.com/user_image_192",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_user_image_192: Option<String>,
    #[serde(
        rename = "https://slack.com/user_image_512",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_user_image_512: Option<String>,
    #[serde(
        rename = "https://slack.com/team_image_34",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_team_image_34: Option<String>,
    #[serde(
        rename = "https://slack.com/team_image_44",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_team_image_44: Option<String>,
    #[serde(
        rename = "https://slack.com/team_image_68",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_team_image_68: Option<String>,
    #[serde(
        rename = "https://slack.com/team_image_88",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_team_image_88: Option<String>,
    #[serde(
        rename = "https://slack.com/team_image_102",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_team_image_102: Option<String>,
    #[serde(
        rename = "https://slack.com/team_image_132",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_team_image_132: Option<String>,
    #[serde(
        rename = "https://slack.com/team_image_230",
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_team_image_230: Option<String>,
    #[serde(
        rename = "https://slack.com/team_image_default",
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_team_image_default: Option<bool>,
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
    /// Calls the Slack Web API method [`openid.connect.token`](https://docs.slack.dev/reference/methods/openid.connect.token): Exchanges a temporary OAuth verifier code for an access token for Sign in with Slack.
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "openid.connect.token")]
    pub async fn openid_connect_token(
        &self,
        request: &OpenidConnectTokenRequest,
    ) -> Result<OpenidConnectTokenResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`openid.connect.userInfo`](https://docs.slack.dev/reference/methods/openid.connect.userInfo): Get the identity of a user who has authorized Sign in with Slack.
    ///
    /// Required scopes:
    ///
    /// - user token: `openid`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "openid.connect.userInfo")]
    pub async fn openid_connect_user_info(
        &self,
        request: &OpenidConnectUserInfoRequest,
    ) -> Result<OpenidConnectUserInfoResponse, SlackError> {
        self.call(request).await
    }
}
