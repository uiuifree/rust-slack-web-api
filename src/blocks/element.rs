use serde::{Deserialize, Serialize};

use super::composition::{
    ButtonStyle, ConfirmationDialog, DispatchActionConfig, Mrkdwn, OptionObject, PlainText,
    SlackFile, TextObject, Workflow,
};
use super::rich_text::RichTextBlock;
use super::select::{
    ChannelsSelectElement, ConversationsSelectElement, ExternalSelectElement,
    MultiChannelsSelectElement, MultiConversationsSelectElement, MultiExternalSelectElement,
    MultiStaticSelectElement, MultiUsersSelectElement, StaticSelectElement, UsersSelectElement,
};
use super::tag::{tagged_enum, Tag, Tagged};

tagged_enum! {
    /// An element allowed in an actions block. <https://docs.slack.dev/reference/block-kit/blocks/actions-block>
    pub enum ActionsElement("type") {
        Button(ButtonElement),
        Checkboxes(CheckboxesElement),
        DatePicker(DatePickerElement),
        DatetimePicker(DatetimePickerElement),
        MultiStaticSelect(MultiStaticSelectElement),
        MultiExternalSelect(MultiExternalSelectElement),
        MultiUsersSelect(MultiUsersSelectElement),
        MultiConversationsSelect(MultiConversationsSelectElement),
        MultiChannelsSelect(MultiChannelsSelectElement),
        Overflow(OverflowElement),
        RadioButtons(RadioButtonsElement),
        RichTextInput(RichTextInputElement),
        StaticSelect(StaticSelectElement),
        ExternalSelect(ExternalSelectElement),
        UsersSelect(UsersSelectElement),
        ConversationsSelect(ConversationsSelectElement),
        ChannelsSelect(ChannelsSelectElement),
        TimePicker(TimePickerElement),
        WorkflowButton(WorkflowButtonElement),
    }
}

tagged_enum! {
    /// An element allowed as the `accessory` of a section block. <https://docs.slack.dev/reference/block-kit/blocks/section-block>
    pub enum SectionAccessory("type") {
        Button(ButtonElement),
        Checkboxes(CheckboxesElement),
        DatePicker(DatePickerElement),
        Image(ImageElement),
        MultiStaticSelect(MultiStaticSelectElement),
        MultiExternalSelect(MultiExternalSelectElement),
        MultiUsersSelect(MultiUsersSelectElement),
        MultiConversationsSelect(MultiConversationsSelectElement),
        MultiChannelsSelect(MultiChannelsSelectElement),
        Overflow(OverflowElement),
        RadioButtons(RadioButtonsElement),
        StaticSelect(StaticSelectElement),
        ExternalSelect(ExternalSelectElement),
        UsersSelect(UsersSelectElement),
        ConversationsSelect(ConversationsSelectElement),
        ChannelsSelect(ChannelsSelectElement),
        TimePicker(TimePickerElement),
        WorkflowButton(WorkflowButtonElement),
    }
}

tagged_enum! {
    /// An element allowed in an input block. <https://docs.slack.dev/reference/block-kit/blocks/input-block>
    pub enum InputElement("type") {
        Checkboxes(CheckboxesElement),
        DatePicker(DatePickerElement),
        DatetimePicker(DatetimePickerElement),
        EmailInput(EmailInputElement),
        FileInput(FileInputElement),
        MultiStaticSelect(MultiStaticSelectElement),
        MultiExternalSelect(MultiExternalSelectElement),
        MultiUsersSelect(MultiUsersSelectElement),
        MultiConversationsSelect(MultiConversationsSelectElement),
        MultiChannelsSelect(MultiChannelsSelectElement),
        NumberInput(NumberInputElement),
        PlainTextInput(PlainTextInputElement),
        RadioButtons(RadioButtonsElement),
        RichTextInput(RichTextInputElement),
        StaticSelect(StaticSelectElement),
        ExternalSelect(ExternalSelectElement),
        UsersSelect(UsersSelectElement),
        ConversationsSelect(ConversationsSelectElement),
        ChannelsSelect(ChannelsSelectElement),
        TimePicker(TimePickerElement),
        UrlInput(UrlInputElement),
    }
}

tagged_enum! {
    /// An element allowed in a context block (images and text). <https://docs.slack.dev/reference/block-kit/blocks/context-block>
    pub enum ContextElement("type") {
        Image(ImageElement),
        PlainText(PlainText),
        Mrkdwn(Mrkdwn),
    }
}

tagged_enum! {
    /// An element allowed in a context actions block. <https://docs.slack.dev/reference/block-kit/blocks/context-actions-block>
    pub enum ContextActionsElement("type") {
        FeedbackButtons(FeedbackButtonsElement),
        IconButton(IconButtonElement),
    }
}

/// A button. <https://docs.slack.dev/reference/block-kit/block-elements/button-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ButtonElement {
    r#type: Tag<Self>,
    pub text: TextObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ButtonStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_prompt_display: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub visible_to_user_ids: Vec<String>,
}

impl Tagged for ButtonElement {
    const TAG: &'static str = "button";
}

impl ButtonElement {
    pub fn new(text: impl Into<TextObject>) -> Self {
        Self {
            r#type: Tag::default(),
            text: text.into(),
            action_id: None,
            url: None,
            value: None,
            style: None,
            confirm: None,
            accessibility_label: None,
            agent_prompt: None,
            agent_prompt_display: None,
            visible_to_user_ids: Vec::new(),
        }
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn accessibility_label(mut self, accessibility_label: impl Into<String>) -> Self {
        self.accessibility_label = Some(accessibility_label.into());
        self
    }

    pub fn agent_prompt(mut self, agent_prompt: impl Into<String>) -> Self {
        self.agent_prompt = Some(agent_prompt.into());
        self
    }

    pub fn agent_prompt_display(mut self, agent_prompt_display: impl Into<String>) -> Self {
        self.agent_prompt_display = Some(agent_prompt_display.into());
        self
    }

    pub fn visible_to_user_ids(mut self, visible_to_user_ids: Vec<String>) -> Self {
        self.visible_to_user_ids = visible_to_user_ids;
        self
    }
}

/// A group of checkboxes. <https://docs.slack.dev/reference/block-kit/block-elements/checkboxes-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckboxesElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    pub options: Vec<OptionObject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial_options: Vec<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub visible_to_user_ids: Vec<String>,
}

impl Tagged for CheckboxesElement {
    const TAG: &'static str = "checkboxes";
}

impl CheckboxesElement {
    pub fn new(options: Vec<OptionObject>) -> Self {
        Self {
            r#type: Tag::default(),
            action_id: None,
            options,
            initial_options: Vec::new(),
            confirm: None,
            focus_on_load: None,
            visible_to_user_ids: Vec::new(),
        }
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn initial_options(mut self, initial_options: Vec<OptionObject>) -> Self {
        self.initial_options = initial_options;
        self
    }

    pub fn confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn focus_on_load(mut self, focus_on_load: bool) -> Self {
        self.focus_on_load = Some(focus_on_load);
        self
    }

    pub fn visible_to_user_ids(mut self, visible_to_user_ids: Vec<String>) -> Self {
        self.visible_to_user_ids = visible_to_user_ids;
        self
    }
}

/// A date picker. <https://docs.slack.dev/reference/block-kit/block-elements/date-picker-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DatePickerElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl Tagged for DatePickerElement {
    const TAG: &'static str = "datepicker";
}

impl DatePickerElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn initial_date(mut self, initial_date: impl Into<String>) -> Self {
        self.initial_date = Some(initial_date.into());
        self
    }

    pub fn confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn focus_on_load(mut self, focus_on_load: bool) -> Self {
        self.focus_on_load = Some(focus_on_load);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<TextObject>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }
}

/// A date and time picker. <https://docs.slack.dev/reference/block-kit/block-elements/datetime-picker-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DatetimePickerElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_date_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
}

impl Tagged for DatetimePickerElement {
    const TAG: &'static str = "datetimepicker";
}

impl DatetimePickerElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn initial_date_time(mut self, initial_date_time: i64) -> Self {
        self.initial_date_time = Some(initial_date_time);
        self
    }

    pub fn confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn focus_on_load(mut self, focus_on_load: bool) -> Self {
        self.focus_on_load = Some(focus_on_load);
        self
    }
}

/// An email input. <https://docs.slack.dev/reference/block-kit/block-elements/email-input-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EmailInputElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatch_action_config: Option<DispatchActionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl Tagged for EmailInputElement {
    const TAG: &'static str = "email_text_input";
}

impl EmailInputElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn initial_value(mut self, initial_value: impl Into<String>) -> Self {
        self.initial_value = Some(initial_value.into());
        self
    }

    pub fn dispatch_action_config(mut self, dispatch_action_config: DispatchActionConfig) -> Self {
        self.dispatch_action_config = Some(dispatch_action_config);
        self
    }

    pub fn focus_on_load(mut self, focus_on_load: bool) -> Self {
        self.focus_on_load = Some(focus_on_load);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<TextObject>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }
}

/// Positive and negative feedback buttons. <https://docs.slack.dev/reference/block-kit/block-elements/feedback-buttons-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeedbackButtonsElement {
    r#type: Tag<Self>,
    pub positive_button: FeedbackButton,
    pub negative_button: FeedbackButton,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
}

impl Tagged for FeedbackButtonsElement {
    const TAG: &'static str = "feedback_buttons";
}

impl FeedbackButtonsElement {
    pub fn new(positive_button: FeedbackButton, negative_button: FeedbackButton) -> Self {
        Self {
            r#type: Tag::default(),
            positive_button,
            negative_button,
            action_id: None,
        }
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }
}

/// One of the feedback buttons. <https://docs.slack.dev/reference/block-kit/block-elements/feedback-buttons-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeedbackButton {
    pub text: TextObject,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility_label: Option<String>,
}

impl FeedbackButton {
    pub fn new(text: impl Into<TextObject>, value: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            value: value.into(),
            accessibility_label: None,
        }
    }

    pub fn accessibility_label(mut self, accessibility_label: impl Into<String>) -> Self {
        self.accessibility_label = Some(accessibility_label.into());
        self
    }
}

/// A file upload input. <https://docs.slack.dev/reference/block-kit/block-elements/file-input-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FileInputElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filetypes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_files: Option<u32>,
}

impl Tagged for FileInputElement {
    const TAG: &'static str = "file_input";
}

impl FileInputElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn filetypes(mut self, filetypes: Vec<String>) -> Self {
        self.filetypes = filetypes;
        self
    }

    pub fn max_files(mut self, max_files: u32) -> Self {
        self.max_files = Some(max_files);
        self
    }
}

/// An icon button. <https://docs.slack.dev/reference/block-kit/block-elements/icon-button-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IconButtonElement {
    r#type: Tag<Self>,
    pub icon: String,
    pub text: TextObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility_label: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub visible_to_user_ids: Vec<String>,
}

impl Tagged for IconButtonElement {
    const TAG: &'static str = "icon_button";
}

impl IconButtonElement {
    pub fn new(icon: impl Into<String>, text: impl Into<TextObject>) -> Self {
        Self {
            r#type: Tag::default(),
            icon: icon.into(),
            text: text.into(),
            action_id: None,
            value: None,
            confirm: None,
            accessibility_label: None,
            visible_to_user_ids: Vec::new(),
        }
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn accessibility_label(mut self, accessibility_label: impl Into<String>) -> Self {
        self.accessibility_label = Some(accessibility_label.into());
        self
    }

    pub fn visible_to_user_ids(mut self, visible_to_user_ids: Vec<String>) -> Self {
        self.visible_to_user_ids = visible_to_user_ids;
        self
    }
}

/// An image element (either `image_url` or `slack_file`). <https://docs.slack.dev/reference/block-kit/block-elements/image-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageElement {
    r#type: Tag<Self>,
    pub alt_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_file: Option<SlackFile>,
}

impl Tagged for ImageElement {
    const TAG: &'static str = "image";
}

impl ImageElement {
    pub fn new(image_url: impl Into<String>, alt_text: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            alt_text: alt_text.into(),
            image_url: Some(image_url.into()),
            slack_file: None,
        }
    }

    pub fn from_slack_file(slack_file: SlackFile, alt_text: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            alt_text: alt_text.into(),
            image_url: None,
            slack_file: Some(slack_file),
        }
    }

    pub fn image_url(mut self, image_url: impl Into<String>) -> Self {
        self.image_url = Some(image_url.into());
        self
    }

    pub fn slack_file(mut self, slack_file: SlackFile) -> Self {
        self.slack_file = Some(slack_file);
        self
    }
}

/// A number input. <https://docs.slack.dev/reference/block-kit/block-elements/number-input-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberInputElement {
    r#type: Tag<Self>,
    pub is_decimal_allowed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatch_action_config: Option<DispatchActionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl Tagged for NumberInputElement {
    const TAG: &'static str = "number_input";
}

impl NumberInputElement {
    pub fn new(is_decimal_allowed: bool) -> Self {
        Self {
            r#type: Tag::default(),
            is_decimal_allowed,
            action_id: None,
            initial_value: None,
            min_value: None,
            max_value: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
        }
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn initial_value(mut self, initial_value: impl Into<String>) -> Self {
        self.initial_value = Some(initial_value.into());
        self
    }

    pub fn min_value(mut self, min_value: impl Into<String>) -> Self {
        self.min_value = Some(min_value.into());
        self
    }

    pub fn max_value(mut self, max_value: impl Into<String>) -> Self {
        self.max_value = Some(max_value.into());
        self
    }

    pub fn dispatch_action_config(mut self, dispatch_action_config: DispatchActionConfig) -> Self {
        self.dispatch_action_config = Some(dispatch_action_config);
        self
    }

    pub fn focus_on_load(mut self, focus_on_load: bool) -> Self {
        self.focus_on_load = Some(focus_on_load);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<TextObject>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }
}

/// An overflow menu. <https://docs.slack.dev/reference/block-kit/block-elements/overflow-menu-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverflowElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    pub options: Vec<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
}

impl Tagged for OverflowElement {
    const TAG: &'static str = "overflow";
}

impl OverflowElement {
    pub fn new(options: Vec<OptionObject>) -> Self {
        Self {
            r#type: Tag::default(),
            action_id: None,
            options,
            confirm: None,
        }
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }
}

/// A plain-text input. <https://docs.slack.dev/reference/block-kit/block-elements/plain-text-input-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PlainTextInputElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatch_action_config: Option<DispatchActionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl Tagged for PlainTextInputElement {
    const TAG: &'static str = "plain_text_input";
}

impl PlainTextInputElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn initial_value(mut self, initial_value: impl Into<String>) -> Self {
        self.initial_value = Some(initial_value.into());
        self
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = Some(multiline);
        self
    }

    pub fn min_length(mut self, min_length: u32) -> Self {
        self.min_length = Some(min_length);
        self
    }

    pub fn max_length(mut self, max_length: u32) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn dispatch_action_config(mut self, dispatch_action_config: DispatchActionConfig) -> Self {
        self.dispatch_action_config = Some(dispatch_action_config);
        self
    }

    pub fn focus_on_load(mut self, focus_on_load: bool) -> Self {
        self.focus_on_load = Some(focus_on_load);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<TextObject>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }
}

/// A radio button group. <https://docs.slack.dev/reference/block-kit/block-elements/radio-button-group-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadioButtonsElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    pub options: Vec<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_option: Option<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
}

impl Tagged for RadioButtonsElement {
    const TAG: &'static str = "radio_buttons";
}

impl RadioButtonsElement {
    pub fn new(options: Vec<OptionObject>) -> Self {
        Self {
            r#type: Tag::default(),
            action_id: None,
            options,
            initial_option: None,
            confirm: None,
            focus_on_load: None,
        }
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn initial_option(mut self, initial_option: OptionObject) -> Self {
        self.initial_option = Some(initial_option);
        self
    }

    pub fn confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn focus_on_load(mut self, focus_on_load: bool) -> Self {
        self.focus_on_load = Some(focus_on_load);
        self
    }
}

/// A rich text input. <https://docs.slack.dev/reference/block-kit/block-elements/rich-text-input-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RichTextInputElement {
    r#type: Tag<Self>,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<RichTextBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatch_action_config: Option<DispatchActionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_lines: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_lines: Option<u32>,
}

impl Tagged for RichTextInputElement {
    const TAG: &'static str = "rich_text_input";
}

impl RichTextInputElement {
    pub fn new(action_id: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            action_id: action_id.into(),
            initial_value: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
            min_lines: None,
            max_lines: None,
        }
    }

    pub fn initial_value(mut self, initial_value: RichTextBlock) -> Self {
        self.initial_value = Some(initial_value);
        self
    }

    pub fn dispatch_action_config(mut self, dispatch_action_config: DispatchActionConfig) -> Self {
        self.dispatch_action_config = Some(dispatch_action_config);
        self
    }

    pub fn focus_on_load(mut self, focus_on_load: bool) -> Self {
        self.focus_on_load = Some(focus_on_load);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<TextObject>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn min_lines(mut self, min_lines: u32) -> Self {
        self.min_lines = Some(min_lines);
        self
    }

    pub fn max_lines(mut self, max_lines: u32) -> Self {
        self.max_lines = Some(max_lines);
        self
    }
}

/// A time picker. <https://docs.slack.dev/reference/block-kit/block-elements/time-picker-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TimePickerElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl Tagged for TimePickerElement {
    const TAG: &'static str = "timepicker";
}

impl TimePickerElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn initial_time(mut self, initial_time: impl Into<String>) -> Self {
        self.initial_time = Some(initial_time.into());
        self
    }

    pub fn confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn focus_on_load(mut self, focus_on_load: bool) -> Self {
        self.focus_on_load = Some(focus_on_load);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<TextObject>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = Some(timezone.into());
        self
    }
}

/// A URL input. <https://docs.slack.dev/reference/block-kit/block-elements/url-input-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UrlInputElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatch_action_config: Option<DispatchActionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl Tagged for UrlInputElement {
    const TAG: &'static str = "url_text_input";
}

impl UrlInputElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn initial_value(mut self, initial_value: impl Into<String>) -> Self {
        self.initial_value = Some(initial_value.into());
        self
    }

    pub fn dispatch_action_config(mut self, dispatch_action_config: DispatchActionConfig) -> Self {
        self.dispatch_action_config = Some(dispatch_action_config);
        self
    }

    pub fn focus_on_load(mut self, focus_on_load: bool) -> Self {
        self.focus_on_load = Some(focus_on_load);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<TextObject>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }
}

/// A button that runs a workflow. <https://docs.slack.dev/reference/block-kit/block-elements/workflow-button-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowButtonElement {
    r#type: Tag<Self>,
    pub text: TextObject,
    pub workflow: Workflow,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ButtonStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility_label: Option<String>,
}

impl Tagged for WorkflowButtonElement {
    const TAG: &'static str = "workflow_button";
}

impl WorkflowButtonElement {
    pub fn new(
        text: impl Into<TextObject>,
        workflow: Workflow,
        action_id: impl Into<String>,
    ) -> Self {
        Self {
            r#type: Tag::default(),
            text: text.into(),
            workflow,
            action_id: action_id.into(),
            style: None,
            accessibility_label: None,
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn accessibility_label(mut self, accessibility_label: impl Into<String>) -> Self {
        self.accessibility_label = Some(accessibility_label.into());
        self
    }
}

/// A URL source shown in a task card block. <https://docs.slack.dev/reference/block-kit/block-elements/url-source-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UrlSourceElement {
    r#type: Tag<Self>,
    pub url: String,
    pub text: String,
}

impl Tagged for UrlSourceElement {
    const TAG: &'static str = "url";
}

impl UrlSourceElement {
    pub fn new(url: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            url: url.into(),
            text: text.into(),
        }
    }
}
