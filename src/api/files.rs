use mpart_async::client::MultipartRequest;
use mpart_async::filestream::FileStream;
use crate::{SlackApiResponse, SlackClient, SlackTypesFile};
use serde_derive::{Deserialize, Serialize};


impl SlackClient {
    /// https://api.slack.com/methods/files.comments.delete
    // Deletes an existing comment on a file.
    pub async fn files_comments_delete(
        &self,
        value: &SlackApiFilesCommentsDeleteRequest,
    ) -> SlackApiResponse<SlackApiFilesCommentsDeleteResponse> {
        self.http_post("files.comments.delete", value).await
    }
    /// https://api.slack.com/methods/files.completeUploadExternal
    // Finishes an upload started with files.getUploadURLExternal
    pub async fn files_complete_upload_external(
        &self,
        value: &SlackApiFilesCompleteUploadExternalRequest,
    ) -> SlackApiResponse<SlackApiFilesCompleteUploadExternalResponse> {
        self.http_post("files.completeUploadExternal", value).await
    }
    /// https://api.slack.com/methods/files.delete
    // Deletes a file.
    pub async fn files_delete(
        &self,
        value: &SlackApiFilesDeleteRequest,
    ) -> SlackApiResponse<SlackApiFilesDeleteResponse> {
        self.http_post("files.delete", value).await
    }

    /// https://api.slack.com/methods/files.getUploadURLExternal
    // Gets a URL for an edge external file upload
    pub async fn files_get_upload_url_external(
        &self,
        value: &SlackApiFilesGetUploadUrlExternalRequest,
    ) -> SlackApiResponse<SlackApiFilesGetUploadUrlExternalResponse> {
        self.http_post("files.getUploadURLExternal", value).await
    }
    /// https://api.slack.com/methods/files.info
    // Gets information about a file.
    pub async fn files_info(
        &self,
        value: &SlackApiFilesInfoRequest,
    ) -> SlackApiResponse<SlackApiFilesInfoResponse> {
        self.http_post("files.info", value).await
    }
    /// https://api.slack.com/methods/files.list
    // List for a team, in a channel, or from a user with applied filters.
    pub async fn files_list(
        &self,
        value: &SlackApiFilesListRequest,
    ) -> SlackApiResponse<SlackApiFilesListResponse> {
        self.http_post("files.list", value).await
    }
    /// https://api.slack.com/methods/files.revokePublicURL
    // Revokes public/external sharing access for a file
    pub async fn files_revoke_public_url(
        &self,
        value: &SlackApiFilesRevokePublicUrlRequest,
    ) -> SlackApiResponse<SlackApiFilesRevokePublicUrlResponse> {
        self.http_post("files.revokePublicURL", value).await
    }

    /// https://api.slack.com/methods/files.sharedPublicURL
    // Enables a file for public/external sharing.
    pub async fn files_shared_public_url(
        &self,
        value: &SlackApiFilesSharePublicUrlRequest,
    ) -> SlackApiResponse<SlackApiFilesSharePublicUrlResponse> {
        self.http_post("files.sharedPublicURL", value).await
    }

    /// https://api.slack.com/methods/files.upload
    // Uploads or creates a file.
    /// https://api.slack.com/methods/chat.delete
    pub async fn files_upload(
        &self,
        value: &SlackApiFilesUploadRequest,
    ) -> SlackApiResponse<SlackApiFilesUploadResponse> {
        self.http_post_data(
            "files.upload",
            value.clone().to_multipart(),
        )
            .await
    }
    /// https://api.slack.com/methods/files.remote.add
    // Adds a file from a remote service
    pub async fn files_remote_add(
        &self,
        value: &SlackApiFilesRemoteAddRequest,
    ) -> SlackApiResponse<SlackApiFilesRemoteAddResponse> {
        self.http_post("files.remote.add", value).await
    }

    /// https://api.slack.com/methods/files.remote.info
    // Retrieve information about a remote file added to Slack
    pub async fn files_remote_info(
        &self,
        value: &SlackApiFilesRemoteInfoRequest,
    ) -> SlackApiResponse<SlackApiFilesRemoteInfoResponse> {
        self.http_post("files.remote.info", value).await
    }

    /// https://api.slack.com/methods/files.remote.list
    // Retrieve information about a remote file added to Slack
    pub async fn files_remote_list(
        &self,
        value: &SlackApiFilesRemoteListRequest,
    ) -> SlackApiResponse<SlackApiFilesRemoteListResponse> {
        self.http_post("files.remote.list", value).await
    }

    /// https://api.slack.com/methods/files.remote.remove
    // Remove a remote file.
    pub async fn files_remote_remove(
        &self,
        value: &SlackApiFilesRemoteRemoveRequest,
    ) -> SlackApiResponse<SlackApiFilesRemoteRemoveResponse> {
        self.http_post("files.remote.remove", value).await
    }

    /// https://api.slack.com/methods/files.remote.share
    // Share a remote file into a channel.
    pub async fn files_remote_share(
        &self,
        value: &SlackApiFilesRemoteShareRequest,
    ) -> SlackApiResponse<SlackApiFilesRemoteShareResponse> {
        self.http_post("files.remote.share", value).await
    }
    /// https://api.slack.com/methods/files.remote.update
    // Updates an existing remote file.
    pub async fn files_remote_update(
        &self,
        value: &SlackApiFilesRemoteUpdateRequest,
    ) -> SlackApiResponse<SlackApiFilesRemoteUpdateResponse> {
        self.http_post("files.remote.update", value).await
    }
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesCommentsDeleteRequest {
    pub channel: String,
    pub file: String,
    pub id: String,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesCommentsDeleteResponse {
    pub ok: bool,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesCompleteUploadExternalRequest {
    pub files: Vec<SlackApiFilesCompleteUploadExternalRequestFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesCompleteUploadExternalRequestFile {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesCompleteUploadExternalResponse {
    pub ok: bool,
    pub files: Vec<SlackApiFilesCompleteUploadExternalRequestFile>,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesDeleteRequest {
    pub file: String,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesDeleteResponse {
    pub ok: bool,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesGetUploadUrlExternalRequest {
    pub filename: String,
    pub length: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_txt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet_type: Option<String>,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesGetUploadUrlExternalResponse {
    pub ok: bool,
    pub upload_url: String,
    pub file_id: String,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesInfoRequest {
    pub file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesInfoResponse {
    pub ok: bool,
    pub upload_url: String,
    pub file_id: String,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesListRequest {
    pub channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_files_hidden_by_limit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesListResponse {
    pub ok: bool,
    pub upload_url: String,
    pub files: Vec<SlackTypesFile>,
    pub paging: SlackTypesFile,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackTypesPaging {
    pub count: u32,
    pub total: u32,
    pub page: u32,
    pub pages: u32,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesRevokePublicUrlRequest {
    pub channel: String,
    pub file: String,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesRevokePublicUrlResponse {
    pub ok: bool,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesSharePublicUrlRequest {
    pub channel: String,
    pub file: String,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesSharePublicUrlResponse {
    pub ok: bool,
    pub file: SlackTypesFile,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesUploadRequest {
    pub channels: Option<String>,
    pub content: Option<String>,
    pub file: Option<String>,
    pub filename: Option<String>,
    pub filetype: Option<String>,
    pub initial_comment: Option<String>,
    pub thread_ts: Option<String>,
    pub title: Option<String>,
}

impl SlackApiFilesUploadRequest {
    fn to_multipart(&self) -> MultipartRequest<FileStream> {
        let mut multipart = MultipartRequest::default();
        let value = self.clone();
        if value.channels.is_some() {
            multipart.add_field("channels", value.channels.unwrap_or_default().as_str());
        }
        if value.content.is_some() {
            multipart.add_field("content", value.content.unwrap_or_default().as_str());
        }
        if value.file.is_some() {
            multipart.add_file("file", value.file.unwrap_or_default());
        }
        if value.filename.is_some() {
            multipart.add_field("filename", value.filename.unwrap_or_default().as_str());
        }
        if value.filetype.is_some() {
            multipart.add_field("filetype", value.filetype.unwrap_or_default().as_str());
        }
        if value.initial_comment.is_some() {
            multipart.add_field("initial_comment", value.initial_comment.unwrap_or_default().as_str());
        }
        if value.thread_ts.is_some() {
            multipart.add_field("thread_ts", value.thread_ts.unwrap_or_default().as_str());
        }
        if value.title.is_some() {
            multipart.add_field("title", value.title.unwrap_or_default().as_str());
        }
        multipart
    }
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesUploadResponse {
    pub ok: bool,
    pub file: SlackTypesFile,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesRemoteAddRequest {
    pub external_id: Option<String>,
    pub external_url: Option<String>,
    pub title: Option<String>,
    pub filetype: Option<String>,
    pub indexable_file_contents: Option<String>,
    pub preview_image: Option<String>,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesRemoteAddResponse {
    pub ok: bool,
    pub file: SlackTypesFile,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesRemoteInfoRequest {
    pub external_id: Option<String>,
    pub file: Option<String>,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesRemoteInfoResponse {
    pub ok: bool,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesRemoteListRequest {
    pub channel: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub ts_from: Option<String>,
    pub ts_to: Option<String>,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesRemoteListResponse {
    pub ok: bool,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesRemoteRemoveRequest {
    pub external_id: Option<String>,
    pub file: Option<String>,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesRemoteRemoveResponse {
    pub ok: bool,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesRemoteShareRequest {
    pub channels: Option<String>,
    pub external_id: Option<String>,
    pub file: Option<String>,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesRemoteShareResponse {
    pub ok: bool,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesRemoteUpdateRequest {
    pub external_id: Option<String>,
    pub external_url: Option<String>,
    pub file: Option<String>,
    pub filetype: Option<String>,
    pub indexable_file_contents: Option<String>,
    pub preview_image: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Default,Clone, Deserialize, Serialize)]
pub struct SlackApiFilesRemoteUpdateResponse {
    pub ok: bool,
}
