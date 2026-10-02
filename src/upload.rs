// ファイルのアップロード（`files.upload` の後継の手順をまとめたもの）。
//
// 1. ファイルごとに `files.getUploadURLExternal` でアップロード先を受け取る
// 2. その URL へ中身をそのまま POST する
// 3. `files.completeUploadExternal` でまとめて確定し、チャンネルに共有する
//
// 1〜2 はファイルごとに並行して行う。

use crate::api::{
    FilesCompleteUploadExternalRequest, FilesCompleteUploadExternalResponse,
    FilesGetUploadUrlExternalRequest,
};
use crate::blocks::Block;
use crate::{SlackClient, SlackError};
use bytes::Bytes;
use serde_json::json;
use tokio::task::JoinSet;

/// A file to upload with [`SlackClient::upload_files`].
#[derive(Debug, Clone, PartialEq)]
pub struct FileUpload {
    filename: String,
    content: Bytes,
    title: Option<String>,
    alt_txt: Option<String>,
    snippet_type: Option<String>,
}

impl FileUpload {
    pub fn new(filename: impl Into<String>, content: impl Into<Bytes>) -> Self {
        Self {
            filename: filename.into(),
            content: content.into(),
            title: None,
            alt_txt: None,
            snippet_type: None,
        }
    }

    /// Title shown in Slack (defaults to the file name).
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Alt text for images.
    pub fn alt_txt(mut self, alt_txt: impl Into<String>) -> Self {
        self.alt_txt = Some(alt_txt.into());
        self
    }

    /// Syntax type when the file is shown as a snippet (`python`, `rust`, ...).
    pub fn snippet_type(mut self, snippet_type: impl Into<String>) -> Self {
        self.snippet_type = Some(snippet_type.into());
        self
    }
}

/// Where uploaded files are shared. With nothing set, the files stay private and unshared.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UploadDestination {
    channel_id: Option<String>,
    thread_ts: Option<String>,
    initial_comment: Option<String>,
    blocks: Option<Vec<Block>>,
}

impl UploadDestination {
    pub fn channel(channel_id: impl Into<String>) -> Self {
        Self {
            channel_id: Some(channel_id.into()),
            ..Self::default()
        }
    }

    /// Posts the files as a reply in this thread.
    pub fn thread_ts(mut self, thread_ts: impl Into<String>) -> Self {
        self.thread_ts = Some(thread_ts.into());
        self
    }

    /// Message posted with the files.
    pub fn initial_comment(mut self, initial_comment: impl Into<String>) -> Self {
        self.initial_comment = Some(initial_comment.into());
        self
    }

    /// Block Kit message posted with the files.
    pub fn blocks(mut self, blocks: Vec<Block>) -> Self {
        self.blocks = Some(blocks);
        self
    }
}

impl SlackClient {
    /// Uploads files and shares them to `destination`, replacing the retired `files.upload`.
    ///
    /// Runs `files.getUploadURLExternal` and the upload for every file concurrently, then
    /// `files.completeUploadExternal` once with the files in the given order.
    ///
    /// # Errors
    ///
    /// Fails on the first API, transport or upload (non-2xx) error. Files uploaded before the
    /// error are not completed and are discarded by Slack.
    pub async fn upload_files(
        &self,
        files: Vec<FileUpload>,
        destination: UploadDestination,
    ) -> Result<FilesCompleteUploadExternalResponse, SlackError> {
        let mut tasks = JoinSet::new();
        for (index, file) in files.into_iter().enumerate() {
            let client = self.clone();
            tasks.spawn(async move { client.upload_one(file).await.map(|entry| (index, entry)) });
        }
        let mut uploaded = Vec::with_capacity(tasks.len());
        while let Some(joined) = tasks.join_next().await {
            uploaded.push(joined.map_err(|e| SlackError::Task(e.to_string()))??);
        }
        uploaded.sort_by_key(|(index, _)| *index);

        let mut request =
            FilesCompleteUploadExternalRequest::new(uploaded.into_iter().map(|(_, e)| e).collect());
        request.channel_id = destination.channel_id;
        request.thread_ts = destination.thread_ts;
        request.initial_comment = destination.initial_comment;
        request.blocks = destination.blocks;
        self.files_complete_upload_external(&request).await
    }

    // 1ファイル分のアップロード先の取得と送信。completeUploadExternal に渡す `{id, title}` を返す
    async fn upload_one(&self, file: FileUpload) -> Result<serde_json::Value, SlackError> {
        let mut request =
            FilesGetUploadUrlExternalRequest::new(file.content.len() as i64, file.filename.clone());
        request.alt_txt = file.alt_txt;
        request.snippet_type = file.snippet_type;
        let target = self.files_get_upload_url_external(&request).await?;
        let (Some(upload_url), Some(file_id)) = (target.upload_url, target.file_id) else {
            return Err(SlackError::MissingField(
                "files.getUploadURLExternal: upload_url / file_id",
            ));
        };

        let response = self
            .http()
            .post(&upload_url)
            .body(file.content)
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(SlackError::Http {
                status: status.as_u16(),
                body,
            });
        }
        Ok(json!({ "id": file_id, "title": file.title.unwrap_or(file.filename) }))
    }
}
