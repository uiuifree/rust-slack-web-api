// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`slackLists.access.delete`](https://docs.slack.dev/reference/methods/slackLists.access.delete): Revoke access to a List for specified entities.
///
/// Send it with [`SlackClient::slack_lists_access_delete`].
#[doc(alias = "slackLists.access.delete")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SlackListsAccessDeleteRequest {
    /// Encoded ID of the List.
    pub list_id: String,
    /// List of channels you wish to update access for. Can only be used if `user_ids` is not provided.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub channel_ids: Option<Vec<String>>,
    /// List of users you wish to update access for. Can only be used if `channel_ids` is not provided.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub user_ids: Option<Vec<String>>,
}

impl SlackListsAccessDeleteRequest {
    pub fn new(list_id: impl Into<String>) -> Self {
        Self {
            list_id: list_id.into(),
            channel_ids: None,
            user_ids: None,
        }
    }

    pub fn channel_ids(mut self, channel_ids: Vec<String>) -> Self {
        self.channel_ids = Some(channel_ids);
        self
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }
}

impl SlackApiMethod for SlackListsAccessDeleteRequest {
    const METHOD: &'static str = "slackLists.access.delete";
    type Response = SlackListsAccessDeleteResponse;
}

/// Successful response of the Slack Web API method [`slackLists.access.delete`](https://docs.slack.dev/reference/methods/slackLists.access.delete).
#[doc(alias = "slackLists.access.delete")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsAccessDeleteResponse {
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

/// Arguments for the Slack Web API method [`slackLists.access.set`](https://docs.slack.dev/reference/methods/slackLists.access.set): Set the access level to a List for specified entities.
///
/// Send it with [`SlackClient::slack_lists_access_set`].
#[doc(alias = "slackLists.access.set")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SlackListsAccessSetRequest {
    /// Encoded ID of the List.
    pub list_id: String,
    /// Desired level of access.
    pub access_level: String,
    /// List of channels you wish to update access for. Can only be used if `user_ids` is not provided.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub channel_ids: Option<Vec<String>>,
    /// List of users you wish to update access for. Can only be used if `channel_ids` is not provided.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub user_ids: Option<Vec<String>>,
}

impl SlackListsAccessSetRequest {
    pub fn new(list_id: impl Into<String>, access_level: impl Into<String>) -> Self {
        Self {
            list_id: list_id.into(),
            access_level: access_level.into(),
            channel_ids: None,
            user_ids: None,
        }
    }

    pub fn channel_ids(mut self, channel_ids: Vec<String>) -> Self {
        self.channel_ids = Some(channel_ids);
        self
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }
}

impl SlackApiMethod for SlackListsAccessSetRequest {
    const METHOD: &'static str = "slackLists.access.set";
    type Response = SlackListsAccessSetResponse;
}

/// Successful response of the Slack Web API method [`slackLists.access.set`](https://docs.slack.dev/reference/methods/slackLists.access.set).
#[doc(alias = "slackLists.access.set")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsAccessSetResponse {
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

/// Arguments for the Slack Web API method [`slackLists.create`](https://docs.slack.dev/reference/methods/slackLists.create): Create a List.
///
/// Send it with [`SlackClient::slack_lists_create`].
#[doc(alias = "slackLists.create")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SlackListsCreateRequest {
    /// Name of the List.
    pub name: String,
    /// A rich text description of the List.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub description_blocks: Option<Vec<crate::blocks::Block>>,
    /// Column definition for the List.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub schema: Option<Vec<serde_json::Value>>,
    /// ID of the List to copy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_from_list_id: Option<String>,
    /// Boolean indicating whether to include records when a List is copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_copied_list_records: Option<bool>,
    /// Boolean indicating whether the List should be used to track todo tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub todo_mode: Option<bool>,
}

impl SlackListsCreateRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description_blocks: None,
            schema: None,
            copy_from_list_id: None,
            include_copied_list_records: None,
            todo_mode: None,
        }
    }

    pub fn description_blocks(mut self, description_blocks: Vec<crate::blocks::Block>) -> Self {
        self.description_blocks = Some(description_blocks);
        self
    }

    pub fn schema(mut self, schema: Vec<serde_json::Value>) -> Self {
        self.schema = Some(schema);
        self
    }

    pub fn copy_from_list_id(mut self, copy_from_list_id: impl Into<String>) -> Self {
        self.copy_from_list_id = Some(copy_from_list_id.into());
        self
    }

    pub fn include_copied_list_records(mut self, include_copied_list_records: bool) -> Self {
        self.include_copied_list_records = Some(include_copied_list_records);
        self
    }

    pub fn todo_mode(mut self, todo_mode: bool) -> Self {
        self.todo_mode = Some(todo_mode);
        self
    }
}

impl SlackApiMethod for SlackListsCreateRequest {
    const METHOD: &'static str = "slackLists.create";
    type Response = SlackListsCreateResponse;
}

/// Successful response of the Slack Web API method [`slackLists.create`](https://docs.slack.dev/reference/methods/slackLists.create).
#[doc(alias = "slackLists.create")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsCreateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_metadata: Option<SlackListsCreateResponseListMetadata>,
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
pub struct SlackListsCreateResponseListMetadata {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub schema: Vec<SlackListsCreateResponseListMetadataSchema>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subtask_schema: Vec<SlackListsCreateResponseListMetadataSubtaskSchema>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsCreateResponseListMetadataSchema {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub key: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_primary_column: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub options: Option<SlackListsCreateResponseListMetadataSchemaOptions>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsCreateResponseListMetadataSchemaOptions {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub precision: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_member_name: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub choices: Vec<SlackListsCreateResponseListMetadataSchemaOptionsChoices>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub format: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub emoji: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub max: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsCreateResponseListMetadataSchemaOptionsChoices {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub label: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsCreateResponseListMetadataSubtaskSchema {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub key: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_primary_column: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub options: Option<SlackListsCreateResponseListMetadataSubtaskSchemaOptions>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsCreateResponseListMetadataSubtaskSchemaOptions {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_member_name: Option<bool>,
}

/// Arguments for the Slack Web API method [`slackLists.download.get`](https://docs.slack.dev/reference/methods/slackLists.download.get): Retrieve List download URL from an export job to download List contents.
///
/// Send it with [`SlackClient::slack_lists_download_get`].
#[doc(alias = "slackLists.download.get")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SlackListsDownloadGetRequest {
    /// ID of the List to export.
    pub list_id: String,
    /// The ID of the recently started job to export the List.
    pub job_id: String,
    /// Format the export was started with. Must match the `format` passed to `slackLists.download.start`. Defaults to `csv`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Must match the `include_threads` passed to `slackLists.download.start`. The returned `download_url` carries this through so the served export matches what was generated. Only applies when `format` is `json`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_threads: Option<bool>,
    /// Must match the `include_attachments` passed to `slackLists.download.start`. The returned `download_url` carries this through so the served export matches what was generated. Only applies when `format` is `json`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_attachments: Option<bool>,
}

impl SlackListsDownloadGetRequest {
    pub fn new(list_id: impl Into<String>, job_id: impl Into<String>) -> Self {
        Self {
            list_id: list_id.into(),
            job_id: job_id.into(),
            format: None,
            include_threads: None,
            include_attachments: None,
        }
    }

    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn include_threads(mut self, include_threads: bool) -> Self {
        self.include_threads = Some(include_threads);
        self
    }

    pub fn include_attachments(mut self, include_attachments: bool) -> Self {
        self.include_attachments = Some(include_attachments);
        self
    }
}

impl SlackApiMethod for SlackListsDownloadGetRequest {
    const METHOD: &'static str = "slackLists.download.get";
    type Response = SlackListsDownloadGetResponse;
}

/// Successful response of the Slack Web API method [`slackLists.download.get`](https://docs.slack.dev/reference/methods/slackLists.download.get).
#[doc(alias = "slackLists.download.get")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsDownloadGetResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub status: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub download_url: Option<String>,
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

/// Arguments for the Slack Web API method [`slackLists.download.start`](https://docs.slack.dev/reference/methods/slackLists.download.start): Initiate a job to export List contents.
///
/// Send it with [`SlackClient::slack_lists_download_start`].
#[doc(alias = "slackLists.download.start")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SlackListsDownloadStartRequest {
    /// ID of the List to export.
    pub list_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_archived: Option<bool>,
    /// Format of the export. Defaults to `csv` for backward compatibility. Use `json` for a complete, hierarchical export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Include each item's conversation thread in the export. Only applies when `format` is `json`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_threads: Option<bool>,
    /// Include file attachment metadata and access paths in the export. Only applies when `format` is `json`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_attachments: Option<bool>,
}

impl SlackListsDownloadStartRequest {
    pub fn new(list_id: impl Into<String>) -> Self {
        Self {
            list_id: list_id.into(),
            include_archived: None,
            format: None,
            include_threads: None,
            include_attachments: None,
        }
    }

    pub fn include_archived(mut self, include_archived: bool) -> Self {
        self.include_archived = Some(include_archived);
        self
    }

    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn include_threads(mut self, include_threads: bool) -> Self {
        self.include_threads = Some(include_threads);
        self
    }

    pub fn include_attachments(mut self, include_attachments: bool) -> Self {
        self.include_attachments = Some(include_attachments);
        self
    }
}

impl SlackApiMethod for SlackListsDownloadStartRequest {
    const METHOD: &'static str = "slackLists.download.start";
    type Response = SlackListsDownloadStartResponse;
}

/// Successful response of the Slack Web API method [`slackLists.download.start`](https://docs.slack.dev/reference/methods/slackLists.download.start).
#[doc(alias = "slackLists.download.start")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsDownloadStartResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub job_id: Option<String>,
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

/// Arguments for the Slack Web API method [`slackLists.items.create`](https://docs.slack.dev/reference/methods/slackLists.items.create): Add a new item to an existing List.
///
/// Send it with [`SlackClient::slack_lists_items_create`].
#[doc(alias = "slackLists.items.create")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SlackListsItemsCreateRequest {
    /// ID of the List to add the item to.
    pub list_id: String,
    /// ID of the record to make a copy of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicated_item_id: Option<String>,
    /// ID of the parent record for this subtask.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_item_id: Option<String>,
    /// Initial item data.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub initial_fields: Option<Vec<serde_json::Value>>,
}

impl SlackListsItemsCreateRequest {
    pub fn new(list_id: impl Into<String>) -> Self {
        Self {
            list_id: list_id.into(),
            duplicated_item_id: None,
            parent_item_id: None,
            initial_fields: None,
        }
    }

    pub fn duplicated_item_id(mut self, duplicated_item_id: impl Into<String>) -> Self {
        self.duplicated_item_id = Some(duplicated_item_id.into());
        self
    }

    pub fn parent_item_id(mut self, parent_item_id: impl Into<String>) -> Self {
        self.parent_item_id = Some(parent_item_id.into());
        self
    }

    pub fn initial_fields(mut self, initial_fields: Vec<serde_json::Value>) -> Self {
        self.initial_fields = Some(initial_fields);
        self
    }
}

impl SlackApiMethod for SlackListsItemsCreateRequest {
    const METHOD: &'static str = "slackLists.items.create";
    type Response = SlackListsItemsCreateResponse;
}

/// Successful response of the Slack Web API method [`slackLists.items.create`](https://docs.slack.dev/reference/methods/slackLists.items.create).
#[doc(alias = "slackLists.items.create")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsCreateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub item: Option<SlackListsItemsCreateResponseItem>,
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
pub struct SlackListsItemsCreateResponseItem {
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
    pub list_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_by: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_by: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fields: Vec<SlackListsItemsCreateResponseItemFields>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsCreateResponseItemFields {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub key: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub text: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rich_text: Vec<SlackListsItemsCreateResponseItemFieldsRichText>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub column_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub select: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub date: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub timestamp: Vec<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsCreateResponseItemFieldsRichText {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub elements: Vec<SlackListsItemsCreateResponseItemFieldsRichTextElements>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsCreateResponseItemFieldsRichTextElements {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub elements: Vec<SlackListsItemsCreateResponseItemFieldsRichTextElementsElements>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsCreateResponseItemFieldsRichTextElementsElements {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub text: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
}

/// Arguments for the Slack Web API method [`slackLists.items.delete`](https://docs.slack.dev/reference/methods/slackLists.items.delete): Deletes an item from an existing List.
///
/// Send it with [`SlackClient::slack_lists_items_delete`].
#[doc(alias = "slackLists.items.delete")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SlackListsItemsDeleteRequest {
    /// ID of the List containing the item.
    pub list_id: String,
    /// ID of item to delete.
    pub id: String,
}

impl SlackListsItemsDeleteRequest {
    pub fn new(list_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            list_id: list_id.into(),
            id: id.into(),
        }
    }
}

impl SlackApiMethod for SlackListsItemsDeleteRequest {
    const METHOD: &'static str = "slackLists.items.delete";
    type Response = SlackListsItemsDeleteResponse;
}

/// Successful response of the Slack Web API method [`slackLists.items.delete`](https://docs.slack.dev/reference/methods/slackLists.items.delete).
#[doc(alias = "slackLists.items.delete")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsDeleteResponse {
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

/// Arguments for the Slack Web API method [`slackLists.items.deleteMultiple`](https://docs.slack.dev/reference/methods/slackLists.items.deleteMultiple): Deletes multiple items from an existing List.
///
/// Send it with [`SlackClient::slack_lists_items_delete_multiple`].
#[doc(alias = "slackLists.items.deleteMultiple")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SlackListsItemsDeleteMultipleRequest {
    /// ID of the List containing the items.
    pub list_id: String,
    /// IDs of items to delete.
    pub ids: Vec<String>,
}

impl SlackListsItemsDeleteMultipleRequest {
    pub fn new(list_id: impl Into<String>, ids: Vec<String>) -> Self {
        Self {
            list_id: list_id.into(),
            ids,
        }
    }
}

impl SlackApiMethod for SlackListsItemsDeleteMultipleRequest {
    const METHOD: &'static str = "slackLists.items.deleteMultiple";
    type Response = SlackListsItemsDeleteMultipleResponse;
}

/// Successful response of the Slack Web API method [`slackLists.items.deleteMultiple`](https://docs.slack.dev/reference/methods/slackLists.items.deleteMultiple).
#[doc(alias = "slackLists.items.deleteMultiple")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsDeleteMultipleResponse {
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

/// Arguments for the Slack Web API method [`slackLists.items.info`](https://docs.slack.dev/reference/methods/slackLists.items.info): Get a row from a List.
///
/// Send it with [`SlackClient::slack_lists_items_info`].
#[doc(alias = "slackLists.items.info")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SlackListsItemsInfoRequest {
    /// ID of the List.
    pub list_id: String,
    /// ID of the row to get.
    pub id: String,
    /// Set to `true` to include `is_subscribed` data for the returned List row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_is_subscribed: Option<bool>,
}

impl SlackListsItemsInfoRequest {
    pub fn new(list_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            list_id: list_id.into(),
            id: id.into(),
            include_is_subscribed: None,
        }
    }

    pub fn include_is_subscribed(mut self, include_is_subscribed: bool) -> Self {
        self.include_is_subscribed = Some(include_is_subscribed);
        self
    }
}

impl SlackApiMethod for SlackListsItemsInfoRequest {
    const METHOD: &'static str = "slackLists.items.info";
    type Response = SlackListsItemsInfoResponse;
}

/// Successful response of the Slack Web API method [`slackLists.items.info`](https://docs.slack.dev/reference/methods/slackLists.items.info).
#[doc(alias = "slackLists.items.info")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsInfoResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub list: Option<SlackListsItemsInfoResponseList>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub record: Option<SlackListsItemsInfoResponseRecord>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subtasks: Vec<serde_json::Value>,
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
pub struct SlackListsItemsInfoResponseList {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub created: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub timestamp: Option<i64>,
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
    pub title: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub mimetype: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub filetype: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub pretty_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_team: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub editable: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub size: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub mode: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_external: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_public: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub public_url_shared: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_as_bot: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub username: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_metadata: Option<SlackListsItemsInfoResponseListListMetadata>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_limits: Option<SlackListsItemsInfoResponseListListLimits>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub url_private: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub url_private_download: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub permalink: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub permalink_public: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_editor: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_csv_download_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_starred: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub skipped_shares: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub teams_shared_with: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_restricted_sharing_enabled: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_rich_preview: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub file_access: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub access: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub org_or_workspace_access: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_ai_suggested: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsInfoResponseListListMetadata {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub schema: Vec<SlackListsItemsInfoResponseListListMetadataSchema>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub views: Vec<SlackListsItemsInfoResponseListListMetadataViews>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub integrations: Vec<serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub icon: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub description_blocks: Vec<crate::blocks::Block>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_trial: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subtask_schema: Vec<SlackListsItemsInfoResponseListListMetadataSubtaskSchema>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub creation_source: Option<SlackListsItemsInfoResponseListListMetadataCreationSource>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub todo_mode: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_view: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsInfoResponseListListMetadataSchema {
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
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub key: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_primary_column: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub options: Option<SlackListsItemsInfoResponseListListMetadataSchemaOptions>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsInfoResponseListListMetadataSchemaOptions {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub precision: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_member_name: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub choices: Vec<SlackListsItemsInfoResponseListListMetadataSchemaOptionsChoices>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub format: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub emoji: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub max: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsInfoResponseListListMetadataSchemaOptionsChoices {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub label: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsInfoResponseListListMetadataViews {
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
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_locked: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub position: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub columns: Vec<SlackListsItemsInfoResponseListListMetadataViewsColumns>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_by: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub stick_column_left: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_all_items_view: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_view_key: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_completed_items: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsInfoResponseListListMetadataViewsColumns {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub visible: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub key: Option<String>,
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
    pub position: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsInfoResponseListListMetadataSubtaskSchema {
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
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub key: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_primary_column: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsInfoResponseListListMetadataCreationSource {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsInfoResponseListListLimits {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub over_row_maximum: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub row_count_limit: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub row_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub archived_row_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub over_column_maximum: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub column_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub column_count_limit: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub over_view_maximum: Option<bool>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub view_count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub view_count_limit: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_attachments_per_cell: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsInfoResponseRecord {
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
    pub list_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_by: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_by: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fields: Vec<serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_timestamp: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_subscribed: Option<bool>,
}

/// Arguments for the Slack Web API method [`slackLists.items.list`](https://docs.slack.dev/reference/methods/slackLists.items.list): Get records from a List.
///
/// Send it with [`SlackClient::slack_lists_items_list`].
#[doc(alias = "slackLists.items.list")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SlackListsItemsListRequest {
    /// ID of the List.
    pub list_id: String,
    /// The maximum number of records to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Next cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Boolean indicating whether archived items or normal items should be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// Set to `true` to also return the parent `list` object, including its title, column schema, and total row count. Defaults to `false` to keep the response small.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_list: Option<bool>,
}

impl SlackListsItemsListRequest {
    pub fn new(list_id: impl Into<String>) -> Self {
        Self {
            list_id: list_id.into(),
            limit: None,
            cursor: None,
            archived: None,
            include_list: None,
        }
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn archived(mut self, archived: bool) -> Self {
        self.archived = Some(archived);
        self
    }

    pub fn include_list(mut self, include_list: bool) -> Self {
        self.include_list = Some(include_list);
        self
    }
}

impl SlackApiMethod for SlackListsItemsListRequest {
    const METHOD: &'static str = "slackLists.items.list";
    type Response = SlackListsItemsListResponse;
}

impl CursorPaginated for SlackListsItemsListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for SlackListsItemsListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`slackLists.items.list`](https://docs.slack.dev/reference/methods/slackLists.items.list).
#[doc(alias = "slackLists.items.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<SlackListsItemsListResponseItems>,
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
pub struct SlackListsItemsListResponseItems {
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
    pub list_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_by: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_by: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fields: Vec<SlackListsItemsListResponseItemsFields>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsListResponseItemsFields {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub key: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub text: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rich_text: Vec<SlackListsItemsListResponseItemsFieldsRichText>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub column_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub number: Vec<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub select: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub date: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub timestamp: Vec<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsListResponseItemsFieldsRichText {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub elements: Vec<SlackListsItemsListResponseItemsFieldsRichTextElements>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsListResponseItemsFieldsRichTextElements {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub elements: Vec<SlackListsItemsListResponseItemsFieldsRichTextElementsElements>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsListResponseItemsFieldsRichTextElementsElements {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub text: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub style: Option<SlackListsItemsListResponseItemsFieldsRichTextElementsElementsStyle>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsListResponseItemsFieldsRichTextElementsElementsStyle {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub bold: Option<bool>,
}

/// Arguments for the Slack Web API method [`slackLists.items.update`](https://docs.slack.dev/reference/methods/slackLists.items.update): Updates cells in a List.
///
/// Send it with [`SlackClient::slack_lists_items_update`].
#[doc(alias = "slackLists.items.update")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SlackListsItemsUpdateRequest {
    /// ID of the List to add or update cells.
    pub list_id: String,
    /// Cells to update.
    #[serde(serialize_with = "crate::form::as_json")]
    pub cells: Vec<serde_json::Value>,
}

impl SlackListsItemsUpdateRequest {
    pub fn new(list_id: impl Into<String>, cells: Vec<serde_json::Value>) -> Self {
        Self {
            list_id: list_id.into(),
            cells,
        }
    }
}

impl SlackApiMethod for SlackListsItemsUpdateRequest {
    const METHOD: &'static str = "slackLists.items.update";
    type Response = SlackListsItemsUpdateResponse;
}

/// Successful response of the Slack Web API method [`slackLists.items.update`](https://docs.slack.dev/reference/methods/slackLists.items.update).
#[doc(alias = "slackLists.items.update")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsItemsUpdateResponse {
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

/// Arguments for the Slack Web API method [`slackLists.update`](https://docs.slack.dev/reference/methods/slackLists.update): Update a List.
///
/// Send it with [`SlackClient::slack_lists_update`].
#[doc(alias = "slackLists.update")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SlackListsUpdateRequest {
    /// The ID of the List to update.
    pub id: String,
    /// The updated name of the List.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A rich text description of the List.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub description_blocks: Option<Vec<crate::blocks::Block>>,
    /// Boolean indicating whether the List should be in todo mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub todo_mode: Option<bool>,
}

impl SlackListsUpdateRequest {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: None,
            description_blocks: None,
            todo_mode: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn description_blocks(mut self, description_blocks: Vec<crate::blocks::Block>) -> Self {
        self.description_blocks = Some(description_blocks);
        self
    }

    pub fn todo_mode(mut self, todo_mode: bool) -> Self {
        self.todo_mode = Some(todo_mode);
        self
    }
}

impl SlackApiMethod for SlackListsUpdateRequest {
    const METHOD: &'static str = "slackLists.update";
    type Response = SlackListsUpdateResponse;
}

/// Successful response of the Slack Web API method [`slackLists.update`](https://docs.slack.dev/reference/methods/slackLists.update).
#[doc(alias = "slackLists.update")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlackListsUpdateResponse {
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
    /// Calls the Slack Web API method [`slackLists.access.delete`](https://docs.slack.dev/reference/methods/slackLists.access.delete): Revoke access to a List for specified entities.
    ///
    /// Required scopes:
    ///
    /// - bot token: `lists:write`
    /// - user token: `lists:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "slackLists.access.delete")]
    pub async fn slack_lists_access_delete(
        &self,
        request: &SlackListsAccessDeleteRequest,
    ) -> Result<SlackListsAccessDeleteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`slackLists.access.set`](https://docs.slack.dev/reference/methods/slackLists.access.set): Set the access level to a List for specified entities.
    ///
    /// Required scopes:
    ///
    /// - bot token: `lists:write`
    /// - user token: `lists:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "slackLists.access.set")]
    pub async fn slack_lists_access_set(
        &self,
        request: &SlackListsAccessSetRequest,
    ) -> Result<SlackListsAccessSetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`slackLists.create`](https://docs.slack.dev/reference/methods/slackLists.create): Create a List.
    ///
    /// Required scopes:
    ///
    /// - bot token: `lists:write`
    /// - user token: `lists:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "slackLists.create")]
    pub async fn slack_lists_create(
        &self,
        request: &SlackListsCreateRequest,
    ) -> Result<SlackListsCreateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`slackLists.download.get`](https://docs.slack.dev/reference/methods/slackLists.download.get): Retrieve List download URL from an export job to download List contents.
    ///
    /// Required scopes:
    ///
    /// - bot token: `lists:read`
    /// - user token: `lists:read`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "slackLists.download.get")]
    pub async fn slack_lists_download_get(
        &self,
        request: &SlackListsDownloadGetRequest,
    ) -> Result<SlackListsDownloadGetResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`slackLists.download.start`](https://docs.slack.dev/reference/methods/slackLists.download.start): Initiate a job to export List contents.
    ///
    /// Required scopes:
    ///
    /// - bot token: `lists:read`
    /// - user token: `lists:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "slackLists.download.start")]
    pub async fn slack_lists_download_start(
        &self,
        request: &SlackListsDownloadStartRequest,
    ) -> Result<SlackListsDownloadStartResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`slackLists.items.create`](https://docs.slack.dev/reference/methods/slackLists.items.create): Add a new item to an existing List.
    ///
    /// Required scopes:
    ///
    /// - bot token: `lists:write`
    /// - user token: `lists:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "slackLists.items.create")]
    pub async fn slack_lists_items_create(
        &self,
        request: &SlackListsItemsCreateRequest,
    ) -> Result<SlackListsItemsCreateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`slackLists.items.delete`](https://docs.slack.dev/reference/methods/slackLists.items.delete): Deletes an item from an existing List.
    ///
    /// Required scopes:
    ///
    /// - bot token: `lists:write`
    /// - user token: `lists:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "slackLists.items.delete")]
    pub async fn slack_lists_items_delete(
        &self,
        request: &SlackListsItemsDeleteRequest,
    ) -> Result<SlackListsItemsDeleteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`slackLists.items.deleteMultiple`](https://docs.slack.dev/reference/methods/slackLists.items.deleteMultiple): Deletes multiple items from an existing List.
    ///
    /// Required scopes:
    ///
    /// - bot token: `lists:write`
    /// - user token: `lists:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "slackLists.items.deleteMultiple")]
    pub async fn slack_lists_items_delete_multiple(
        &self,
        request: &SlackListsItemsDeleteMultipleRequest,
    ) -> Result<SlackListsItemsDeleteMultipleResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`slackLists.items.info`](https://docs.slack.dev/reference/methods/slackLists.items.info): Get a row from a List.
    ///
    /// Required scopes:
    ///
    /// - bot token: `lists:read`
    /// - user token: `lists:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "slackLists.items.info")]
    pub async fn slack_lists_items_info(
        &self,
        request: &SlackListsItemsInfoRequest,
    ) -> Result<SlackListsItemsInfoResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`slackLists.items.list`](https://docs.slack.dev/reference/methods/slackLists.items.list): Get records from a List.
    ///
    /// Required scopes:
    ///
    /// - bot token: `lists:read`
    /// - user token: `lists:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "slackLists.items.list")]
    pub async fn slack_lists_items_list(
        &self,
        request: &SlackListsItemsListRequest,
    ) -> Result<SlackListsItemsListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`slackLists.items.update`](https://docs.slack.dev/reference/methods/slackLists.items.update): Updates cells in a List.
    ///
    /// Required scopes:
    ///
    /// - bot token: `lists:write`
    /// - user token: `lists:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "slackLists.items.update")]
    pub async fn slack_lists_items_update(
        &self,
        request: &SlackListsItemsUpdateRequest,
    ) -> Result<SlackListsItemsUpdateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`slackLists.update`](https://docs.slack.dev/reference/methods/slackLists.update): Update a List.
    ///
    /// Required scopes:
    ///
    /// - bot token: `lists:write`
    /// - user token: `lists:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "slackLists.update")]
    pub async fn slack_lists_update(
        &self,
        request: &SlackListsUpdateRequest,
    ) -> Result<SlackListsUpdateResponse, SlackError> {
        self.call(request).await
    }
}
