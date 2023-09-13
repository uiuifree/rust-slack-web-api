use crate::{BlockElementPlaneTextInput, CompositionObjectText, SlackBlockElement};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SlackBlock {
    Action(SlackBlockAction),
    Context(SlackBlockContext),
    Divider(SlackBlockDivider),
    File(SlackBlockFile),
    Header(SlackBlockHeader),
    Image(SlackBlockImage),
    Input(SlackBlockInput),
    Section(SlackBlockSection),
    Video(SlackBlockVideo),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackBlockAction {
    #[serde(rename = "type")]
    pub block_type: String,
    pub elements: Vec<SlackBlockElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Default for SlackBlockAction {
    fn default() -> Self {
        SlackBlockAction {
            block_type: "actions".to_string(),
            elements: vec![],
            block_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackBlockContext {
    #[serde(rename = "type")]
    pub block_type: String,
    pub elements: Vec<SlackBlockElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Default for SlackBlockContext {
    fn default() -> Self {
        SlackBlockContext {
            block_type: "context".to_string(),

            elements: vec![],
            block_id: None,
        }
    }
}
impl From<SlackBlockContext> for SlackBlock {
    fn from(value: SlackBlockContext) -> Self {
        SlackBlock::Context(value)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackBlockDivider {
    #[serde(rename = "type")]
    pub block_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Default for SlackBlockDivider {
    fn default() -> Self {
        SlackBlockDivider {
            block_type: "divider".to_string(),

            block_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackBlockFile {
    #[serde(rename = "type")]
    pub block_type: String,
    pub external_id: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Default for SlackBlockFile {
    fn default() -> Self {
        SlackBlockFile {
            block_type: "file".to_string(),

            external_id: "".to_string(),
            source: "".to_string(),
            block_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackBlockHeader {
    #[serde(rename = "type")]
    pub block_type: String,
    pub text: CompositionObjectText,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Default for SlackBlockHeader {
    fn default() -> Self {
        SlackBlockHeader {
            block_type: "header".to_string(),

            text: Default::default(),
            block_id: None,
        }
    }
}
impl From<SlackBlockHeader> for SlackBlock {
    fn from(value: SlackBlockHeader) -> Self {
        SlackBlock::Header(value)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackBlockImage {
    #[serde(rename = "type")]
    pub block_type: String,
    pub image_url: String,
    pub alt_text: String,
    pub title: BlockElementPlaneTextInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Default for SlackBlockImage {
    fn default() -> Self {
        SlackBlockImage {
            block_type: "image".to_string(),

            image_url: "".to_string(),
            alt_text: "".to_string(),
            title: Default::default(),
            block_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackBlockInput {
    #[serde(rename = "type")]
    pub block_type: String,
    pub label: BlockElementPlaneTextInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element: Option<SlackBlockElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatch_action: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<BlockElementPlaneTextInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

impl Default for SlackBlockInput {
    fn default() -> Self {
        SlackBlockInput {
            block_type: "input".to_string(),
            label: Default::default(),
            element: None,
            dispatch_action: None,
            block_id: None,
            hint: None,
            optional: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackBlockSection {
    #[serde(rename = "type")]
    pub block_type: String,
    pub text: BlockElementPlaneTextInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<BlockElementPlaneTextInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessory: Option<SlackBlockElement>,
}

impl Default for SlackBlockSection {
    fn default() -> Self {
        SlackBlockSection {
            block_type: "section".to_string(),

            text: Default::default(),
            block_id: None,
            fields: None,
            accessory: None,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SlackBlockVideo {
    pub alt_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<BlockElementPlaneTextInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<BlockElementPlaneTextInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_url: Option<String>,
    pub thumbnail_url: String,
    pub video_url: String,
}
