use crate::{SlackApiError, SlackError};
use bytes::Bytes;
use reqwest::header::{CONTENT_TYPE, RETRY_AFTER};
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://slack.com/api/";
const DEFAULT_MAX_RETRIES: u32 = 3;
// 429 に Retry-After が無いときの待ち時間
const DEFAULT_RETRY_AFTER: Duration = Duration::from_secs(1);
const FORM_CONTENT_TYPE: &str = "application/x-www-form-urlencoded";

/// A request for one Web API method. Implemented by every generated `*Request` type.
pub trait SlackApiMethod: Serialize {
    /// Method name, such as `chat.postMessage`.
    const METHOD: &'static str;
    /// Type of a successful response.
    type Response: DeserializeOwned;
}

/// A request that supports cursor-based pagination.
pub trait CursorPaginated: SlackApiMethod<Response: NextCursor> {
    fn set_cursor(&mut self, cursor: String);
}

/// A response that carries the cursor for the next page.
pub trait NextCursor {
    /// Cursor for the next page, or `None` on the last page.
    fn next_cursor(&self) -> Option<&str>;
}

/// Slack Web API client.
///
/// Clones share the underlying HTTP client and its connection pool, so cloning is cheap.
/// Create one per application. To call with another token, use [`SlackClient::with_token`].
///
/// Every Web API method has a function here named after it (`chat.postMessage` is
/// [`SlackClient::chat_post_message`]); see [`crate::api`] for the full list.
///
/// ```no_run
/// use slack_web_api::api::UsersInfoRequest;
/// use slack_web_api::SlackClient;
///
/// # async fn run() -> Result<(), slack_web_api::SlackError> {
/// let client = SlackClient::new(std::env::var("SLACK_BOT_TOKEN").unwrap());
/// let user = client.users_info(&UsersInfoRequest::new().user("U0123456789")).await?;
/// println!("{:?}", user.user.and_then(|u| u.real_name));
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct SlackClient {
    inner: Arc<Inner>,
}

struct Inner {
    http: reqwest::Client,
    token: Option<String>,
    base_url: String,
    max_retries: u32,
}

impl std::fmt::Debug for SlackClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SlackClient")
            .field("token", &self.inner.token.as_ref().map(|_| "***"))
            .field("base_url", &self.inner.base_url)
            .field("max_retries", &self.inner.max_retries)
            .finish()
    }
}

impl SlackClient {
    /// Creates a client that authenticates with `token` (`xoxb-`, `xoxp-`, ...).
    pub fn new(token: impl Into<String>) -> Self {
        Self::builder().token(token).build()
    }

    pub fn builder() -> SlackClientBuilder {
        SlackClientBuilder::default()
    }

    /// Returns a client that uses another token but shares this client's connection pool and settings.
    /// Useful for apps installed in many workspaces.
    pub fn with_token(&self, token: impl Into<String>) -> Self {
        Self {
            inner: Arc::new(Inner {
                http: self.inner.http.clone(),
                token: Some(token.into()),
                base_url: self.inner.base_url.clone(),
                max_retries: self.inner.max_retries,
            }),
        }
    }

    /// Calls a method with a typed request. The per-method functions such as `chat_post_message` call this.
    ///
    /// # Errors
    ///
    /// Returns [`SlackError::Api`] when Slack answers `"ok": false`, [`SlackError::RateLimited`] when
    /// 429 persists past the retry limit, and transport, HTTP or decode errors otherwise.
    pub async fn call<M: SlackApiMethod>(&self, request: &M) -> Result<M::Response, SlackError> {
        let (status, body) = self.post(M::METHOD, request).await?;
        decode(status, &body)
    }

    /// Calls a method by name with arbitrary parameters and returns the raw JSON.
    /// Use it for methods or arguments that the generated types do not cover yet.
    ///
    /// # Errors
    ///
    /// Same as [`SlackClient::call`], plus [`SlackError::Encode`] when `params` is not a struct or map.
    ///
    /// ```no_run
    /// # async fn run(client: slack_web_api::SlackClient) -> Result<(), slack_web_api::SlackError> {
    /// let value = client
    ///     .call_raw("chat.postMessage", &serde_json::json!({"channel": "C0123456789", "text": "hi"}))
    ///     .await?;
    /// println!("{}", value["ts"]);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn call_raw<P: Serialize + ?Sized>(
        &self,
        method: &str,
        params: &P,
    ) -> Result<serde_json::Value, SlackError> {
        let (status, body) = self.post(method, params).await?;
        decode(status, &body)
    }

    /// Iterates over the pages of a cursor-paginated method.
    /// Drive it with `while let Some(page) = pages.next_page().await`.
    pub fn pages<M: CursorPaginated>(&self, request: M) -> Pages<'_, M> {
        Pages {
            client: self,
            request: Some(request),
        }
    }

    // JSON ではなくファイルそのものを返すメソッド用（`admin.analytics.getFile`）。
    // 失敗時は他のメソッドと同じく JSON のエラーが返る
    pub(crate) async fn call_bytes<P: Serialize + ?Sized>(
        &self,
        method: &str,
        params: &P,
    ) -> Result<Bytes, SlackError> {
        let (status, body) = self.post(method, params).await?;
        if is_json(&body) {
            decode::<serde::de::IgnoredAny>(status, &body)?;
        }
        if !status.is_success() {
            return Err(SlackError::Http {
                status: status.as_u16(),
                body: String::from_utf8_lossy(&body).into_owned(),
            });
        }
        Ok(body)
    }

    pub(crate) fn http(&self) -> &reqwest::Client {
        &self.inner.http
    }

    // フォーム本文で POST し、429 なら Retry-After だけ待って再送する
    async fn post<P: Serialize + ?Sized>(
        &self,
        method: &str,
        params: &P,
    ) -> Result<(StatusCode, Bytes), SlackError> {
        let form = crate::form::to_form(params).map_err(|e| SlackError::Encode(e.to_string()))?;
        let body = Bytes::from(form);
        let url = format!("{}{}", self.inner.base_url, method);
        let mut attempt = 0;
        loop {
            let mut request = self
                .inner
                .http
                .post(&url)
                .header(CONTENT_TYPE, FORM_CONTENT_TYPE)
                .body(body.clone());
            if let Some(token) = &self.inner.token {
                request = request.bearer_auth(token);
            }
            let response = request.send().await?;
            let status = response.status();
            if status == StatusCode::TOO_MANY_REQUESTS {
                let wait = retry_after(&response);
                if attempt >= self.inner.max_retries {
                    return Err(SlackError::RateLimited { retry_after: wait });
                }
                attempt += 1;
                tokio::time::sleep(wait.unwrap_or(DEFAULT_RETRY_AFTER)).await;
                continue;
            }
            return Ok((status, response.bytes().await?));
        }
    }
}

/// Builder for [`SlackClient`].
#[derive(Default)]
pub struct SlackClientBuilder {
    http: Option<reqwest::Client>,
    token: Option<String>,
    base_url: Option<String>,
    max_retries: Option<u32>,
}

impl SlackClientBuilder {
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Uses an existing `reqwest::Client` so the application and Slack share one connection pool.
    /// Defaults to `reqwest::Client::default()`.
    pub fn http_client(mut self, http: reqwest::Client) -> Self {
        self.http = Some(http);
        self
    }

    /// Base URL of the API (default `https://slack.com/api/`). Set it for GovSlack
    /// (`https://slack-gov.com/api/`) or for a mock server in tests.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        let mut base_url = base_url.into();
        if !base_url.ends_with('/') {
            base_url.push('/');
        }
        self.base_url = Some(base_url);
        self
    }

    /// How many times a request is resent after a 429, waiting for `Retry-After` each time
    /// (default 3, `0` disables retries).
    /// Other failures (5xx, network errors) are never retried, to avoid posting twice.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    pub fn build(self) -> SlackClient {
        SlackClient {
            inner: Arc::new(Inner {
                http: self.http.unwrap_or_default(),
                token: self.token,
                base_url: self.base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_owned()),
                max_retries: self.max_retries.unwrap_or(DEFAULT_MAX_RETRIES),
            }),
        }
    }
}

/// Cursor pagination over a method. Created by [`SlackClient::pages`].
pub struct Pages<'a, M> {
    client: &'a SlackClient,
    // 次に送るリクエスト。最後のページを受け取ったら None
    request: Option<M>,
}

impl<M: CursorPaginated> Pages<'_, M> {
    /// Fetches the next page. Returns `None` after the last page and after an error.
    pub async fn next_page(&mut self) -> Option<Result<M::Response, SlackError>> {
        let mut request = self.request.take()?;
        let result = self.client.call(&request).await;
        if let Ok(response) = &result {
            if let Some(cursor) = response.next_cursor().filter(|c| !c.is_empty()) {
                request.set_cursor(cursor.to_owned());
                self.request = Some(request);
            }
        }
        Some(result)
    }
}

// 応答の `ok` と失敗時の詳細だけを先に読む（他の項目は読み飛ばす）
#[derive(Deserialize)]
struct Envelope {
    ok: bool,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    warning: Option<String>,
    #[serde(default)]
    response_metadata: Option<crate::ResponseMetadata>,
}

fn decode<R: DeserializeOwned>(status: StatusCode, body: &[u8]) -> Result<R, SlackError> {
    let envelope: Envelope = match serde_json::from_slice(body) {
        Ok(envelope) => envelope,
        Err(source) => {
            let body = String::from_utf8_lossy(body).into_owned();
            if !status.is_success() {
                return Err(SlackError::Http {
                    status: status.as_u16(),
                    body,
                });
            }
            return Err(SlackError::Decode { source, body });
        }
    };
    if !envelope.ok {
        return Err(SlackError::Api(Box::new(SlackApiError {
            error: envelope.error.unwrap_or_default(),
            warning: envelope.warning,
            response_metadata: envelope.response_metadata,
        })));
    }
    serde_json::from_slice(body).map_err(|source| SlackError::Decode {
        source,
        body: String::from_utf8_lossy(body).into_owned(),
    })
}

fn is_json(body: &[u8]) -> bool {
    body.iter().find(|b| !b.is_ascii_whitespace()) == Some(&b'{')
}

fn retry_after(response: &Response) -> Option<Duration> {
    let secs = response
        .headers()
        .get(RETRY_AFTER)?
        .to_str()
        .ok()?
        .trim()
        .parse::<u64>()
        .ok()?;
    Some(Duration::from_secs(secs))
}
