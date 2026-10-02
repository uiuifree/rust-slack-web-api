use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::block::Block;
use super::composition::TextObject;

/// A view for views.open / publish / update / push (the later fields are only returned by Slack). <https://docs.slack.dev/reference/views>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct View {
    pub r#type: ViewType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<TextObject>,
    pub blocks: Vec<Block>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_on_close: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_close: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ViewState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_view_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_view_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_installed_team_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
}

impl View {
    pub fn modal(title: impl Into<TextObject>, blocks: Vec<Block>) -> Self {
        Self {
            title: Some(title.into()),
            ..Self::with_type(ViewType::Modal, blocks)
        }
    }

    pub fn home(blocks: Vec<Block>) -> Self {
        Self::with_type(ViewType::Home, blocks)
    }

    pub fn workflow_step(blocks: Vec<Block>) -> Self {
        Self::with_type(ViewType::WorkflowStep, blocks)
    }

    fn with_type(r#type: ViewType, blocks: Vec<Block>) -> Self {
        Self {
            r#type,
            title: None,
            blocks,
            close: None,
            submit: None,
            private_metadata: None,
            callback_id: None,
            clear_on_close: None,
            notify_on_close: None,
            external_id: None,
            submit_disabled: None,
            id: None,
            team_id: None,
            state: None,
            hash: None,
            previous_view_id: None,
            root_view_id: None,
            app_id: None,
            app_installed_team_id: None,
            bot_id: None,
        }
    }

    pub fn title(mut self, title: impl Into<TextObject>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn close(mut self, close: impl Into<TextObject>) -> Self {
        self.close = Some(close.into());
        self
    }

    pub fn submit(mut self, submit: impl Into<TextObject>) -> Self {
        self.submit = Some(submit.into());
        self
    }

    pub fn private_metadata(mut self, private_metadata: impl Into<String>) -> Self {
        self.private_metadata = Some(private_metadata.into());
        self
    }

    pub fn callback_id(mut self, callback_id: impl Into<String>) -> Self {
        self.callback_id = Some(callback_id.into());
        self
    }

    pub fn clear_on_close(mut self, clear_on_close: bool) -> Self {
        self.clear_on_close = Some(clear_on_close);
        self
    }

    pub fn notify_on_close(mut self, notify_on_close: bool) -> Self {
        self.notify_on_close = Some(notify_on_close);
        self
    }

    pub fn external_id(mut self, external_id: impl Into<String>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }

    pub fn submit_disabled(mut self, submit_disabled: bool) -> Self {
        self.submit_disabled = Some(submit_disabled);
        self
    }
}

/// The type of a view. Unknown values go to `Other`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewType {
    Modal,
    Home,
    WorkflowStep,
    #[serde(untagged)]
    Other(String),
}

/// Current input values (`block_id` -> `action_id` -> value). Kept as raw JSON because the shape differs by element type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ViewState {
    pub values: BTreeMap<String, BTreeMap<String, serde_json::Value>>,
}
