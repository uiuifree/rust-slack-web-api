use serde::{Deserialize, Serialize};

use super::tag::{tagged_enum, Tag, Tagged};

tagged_enum! {
    /// A text object (`plain_text` or `mrkdwn`). <https://docs.slack.dev/reference/block-kit/composition-objects/text-object>
    pub enum TextObject("type") {
        PlainText(PlainText),
        Mrkdwn(Mrkdwn),
    }
}

impl TextObject {
    pub fn plain(text: impl Into<String>) -> Self {
        PlainText::new(text).into()
    }

    pub fn mrkdwn(text: impl Into<String>) -> Self {
        Mrkdwn::new(text).into()
    }
}

/// A `plain_text` text object. <https://docs.slack.dev/reference/block-kit/composition-objects/text-object>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlainText {
    r#type: Tag<Self>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<bool>,
}

impl Tagged for PlainText {
    const TAG: &'static str = "plain_text";
}

impl PlainText {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            text: text.into(),
            emoji: None,
        }
    }

    pub fn emoji(mut self, emoji: bool) -> Self {
        self.emoji = Some(emoji);
        self
    }
}

/// A `mrkdwn` text object. <https://docs.slack.dev/reference/block-kit/composition-objects/text-object>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mrkdwn {
    r#type: Tag<Self>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbatim: Option<bool>,
}

impl Tagged for Mrkdwn {
    const TAG: &'static str = "mrkdwn";
}

impl Mrkdwn {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            text: text.into(),
            verbatim: None,
        }
    }

    pub fn verbatim(mut self, verbatim: bool) -> Self {
        self.verbatim = Some(verbatim);
        self
    }
}

/// A confirmation dialog. <https://docs.slack.dev/reference/block-kit/composition-objects/confirmation-dialog-object>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfirmationDialog {
    pub title: TextObject,
    pub text: TextObject,
    pub confirm: TextObject,
    pub deny: TextObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ButtonStyle>,
}

impl ConfirmationDialog {
    pub fn new(
        title: impl Into<TextObject>,
        text: impl Into<TextObject>,
        confirm: impl Into<TextObject>,
        deny: impl Into<TextObject>,
    ) -> Self {
        Self {
            title: title.into(),
            text: text.into(),
            confirm: confirm.into(),
            deny: deny.into(),
            style: None,
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// An option. <https://docs.slack.dev/reference/block-kit/composition-objects/option-object>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionObject {
    pub text: TextObject,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl OptionObject {
    pub fn new(text: impl Into<TextObject>, value: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            value: value.into(),
            description: None,
            url: None,
        }
    }

    pub fn description(mut self, description: impl Into<TextObject>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

/// A group of options. <https://docs.slack.dev/reference/block-kit/composition-objects/option-group-object>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionGroup {
    pub label: TextObject,
    pub options: Vec<OptionObject>,
}

impl OptionGroup {
    pub fn new(label: impl Into<TextObject>, options: Vec<OptionObject>) -> Self {
        Self {
            label: label.into(),
            options,
        }
    }
}

/// A filter for conversation select menus. <https://docs.slack.dev/reference/block-kit/composition-objects/conversation-filter-object>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationFilter {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<ConversationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_external_shared_channels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_bot_users: Option<bool>,
}

impl ConversationFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn include(mut self, include: Vec<ConversationType>) -> Self {
        self.include = include;
        self
    }

    pub fn exclude_external_shared_channels(
        mut self,
        exclude_external_shared_channels: bool,
    ) -> Self {
        self.exclude_external_shared_channels = Some(exclude_external_shared_channels);
        self
    }

    pub fn exclude_bot_users(mut self, exclude_bot_users: bool) -> Self {
        self.exclude_bot_users = Some(exclude_bot_users);
        self
    }
}

/// Configures when typing in an input dispatches a `block_actions` payload. <https://docs.slack.dev/reference/block-kit/composition-objects/dispatch-action-configuration-object>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DispatchActionConfig {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trigger_actions_on: Vec<TriggerAction>,
}

impl DispatchActionConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trigger_actions_on(mut self, trigger_actions_on: Vec<TriggerAction>) -> Self {
        self.trigger_actions_on = trigger_actions_on;
        self
    }
}

/// A Slack file used as an image (either `url` or `id`). <https://docs.slack.dev/reference/block-kit/composition-objects/slack-file-object>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlackFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl SlackFile {
    pub fn from_url(url: impl Into<String>) -> Self {
        Self {
            url: Some(url.into()),
            id: None,
        }
    }

    pub fn from_id(id: impl Into<String>) -> Self {
        Self {
            url: None,
            id: Some(id.into()),
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// A built-in Slack icon. <https://docs.slack.dev/reference/block-kit/composition-objects/slack-icon-object>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlackIcon {
    r#type: Tag<Self>,
    pub name: String,
}

impl Tagged for SlackIcon {
    const TAG: &'static str = "icon";
}

impl SlackIcon {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            name: name.into(),
        }
    }
}

/// The workflow run by a workflow button. <https://docs.slack.dev/reference/block-kit/composition-objects/workflow-object>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    pub trigger: Trigger,
}

impl Workflow {
    pub fn new(trigger: Trigger) -> Self {
        Self { trigger }
    }
}

/// A workflow link trigger. <https://docs.slack.dev/reference/block-kit/composition-objects/trigger-object>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trigger {
    pub url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub customizable_input_parameters: Vec<InputParameter>,
}

impl Trigger {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            customizable_input_parameters: Vec::new(),
        }
    }

    pub fn customizable_input_parameters(
        mut self,
        customizable_input_parameters: Vec<InputParameter>,
    ) -> Self {
        self.customizable_input_parameters = customizable_input_parameters;
        self
    }
}

/// An input parameter passed to a trigger. <https://docs.slack.dev/reference/block-kit/composition-objects/trigger-object>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputParameter {
    pub name: String,
    pub value: String,
}

impl InputParameter {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

/// The color scheme of a button. Unknown values go to `Other`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ButtonStyle {
    Primary,
    Danger,
    #[serde(untagged)]
    Other(String),
}

/// A conversation type. Unknown values go to `Other`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConversationType {
    Im,
    Mpim,
    Private,
    Public,
    #[serde(untagged)]
    Other(String),
}

/// An event that dispatches a `block_actions` payload. Unknown values go to `Other`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerAction {
    OnEnterPressed,
    OnCharacterEntered,
    #[serde(untagged)]
    Other(String),
}
