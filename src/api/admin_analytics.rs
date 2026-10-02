// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`admin.analytics.getFile`](https://docs.slack.dev/reference/methods/admin.analytics.getFile): Retrieve analytics data for a given date, presented as a compressed JSON file
///
/// Send it with [`SlackClient::admin_analytics_get_file`].
#[doc(alias = "admin.analytics.getFile")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminAnalyticsGetFileRequest {
    /// The type of analytics to retrieve. The options are currently limited to `member` (for Enterprise org member analytics) and `public_channel` (for public channel analytics).
    pub r#type: String,
    /// Date to retrieve the analytics data for, expressed as `YYYY-MM-DD` in UTC. Required unless `metadata_only` is set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Retrieve metadata for the `type` of analytics indicated. Can be used only with `type` set to `public_channel` analytics. See [detail below](#metadata_only). Omit the `date` parameter when using this argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_only: Option<bool>,
}

impl AdminAnalyticsGetFileRequest {
    pub fn new(r#type: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            date: None,
            metadata_only: None,
        }
    }

    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }

    pub fn metadata_only(mut self, metadata_only: bool) -> Self {
        self.metadata_only = Some(metadata_only);
        self
    }
}

/// Arguments for the Slack Web API method [`admin.analytics.messages.activity`](https://docs.slack.dev/reference/methods/admin.analytics.messages.activity): Retrieves activity metrics for messages from a given channel.
///
/// Send it with [`SlackClient::admin_analytics_messages_activity`].
#[doc(alias = "admin.analytics.messages.activity")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminAnalyticsMessagesActivityRequest {
    /// Channel ID for channel of the message activity to query.
    pub channel: String,
    /// Oldest timestamp to include in the results. Defaults to 7 days before current time. If not passed while still passing the `latest_ts` parameter, defaults to 7 days before `latest_ts`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_ts: Option<String>,
    /// Most recent timestamp to include in results. Defaults to current time. If not passed while still passing the `oldest_ts` parameter, defaults to 7 days after `oldest_ts`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_ts: Option<String>,
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Maximum number of entries to return. Defaults to 50 if not passed. Max allowed is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl AdminAnalyticsMessagesActivityRequest {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            oldest_ts: None,
            latest_ts: None,
            cursor: None,
            limit: None,
        }
    }

    pub fn oldest_ts(mut self, oldest_ts: impl Into<String>) -> Self {
        self.oldest_ts = Some(oldest_ts.into());
        self
    }

    pub fn latest_ts(mut self, latest_ts: impl Into<String>) -> Self {
        self.latest_ts = Some(latest_ts.into());
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

impl SlackApiMethod for AdminAnalyticsMessagesActivityRequest {
    const METHOD: &'static str = "admin.analytics.messages.activity";
    type Response = AdminAnalyticsMessagesActivityResponse;
}

impl CursorPaginated for AdminAnalyticsMessagesActivityRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminAnalyticsMessagesActivityResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.analytics.messages.activity`](https://docs.slack.dev/reference/methods/admin.analytics.messages.activity).
#[doc(alias = "admin.analytics.messages.activity")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAnalyticsMessagesActivityResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub message_activities: Vec<AdminAnalyticsMessagesActivityResponseMessageActivities>,
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
pub struct AdminAnalyticsMessagesActivityResponseMessageActivities {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub timestamp: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_user_reactions_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_user_shares_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_user_views_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_user_clicks_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_views_client:
        Option<AdminAnalyticsMessagesActivityResponseMessageActivitiesUniqueViewsClient>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub unique_stats_by_department:
        Vec<AdminAnalyticsMessagesActivityResponseMessageActivitiesUniqueStatsByDepartment>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub unique_stats_by_org:
        Vec<AdminAnalyticsMessagesActivityResponseMessageActivitiesUniqueStatsByOrg>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAnalyticsMessagesActivityResponseMessageActivitiesUniqueViewsClient {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub desktop_views_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub mobile_views_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub web_views_count: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAnalyticsMessagesActivityResponseMessageActivitiesUniqueStatsByDepartment {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub department: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub views: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub reactions: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub shares: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub clicks: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAnalyticsMessagesActivityResponseMessageActivitiesUniqueStatsByOrg {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub team_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub views: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub reactions: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub shares: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub clicks: Option<i64>,
}

/// Arguments for the Slack Web API method [`admin.analytics.messages.metadata`](https://docs.slack.dev/reference/methods/admin.analytics.messages.metadata): Retrieves metadata for a list of messages from a given channel.
///
/// Send it with [`SlackClient::admin_analytics_messages_metadata`].
#[doc(alias = "admin.analytics.messages.metadata")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AdminAnalyticsMessagesMetadataRequest {
    /// Channel ID for channel containing the messages to query.
    pub channel: String,
    /// Oldest timestamp to include in the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_ts: Option<String>,
    /// Most recent timestamp to include in the results. If not passed, defaults to current time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_ts: Option<String>,
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](https://docs.slack.dev/apis/web-api/pagination.md) for more detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AdminAnalyticsMessagesMetadataRequest {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            oldest_ts: None,
            latest_ts: None,
            cursor: None,
        }
    }

    pub fn oldest_ts(mut self, oldest_ts: impl Into<String>) -> Self {
        self.oldest_ts = Some(oldest_ts.into());
        self
    }

    pub fn latest_ts(mut self, latest_ts: impl Into<String>) -> Self {
        self.latest_ts = Some(latest_ts.into());
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

impl SlackApiMethod for AdminAnalyticsMessagesMetadataRequest {
    const METHOD: &'static str = "admin.analytics.messages.metadata";
    type Response = AdminAnalyticsMessagesMetadataResponse;
}

impl CursorPaginated for AdminAnalyticsMessagesMetadataRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for AdminAnalyticsMessagesMetadataResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`admin.analytics.messages.metadata`](https://docs.slack.dev/reference/methods/admin.analytics.messages.metadata).
#[doc(alias = "admin.analytics.messages.metadata")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminAnalyticsMessagesMetadataResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub messages: Vec<Message>,
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
    /// Calls the Slack Web API method [`admin.analytics.getFile`](https://docs.slack.dev/reference/methods/admin.analytics.getFile): Retrieve analytics data for a given date, presented as a compressed JSON file
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.analytics:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// Returns the raw file body (a gzip-compressed JSON Lines file), not JSON.
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.analytics.getFile")]
    pub async fn admin_analytics_get_file(
        &self,
        request: &AdminAnalyticsGetFileRequest,
    ) -> Result<bytes::Bytes, SlackError> {
        self.call_bytes("admin.analytics.getFile", request).await
    }

    /// Calls the Slack Web API method [`admin.analytics.messages.activity`](https://docs.slack.dev/reference/methods/admin.analytics.messages.activity): Retrieves activity metrics for messages from a given channel.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.analytics:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.analytics.messages.activity")]
    pub async fn admin_analytics_messages_activity(
        &self,
        request: &AdminAnalyticsMessagesActivityRequest,
    ) -> Result<AdminAnalyticsMessagesActivityResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`admin.analytics.messages.metadata`](https://docs.slack.dev/reference/methods/admin.analytics.messages.metadata): Retrieves metadata for a list of messages from a given channel.
    ///
    /// Required scopes:
    ///
    /// - user token: `admin.analytics:read`
    ///
    /// Rate limit: 1200 requests per minute with a burst allowance of 2000.
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "admin.analytics.messages.metadata")]
    pub async fn admin_analytics_messages_metadata(
        &self,
        request: &AdminAnalyticsMessagesMetadataRequest,
    ) -> Result<AdminAnalyticsMessagesMetadataResponse, SlackError> {
        self.call(request).await
    }
}
