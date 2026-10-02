//! # slack-web-api
//!
//! Typed async Rust client for **every Slack Web API method** — all 330 of them, including `admin.*` —
//! built on reqwest 0.13 and rustls (no OpenSSL).
//!
//! - One function per Web API method: `chat.postMessage` is [`SlackClient::chat_post_message`],
//!   `conversations.history` is [`SlackClient::conversations_history`], and so on. The full list is in [`api`].
//! - Request and response types for every method, core objects in [`objects`], and typed Block Kit in [`blocks`].
//! - One shared connection pool (HTTP/2), a 30 second default timeout, optional retry on HTTP 429,
//!   cursor pagination and file upload.
//!
//! This crate only calls the Web API. It does not receive events (Socket Mode, Events API, interactivity).
//!
//! ## Send a message
//!
//! ```no_run
//! use slack_web_api::api::ChatPostMessageRequest;
//! use slack_web_api::SlackClient;
//!
//! # async fn run() -> Result<(), slack_web_api::SlackError> {
//! let client = SlackClient::new("xoxb-your-bot-token");
//! let res = client
//!     .chat_post_message(&ChatPostMessageRequest::new("C0123456789").text("Hello from Rust"))
//!     .await?;
//! println!("posted at {:?}", res.ts);
//! # Ok(())
//! # }
//! ```
//!
//! Required arguments are the parameters of `new`; every optional argument is a method of the same name.
//!
//! ## Send a Block Kit message
//!
//! ```no_run
//! use slack_web_api::api::ChatPostMessageRequest;
//! use slack_web_api::blocks::{ActionsBlock, Block, ButtonElement, HeaderBlock, SectionBlock, TextObject};
//! # async fn run(client: slack_web_api::SlackClient) -> Result<(), slack_web_api::SlackError> {
//! let blocks = vec![
//!     Block::from(HeaderBlock::new(TextObject::plain("Deploy finished"))),
//!     Block::from(SectionBlock::new().text(TextObject::mrkdwn("*api-server* is now on `v2.3.0`"))),
//!     Block::from(ActionsBlock::new(vec![
//!         ButtonElement::new(TextObject::plain("Open")).url("https://example.com").into(),
//!     ])),
//! ];
//! client
//!     .chat_post_message(&ChatPostMessageRequest::new("C0123456789").text("Deploy finished").blocks(blocks))
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Read channel history with pagination
//!
//! ```no_run
//! use slack_web_api::api::ConversationsHistoryRequest;
//! # async fn run(client: slack_web_api::SlackClient) -> Result<(), slack_web_api::SlackError> {
//! let mut pages = client.pages(ConversationsHistoryRequest::new("C0123456789").limit(200));
//! while let Some(page) = pages.next_page().await {
//!     for message in page?.messages {
//!         println!("{:?}: {:?}", message.user, message.text);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Upload a file
//!
//! `files.upload` was retired on 2025-11-12. [`SlackClient::upload_files`] runs its replacement
//! (`files.getUploadURLExternal`, the upload, then `files.completeUploadExternal`).
//!
//! ```no_run
//! use slack_web_api::{FileUpload, UploadDestination};
//! # async fn run(client: slack_web_api::SlackClient) -> Result<(), slack_web_api::SlackError> {
//! client
//!     .upload_files(
//!         vec![FileUpload::new("report.csv", "date,count\n2026-10-01,42\n")],
//!         UploadDestination::channel("C0123456789").initial_comment("Daily report"),
//!     )
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Handle errors
//!
//! ```no_run
//! use slack_web_api::api::ConversationsInfoRequest;
//! use slack_web_api::SlackError;
//! # async fn run(client: slack_web_api::SlackClient) {
//! match client.conversations_info(&ConversationsInfoRequest::new("C0123456789")).await {
//!     Ok(res) => println!("{:?}", res.channel.and_then(|c| c.name)),
//!     Err(err) if err.api_error() == Some("channel_not_found") => println!("no such channel"),
//!     Err(SlackError::RateLimited { retry_after }) => println!("still rate limited: {retry_after:?}"),
//!     Err(err) => eprintln!("{err}"),
//! }
//! # }
//! ```
//!
//! ## Share the client
//!
//! [`SlackClient`] is cheap to clone and every clone shares one connection pool, so create it once.
//! To reuse your application's `reqwest::Client`, or to change the retry count or base URL, use the builder:
//!
//! ```no_run
//! use slack_web_api::SlackClient;
//!
//! let http = reqwest::Client::new();
//! let client = SlackClient::builder().http_client(http).token("xoxb-...").max_retries(5).build();
//! // Apps installed in many workspaces can switch the token and keep the pool.
//! let other_workspace = client.with_token("xoxb-other-workspace");
//! ```
//!
//! ## Methods without types
//!
//! [`SlackClient::call_raw`] calls any method by name with any parameters and returns the JSON as is.
//!
//! ## How requests and responses are handled
//!
//! - Requests are sent as `application/x-www-form-urlencoded`, which every method accepts. Lists of strings are
//!   sent comma-separated and objects (blocks, attachments) as JSON.
//! - The built-in HTTP client has a 30 second request timeout and a 10 second connect timeout. A client
//!   passed with [`SlackClientBuilder::http_client`] keeps its own settings.
//! - HTTP 429 is returned at once as [`SlackError::RateLimited`] with its `Retry-After`, because only the caller
//!   knows how long a request may wait. With [`SlackClientBuilder::max_retries`] the client waits for `Retry-After`
//!   and resends. Other failures are never retried, so a message is never posted twice.
//! - Response fields are all optional. Scalars are read leniently, because Slack sometimes returns the same field
//!   as a string in one place and a number in another; unknown fields are skipped.

pub mod api;
pub mod blocks;
mod client;
pub mod de;
mod error;
mod form;
pub mod objects;
mod response;
mod upload;

pub use client::{
    CursorPaginated, NextCursor, Pages, SlackApiMethod, SlackClient, SlackClientBuilder,
};
pub use error::{SlackApiError, SlackError};
pub use form::as_json;
pub use response::ResponseMetadata;
pub use upload::{FileUpload, UploadDestination};
