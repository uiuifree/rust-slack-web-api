// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`admin.audit.anomaly.allow.getItem`](https://docs.slack.dev/reference/methods/admin.audit.anomaly.allow.getItem): API to allow Enterprise org admins to read the allow list of IP blocks and ASNs from the enterprise configuration.
///
/// Send it with [`SlackClient::admin_audit_anomaly_allow_get_item`].
#[doc(alias = "admin.audit.anomaly.allow.getItem")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminAuditAnomalyAllowGetItemRequest {}

impl AdminAuditAnomalyAllowGetItemRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl SlackApiMethod for AdminAuditAnomalyAllowGetItemRequest {
    const METHOD: &'static str = "admin.audit.anomaly.allow.getItem";
    type Response = AdminAuditAnomalyAllowGetItemResponse;
}

/// Successful response of the Slack Web API method [`admin.audit.anomaly.allow.getItem`](https://docs.slack.dev/reference/methods/admin.audit.anomaly.allow.getItem).
#[doc(alias = "admin.audit.anomaly.allow.getItem")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAuditAnomalyAllowGetItemResponse {
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

/// Arguments for the Slack Web API method [`admin.audit.anomaly.allow.updateItem`](https://docs.slack.dev/reference/methods/admin.audit.anomaly.allow.updateItem): API to allow Enterprise org admins to write/overwrite the allow list of IP blocks and ASNs from the enterprise configuration.
///
/// Send it with [`SlackClient::admin_audit_anomaly_allow_update_item`].
#[doc(alias = "admin.audit.anomaly.allow.updateItem")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AdminAuditAnomalyAllowUpdateItemRequest {
    /// allow list of IPv4 addresses using cidr notation in the Enterprise organization configuration
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub trusted_cidr: Option<Vec<serde_json::Value>>,
    /// allow list of Autonomous System Numbers (ASN) in the Enterprise organization configuration
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub trusted_asns: Option<Vec<serde_json::Value>>,
}

impl AdminAuditAnomalyAllowUpdateItemRequest {
    pub fn new() -> Self {
        Self {
            trusted_cidr: None,
            trusted_asns: None,
        }
    }

    pub fn trusted_cidr(mut self, trusted_cidr: Vec<serde_json::Value>) -> Self {
        self.trusted_cidr = Some(trusted_cidr);
        self
    }

    pub fn trusted_asns(mut self, trusted_asns: Vec<serde_json::Value>) -> Self {
        self.trusted_asns = Some(trusted_asns);
        self
    }
}

impl SlackApiMethod for AdminAuditAnomalyAllowUpdateItemRequest {
    const METHOD: &'static str = "admin.audit.anomaly.allow.updateItem";
    type Response = AdminAuditAnomalyAllowUpdateItemResponse;
}

/// Successful response of the Slack Web API method [`admin.audit.anomaly.allow.updateItem`](https://docs.slack.dev/reference/methods/admin.audit.anomaly.allow.updateItem).
#[doc(alias = "admin.audit.anomaly.allow.updateItem")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAuditAnomalyAllowUpdateItemResponse {
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
    /// Calls the Slack Web API method [`admin.audit.anomaly.allow.getItem`](https://docs.slack.dev/reference/methods/admin.audit.anomaly.allow.getItem): API to allow Enterprise org admins to read the allow list of IP blocks and ASNs from the enterprise configuration.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.audit.anomaly.allow.getItem")]
    pub async fn admin_audit_anomaly_allow_get_item(
        &self,
        request: &AdminAuditAnomalyAllowGetItemRequest,
    ) -> Result<AdminAuditAnomalyAllowGetItemResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.audit.anomaly.allow.updateItem`](https://docs.slack.dev/reference/methods/admin.audit.anomaly.allow.updateItem): API to allow Enterprise org admins to write/overwrite the allow list of IP blocks and ASNs from the enterprise configuration.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.audit.anomaly.allow.updateItem")]
    pub async fn admin_audit_anomaly_allow_update_item(
        &self,
        request: &AdminAuditAnomalyAllowUpdateItemRequest,
    ) -> Result<AdminAuditAnomalyAllowUpdateItemResponse, SlackError> {
        self.call(request).await
    }
}
