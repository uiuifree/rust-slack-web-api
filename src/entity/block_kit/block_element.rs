use crate::{
    CompositionObjectConfirmationDialog, CompositionObjectConversationFilter,
    CompositionObjectDispatchActionConfiguration, CompositionObjectOption,
    CompositionObjectOptionGroup, CompositionObjectText, CompositionObjectWorkflow,
    SlackBlockKitCompositionObject,
};
use serde_derive::{Deserialize, Serialize};

/// https://api.slack.com/reference/block-kit/block-elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SlackBlockElement {
    Button(BlockElementButton),
    Checkbox(BlockElementCheckbox),
    DatePicker(BlockElementDatePicker),
    DatetimePicker(BlockElementDatetimePicker),
    EmailTextInput(BlockElementEmailTextInput),
    Image(BlockElementImage),
    MultiSelectMenu(BlockElementMultiSelectMenu),
    ExternalDataSource(BlockElementExternalDataSource),
    MultiUsersSelect(BlockElementMultiUsersSelect),
    MultiConversationsSelect(BlockElementMultiConversationsSelect),
    MultiChannelSelect(BlockElementMultiChannelSelect),
    NumberInput(BlockElementNumberInput),
    OverflowMenu(BlockElementOverflowMenu),
    PlaneTextInput(BlockElementPlaneTextInput),
    Radio(BlockElementRadio),
    SelectMenuStaticOption(BlockElementSelectMenuStaticOption),
    SelectMenuExternalDataSource(BlockElementSelectMenuExternalDataSource),
    SelectMenuUsers(BlockElementSelectMenuUsers),
    SelectMenuConversations(BlockElementSelectMenuConversations),
    SelectMenuPublicChannel(BlockElementSelectMenuPublicChannel),
    TimePicker(BlockElementTimePicker),
    UrlInput(BlockElementUrlInput),
    WorkflowButton(BlockElementWorkflowButton),
}

/// https://api.slack.com/reference/block-kit/block-elements#button
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementButton {
    #[serde(rename = "type")]
    pub element_type: String,
    pub text: CompositionObjectText,
    pub action_id: String,
    pub url: Option<String>,
    pub value: Option<String>,
    pub style: Option<String>,
    pub confirm: Option<CompositionObjectConfirmationDialog>,
    pub accessibility_label: Option<String>,
}

impl Default for BlockElementButton {
    fn default() -> Self {
        BlockElementButton {
            element_type: "button".to_string(),

            text: Default::default(),
            action_id: "".to_string(),
            url: None,
            value: None,
            style: None,
            confirm: None,
            accessibility_label: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#checkboxes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementCheckbox {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub options: Vec<CompositionObjectOption>,
    pub initial_options: Option<Vec<CompositionObjectOption>>,
    pub confirm: Option<Vec<CompositionObjectConfirmationDialog>>,
    pub focus_on_load: Option<bool>,
}

impl Default for BlockElementCheckbox {
    fn default() -> Self {
        BlockElementCheckbox {
            element_type: "checkboxes".to_string(),

            action_id: "".to_string(),
            options: vec![],
            initial_options: None,
            confirm: None,
            focus_on_load: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#datepicker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementDatePicker {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub initial_date: Option<String>,
    pub confirm: Option<CompositionObjectConfirmationDialog>,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<CompositionObjectText>,
}

impl Default for BlockElementDatePicker {
    fn default() -> Self {
        BlockElementDatePicker {
            element_type: "datepicker".to_string(),

            action_id: "".to_string(),
            initial_date: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#datetimepicker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementDatetimePicker {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub initial_date_time: Option<i32>,
    pub confirm: Option<CompositionObjectConfirmationDialog>,
    pub focus_on_load: Option<bool>,
}

impl Default for BlockElementDatetimePicker {
    fn default() -> Self {
        BlockElementDatetimePicker {
            element_type: "datetimepicker".to_string(),

            action_id: "".to_string(),
            initial_date_time: None,
            confirm: None,
            focus_on_load: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementEmailTextInput {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub initial_value: Option<String>,
    pub dispatch_action_config: Option<CompositionObjectDispatchActionConfiguration>,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<CompositionObjectText>,
}

impl Default for BlockElementEmailTextInput {
    fn default() -> Self {
        BlockElementEmailTextInput {
            element_type: "email_text_input".to_string(),

            action_id: "".to_string(),
            initial_value: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementImage {
    #[serde(rename = "type")]
    pub element_type: String,
    pub image_url: String,
    pub alt_text: String,
}

impl Default for BlockElementImage {
    fn default() -> Self {
        BlockElementImage {
            element_type: "image".to_string(),

            image_url: "".to_string(),
            alt_text: "".to_string(),
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#multi_select
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementMultiSelectMenu {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub options: Vec<CompositionObjectOption>,
    pub option_groups: Option<Vec<CompositionObjectOptionGroup>>,
    pub initial_options: Option<Vec<SlackBlockKitCompositionObject>>,
    pub confirm: Option<CompositionObjectConfirmationDialog>,
    pub max_selected_items: Option<u32>,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<CompositionObjectText>,
}

impl Default for BlockElementMultiSelectMenu {
    fn default() -> Self {
        BlockElementMultiSelectMenu {
            element_type: "multi_static_select".to_string(),

            action_id: "".to_string(),
            options: vec![],
            option_groups: None,
            initial_options: None,
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#external_multi_select
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementExternalDataSource {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub min_query_length: Option<i32>,
    pub initial_options: Option<Vec<SlackBlockKitCompositionObject>>,
    pub confirm: Option<CompositionObjectConfirmationDialog>,
    pub max_selected_items: Option<u32>,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<CompositionObjectText>,
}

impl Default for BlockElementExternalDataSource {
    fn default() -> Self {
        BlockElementExternalDataSource {
            element_type: "multi_external_select".to_string(),

            action_id: "".to_string(),
            min_query_length: None,
            initial_options: None,
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#external_multi_select
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementMultiUsersSelect {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub initial_options: Option<Vec<SlackBlockKitCompositionObject>>,
    pub confirm: Option<CompositionObjectConfirmationDialog>,
    pub max_selected_items: Option<u32>,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<CompositionObjectText>,
}

impl Default for BlockElementMultiUsersSelect {
    fn default() -> Self {
        BlockElementMultiUsersSelect {
            element_type: "multi_users_select".to_string(),

            action_id: "".to_string(),
            initial_options: None,
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#conversation_multi_select
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementMultiConversationsSelect {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub initial_conversations: Option<Vec<String>>,
    pub default_to_current_conversation: Option<bool>,
    pub confirm: Option<CompositionObjectConfirmationDialog>,
    pub max_selected_items: Option<u32>,
    pub filter: Option<CompositionObjectConversationFilter>,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<CompositionObjectText>,
}

impl Default for BlockElementMultiConversationsSelect {
    fn default() -> Self {
        BlockElementMultiConversationsSelect {
            element_type: "multi_conversations_select".to_string(),

            action_id: "".to_string(),
            initial_conversations: None,
            default_to_current_conversation: None,
            confirm: None,
            max_selected_items: None,
            filter: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#channel_multi_select
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementMultiChannelSelect {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub initial_channels: Option<Vec<String>>,
    pub confirm: Option<CompositionObjectConfirmationDialog>,
    pub max_selected_items: Option<u32>,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<CompositionObjectText>,
}

impl Default for BlockElementMultiChannelSelect {
    fn default() -> Self {
        BlockElementMultiChannelSelect {
            element_type: "multi_channels_select".to_string(),

            action_id: "".to_string(),
            initial_channels: None,
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#number
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementNumberInput {
    #[serde(rename = "type")]
    pub element_type: String,
    pub is_decimal_allowed: bool,
    pub action_id: String,
    pub initial_value: Option<String>,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
    pub dispatch_action_config: Option<CompositionObjectConfirmationDialog>,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<CompositionObjectText>,
}

impl Default for BlockElementNumberInput {
    fn default() -> Self {
        BlockElementNumberInput {
            element_type: "number_input".to_string(),

            is_decimal_allowed: false,
            action_id: "".to_string(),
            initial_value: None,
            min_value: None,
            max_value: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#overflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementOverflowMenu {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub options: CompositionObjectOption,
    pub confirm: Option<CompositionObjectConfirmationDialog>,
}

impl Default for BlockElementOverflowMenu {
    fn default() -> Self {
        BlockElementOverflowMenu {
            element_type: "overflow".to_string(),

            action_id: "".to_string(),
            options: Default::default(),
            confirm: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementPlaneTextInput {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub initial_value: Option<String>,
    pub multiline: Option<bool>,
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
    pub dispatch_action_config: Option<CompositionObjectDispatchActionConfiguration>,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<CompositionObjectText>,
}

impl Default for BlockElementPlaneTextInput {
    fn default() -> Self {
        BlockElementPlaneTextInput {
            element_type: "plain_text_input".to_string(),

            action_id: "".to_string(),
            initial_value: None,
            multiline: None,
            min_length: None,
            max_length: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#radio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementRadio {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub options: Vec<CompositionObjectOption>,
    pub initial_option: Option<CompositionObjectOption>,
    pub confirm: Option<CompositionObjectConfirmationDialog>,
    pub focus_on_load: Option<bool>,
}

impl Default for BlockElementRadio {
    fn default() -> Self {
        BlockElementRadio {
            element_type: "radio_buttons".to_string(),

            action_id: "".to_string(),
            options: vec![],
            initial_option: None,
            confirm: None,
            focus_on_load: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#select
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementSelectMenuStaticOption {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub options: Vec<CompositionObjectOption>,
    pub option_groups: Option<Vec<CompositionObjectOptionGroup>>,
    pub initial_option: Option<CompositionObjectOption>,
    pub confirm: Option<CompositionObjectConfirmationDialog>,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<CompositionObjectText>,
}

impl Default for BlockElementSelectMenuStaticOption {
    fn default() -> Self {
        BlockElementSelectMenuStaticOption {
            element_type: "static_select".to_string(),

            action_id: "".to_string(),
            options: vec![],
            option_groups: None,
            initial_option: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#external_select
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementSelectMenuExternalDataSource {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub initial_option: Option<CompositionObjectOption>,
    pub min_query_length: Option<i32>,
    pub confirm: Option<CompositionObjectConfirmationDialog>,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<CompositionObjectText>,
}

impl Default for BlockElementSelectMenuExternalDataSource {
    fn default() -> Self {
        BlockElementSelectMenuExternalDataSource {
            element_type: "external_select".to_string(),

            action_id: "".to_string(),
            initial_option: None,
            min_query_length: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#external_select
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementSelectMenuUsers {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub initial_user: Option<String>,
    pub confirm: Option<CompositionObjectConfirmationDialog>,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<CompositionObjectText>,
}

impl Default for BlockElementSelectMenuUsers {
    fn default() -> Self {
        BlockElementSelectMenuUsers {
            element_type: "users_select".to_string(),

            action_id: "".to_string(),
            initial_user: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#conversations_select
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementSelectMenuConversations {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub initial_conversation: Option<String>,
    pub default_to_current_conversation: Option<bool>,
    pub confirm: Option<CompositionObjectConfirmationDialog>,
    pub response_url_enabled: Option<bool>,
    pub filter: Option<CompositionObjectConversationFilter>,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<CompositionObjectText>,
}

impl Default for BlockElementSelectMenuConversations {
    fn default() -> Self {
        BlockElementSelectMenuConversations {
            element_type: "conversations_select".to_string(),

            action_id: "".to_string(),
            initial_conversation: None,
            default_to_current_conversation: None,
            confirm: None,
            response_url_enabled: None,
            filter: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#channels_select
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementSelectMenuPublicChannel {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub initial_channel: Option<String>,
    pub confirm: Option<CompositionObjectConfirmationDialog>,
    pub response_url_enabled: Option<bool>,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<CompositionObjectText>,
}

impl Default for BlockElementSelectMenuPublicChannel {
    fn default() -> Self {
        BlockElementSelectMenuPublicChannel {
            element_type: "channels_select".to_string(),

            action_id: "".to_string(),
            initial_channel: None,
            confirm: None,
            response_url_enabled: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#timepicker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementTimePicker {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub initial_time: Option<String>,
    pub confirm: Option<CompositionObjectConfirmationDialog>,
    pub response_url_enabled: Option<bool>,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<CompositionObjectText>,
    pub timezone: Option<String>,
}

impl Default for BlockElementTimePicker {
    fn default() -> Self {
        BlockElementTimePicker {
            element_type: "timepicker".to_string(),

            action_id: "".to_string(),
            initial_time: None,
            confirm: None,
            response_url_enabled: None,
            focus_on_load: None,
            placeholder: None,
            timezone: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#url
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementUrlInput {
    #[serde(rename = "type")]
    pub element_type: String,
    pub action_id: String,
    pub initial_value: Option<String>,
    pub dispatch_action_config: Option<CompositionObjectDispatchActionConfiguration>,
    pub placeholder: Option<CompositionObjectText>,
    pub timezone: Option<String>,
}

impl Default for BlockElementUrlInput {
    fn default() -> Self {
        BlockElementUrlInput {
            element_type: "url_text_input".to_string(),

            action_id: "".to_string(),
            initial_value: None,
            dispatch_action_config: None,
            placeholder: None,
            timezone: None,
        }
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#workflow_button
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockElementWorkflowButton {
    #[serde(rename = "type")]
    pub element_type: String,
    pub text: CompositionObjectText,
    pub workflow: CompositionObjectWorkflow,
    pub style: Option<String>,
    pub accessibility_label: Option<CompositionObjectText>,
}

impl Default for BlockElementWorkflowButton {
    fn default() -> Self {
        BlockElementWorkflowButton {
            element_type: "workflow_button".to_string(),

            text: Default::default(),
            workflow: Default::default(),
            style: None,
            accessibility_label: None,
        }
    }
}
