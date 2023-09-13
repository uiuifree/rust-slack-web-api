pub mod api;
mod entity;
mod error;

pub use error::*;

pub use entity::*;
use hyper::http::request::Builder;
use hyper::{Body, Method};
use hyper_tls::HttpsConnector;

use serde_json::{json, Value};

pub struct SlackClient {
    pub(crate) context: SlackContext,
}

impl SlackClient {
    pub fn new(token: &str) -> Self {
        Self {
            context: SlackContext {
                token: Some(token.to_string()),
            },
        }
    }
}

struct SlackContext {
    token: Option<String>,
}

pub type SlackApiResponse<T> = Result<T, SlackError>;

impl SlackClient {
    async fn http_response<R>(response: hyper::Result<hyper::Response<Body>>) -> SlackApiResponse<R>
        where
            R: for<'de> serde::Deserialize<'de>,
    {
        let response = match response {
            Ok(v) => v,
            Err(e) => return Err(SlackSystemError::new(e.to_string()).into()),
        };

        let status = response.status();
        let body = match hyper::body::to_bytes(response.into_body()).await {
            Ok(v) => v.to_vec(),
            Err(e) => {
                return Err(SlackHttpError::new(status.as_u16(), e.message().to_string()).into());
            }
        };

        let http_response_body = match String::from_utf8(body) {
            Ok(v) => v,
            Err(e) => return Err(SlackSystemError::new(e.to_string()).into()),
        };

        let value = match serde_json::from_str::<Value>(http_response_body.as_str()) {
            Ok(v) => v,
            Err(e) => return Err(SlackSystemError::new(e.to_string()).into()),
        };
        dbg!(&value);

        if let Some(ok) = value.get("ok") {
            if ok.as_bool() == Some(false) {
                return Err(SlackApiError {
                    status: status.as_u16(),
                    errors: Some(get_error_value(&value)),
                    warnings: None,
                    http_response_body: Some(http_response_body),
                }
                    .into());
            }
        }

        Ok(serde_json::from_value(value).unwrap())
    }
    pub(crate) async fn http_get<P, R>(
        &self,
        token: &str,
        url: &str,
        value: &P,
    ) -> SlackApiResponse<R>
        where
            P: serde::Serialize,
            R: for<'de> serde::Deserialize<'de>,
    {
        let build = builder(url, token).method(Method::GET);
        let request = build.body(Body::from(json!(value).to_string())).unwrap();
        let client = hyper::Client::builder().build(HttpsConnector::new());

        let response = client.request(request).await;
        SlackClient::http_response(response).await
        // let body = hyper::body::to_bytes(response.into_body())
        //     .await
        //     .unwrap()
        //     .to_vec();
        // let body = String::from_utf8(body).unwrap();
        // let value = serde_json::from_str::<Value>(body.as_str());
        // dbg!(value);
        //
        // Err(SlackError::Http(SlackHttpError {
        //     status: 0,
        //     http_response_body: None,
        // }))
    }
    pub(crate) async fn http_post<P, R>(&self, url: &str, value: &P) -> SlackApiResponse<R>
        where
            P: serde::Serialize,
            R: for<'de> serde::Deserialize<'de>,
    {
        println!("hoge1");
        let build = builder(url, self.context.token.clone().unwrap_or_default().as_str()).method(Method::POST);
        println!("hoge2 {}", serde_json::to_string(value).unwrap());
        let request = build.body(Body::from(json!(value).to_string())).unwrap();
        println!("hoge3");
        let response = hyper::Client::builder()
            .build(HttpsConnector::new())
            .request(request)
            .await;
        println!("hoge4");
        Self::http_response(response).await
    }
}

fn builder(url: &str, token: &str) -> Builder {
    let mut build = hyper::Request::builder().uri(format!("https://slack.com/api/{}", url));
    if !token.is_empty() {
        build = build.header("Authorization", format!("Bearer {}", token))
    }
    build.header("Content-type", "application/json; charset=UTF-8")
}

fn get_error_value(value: &Value) -> Vec<String> {
    match value.get("error") {
        None => {
            vec![]
        }
        Some(v) => {
            if v.is_string() {
                return match v.as_str() {
                    None => {
                        vec![]
                    }
                    Some(v) => {
                        vec![v.to_string()]
                    }
                };
            }
            vec![]
        }
    }
}
