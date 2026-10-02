// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`apps.activities.list`](https://docs.slack.dev/reference/methods/apps.activities.list): Get logs for a specified app
///
/// Send it with [`SlackClient::apps_activities_list`].
#[doc(alias = "apps.activities.list")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsActivitiesListRequest {
    /// The id of the app to get activities from.
    pub app_id: String,
    /// The team who owns this log.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. See [pagination](https://docs.slack.dev/apis/web-api/pagination.md) for more detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The maximum number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The minimum log level of the log events to be returned. Defaults to 'info'. Acceptable values (in order of relative importance from smallest to largest) are ('trace', 'debug', 'info', 'warn', 'error', 'fatal').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_log_level: Option<String>,
    /// The event type of log events to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_event_type: Option<String>,
    /// The source of log events to be returned. Acceptable values are ('slack', 'developer').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// The component type of log events to be returned. Acceptable values are ('events\_api', 'workflows', 'functions', 'tables').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// The component id of log events to be returned. Will be 'FnXXXXXX' for functions, and 'WfXXXXXX' for workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// The trace id of log events to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// The earliest timestamp of the log to retrieve (epoch microseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_date_created: Option<i64>,
    /// The latest timestamp of the log to retrieve (epoch microseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_date_created: Option<i64>,
    /// The direction you want the data sorted by (always by timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
}

impl AppsActivitiesListRequest {
    pub fn new(app_id: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            team_id: None,
            cursor: None,
            limit: None,
            min_log_level: None,
            log_event_type: None,
            source: None,
            component_type: None,
            component_id: None,
            trace_id: None,
            min_date_created: None,
            max_date_created: None,
            sort_direction: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn min_log_level(mut self, min_log_level: impl Into<String>) -> Self {
        self.min_log_level = Some(min_log_level.into());
        self
    }

    pub fn log_event_type(mut self, log_event_type: impl Into<String>) -> Self {
        self.log_event_type = Some(log_event_type.into());
        self
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn component_type(mut self, component_type: impl Into<String>) -> Self {
        self.component_type = Some(component_type.into());
        self
    }

    pub fn component_id(mut self, component_id: impl Into<String>) -> Self {
        self.component_id = Some(component_id.into());
        self
    }

    pub fn trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    pub fn min_date_created(mut self, min_date_created: i64) -> Self {
        self.min_date_created = Some(min_date_created);
        self
    }

    pub fn max_date_created(mut self, max_date_created: i64) -> Self {
        self.max_date_created = Some(max_date_created);
        self
    }

    pub fn sort_direction(mut self, sort_direction: impl Into<String>) -> Self {
        self.sort_direction = Some(sort_direction.into());
        self
    }
}

impl SlackApiMethod for AppsActivitiesListRequest {
    const METHOD: &'static str = "apps.activities.list";
    type Response = AppsActivitiesListResponse;
}

impl CursorPaginated for AppsActivitiesListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AppsActivitiesListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`apps.activities.list`](https://docs.slack.dev/reference/methods/apps.activities.list).
#[doc(alias = "apps.activities.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsActivitiesListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub activities: Vec<AppsActivitiesListResponseActivities>,
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
pub struct AppsActivitiesListResponseActivities {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub level: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub event_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub source: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub component_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub component_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub payload: Option<AppsActivitiesListResponseActivitiesPayload>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub created: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsActivitiesListResponseActivitiesPayload {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub function_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub function_type: Option<String>,
}

/// Arguments for the Slack Web API method [`apps.auth.external.delete`](https://docs.slack.dev/reference/methods/apps.auth.external.delete): Delete external auth tokens only on the Slack side
///
/// Send it with [`SlackClient::apps_auth_external_delete`].
#[doc(alias = "apps.auth.external.delete")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AppsAuthExternalDeleteRequest {
    /// The id of the app whose tokens you want to delete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// The provider key of the provider whose tokens you want to delete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_key: Option<String>,
    /// The id of the token that you want to delete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_token_id: Option<String>,
}

impl AppsAuthExternalDeleteRequest {
    pub fn new() -> Self {
        Self {
            app_id: None,
            provider_key: None,
            external_token_id: None,
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    pub fn provider_key(mut self, provider_key: impl Into<String>) -> Self {
        self.provider_key = Some(provider_key.into());
        self
    }

    pub fn external_token_id(mut self, external_token_id: impl Into<String>) -> Self {
        self.external_token_id = Some(external_token_id.into());
        self
    }
}

impl SlackApiMethod for AppsAuthExternalDeleteRequest {
    const METHOD: &'static str = "apps.auth.external.delete";
    type Response = AppsAuthExternalDeleteResponse;
}

/// Successful response of the Slack Web API method [`apps.auth.external.delete`](https://docs.slack.dev/reference/methods/apps.auth.external.delete).
#[doc(alias = "apps.auth.external.delete")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsAuthExternalDeleteResponse {
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

/// Arguments for the Slack Web API method [`apps.auth.external.get`](https://docs.slack.dev/reference/methods/apps.auth.external.get): Get the access token for the provided token ID
///
/// Send it with [`SlackClient::apps_auth_external_get`].
#[doc(alias = "apps.auth.external.get")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsAuthExternalGetRequest {
    /// The id of the token you want to get the token for
    pub external_token_id: String,
    /// Always refresh existing token before returning even when the token has not expired
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_refresh: Option<bool>,
}

impl AppsAuthExternalGetRequest {
    pub fn new(external_token_id: impl Into<String>) -> Self {
        Self {
            external_token_id: external_token_id.into(),
            force_refresh: None,
        }
    }

    pub fn force_refresh(mut self, force_refresh: bool) -> Self {
        self.force_refresh = Some(force_refresh);
        self
    }
}

impl SlackApiMethod for AppsAuthExternalGetRequest {
    const METHOD: &'static str = "apps.auth.external.get";
    type Response = AppsAuthExternalGetResponse;
}

/// Successful response of the Slack Web API method [`apps.auth.external.get`](https://docs.slack.dev/reference/methods/apps.auth.external.get).
#[doc(alias = "apps.auth.external.get")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsAuthExternalGetResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_token: Option<String>,
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

/// Arguments for the Slack Web API method [`apps.connections.open`](https://docs.slack.dev/reference/methods/apps.connections.open): Generate a temporary Socket Mode WebSocket URL that your app can connect to in order to receive events and interactive payloads over.
///
/// Send it with [`SlackClient::apps_connections_open`].
#[doc(alias = "apps.connections.open")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AppsConnectionsOpenRequest {}

impl AppsConnectionsOpenRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl SlackApiMethod for AppsConnectionsOpenRequest {
    const METHOD: &'static str = "apps.connections.open";
    type Response = AppsConnectionsOpenResponse;
}

/// Successful response of the Slack Web API method [`apps.connections.open`](https://docs.slack.dev/reference/methods/apps.connections.open).
#[doc(alias = "apps.connections.open")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsConnectionsOpenResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
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

/// Arguments for the Slack Web API method [`apps.datastore.bulkDelete`](https://docs.slack.dev/reference/methods/apps.datastore.bulkDelete): Delete items from a datastore in bulk
///
/// Send it with [`SlackClient::apps_datastore_bulk_delete`].
#[doc(alias = "apps.datastore.bulkDelete")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsDatastoreBulkDeleteRequest {
    /// name of the datastore
    pub datastore: String,
    /// IDs of items to be deleted
    #[serde(serialize_with = "crate::form::as_json")]
    pub ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl AppsDatastoreBulkDeleteRequest {
    pub fn new(datastore: impl Into<String>, ids: Vec<String>) -> Self {
        Self {
            datastore: datastore.into(),
            ids,
            app_id: None,
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }
}

impl SlackApiMethod for AppsDatastoreBulkDeleteRequest {
    const METHOD: &'static str = "apps.datastore.bulkDelete";
    type Response = AppsDatastoreBulkDeleteResponse;
}

/// Successful response of the Slack Web API method [`apps.datastore.bulkDelete`](https://docs.slack.dev/reference/methods/apps.datastore.bulkDelete).
#[doc(alias = "apps.datastore.bulkDelete")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsDatastoreBulkDeleteResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub failed_items: Vec<serde_json::Value>,
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

/// Arguments for the Slack Web API method [`apps.datastore.bulkGet`](https://docs.slack.dev/reference/methods/apps.datastore.bulkGet): Get items from a datastore in bulk
///
/// Send it with [`SlackClient::apps_datastore_bulk_get`].
#[doc(alias = "apps.datastore.bulkGet")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsDatastoreBulkGetRequest {
    /// name of the datastore
    pub datastore: String,
    /// items' ids
    #[serde(serialize_with = "crate::form::as_json")]
    pub ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl AppsDatastoreBulkGetRequest {
    pub fn new(datastore: impl Into<String>, ids: Vec<String>) -> Self {
        Self {
            datastore: datastore.into(),
            ids,
            app_id: None,
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }
}

impl SlackApiMethod for AppsDatastoreBulkGetRequest {
    const METHOD: &'static str = "apps.datastore.bulkGet";
    type Response = AppsDatastoreBulkGetResponse;
}

/// Successful response of the Slack Web API method [`apps.datastore.bulkGet`](https://docs.slack.dev/reference/methods/apps.datastore.bulkGet).
#[doc(alias = "apps.datastore.bulkGet")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsDatastoreBulkGetResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub datastore: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<serde_json::Map<String, serde_json::Value>>,
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

/// Arguments for the Slack Web API method [`apps.datastore.bulkPut`](https://docs.slack.dev/reference/methods/apps.datastore.bulkPut): Creates or replaces existing items in bulk
///
/// Send it with [`SlackClient::apps_datastore_bulk_put`].
#[doc(alias = "apps.datastore.bulkPut")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsDatastoreBulkPutRequest {
    /// name of the datastore
    pub datastore: String,
    /// attribute names and values of the items; limit of 25
    #[serde(serialize_with = "crate::form::as_json")]
    pub items: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl AppsDatastoreBulkPutRequest {
    pub fn new(datastore: impl Into<String>, items: Vec<serde_json::Value>) -> Self {
        Self {
            datastore: datastore.into(),
            items,
            app_id: None,
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }
}

impl SlackApiMethod for AppsDatastoreBulkPutRequest {
    const METHOD: &'static str = "apps.datastore.bulkPut";
    type Response = AppsDatastoreBulkPutResponse;
}

/// Successful response of the Slack Web API method [`apps.datastore.bulkPut`](https://docs.slack.dev/reference/methods/apps.datastore.bulkPut).
#[doc(alias = "apps.datastore.bulkPut")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsDatastoreBulkPutResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub datastore: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub failed_items: Vec<serde_json::Value>,
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

/// Arguments for the Slack Web API method [`apps.datastore.count`](https://docs.slack.dev/reference/methods/apps.datastore.count): Count the number of items in a datastore that match a query
///
/// Send it with [`SlackClient::apps_datastore_count`].
#[doc(alias = "apps.datastore.count")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsDatastoreCountRequest {
    /// Name of the datastore
    pub datastore: String,
    /// A query filter expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// A map of attributes referenced in expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attributes: Option<serde_json::Value>,
    /// A map of values referenced in expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_values: Option<serde_json::Value>,
    /// Required if calling with user token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl AppsDatastoreCountRequest {
    pub fn new(datastore: impl Into<String>) -> Self {
        Self {
            datastore: datastore.into(),
            expression: None,
            expression_attributes: None,
            expression_values: None,
            app_id: None,
        }
    }

    pub fn expression(mut self, expression: impl Into<String>) -> Self {
        self.expression = Some(expression.into());
        self
    }

    pub fn expression_attributes(mut self, expression_attributes: serde_json::Value) -> Self {
        self.expression_attributes = Some(expression_attributes);
        self
    }

    pub fn expression_values(mut self, expression_values: serde_json::Value) -> Self {
        self.expression_values = Some(expression_values);
        self
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }
}

impl SlackApiMethod for AppsDatastoreCountRequest {
    const METHOD: &'static str = "apps.datastore.count";
    type Response = AppsDatastoreCountResponse;
}

/// Successful response of the Slack Web API method [`apps.datastore.count`](https://docs.slack.dev/reference/methods/apps.datastore.count).
#[doc(alias = "apps.datastore.count")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsDatastoreCountResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub datastore: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub count: Option<i64>,
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

/// Arguments for the Slack Web API method [`apps.datastore.delete`](https://docs.slack.dev/reference/methods/apps.datastore.delete): Delete an item from a datastore
///
/// Send it with [`SlackClient::apps_datastore_delete`].
#[doc(alias = "apps.datastore.delete")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsDatastoreDeleteRequest {
    /// name of the datastore
    pub datastore: String,
    /// item id
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl AppsDatastoreDeleteRequest {
    pub fn new(datastore: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            datastore: datastore.into(),
            id: id.into(),
            app_id: None,
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }
}

impl SlackApiMethod for AppsDatastoreDeleteRequest {
    const METHOD: &'static str = "apps.datastore.delete";
    type Response = AppsDatastoreDeleteResponse;
}

/// Successful response of the Slack Web API method [`apps.datastore.delete`](https://docs.slack.dev/reference/methods/apps.datastore.delete).
#[doc(alias = "apps.datastore.delete")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsDatastoreDeleteResponse {
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

/// Arguments for the Slack Web API method [`apps.datastore.get`](https://docs.slack.dev/reference/methods/apps.datastore.get): Get an item from a datastore
///
/// Send it with [`SlackClient::apps_datastore_get`].
#[doc(alias = "apps.datastore.get")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsDatastoreGetRequest {
    /// name of the datastore
    pub datastore: String,
    /// item id
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl AppsDatastoreGetRequest {
    pub fn new(datastore: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            datastore: datastore.into(),
            id: id.into(),
            app_id: None,
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }
}

impl SlackApiMethod for AppsDatastoreGetRequest {
    const METHOD: &'static str = "apps.datastore.get";
    type Response = AppsDatastoreGetResponse;
}

/// Successful response of the Slack Web API method [`apps.datastore.get`](https://docs.slack.dev/reference/methods/apps.datastore.get).
#[doc(alias = "apps.datastore.get")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsDatastoreGetResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub datastore: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub item: Option<serde_json::Map<String, serde_json::Value>>,
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

/// Arguments for the Slack Web API method [`apps.datastore.put`](https://docs.slack.dev/reference/methods/apps.datastore.put): Creates a new item, or replaces an old item with a new item.
///
/// Send it with [`SlackClient::apps_datastore_put`].
#[doc(alias = "apps.datastore.put")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsDatastorePutRequest {
    /// name of the datastore
    pub datastore: String,
    /// attribute names and values of the item
    pub item: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl AppsDatastorePutRequest {
    pub fn new(datastore: impl Into<String>, item: serde_json::Value) -> Self {
        Self {
            datastore: datastore.into(),
            item,
            app_id: None,
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }
}

impl SlackApiMethod for AppsDatastorePutRequest {
    const METHOD: &'static str = "apps.datastore.put";
    type Response = AppsDatastorePutResponse;
}

/// Successful response of the Slack Web API method [`apps.datastore.put`](https://docs.slack.dev/reference/methods/apps.datastore.put).
#[doc(alias = "apps.datastore.put")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsDatastorePutResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub datastore: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub item: Option<serde_json::Map<String, serde_json::Value>>,
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

/// Arguments for the Slack Web API method [`apps.datastore.query`](https://docs.slack.dev/reference/methods/apps.datastore.query): Query a datastore for items
///
/// Send it with [`SlackClient::apps_datastore_query`].
#[doc(alias = "apps.datastore.query")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsDatastoreQueryRequest {
    /// Name of the datastore
    pub datastore: String,
    /// A query filter expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// A map of attributes referenced in expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attributes: Option<serde_json::Value>,
    /// A map of values referenced in expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_values: Option<serde_json::Value>,
    /// Required if calling with user token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The maximum number of items to evaluate for a given request (not necessarily the number of matching items). If the given request dataset size exceeds 1 MB before reaching the limit, the returned item count will likely be less than the limit. In any case where there are more items available beyond an imposed limit, a `next_cursor` value will be provided for use in subsequent requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl AppsDatastoreQueryRequest {
    pub fn new(datastore: impl Into<String>) -> Self {
        Self {
            datastore: datastore.into(),
            expression: None,
            expression_attributes: None,
            expression_values: None,
            app_id: None,
            cursor: None,
            limit: None,
        }
    }

    pub fn expression(mut self, expression: impl Into<String>) -> Self {
        self.expression = Some(expression.into());
        self
    }

    pub fn expression_attributes(mut self, expression_attributes: serde_json::Value) -> Self {
        self.expression_attributes = Some(expression_attributes);
        self
    }

    pub fn expression_values(mut self, expression_values: serde_json::Value) -> Self {
        self.expression_values = Some(expression_values);
        self
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl SlackApiMethod for AppsDatastoreQueryRequest {
    const METHOD: &'static str = "apps.datastore.query";
    type Response = AppsDatastoreQueryResponse;
}

impl CursorPaginated for AppsDatastoreQueryRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AppsDatastoreQueryResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`apps.datastore.query`](https://docs.slack.dev/reference/methods/apps.datastore.query).
#[doc(alias = "apps.datastore.query")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsDatastoreQueryResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub datastore: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<serde_json::Map<String, serde_json::Value>>,
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

/// Arguments for the Slack Web API method [`apps.datastore.update`](https://docs.slack.dev/reference/methods/apps.datastore.update): Edits an existing item's attributes, or adds a new item if it does not already exist.
///
/// Send it with [`SlackClient::apps_datastore_update`].
#[doc(alias = "apps.datastore.update")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsDatastoreUpdateRequest {
    /// name of the datastore
    pub datastore: String,
    /// attribute names and values to be updated
    pub item: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl AppsDatastoreUpdateRequest {
    pub fn new(datastore: impl Into<String>, item: serde_json::Value) -> Self {
        Self {
            datastore: datastore.into(),
            item,
            app_id: None,
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }
}

impl SlackApiMethod for AppsDatastoreUpdateRequest {
    const METHOD: &'static str = "apps.datastore.update";
    type Response = AppsDatastoreUpdateResponse;
}

/// Successful response of the Slack Web API method [`apps.datastore.update`](https://docs.slack.dev/reference/methods/apps.datastore.update).
#[doc(alias = "apps.datastore.update")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsDatastoreUpdateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub datastore: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub item: Option<serde_json::Map<String, serde_json::Value>>,
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

/// Arguments for the Slack Web API method [`apps.event.authorizations.list`](https://docs.slack.dev/reference/methods/apps.event.authorizations.list): Get a list of authorizations for the given event context. Each authorization represents an app installation that the event is visible to.
///
/// Send it with [`SlackClient::apps_event_authorizations_list`].
#[doc(alias = "apps.event.authorizations.list")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsEventAuthorizationsListRequest {
    pub event_context: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl AppsEventAuthorizationsListRequest {
    pub fn new(event_context: impl Into<String>) -> Self {
        Self {
            event_context: event_context.into(),
            cursor: None,
            limit: None,
        }
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl SlackApiMethod for AppsEventAuthorizationsListRequest {
    const METHOD: &'static str = "apps.event.authorizations.list";
    type Response = AppsEventAuthorizationsListResponse;
}

impl CursorPaginated for AppsEventAuthorizationsListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AppsEventAuthorizationsListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`apps.event.authorizations.list`](https://docs.slack.dev/reference/methods/apps.event.authorizations.list).
#[doc(alias = "apps.event.authorizations.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsEventAuthorizationsListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub authorizations: Vec<AppsEventAuthorizationsListResponseAuthorizations>,
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
pub struct AppsEventAuthorizationsListResponseAuthorizations {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub enterprise_id: Option<String>,
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
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_bot: Option<String>,
}

/// Arguments for the Slack Web API method [`apps.icon.set`](https://docs.slack.dev/reference/methods/apps.icon.set): Sets the app icon
///
/// Send it with [`SlackClient::apps_icon_set`].
#[doc(alias = "apps.icon.set")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsIconSetRequest {
    /// The ID of the app whose icon you want to set.
    pub app_id: String,
    /// File contents via `multipart/form-data`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// URL of a publicly hosted image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl AppsIconSetRequest {
    pub fn new(app_id: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            file: None,
            url: None,
        }
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

impl SlackApiMethod for AppsIconSetRequest {
    const METHOD: &'static str = "apps.icon.set";
    type Response = AppsIconSetResponse;
}

/// Successful response of the Slack Web API method [`apps.icon.set`](https://docs.slack.dev/reference/methods/apps.icon.set).
#[doc(alias = "apps.icon.set")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsIconSetResponse {
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

/// Arguments for the Slack Web API method [`apps.managed.permissions.set`](https://docs.slack.dev/reference/methods/apps.managed.permissions.set): Set who can interact with a managed app
///
/// Send it with [`SlackClient::apps_managed_permissions_set`].
#[doc(alias = "apps.managed.permissions.set")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsManagedPermissionsSetRequest {
    /// Encoded ID of the managed app to configure
    pub app_id: String,
    /// Who can interact with the app. Use everyone to allow all members, or app\_owner to restrict access to the app's owner.
    pub permissions: String,
}

impl AppsManagedPermissionsSetRequest {
    pub fn new(app_id: impl Into<String>, permissions: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            permissions: permissions.into(),
        }
    }
}

impl SlackApiMethod for AppsManagedPermissionsSetRequest {
    const METHOD: &'static str = "apps.managed.permissions.set";
    type Response = AppsManagedPermissionsSetResponse;
}

/// Successful response of the Slack Web API method [`apps.managed.permissions.set`](https://docs.slack.dev/reference/methods/apps.managed.permissions.set).
#[doc(alias = "apps.managed.permissions.set")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManagedPermissionsSetResponse {
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

/// Arguments for the Slack Web API method [`apps.manifest.create`](https://docs.slack.dev/reference/methods/apps.manifest.create): Create an app from an app manifest.
///
/// Send it with [`SlackClient::apps_manifest_create`].
#[doc(alias = "apps.manifest.create")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsManifestCreateRequest {
    /// A JSON app manifest encoded as a string. This manifest **must** use a valid [app manifest schema - read our guide to creating one](https://docs.slack.dev/app-manifests/configuring-apps-with-app-manifests.md#fields).
    pub manifest: String,
    /// When called with an org token, which specific team to create app on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl AppsManifestCreateRequest {
    pub fn new(manifest: impl Into<String>) -> Self {
        Self {
            manifest: manifest.into(),
            team_id: None,
        }
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }
}

impl SlackApiMethod for AppsManifestCreateRequest {
    const METHOD: &'static str = "apps.manifest.create";
    type Response = AppsManifestCreateResponse;
}

/// Successful response of the Slack Web API method [`apps.manifest.create`](https://docs.slack.dev/reference/methods/apps.manifest.create).
#[doc(alias = "apps.manifest.create")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManifestCreateResponse {
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
    pub credentials: Option<AppsManifestCreateResponseCredentials>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub oauth_authorize_url: Option<String>,
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
pub struct AppsManifestCreateResponseCredentials {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_secret: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_token: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub signing_secret: Option<String>,
}

/// Arguments for the Slack Web API method [`apps.manifest.delete`](https://docs.slack.dev/reference/methods/apps.manifest.delete): Permanently deletes an app created through app manifests
///
/// Send it with [`SlackClient::apps_manifest_delete`].
#[doc(alias = "apps.manifest.delete")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsManifestDeleteRequest {
    /// The ID of the app you want to delete.
    pub app_id: String,
}

impl AppsManifestDeleteRequest {
    pub fn new(app_id: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
        }
    }
}

impl SlackApiMethod for AppsManifestDeleteRequest {
    const METHOD: &'static str = "apps.manifest.delete";
    type Response = AppsManifestDeleteResponse;
}

/// Successful response of the Slack Web API method [`apps.manifest.delete`](https://docs.slack.dev/reference/methods/apps.manifest.delete).
#[doc(alias = "apps.manifest.delete")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManifestDeleteResponse {
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

/// Arguments for the Slack Web API method [`apps.manifest.export`](https://docs.slack.dev/reference/methods/apps.manifest.export): Export an app manifest from an existing app
///
/// Send it with [`SlackClient::apps_manifest_export`].
#[doc(alias = "apps.manifest.export")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsManifestExportRequest {
    /// The ID of the app whose configuration you want to export as a manifest.
    pub app_id: String,
}

impl AppsManifestExportRequest {
    pub fn new(app_id: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
        }
    }
}

impl SlackApiMethod for AppsManifestExportRequest {
    const METHOD: &'static str = "apps.manifest.export";
    type Response = AppsManifestExportResponse;
}

/// Successful response of the Slack Web API method [`apps.manifest.export`](https://docs.slack.dev/reference/methods/apps.manifest.export).
#[doc(alias = "apps.manifest.export")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManifestExportResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub manifest: Option<AppsManifestExportResponseManifest>,
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
pub struct AppsManifestExportResponseManifest {
    #[serde(
        rename = "_metadata",
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata: Option<AppsManifestExportResponseManifestMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_information: Option<AppsManifestExportResponseManifestDisplayInformation>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub features: Option<AppsManifestExportResponseManifestFeatures>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub oauth_config: Option<AppsManifestExportResponseManifestOauthConfig>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub settings: Option<AppsManifestExportResponseManifestSettings>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManifestExportResponseManifestMetadata {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub major_version: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub minor_version: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManifestExportResponseManifestDisplayInformation {
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
    pub description: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub background_color: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub long_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManifestExportResponseManifestFeatures {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_home: Option<AppsManifestExportResponseManifestFeaturesAppHome>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub bot_user: Option<AppsManifestExportResponseManifestFeaturesBotUser>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub slash_commands: Vec<AppsManifestExportResponseManifestFeaturesSlashCommands>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub workflow_steps: Vec<AppsManifestExportResponseManifestFeaturesWorkflowSteps>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManifestExportResponseManifestFeaturesAppHome {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub home_tab_enabled: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub messages_tab_enabled: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub messages_tab_read_only_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManifestExportResponseManifestFeaturesBotUser {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub always_online: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManifestExportResponseManifestFeaturesSlashCommands {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub command: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub usage_hint: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub should_escape: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManifestExportResponseManifestFeaturesWorkflowSteps {
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
    pub callback_id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManifestExportResponseManifestOauthConfig {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub redirect_urls: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub scopes: Option<AppsManifestExportResponseManifestOauthConfigScopes>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManifestExportResponseManifestOauthConfigScopes {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bot: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManifestExportResponseManifestSettings {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub event_subscriptions: Option<AppsManifestExportResponseManifestSettingsEventSubscriptions>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub interactivity: Option<AppsManifestExportResponseManifestSettingsInteractivity>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub org_deploy_enabled: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub socket_mode_enabled: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_hosted: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub token_rotation_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManifestExportResponseManifestSettingsEventSubscriptions {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bot_events: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManifestExportResponseManifestSettingsInteractivity {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_enabled: Option<bool>,
}

/// Arguments for the Slack Web API method [`apps.manifest.update`](https://docs.slack.dev/reference/methods/apps.manifest.update): Update an app from an app manifest
///
/// Send it with [`SlackClient::apps_manifest_update`].
#[doc(alias = "apps.manifest.update")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsManifestUpdateRequest {
    /// A JSON app manifest encoded as a string. This manifest **must** use a valid [app manifest schema - read our guide to creating one](https://docs.slack.dev/app-manifests/configuring-apps-with-app-manifests.md#fields). As this method entirely _replaces_ any previous configuration, manifest must contain both unmodified and modified fields.
    pub manifest: String,
    /// The ID of the app whose configuration you want to update.
    pub app_id: String,
}

impl AppsManifestUpdateRequest {
    pub fn new(manifest: impl Into<String>, app_id: impl Into<String>) -> Self {
        Self {
            manifest: manifest.into(),
            app_id: app_id.into(),
        }
    }
}

impl SlackApiMethod for AppsManifestUpdateRequest {
    const METHOD: &'static str = "apps.manifest.update";
    type Response = AppsManifestUpdateResponse;
}

/// Successful response of the Slack Web API method [`apps.manifest.update`](https://docs.slack.dev/reference/methods/apps.manifest.update).
#[doc(alias = "apps.manifest.update")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManifestUpdateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub permissions_updated: Option<bool>,
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

/// Arguments for the Slack Web API method [`apps.manifest.validate`](https://docs.slack.dev/reference/methods/apps.manifest.validate): Validate an app manifest
///
/// Send it with [`SlackClient::apps_manifest_validate`].
#[doc(alias = "apps.manifest.validate")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsManifestValidateRequest {
    /// The manifest to be validated. Will be validated against the [app manifest schema - read our guide](https://docs.slack.dev/app-manifests/configuring-apps-with-app-manifests.md#fields).
    pub manifest: String,
    /// The ID of the app whose configuration you want to validate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl AppsManifestValidateRequest {
    pub fn new(manifest: impl Into<String>) -> Self {
        Self {
            manifest: manifest.into(),
            app_id: None,
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }
}

impl SlackApiMethod for AppsManifestValidateRequest {
    const METHOD: &'static str = "apps.manifest.validate";
    type Response = AppsManifestValidateResponse;
}

/// Successful response of the Slack Web API method [`apps.manifest.validate`](https://docs.slack.dev/reference/methods/apps.manifest.validate).
#[doc(alias = "apps.manifest.validate")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsManifestValidateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<serde_json::Value>,
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

/// Arguments for the Slack Web API method [`apps.uninstall`](https://docs.slack.dev/reference/methods/apps.uninstall): Uninstalls your app from a workspace.
///
/// Send it with [`SlackClient::apps_uninstall`].
#[doc(alias = "apps.uninstall")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsUninstallRequest {
    /// Issued when you created your application.
    pub client_id: String,
    /// Issued when you created your application.
    pub client_secret: String,
}

impl AppsUninstallRequest {
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }
}

impl SlackApiMethod for AppsUninstallRequest {
    const METHOD: &'static str = "apps.uninstall";
    type Response = AppsUninstallResponse;
}

/// Successful response of the Slack Web API method [`apps.uninstall`](https://docs.slack.dev/reference/methods/apps.uninstall).
#[doc(alias = "apps.uninstall")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsUninstallResponse {
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

/// Arguments for the Slack Web API method [`apps.user.connection.update`](https://docs.slack.dev/reference/methods/apps.user.connection.update): Updates the connection status between a user and an app.
///
/// Send it with [`SlackClient::apps_user_connection_update`].
#[doc(alias = "apps.user.connection.update")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AppsUserConnectionUpdateRequest {
    /// The ID of the user for the status update.
    pub user_id: String,
    /// The status that should be set for the user.
    pub status: String,
}

impl AppsUserConnectionUpdateRequest {
    pub fn new(user_id: impl Into<String>, status: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            status: status.into(),
        }
    }
}

impl SlackApiMethod for AppsUserConnectionUpdateRequest {
    const METHOD: &'static str = "apps.user.connection.update";
    type Response = AppsUserConnectionUpdateResponse;
}

/// Successful response of the Slack Web API method [`apps.user.connection.update`](https://docs.slack.dev/reference/methods/apps.user.connection.update).
#[doc(alias = "apps.user.connection.update")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppsUserConnectionUpdateResponse {
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
    /// Calls the Slack Web API method [`apps.activities.list`](https://docs.slack.dev/reference/methods/apps.activities.list): Get logs for a specified app
    ///
    /// Required scopes:
    ///
    /// - user token: `hosting:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.activities.list")]
    pub async fn apps_activities_list(
        &self,
        request: &AppsActivitiesListRequest,
    ) -> Result<AppsActivitiesListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.auth.external.delete`](https://docs.slack.dev/reference/methods/apps.auth.external.delete): Delete external auth tokens only on the Slack side
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.auth.external.delete")]
    pub async fn apps_auth_external_delete(
        &self,
        request: &AppsAuthExternalDeleteRequest,
    ) -> Result<AppsAuthExternalDeleteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.auth.external.get`](https://docs.slack.dev/reference/methods/apps.auth.external.get): Get the access token for the provided token ID
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.auth.external.get")]
    pub async fn apps_auth_external_get(
        &self,
        request: &AppsAuthExternalGetRequest,
    ) -> Result<AppsAuthExternalGetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.connections.open`](https://docs.slack.dev/reference/methods/apps.connections.open): Generate a temporary Socket Mode WebSocket URL that your app can connect to in order to receive events and interactive payloads over.
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.connections.open")]
    pub async fn apps_connections_open(
        &self,
        request: &AppsConnectionsOpenRequest,
    ) -> Result<AppsConnectionsOpenResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.datastore.bulkDelete`](https://docs.slack.dev/reference/methods/apps.datastore.bulkDelete): Delete items from a datastore in bulk
    ///
    /// Required scopes:
    ///
    /// - bot token: `datastore:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.datastore.bulkDelete")]
    pub async fn apps_datastore_bulk_delete(
        &self,
        request: &AppsDatastoreBulkDeleteRequest,
    ) -> Result<AppsDatastoreBulkDeleteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.datastore.bulkGet`](https://docs.slack.dev/reference/methods/apps.datastore.bulkGet): Get items from a datastore in bulk
    ///
    /// Required scopes:
    ///
    /// - bot token: `datastore:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.datastore.bulkGet")]
    pub async fn apps_datastore_bulk_get(
        &self,
        request: &AppsDatastoreBulkGetRequest,
    ) -> Result<AppsDatastoreBulkGetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.datastore.bulkPut`](https://docs.slack.dev/reference/methods/apps.datastore.bulkPut): Creates or replaces existing items in bulk
    ///
    /// Required scopes:
    ///
    /// - bot token: `datastore:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.datastore.bulkPut")]
    pub async fn apps_datastore_bulk_put(
        &self,
        request: &AppsDatastoreBulkPutRequest,
    ) -> Result<AppsDatastoreBulkPutResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.datastore.count`](https://docs.slack.dev/reference/methods/apps.datastore.count): Count the number of items in a datastore that match a query
    ///
    /// Required scopes:
    ///
    /// - bot token: `datastore:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.datastore.count")]
    pub async fn apps_datastore_count(
        &self,
        request: &AppsDatastoreCountRequest,
    ) -> Result<AppsDatastoreCountResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.datastore.delete`](https://docs.slack.dev/reference/methods/apps.datastore.delete): Delete an item from a datastore
    ///
    /// Required scopes:
    ///
    /// - bot token: `datastore:write`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.datastore.delete")]
    pub async fn apps_datastore_delete(
        &self,
        request: &AppsDatastoreDeleteRequest,
    ) -> Result<AppsDatastoreDeleteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.datastore.get`](https://docs.slack.dev/reference/methods/apps.datastore.get): Get an item from a datastore
    ///
    /// Required scopes:
    ///
    /// - bot token: `datastore:read`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.datastore.get")]
    pub async fn apps_datastore_get(
        &self,
        request: &AppsDatastoreGetRequest,
    ) -> Result<AppsDatastoreGetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.datastore.put`](https://docs.slack.dev/reference/methods/apps.datastore.put): Creates a new item, or replaces an old item with a new item.
    ///
    /// Required scopes:
    ///
    /// - bot token: `datastore:write`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.datastore.put")]
    pub async fn apps_datastore_put(
        &self,
        request: &AppsDatastorePutRequest,
    ) -> Result<AppsDatastorePutResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.datastore.query`](https://docs.slack.dev/reference/methods/apps.datastore.query): Query a datastore for items
    ///
    /// Required scopes:
    ///
    /// - bot token: `datastore:read`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.datastore.query")]
    pub async fn apps_datastore_query(
        &self,
        request: &AppsDatastoreQueryRequest,
    ) -> Result<AppsDatastoreQueryResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.datastore.update`](https://docs.slack.dev/reference/methods/apps.datastore.update): Edits an existing item's attributes, or adds a new item if it does not already exist.
    ///
    /// Required scopes:
    ///
    /// - bot token: `datastore:write`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.datastore.update")]
    pub async fn apps_datastore_update(
        &self,
        request: &AppsDatastoreUpdateRequest,
    ) -> Result<AppsDatastoreUpdateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.event.authorizations.list`](https://docs.slack.dev/reference/methods/apps.event.authorizations.list): Get a list of authorizations for the given event context. Each authorization represents an app installation that the event is visible to.
    ///
    /// Rate limit: Tier 5 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.event.authorizations.list")]
    pub async fn apps_event_authorizations_list(
        &self,
        request: &AppsEventAuthorizationsListRequest,
    ) -> Result<AppsEventAuthorizationsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.icon.set`](https://docs.slack.dev/reference/methods/apps.icon.set): Sets the app icon
    ///
    /// Required scopes:
    ///
    /// - user token: `app_configurations:write`
    ///
    /// Rate limit: Tier 1 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.icon.set")]
    pub async fn apps_icon_set(
        &self,
        request: &AppsIconSetRequest,
    ) -> Result<AppsIconSetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.managed.permissions.set`](https://docs.slack.dev/reference/methods/apps.managed.permissions.set): Set who can interact with a managed app
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.managed.permissions.set")]
    pub async fn apps_managed_permissions_set(
        &self,
        request: &AppsManagedPermissionsSetRequest,
    ) -> Result<AppsManagedPermissionsSetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.manifest.create`](https://docs.slack.dev/reference/methods/apps.manifest.create): Create an app from an app manifest.
    ///
    /// Rate limit: Tier 1 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.manifest.create")]
    pub async fn apps_manifest_create(
        &self,
        request: &AppsManifestCreateRequest,
    ) -> Result<AppsManifestCreateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.manifest.delete`](https://docs.slack.dev/reference/methods/apps.manifest.delete): Permanently deletes an app created through app manifests
    ///
    /// Rate limit: Tier 1 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.manifest.delete")]
    pub async fn apps_manifest_delete(
        &self,
        request: &AppsManifestDeleteRequest,
    ) -> Result<AppsManifestDeleteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.manifest.export`](https://docs.slack.dev/reference/methods/apps.manifest.export): Export an app manifest from an existing app
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.manifest.export")]
    pub async fn apps_manifest_export(
        &self,
        request: &AppsManifestExportRequest,
    ) -> Result<AppsManifestExportResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.manifest.update`](https://docs.slack.dev/reference/methods/apps.manifest.update): Update an app from an app manifest
    ///
    /// Rate limit: Tier 1 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.manifest.update")]
    pub async fn apps_manifest_update(
        &self,
        request: &AppsManifestUpdateRequest,
    ) -> Result<AppsManifestUpdateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.manifest.validate`](https://docs.slack.dev/reference/methods/apps.manifest.validate): Validate an app manifest
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.manifest.validate")]
    pub async fn apps_manifest_validate(
        &self,
        request: &AppsManifestValidateRequest,
    ) -> Result<AppsManifestValidateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.uninstall`](https://docs.slack.dev/reference/methods/apps.uninstall): Uninstalls your app from a workspace.
    ///
    /// Rate limit: Tier 1 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.uninstall")]
    pub async fn apps_uninstall(
        &self,
        request: &AppsUninstallRequest,
    ) -> Result<AppsUninstallResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`apps.user.connection.update`](https://docs.slack.dev/reference/methods/apps.user.connection.update): Updates the connection status between a user and an app.
    ///
    /// Required scopes:
    ///
    /// - user token: `users:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "apps.user.connection.update")]
    pub async fn apps_user_connection_update(
        &self,
        request: &AppsUserConnectionUpdateRequest,
    ) -> Result<AppsUserConnectionUpdateResponse, SlackError> {
        self.call(request).await
    }
}
