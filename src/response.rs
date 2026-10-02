use serde::{Deserialize, Serialize};

/// The `response_metadata` of a response: the pagination cursor plus warning and error details.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ResponseMetadata {
    /// Cursor for the next page. Empty or absent on the last page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Warning codes such as `missing_charset`.
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::de::vec"
    )]
    pub warnings: Vec<String>,
    /// Detailed reasons for an error or warning (for example, what is wrong with `invalid_blocks`).
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::de::vec"
    )]
    pub messages: Vec<String>,
    /// Scopes granted to the token.
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::de::vec"
    )]
    pub scopes: Vec<String>,
    /// Scopes accepted by the method.
    #[serde(
        rename = "acceptedScopes",
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::de::vec"
    )]
    pub accepted_scopes: Vec<String>,
}
