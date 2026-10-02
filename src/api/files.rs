// このファイルは codegen/generate.py が生成する。手で編集しない

#![allow(unused_imports)]

use crate::objects::*;
use crate::{
    CursorPaginated, NextCursor, ResponseMetadata, SlackApiMethod, SlackClient, SlackError,
};
use serde::{Deserialize, Serialize};

/// Arguments for the Slack Web API method [`files.comments.delete`](https://docs.slack.dev/reference/methods/files.comments.delete): Deletes an existing comment on a file.
///
/// Send it with [`SlackClient::files_comments_delete`].
#[doc(alias = "files.comments.delete")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct FilesCommentsDeleteRequest {
    /// File to delete a comment from.
    pub file: String,
    /// The comment to delete.
    pub id: String,
}

impl FilesCommentsDeleteRequest {
    pub fn new(file: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            file: file.into(),
            id: id.into(),
        }
    }
}

impl SlackApiMethod for FilesCommentsDeleteRequest {
    const METHOD: &'static str = "files.comments.delete";
    type Response = FilesCommentsDeleteResponse;
}

/// Successful response of the Slack Web API method [`files.comments.delete`](https://docs.slack.dev/reference/methods/files.comments.delete).
#[doc(alias = "files.comments.delete")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilesCommentsDeleteResponse {
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

/// Arguments for the Slack Web API method [`files.completeUploadExternal`](https://docs.slack.dev/reference/methods/files.completeUploadExternal): Finishes an upload started with files.getUploadURLExternal
///
/// Send it with [`SlackClient::files_complete_upload_external`].
#[doc(alias = "files.completeUploadExternal")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct FilesCompleteUploadExternalRequest {
    /// Array of file ids and their corresponding (optional) titles.
    #[serde(serialize_with = "crate::form::as_json")]
    pub files: Vec<serde_json::Value>,
    /// Channel ID where the file will be shared. If not specified the file will be private.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// Provide another message's `ts` value to upload this file as a reply. Never use a reply's `ts` value; use its parent instead. Also make sure to provide only one channel when using 'thread\_ts'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
    /// Comma-separated string of channel IDs or user IDs where the file will be shared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<String>,
    /// The message text introducing the file in specified channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_comment: Option<String>,
    /// A JSON-based array of structured rich text blocks, presented as a URL-encoded string. If the `initial_comment` field is provided, the `blocks` field is ignored
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::form::as_json"
    )]
    pub blocks: Option<Vec<crate::blocks::Block>>,
    /// Set your bot's user name for the file share message. Requires the [`chat:write.customize`](https://docs.slack.dev/reference/scopes/chat.write.customize.md) scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// URL to an image to use as the icon for the file share message. Requires the [`chat:write.customize`](https://docs.slack.dev/reference/scopes/chat.write.customize.md) scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Emoji to use as the icon for the file share message. Overrides `icon_url`. Requires the [`chat:write.customize`](https://docs.slack.dev/reference/scopes/chat.write.customize.md) scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_emoji: Option<String>,
}

impl FilesCompleteUploadExternalRequest {
    pub fn new(files: Vec<serde_json::Value>) -> Self {
        Self {
            files,
            channel_id: None,
            thread_ts: None,
            channels: None,
            initial_comment: None,
            blocks: None,
            username: None,
            icon_url: None,
            icon_emoji: None,
        }
    }

    pub fn channel_id(mut self, channel_id: impl Into<String>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    pub fn thread_ts(mut self, thread_ts: impl Into<String>) -> Self {
        self.thread_ts = Some(thread_ts.into());
        self
    }

    pub fn channels(mut self, channels: impl Into<String>) -> Self {
        self.channels = Some(channels.into());
        self
    }

    pub fn initial_comment(mut self, initial_comment: impl Into<String>) -> Self {
        self.initial_comment = Some(initial_comment.into());
        self
    }

    pub fn blocks(mut self, blocks: Vec<crate::blocks::Block>) -> Self {
        self.blocks = Some(blocks);
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }

    pub fn icon_emoji(mut self, icon_emoji: impl Into<String>) -> Self {
        self.icon_emoji = Some(icon_emoji.into());
        self
    }
}

impl SlackApiMethod for FilesCompleteUploadExternalRequest {
    const METHOD: &'static str = "files.completeUploadExternal";
    type Response = FilesCompleteUploadExternalResponse;
}

/// Successful response of the Slack Web API method [`files.completeUploadExternal`](https://docs.slack.dev/reference/methods/files.completeUploadExternal).
#[doc(alias = "files.completeUploadExternal")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilesCompleteUploadExternalResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub files: Vec<FilesCompleteUploadExternalResponseFiles>,
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
pub struct FilesCompleteUploadExternalResponseFiles {
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
    pub title: Option<String>,
}

/// Arguments for the Slack Web API method [`files.delete`](https://docs.slack.dev/reference/methods/files.delete): Deletes a file.
///
/// Send it with [`SlackClient::files_delete`].
#[doc(alias = "files.delete")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct FilesDeleteRequest {
    /// ID of file to delete.
    pub file: String,
}

impl FilesDeleteRequest {
    pub fn new(file: impl Into<String>) -> Self {
        Self { file: file.into() }
    }
}

impl SlackApiMethod for FilesDeleteRequest {
    const METHOD: &'static str = "files.delete";
    type Response = FilesDeleteResponse;
}

/// Successful response of the Slack Web API method [`files.delete`](https://docs.slack.dev/reference/methods/files.delete).
#[doc(alias = "files.delete")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilesDeleteResponse {
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

/// Arguments for the Slack Web API method [`files.getUploadURLExternal`](https://docs.slack.dev/reference/methods/files.getUploadURLExternal): Gets a URL for an edge external file upload
///
/// Send it with [`SlackClient::files_get_upload_url_external`].
#[doc(alias = "files.getUploadURLExternal")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct FilesGetUploadUrlExternalRequest {
    /// Size in bytes of the file being uploaded.
    pub length: i64,
    /// Name of the file being uploaded.
    pub filename: String,
    /// Syntax type of the snippet being uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet_type: Option<String>,
    /// Description of image for screen-reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_txt: Option<String>,
}

impl FilesGetUploadUrlExternalRequest {
    pub fn new(length: i64, filename: impl Into<String>) -> Self {
        Self {
            length,
            filename: filename.into(),
            snippet_type: None,
            alt_txt: None,
        }
    }

    pub fn snippet_type(mut self, snippet_type: impl Into<String>) -> Self {
        self.snippet_type = Some(snippet_type.into());
        self
    }

    pub fn alt_txt(mut self, alt_txt: impl Into<String>) -> Self {
        self.alt_txt = Some(alt_txt.into());
        self
    }
}

impl SlackApiMethod for FilesGetUploadUrlExternalRequest {
    const METHOD: &'static str = "files.getUploadURLExternal";
    type Response = FilesGetUploadUrlExternalResponse;
}

/// Successful response of the Slack Web API method [`files.getUploadURLExternal`](https://docs.slack.dev/reference/methods/files.getUploadURLExternal).
#[doc(alias = "files.getUploadURLExternal")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilesGetUploadUrlExternalResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub upload_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub file_id: Option<String>,
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

/// Arguments for the Slack Web API method [`files.info`](https://docs.slack.dev/reference/methods/files.info): Gets information about a file.
///
/// Send it with [`SlackClient::files_info`].
#[doc(alias = "files.info")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct FilesInfoRequest {
    /// Specify a file by providing its ID.
    pub file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Parameter for pagination. File comments are paginated for a single file. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first "page" of the collection of comments. See [pagination](https://docs.slack.dev/apis/web-api/pagination.md) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl FilesInfoRequest {
    pub fn new(file: impl Into<String>) -> Self {
        Self {
            file: file.into(),
            count: None,
            cursor: None,
            limit: None,
            page: None,
        }
    }

    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
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

    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }
}

impl SlackApiMethod for FilesInfoRequest {
    const METHOD: &'static str = "files.info";
    type Response = FilesInfoResponse;
}

impl CursorPaginated for FilesInfoRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for FilesInfoResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`files.info`](https://docs.slack.dev/reference/methods/files.info).
#[doc(alias = "files.info")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilesInfoResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub file: Option<File>,
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub comments: Vec<serde_json::Value>,
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

/// Arguments for the Slack Web API method [`files.list`](https://docs.slack.dev/reference/methods/files.list): List for a team, in a channel, or from a user with applied filters.
///
/// Send it with [`SlackClient::files_list`].
#[doc(alias = "files.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct FilesListRequest {
    /// Filter files appearing in a specific channel, indicated by its ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Show truncated file info for files hidden due to being too old, and the team who owns the file being over the file limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_files_hidden_by_limit: Option<bool>,
    /// encoded team id to list files in, required if org token is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Filter files created after this timestamp (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts_from: Option<String>,
    /// Filter files created before this timestamp (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts_to: Option<String>,
    /// Filter files by type ([see below](#file_types)). You can pass multiple values in the types argument, like `types=spaces,snippets`.The default value is `all`, which does not filter the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    /// Filter files created by a single user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl FilesListRequest {
    pub fn new() -> Self {
        Self {
            channel: None,
            count: None,
            page: None,
            show_files_hidden_by_limit: None,
            team_id: None,
            ts_from: None,
            ts_to: None,
            types: None,
            user: None,
        }
    }

    pub fn channel(mut self, channel: impl Into<String>) -> Self {
        self.channel = Some(channel.into());
        self
    }

    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }

    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }

    pub fn show_files_hidden_by_limit(mut self, show_files_hidden_by_limit: bool) -> Self {
        self.show_files_hidden_by_limit = Some(show_files_hidden_by_limit);
        self
    }

    pub fn team_id(mut self, team_id: impl Into<String>) -> Self {
        self.team_id = Some(team_id.into());
        self
    }

    pub fn ts_from(mut self, ts_from: impl Into<String>) -> Self {
        self.ts_from = Some(ts_from.into());
        self
    }

    pub fn ts_to(mut self, ts_to: impl Into<String>) -> Self {
        self.ts_to = Some(ts_to.into());
        self
    }

    pub fn types(mut self, types: impl Into<String>) -> Self {
        self.types = Some(types.into());
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

impl SlackApiMethod for FilesListRequest {
    const METHOD: &'static str = "files.list";
    type Response = FilesListResponse;
}

/// Successful response of the Slack Web API method [`files.list`](https://docs.slack.dev/reference/methods/files.list).
#[doc(alias = "files.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilesListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub files: Vec<File>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub paging: Option<FilesListResponsePaging>,
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
pub struct FilesListResponsePaging {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub total: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub page: Option<i64>,
    #[serde(
        default,
        deserialize_with = "crate::de::opt_i64",
        skip_serializing_if = "Option::is_none"
    )]
    pub pages: Option<i64>,
}

/// Arguments for the Slack Web API method [`files.remote.add`](https://docs.slack.dev/reference/methods/files.remote.add): Adds a file from a remote service
///
/// Send it with [`SlackClient::files_remote_add`].
#[doc(alias = "files.remote.add")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct FilesRemoteAddRequest {
    /// Creator defined GUID for the file.
    pub external_id: String,
    /// URL of the remote file.
    pub external_url: String,
    /// Title of the file being shared.
    pub title: String,
    /// type of file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filetype: Option<String>,
    /// A text file (txt, pdf, doc, etc.) containing textual search terms that are used to improve discovery of the remote file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexable_file_contents: Option<String>,
    /// Preview of the document via `multipart/form-data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image: Option<String>,
}

impl FilesRemoteAddRequest {
    pub fn new(
        external_id: impl Into<String>,
        external_url: impl Into<String>,
        title: impl Into<String>,
    ) -> Self {
        Self {
            external_id: external_id.into(),
            external_url: external_url.into(),
            title: title.into(),
            filetype: None,
            indexable_file_contents: None,
            preview_image: None,
        }
    }

    pub fn filetype(mut self, filetype: impl Into<String>) -> Self {
        self.filetype = Some(filetype.into());
        self
    }

    pub fn indexable_file_contents(mut self, indexable_file_contents: impl Into<String>) -> Self {
        self.indexable_file_contents = Some(indexable_file_contents.into());
        self
    }

    pub fn preview_image(mut self, preview_image: impl Into<String>) -> Self {
        self.preview_image = Some(preview_image.into());
        self
    }
}

impl SlackApiMethod for FilesRemoteAddRequest {
    const METHOD: &'static str = "files.remote.add";
    type Response = FilesRemoteAddResponse;
}

/// Successful response of the Slack Web API method [`files.remote.add`](https://docs.slack.dev/reference/methods/files.remote.add).
#[doc(alias = "files.remote.add")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilesRemoteAddResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub file: Option<File>,
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

/// Arguments for the Slack Web API method [`files.remote.info`](https://docs.slack.dev/reference/methods/files.remote.info): Retrieve information about a remote file added to Slack
///
/// Send it with [`SlackClient::files_remote_info`].
#[doc(alias = "files.remote.info")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct FilesRemoteInfoRequest {
    /// Creator defined GUID for the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Specify a file by providing its ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

impl FilesRemoteInfoRequest {
    pub fn new() -> Self {
        Self {
            external_id: None,
            file: None,
        }
    }

    pub fn external_id(mut self, external_id: impl Into<String>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }
}

impl SlackApiMethod for FilesRemoteInfoRequest {
    const METHOD: &'static str = "files.remote.info";
    type Response = FilesRemoteInfoResponse;
}

/// Successful response of the Slack Web API method [`files.remote.info`](https://docs.slack.dev/reference/methods/files.remote.info).
#[doc(alias = "files.remote.info")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilesRemoteInfoResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub file: Option<File>,
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

/// Arguments for the Slack Web API method [`files.remote.list`](https://docs.slack.dev/reference/methods/files.remote.list): Retrieve information about a remote file added to Slack
///
/// Send it with [`SlackClient::files_remote_list`].
#[doc(alias = "files.remote.list")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct FilesRemoteListRequest {
    /// Filter files appearing in a specific channel, indicated by its ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](https://docs.slack.dev/apis/web-api/pagination.md) for more detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The maximum number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Filter files created after this timestamp (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts_from: Option<String>,
    /// Filter files created before this timestamp (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts_to: Option<String>,
}

impl FilesRemoteListRequest {
    pub fn new() -> Self {
        Self {
            channel: None,
            cursor: None,
            limit: None,
            ts_from: None,
            ts_to: None,
        }
    }

    pub fn channel(mut self, channel: impl Into<String>) -> Self {
        self.channel = Some(channel.into());
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

    pub fn ts_from(mut self, ts_from: impl Into<String>) -> Self {
        self.ts_from = Some(ts_from.into());
        self
    }

    pub fn ts_to(mut self, ts_to: impl Into<String>) -> Self {
        self.ts_to = Some(ts_to.into());
        self
    }
}

impl SlackApiMethod for FilesRemoteListRequest {
    const METHOD: &'static str = "files.remote.list";
    type Response = FilesRemoteListResponse;
}

impl CursorPaginated for FilesRemoteListRequest {
    fn set_cursor(&mut self, cursor: String) {
        self.cursor = Some(cursor);
    }
}

impl NextCursor for FilesRemoteListResponse {
    fn next_cursor(&self) -> Option<&str> {
        self.response_metadata.as_ref()?.next_cursor.as_deref()
    }
}

/// Successful response of the Slack Web API method [`files.remote.list`](https://docs.slack.dev/reference/methods/files.remote.list).
#[doc(alias = "files.remote.list")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilesRemoteListResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub files: Vec<serde_json::Value>,
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

/// Arguments for the Slack Web API method [`files.remote.remove`](https://docs.slack.dev/reference/methods/files.remote.remove): Remove a remote file.
///
/// Send it with [`SlackClient::files_remote_remove`].
#[doc(alias = "files.remote.remove")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct FilesRemoteRemoveRequest {
    /// Creator defined GUID for the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Specify a file by providing its ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

impl FilesRemoteRemoveRequest {
    pub fn new() -> Self {
        Self {
            external_id: None,
            file: None,
        }
    }

    pub fn external_id(mut self, external_id: impl Into<String>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }
}

impl SlackApiMethod for FilesRemoteRemoveRequest {
    const METHOD: &'static str = "files.remote.remove";
    type Response = FilesRemoteRemoveResponse;
}

/// Successful response of the Slack Web API method [`files.remote.remove`](https://docs.slack.dev/reference/methods/files.remote.remove).
#[doc(alias = "files.remote.remove")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilesRemoteRemoveResponse {
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

/// Arguments for the Slack Web API method [`files.remote.share`](https://docs.slack.dev/reference/methods/files.remote.share): Share a remote file into a channel.
///
/// Send it with [`SlackClient::files_remote_share`].
#[doc(alias = "files.remote.share")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct FilesRemoteShareRequest {
    /// Comma-separated list of channel IDs where the file will be shared.
    pub channels: String,
    /// The globally unique identifier (GUID) for the file, as set by the app registering the file with Slack. Either this field or `file` or both are required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Specify a file registered with Slack by providing its ID. Either this field or `external_id` or both are required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

impl FilesRemoteShareRequest {
    pub fn new(channels: impl Into<String>) -> Self {
        Self {
            channels: channels.into(),
            external_id: None,
            file: None,
        }
    }

    pub fn external_id(mut self, external_id: impl Into<String>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }
}

impl SlackApiMethod for FilesRemoteShareRequest {
    const METHOD: &'static str = "files.remote.share";
    type Response = FilesRemoteShareResponse;
}

/// Successful response of the Slack Web API method [`files.remote.share`](https://docs.slack.dev/reference/methods/files.remote.share).
#[doc(alias = "files.remote.share")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilesRemoteShareResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub file: Option<File>,
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

/// Arguments for the Slack Web API method [`files.remote.update`](https://docs.slack.dev/reference/methods/files.remote.update): Updates an existing remote file.
///
/// Send it with [`SlackClient::files_remote_update`].
#[doc(alias = "files.remote.update")]
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct FilesRemoteUpdateRequest {
    /// Creator defined GUID for the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// URL of the remote file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    /// Specify a file by providing its ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// type of file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filetype: Option<String>,
    /// File containing contents that can be used to improve searchability for the remote file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexable_file_contents: Option<String>,
    /// Preview of the document via `multipart/form-data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image: Option<String>,
    /// Title of the file being shared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl FilesRemoteUpdateRequest {
    pub fn new() -> Self {
        Self {
            external_id: None,
            external_url: None,
            file: None,
            filetype: None,
            indexable_file_contents: None,
            preview_image: None,
            title: None,
        }
    }

    pub fn external_id(mut self, external_id: impl Into<String>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }

    pub fn external_url(mut self, external_url: impl Into<String>) -> Self {
        self.external_url = Some(external_url.into());
        self
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn filetype(mut self, filetype: impl Into<String>) -> Self {
        self.filetype = Some(filetype.into());
        self
    }

    pub fn indexable_file_contents(mut self, indexable_file_contents: impl Into<String>) -> Self {
        self.indexable_file_contents = Some(indexable_file_contents.into());
        self
    }

    pub fn preview_image(mut self, preview_image: impl Into<String>) -> Self {
        self.preview_image = Some(preview_image.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl SlackApiMethod for FilesRemoteUpdateRequest {
    const METHOD: &'static str = "files.remote.update";
    type Response = FilesRemoteUpdateResponse;
}

/// Successful response of the Slack Web API method [`files.remote.update`](https://docs.slack.dev/reference/methods/files.remote.update).
#[doc(alias = "files.remote.update")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilesRemoteUpdateResponse {
    #[serde(
        default,
        deserialize_with = "crate::de::opt_object",
        skip_serializing_if = "Option::is_none"
    )]
    pub file: Option<File>,
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

/// Arguments for the Slack Web API method [`files.revokePublicURL`](https://docs.slack.dev/reference/methods/files.revokePublicURL): Revokes public/external sharing access for a file
///
/// Send it with [`SlackClient::files_revoke_public_url`].
#[doc(alias = "files.revokePublicURL")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct FilesRevokePublicUrlRequest {
    /// File to revoke
    pub file: String,
}

impl FilesRevokePublicUrlRequest {
    pub fn new(file: impl Into<String>) -> Self {
        Self { file: file.into() }
    }
}

impl SlackApiMethod for FilesRevokePublicUrlRequest {
    const METHOD: &'static str = "files.revokePublicURL";
    type Response = FilesRevokePublicUrlResponse;
}

/// Successful response of the Slack Web API method [`files.revokePublicURL`](https://docs.slack.dev/reference/methods/files.revokePublicURL).
#[doc(alias = "files.revokePublicURL")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilesRevokePublicUrlResponse {
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

/// Arguments for the Slack Web API method [`files.sharedPublicURL`](https://docs.slack.dev/reference/methods/files.sharedPublicURL): Enables a file for public/external sharing.
///
/// Send it with [`SlackClient::files_shared_public_url`].
#[doc(alias = "files.sharedPublicURL")]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct FilesSharedPublicUrlRequest {
    /// File to share
    pub file: String,
}

impl FilesSharedPublicUrlRequest {
    pub fn new(file: impl Into<String>) -> Self {
        Self { file: file.into() }
    }
}

impl SlackApiMethod for FilesSharedPublicUrlRequest {
    const METHOD: &'static str = "files.sharedPublicURL";
    type Response = FilesSharedPublicUrlResponse;
}

/// Successful response of the Slack Web API method [`files.sharedPublicURL`](https://docs.slack.dev/reference/methods/files.sharedPublicURL).
#[doc(alias = "files.sharedPublicURL")]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilesSharedPublicUrlResponse {
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
    /// Calls the Slack Web API method [`files.comments.delete`](https://docs.slack.dev/reference/methods/files.comments.delete): Deletes an existing comment on a file.
    ///
    /// Required scopes:
    ///
    /// - bot token: `files:write`
    /// - user token: `files:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "files.comments.delete")]
    pub async fn files_comments_delete(
        &self,
        request: &FilesCommentsDeleteRequest,
    ) -> Result<FilesCommentsDeleteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`files.completeUploadExternal`](https://docs.slack.dev/reference/methods/files.completeUploadExternal): Finishes an upload started with files.getUploadURLExternal
    ///
    /// Required scopes:
    ///
    /// - bot token: `files:write`
    /// - user token: `files:write`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "files.completeUploadExternal")]
    pub async fn files_complete_upload_external(
        &self,
        request: &FilesCompleteUploadExternalRequest,
    ) -> Result<FilesCompleteUploadExternalResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`files.delete`](https://docs.slack.dev/reference/methods/files.delete): Deletes a file.
    ///
    /// Required scopes:
    ///
    /// - bot token: `files:write`
    /// - user token: `files:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "files.delete")]
    pub async fn files_delete(
        &self,
        request: &FilesDeleteRequest,
    ) -> Result<FilesDeleteResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`files.getUploadURLExternal`](https://docs.slack.dev/reference/methods/files.getUploadURLExternal): Gets a URL for an edge external file upload
    ///
    /// Required scopes:
    ///
    /// - bot token: `files:write`
    /// - user token: `files:write`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "files.getUploadURLExternal")]
    pub async fn files_get_upload_url_external(
        &self,
        request: &FilesGetUploadUrlExternalRequest,
    ) -> Result<FilesGetUploadUrlExternalResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`files.info`](https://docs.slack.dev/reference/methods/files.info): Gets information about a file.
    ///
    /// Required scopes:
    ///
    /// - bot token: `files:read`
    /// - user token: `files:read`
    ///
    /// Rate limit: Tier 4 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "files.info")]
    pub async fn files_info(
        &self,
        request: &FilesInfoRequest,
    ) -> Result<FilesInfoResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`files.list`](https://docs.slack.dev/reference/methods/files.list): List for a team, in a channel, or from a user with applied filters.
    ///
    /// Required scopes:
    ///
    /// - bot token: `files:read`
    /// - user token: `files:read`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "files.list")]
    pub async fn files_list(
        &self,
        request: &FilesListRequest,
    ) -> Result<FilesListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`files.remote.add`](https://docs.slack.dev/reference/methods/files.remote.add): Adds a file from a remote service
    ///
    /// Required scopes:
    ///
    /// - bot token: `remote_files:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "files.remote.add")]
    pub async fn files_remote_add(
        &self,
        request: &FilesRemoteAddRequest,
    ) -> Result<FilesRemoteAddResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`files.remote.info`](https://docs.slack.dev/reference/methods/files.remote.info): Retrieve information about a remote file added to Slack
    ///
    /// Required scopes:
    ///
    /// - bot token: `remote_files:read`
    /// - user token: `remote_files:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "files.remote.info")]
    pub async fn files_remote_info(
        &self,
        request: &FilesRemoteInfoRequest,
    ) -> Result<FilesRemoteInfoResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`files.remote.list`](https://docs.slack.dev/reference/methods/files.remote.list): Retrieve information about a remote file added to Slack
    ///
    /// Required scopes:
    ///
    /// - bot token: `remote_files:read`
    /// - user token: `remote_files:read`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "files.remote.list")]
    pub async fn files_remote_list(
        &self,
        request: &FilesRemoteListRequest,
    ) -> Result<FilesRemoteListResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`files.remote.remove`](https://docs.slack.dev/reference/methods/files.remote.remove): Remove a remote file.
    ///
    /// Required scopes:
    ///
    /// - bot token: `remote_files:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "files.remote.remove")]
    pub async fn files_remote_remove(
        &self,
        request: &FilesRemoteRemoveRequest,
    ) -> Result<FilesRemoteRemoveResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`files.remote.share`](https://docs.slack.dev/reference/methods/files.remote.share): Share a remote file into a channel.
    ///
    /// Required scopes:
    ///
    /// - bot token: `remote_files:share`
    /// - user token: `remote_files:share`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "files.remote.share")]
    pub async fn files_remote_share(
        &self,
        request: &FilesRemoteShareRequest,
    ) -> Result<FilesRemoteShareResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`files.remote.update`](https://docs.slack.dev/reference/methods/files.remote.update): Updates an existing remote file.
    ///
    /// Required scopes:
    ///
    /// - bot token: `remote_files:write`
    ///
    /// Rate limit: Tier 2 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "files.remote.update")]
    pub async fn files_remote_update(
        &self,
        request: &FilesRemoteUpdateRequest,
    ) -> Result<FilesRemoteUpdateResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`files.revokePublicURL`](https://docs.slack.dev/reference/methods/files.revokePublicURL): Revokes public/external sharing access for a file
    ///
    /// Required scopes:
    ///
    /// - bot token: `files:write`
    /// - user token: `files:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "files.revokePublicURL")]
    pub async fn files_revoke_public_url(
        &self,
        request: &FilesRevokePublicUrlRequest,
    ) -> Result<FilesRevokePublicUrlResponse, SlackError> {
        self.call(request).await
    }

    /// Calls the Slack Web API method [`files.sharedPublicURL`](https://docs.slack.dev/reference/methods/files.sharedPublicURL): Enables a file for public/external sharing.
    ///
    /// Required scopes:
    ///
    /// - user token: `files:write`
    ///
    /// Rate limit: Tier 3 (<https://docs.slack.dev/apis/web-api/rate-limits>).
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`; see [`SlackClient::call`] for the other cases.
    #[doc(alias = "files.sharedPublicURL")]
    pub async fn files_shared_public_url(
        &self,
        request: &FilesSharedPublicUrlRequest,
    ) -> Result<FilesSharedPublicUrlResponse, SlackError> {
        self.call(request).await
    }
}
