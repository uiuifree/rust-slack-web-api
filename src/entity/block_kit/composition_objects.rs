use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SlackBlockKitCompositionObject {
    ConfirmationDialog(CompositionObjectConfirmationDialog),
    ConversationFilter(CompositionObjectConversationFilter),
    DispatchActionConfiguration(CompositionObjectDispatchActionConfiguration),
    InputParameter(CompositionObjectInputParameter),
    Option(CompositionObjectOption),
    OptionGroup(CompositionObjectOptionGroup),
    Text(CompositionObjectText),
    Trigger(CompositionObjectTrigger),
    Workflow(CompositionObjectWorkflow),
}

/// https://api.slack.com/reference/block-kit/composition-objects#confirm
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CompositionObjectConfirmationDialog {
    pub title: CompositionObjectText,
    pub text: CompositionObjectText,
    pub confirm: CompositionObjectText,
    pub deny: CompositionObjectText,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// https://api.slack.com/reference/block-kit/composition-objects#filter_conversations
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CompositionObjectConversationFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_external_shared_channels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_bot_users: Option<bool>,
}

/// https://api.slack.com/reference/block-kit/composition-objects#dispatch_action_config
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CompositionObjectDispatchActionConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_actions_on: Option<Vec<String>>,
}

/// https://api.slack.com/reference/block-kit/composition-objects#input_parameter
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CompositionObjectInputParameter {
    pub name: String,
    pub value: String,
}

/// https://api.slack.com/reference/block-kit/composition-objects#option
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CompositionObjectOption {
    pub text: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<CompositionObjectText>,
    pub url: String,
}

/// https://api.slack.com/reference/block-kit/composition-objects#option_group
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CompositionObjectOptionGroup {
    pub label: CompositionObjectText,
    pub options: Vec<CompositionObjectOption>,
}

/// https://api.slack.com/reference/block-kit/composition-objects#text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionObjectText {
    #[serde(rename = "type")]
    pub object_type: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub varbatim: Option<bool>,
}

impl Default for CompositionObjectText {
    fn default() -> Self {
        CompositionObjectText {
            object_type: "plain_text".to_string(),
            text: "".to_string(),
            emoji: None,
            varbatim: None,
        }
    }
}

impl CompositionObjectText {
    pub fn new(text: &str) -> Self {
        CompositionObjectText {
            text: text.to_string(),
            ..CompositionObjectText::default()
        }
    }
}

/// https://api.slack.com/reference/block-kit/composition-objects#trigger
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CompositionObjectTrigger {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customizable_input_parameters: Option<Vec<CompositionObjectInputParameter>>,
}

/// https://api.slack.com/reference/block-kit/composition-objects#workflow
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CompositionObjectWorkflow {
    pub trigger: CompositionObjectTrigger,
}
