use std::path::Path;
use multipart::client::lazy::Multipart;
use crate::{
    SlackApiResponse, SlackBlock, SlackClient,
};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Serialize, Deserialize)]
struct Test {
    channels: String,
    file: String,
}


impl Test {
    fn to_multipart(self) -> Multipart<'static, 'static> {
        let mut multipart = Multipart::new();
        multipart.add_text("channels", self.channels);
        multipart.add_file("file", self.file);
        multipart
    }
}

impl SlackClient {
    // files.comments.delete
    // Deletes an existing comment on a file.
    pub async fn files_comments_delete(&self, value: &Value) -> SlackApiResponse<Value> {
        self.http_post("files.comments.delete", value).await
    }
    // files.completeUploadExternal
    // Finishes an upload started with files.getUploadURLExternal
    pub async fn files_complete_upload_external(&self, value: &Value) -> SlackApiResponse<Value> {
        self.http_post("files.completeUploadExternal", value).await
    }
    // files.delete
    // Deletes a file.
    pub async fn files_delete(&self, value: &Value) -> SlackApiResponse<Value> {
        self.http_post("files.delete", value).await
    }

    // files.getUploadURLExternal
    // Gets a URL for an edge external file upload
    pub async fn files_get_upload_url_external(&self, value: &Value) -> SlackApiResponse<Value> {
        self.http_post("files.getUploadURLExternal", value).await
    }
    // files.info
    // Gets information about a file.
    pub async fn files_info(&self, value: &Value) -> SlackApiResponse<Value> {
        self.http_post("files.info", value).await
    }
    // files.list
    // List for a team, in a channel, or from a user with applied filters.
    pub async fn files_list(&self, value: &Value) -> SlackApiResponse<Value> {
        self.http_post("files.list", value).await
    }
    // files.revokePublicURL
    // Revokes public/external sharing access for a file
    pub async fn files_revoke_public_url(&self, value: &Value) -> SlackApiResponse<Value> {
        self.http_post("files.revokePublicURL", value).await
    }

    // files.sharedPublicURL
    // Enables a file for public/external sharing.
    pub async fn files_shared_public_url(&self, value: &Value) -> SlackApiResponse<Value> {
        self.http_post("files.sharedPublicURL", value).await
    }

    // files.upload
    // Uploads or creates a file.
    /// https://api.slack.com/methods/chat.delete
    pub async fn files_upload(&self) -> SlackApiResponse<Value> {
        self.http_post_data("files.upload", Test {
            channels: "test-chanel".to_string(),
            file: "/Users/uiuifree/CLionProjects/rust-slack-web-api/test.txt".to_string(),
        }.to_multipart()).await
    }
    // files.remote.add
    // Adds a file from a remote service
    pub async fn files_remote_add(&self, value: &Value) -> SlackApiResponse<Value> {
        self.http_post("files.remote.add", value).await
    }

    // files.remote.info
    // Retrieve information about a remote file added to Slack
    pub async fn files_remote_info(&self, value: &Value) -> SlackApiResponse<Value> {
        self.http_post("files.remote.info", value).await
    }

    // files.remote.list
    // Retrieve information about a remote file added to Slack
    pub async fn files_remote_list(&self, value: &Value) -> SlackApiResponse<Value> {
        self.http_post("files.remote.list", value).await
    }

    // files.remote.remove
    // Remove a remote file.
    pub async fn files_remote_remove(&self, value: &Value) -> SlackApiResponse<Value> {
        self.http_post("files.remote.remove", value).await
    }

    // files.remote.share
    // Share a remote file into a channel.
    pub async fn files_remote_share(&self, value: &Value) -> SlackApiResponse<Value> {
        self.http_post("files.remote.share", value).await
    }
    // files.remote.update
    // Updates an existing remote file.
    pub async fn files_remote_update(&self, value: &Value) -> SlackApiResponse<Value> {
        self.http_post("files.remote.update", value).await
    }
}
