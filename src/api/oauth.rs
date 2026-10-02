// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`oauth.access`](https://docs.slack.dev/reference/methods/oauth.access): Exchanges a temporary OAuth verifier code for an access token.
///
/// Send it with [`SlackClient::oauth_access`].
#[doc(alias = "oauth.access")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct OauthAccessRequest {
    /// Issued when you created your application. If possible, avoid sending `client_id` and `client_secret` as parameters in your request and instead supply the Client ID and Client Secret using the HTTP Basic authentication scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Issued when you created your application. If possible, avoid sending `client_id` and `client_secret` as parameters in your request and instead supply the Client ID and Client Secret using the HTTP Basic authentication scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// The `code` param returned via the OAuth callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// This must match the originally submitted URI (if one was sent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    /// Request the user to add your app only to a single channel. Only valid with a [legacy workspace app](https://docs.slack.dev/changelog/2021-03-workspace-apps-to-retire-in-august-2021.md).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_channel: Option<bool>,
}

impl OauthAccessRequest {
    pub fn new() -> Self {
        Self {
            client_id: None,
            client_secret: None,
            code: None,
            redirect_uri: None,
            single_channel: None,
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

    pub fn single_channel(mut self, single_channel: bool) -> Self {
        self.single_channel = Some(single_channel);
        self
    }
}

impl SlackApiMethod for OauthAccessRequest {
    const METHOD: &'static str = "oauth.access";
    type Response = OauthAccessResponse;
}

/// Successful response of the Slack Web API method [`oauth.access`](https://docs.slack.dev/reference/methods/oauth.access).
#[doc(alias = "oauth.access")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OauthAccessResponse {
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
    pub app_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_user_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise_id: Option<serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorizing_user: Option<OauthAccessResponseAuthorizingUser>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub installer_user: Option<OauthAccessResponseInstallerUser>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub scopes: Option<OauthAccessResponseScopes>,
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
pub struct OauthAccessResponseAuthorizingUser {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_home: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OauthAccessResponseInstallerUser {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_home: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OauthAccessResponseScopes {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub app_home: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub team: Vec<serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub channel: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mpim: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub im: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user: Vec<serde_json::Value>,
}

/// Arguments for the Slack Web API method [`oauth.v2.access`](https://docs.slack.dev/reference/methods/oauth.v2.access): Exchanges a temporary OAuth verifier code for an access token.
///
/// Send it with [`SlackClient::oauth_v2_access`].
#[doc(alias = "oauth.v2.access")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct OauthV2AccessRequest {
    /// Issued when you created your application. If possible, avoid sending `client_id` and `client_secret` as parameters in your request and instead supply the Client ID and Client Secret using the HTTP Basic authentication scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Issued when you created your application. If possible, avoid sending `client_id` and `client_secret` as parameters in your request and instead supply the Client ID and Client Secret using the HTTP Basic authentication scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// The `code` param returned via the OAuth callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The code\_verifier param used to generate the code\_challenge originally. Used for PKCE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verifier: Option<String>,
    /// This must match the originally submitted URI (if one was sent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    /// The `grant_type` param as described in the OAuth spec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<String>,
    /// The `refresh_token` param as described in the OAuth spec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// Identity assertion JWT authorization grant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertion: Option<String>,
}

impl OauthV2AccessRequest {
    pub fn new() -> Self {
        Self {
            client_id: None,
            client_secret: None,
            code: None,
            code_verifier: None,
            redirect_uri: None,
            grant_type: None,
            refresh_token: None,
            assertion: None,
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

    pub fn code_verifier(mut self, code_verifier: impl Into<String>) -> Self {
        self.code_verifier = Some(code_verifier.into());
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

    pub fn assertion(mut self, assertion: impl Into<String>) -> Self {
        self.assertion = Some(assertion.into());
        self
    }
}

impl SlackApiMethod for OauthV2AccessRequest {
    const METHOD: &'static str = "oauth.v2.access";
    type Response = OauthV2AccessResponse;
}

/// Successful response of the Slack Web API method [`oauth.v2.access`](https://docs.slack.dev/reference/methods/oauth.v2.access).
#[doc(alias = "oauth.v2.access")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OauthV2AccessResponse {
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
    pub scope: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub bot_user_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub team: Option<OauthV2AccessResponseTeam>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub enterprise: Option<Enterprise>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub authed_user: Option<OauthV2AccessResponseAuthedUser>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires_in: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_token: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_enterprise_install: Option<bool>,
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
pub struct OauthV2AccessResponseTeam {
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
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OauthV2AccessResponseAuthedUser {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub scope: Option<String>,
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
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires_in: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_token: Option<String>,
}

/// Arguments for the Slack Web API method [`oauth.v2.beginShortTokenRotation`](https://docs.slack.dev/reference/methods/oauth.v2.beginShortTokenRotation): Begins rotating the secret on an API token with a short secret.
///
/// Send it with [`SlackClient::oauth_v2_begin_short_token_rotation`].
#[doc(alias = "oauth.v2.beginShortTokenRotation")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct OauthV2BeginShortTokenRotationRequest {
    /// Issued when you created your application. Must be the app the token being rotated was issued to.
    pub client_id: String,
    /// Issued when you created your application.
    pub client_secret: String,
}

impl OauthV2BeginShortTokenRotationRequest {
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }
}

impl SlackApiMethod for OauthV2BeginShortTokenRotationRequest {
    const METHOD: &'static str = "oauth.v2.beginShortTokenRotation";
    type Response = OauthV2BeginShortTokenRotationResponse;
}

/// Successful response of the Slack Web API method [`oauth.v2.beginShortTokenRotation`](https://docs.slack.dev/reference/methods/oauth.v2.beginShortTokenRotation).
#[doc(alias = "oauth.v2.beginShortTokenRotation")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OauthV2BeginShortTokenRotationResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub new_token: Option<String>,
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

/// Arguments for the Slack Web API method [`oauth.v2.completeShortTokenRotation`](https://docs.slack.dev/reference/methods/oauth.v2.completeShortTokenRotation): Finishes rotating the secret on an API token with a short secret.
///
/// Send it with [`SlackClient::oauth_v2_complete_short_token_rotation`].
#[doc(alias = "oauth.v2.completeShortTokenRotation")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct OauthV2CompleteShortTokenRotationRequest {
    /// Issued when you created your application. Must be the app the token being rotated was issued to.
    pub client_id: String,
    /// Issued when you created your application.
    pub client_secret: String,
    /// The new xoxp token returned by oauth.v2.beginShortTokenRotation.
    pub new_token: String,
}

impl OauthV2CompleteShortTokenRotationRequest {
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        new_token: impl Into<String>,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            new_token: new_token.into(),
        }
    }
}

impl SlackApiMethod for OauthV2CompleteShortTokenRotationRequest {
    const METHOD: &'static str = "oauth.v2.completeShortTokenRotation";
    type Response = OauthV2CompleteShortTokenRotationResponse;
}

/// Successful response of the Slack Web API method [`oauth.v2.completeShortTokenRotation`](https://docs.slack.dev/reference/methods/oauth.v2.completeShortTokenRotation).
#[doc(alias = "oauth.v2.completeShortTokenRotation")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OauthV2CompleteShortTokenRotationResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub token: Option<String>,
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

/// Arguments for the Slack Web API method [`oauth.v2.exchange`](https://docs.slack.dev/reference/methods/oauth.v2.exchange): Exchanges a legacy access token for a new expiring access token and refresh token
///
/// Send it with [`SlackClient::oauth_v2_exchange`].
#[doc(alias = "oauth.v2.exchange")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct OauthV2ExchangeRequest {
    /// Issued when you created your application.
    pub client_id: String,
    /// Issued when you created your application.
    pub client_secret: String,
}

impl OauthV2ExchangeRequest {
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }
}

impl SlackApiMethod for OauthV2ExchangeRequest {
    const METHOD: &'static str = "oauth.v2.exchange";
    type Response = OauthV2ExchangeResponse;
}

/// Successful response of the Slack Web API method [`oauth.v2.exchange`](https://docs.slack.dev/reference/methods/oauth.v2.exchange).
#[doc(alias = "oauth.v2.exchange")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OauthV2ExchangeResponse {
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

/// Arguments for the Slack Web API method [`oauth.v2.user.access`](https://docs.slack.dev/reference/methods/oauth.v2.user.access): Exchanges a temporary OAuth verifier code for a user access token.
///
/// Send it with [`SlackClient::oauth_v2_user_access`].
#[doc(alias = "oauth.v2.user.access")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct OauthV2UserAccessRequest {
    /// Issued when you created your application. If possible, avoid sending `client_id` and `client_secret` as parameters in your request and instead supply the Client ID and Client Secret using the HTTP Basic authentication scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Issued when you created your application. If possible, avoid sending `client_id` and `client_secret` as parameters in your request and instead supply the Client ID and Client Secret using the HTTP Basic authentication scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// The `code` param returned via the OAuth callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The code\_verifier param used to generate the code\_challenge originally. Used for PKCE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verifier: Option<String>,
    /// This must match the originally submitted URI (if one was sent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    /// The `grant_type` param as described in the OAuth spec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<String>,
    /// The `refresh_token` param as described in the OAuth spec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// Identity assertion JWT authorization grant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertion: Option<String>,
}

impl OauthV2UserAccessRequest {
    pub fn new() -> Self {
        Self {
            client_id: None,
            client_secret: None,
            code: None,
            code_verifier: None,
            redirect_uri: None,
            grant_type: None,
            refresh_token: None,
            assertion: None,
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

    pub fn code_verifier(mut self, code_verifier: impl Into<String>) -> Self {
        self.code_verifier = Some(code_verifier.into());
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

    pub fn assertion(mut self, assertion: impl Into<String>) -> Self {
        self.assertion = Some(assertion.into());
        self
    }
}

impl SlackApiMethod for OauthV2UserAccessRequest {
    const METHOD: &'static str = "oauth.v2.user.access";
    type Response = OauthV2UserAccessResponse;
}

/// Successful response of the Slack Web API method [`oauth.v2.user.access`](https://docs.slack.dev/reference/methods/oauth.v2.user.access).
#[doc(alias = "oauth.v2.user.access")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OauthV2UserAccessResponse {
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
    pub authed_user: Option<OauthV2UserAccessResponseAuthedUser>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub team: Option<OauthV2UserAccessResponseTeam>,
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
pub struct OauthV2UserAccessResponseAuthedUser {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OauthV2UserAccessResponseTeam {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
}

impl SlackClient {
    /// Calls the Slack Web API method [`oauth.access`](https://docs.slack.dev/reference/methods/oauth.access): Exchanges a temporary OAuth verifier code for an access token.
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "oauth.access")]
    pub async fn oauth_access(
        &self,
        request: &OauthAccessRequest,
    ) -> Result<OauthAccessResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`oauth.v2.access`](https://docs.slack.dev/reference/methods/oauth.v2.access): Exchanges a temporary OAuth verifier code for an access token.
    ///
    /// Rate limit: Tier 5 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "oauth.v2.access")]
    pub async fn oauth_v2_access(
        &self,
        request: &OauthV2AccessRequest,
    ) -> Result<OauthV2AccessResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`oauth.v2.beginShortTokenRotation`](https://docs.slack.dev/reference/methods/oauth.v2.beginShortTokenRotation): Begins rotating the secret on an API token with a short secret.
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "oauth.v2.beginShortTokenRotation")]
    pub async fn oauth_v2_begin_short_token_rotation(
        &self,
        request: &OauthV2BeginShortTokenRotationRequest,
    ) -> Result<OauthV2BeginShortTokenRotationResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`oauth.v2.completeShortTokenRotation`](https://docs.slack.dev/reference/methods/oauth.v2.completeShortTokenRotation): Finishes rotating the secret on an API token with a short secret.
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "oauth.v2.completeShortTokenRotation")]
    pub async fn oauth_v2_complete_short_token_rotation(
        &self,
        request: &OauthV2CompleteShortTokenRotationRequest,
    ) -> Result<OauthV2CompleteShortTokenRotationResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`oauth.v2.exchange`](https://docs.slack.dev/reference/methods/oauth.v2.exchange): Exchanges a legacy access token for a new expiring access token and refresh token
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "oauth.v2.exchange")]
    pub async fn oauth_v2_exchange(
        &self,
        request: &OauthV2ExchangeRequest,
    ) -> Result<OauthV2ExchangeResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`oauth.v2.user.access`](https://docs.slack.dev/reference/methods/oauth.v2.user.access): Exchanges a temporary OAuth verifier code for a user access token.
    ///
    /// Rate limit: Tier 5 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "oauth.v2.user.access")]
    pub async fn oauth_v2_user_access(
        &self,
        request: &OauthV2UserAccessRequest,
    ) -> Result<OauthV2UserAccessResponse, SlackError> {
        self.call(request).await
    }
}
