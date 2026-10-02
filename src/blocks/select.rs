use serde::{Deserialize, Serialize};

use super::composition::{
    ConfirmationDialog, ConversationFilter, OptionGroup, OptionObject, TextObject,
};
use super::tag::{Tag, Tagged};

/// A select menu with static options (either `options` or `option_groups`). <https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaticSelectElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<OptionObject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub option_groups: Vec<OptionGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_option: Option<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl Tagged for StaticSelectElement {
    const TAG: &'static str = "static_select";
}

impl StaticSelectElement {
    pub fn new(options: Vec<OptionObject>) -> Self {
        Self {
            r#type: Tag::default(),
            action_id: None,
            options,
            option_groups: Vec::new(),
            initial_option: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
        }
    }

    pub fn from_option_groups(option_groups: Vec<OptionGroup>) -> Self {
        Self {
            option_groups,
            ..Self::new(Vec::new())
        }
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn option_groups(mut self, option_groups: Vec<OptionGroup>) -> Self {
        self.option_groups = option_groups;
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

    pub fn placeholder(mut self, placeholder: impl Into<TextObject>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }
}

/// A select menu that loads options from an external data source. <https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ExternalSelectElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_option: Option<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_query_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl Tagged for ExternalSelectElement {
    const TAG: &'static str = "external_select";
}

impl ExternalSelectElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn initial_option(mut self, initial_option: OptionObject) -> Self {
        self.initial_option = Some(initial_option);
        self
    }

    pub fn min_query_length(mut self, min_query_length: u32) -> Self {
        self.min_query_length = Some(min_query_length);
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

/// A select menu of users. <https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersSelectElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl Tagged for UsersSelectElement {
    const TAG: &'static str = "users_select";
}

impl UsersSelectElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn initial_user(mut self, initial_user: impl Into<String>) -> Self {
        self.initial_user = Some(initial_user.into());
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

/// A select menu of conversations. <https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationsSelectElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_conversation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_to_current_conversation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_url_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ConversationFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl Tagged for ConversationsSelectElement {
    const TAG: &'static str = "conversations_select";
}

impl ConversationsSelectElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn initial_conversation(mut self, initial_conversation: impl Into<String>) -> Self {
        self.initial_conversation = Some(initial_conversation.into());
        self
    }

    pub fn default_to_current_conversation(
        mut self,
        default_to_current_conversation: bool,
    ) -> Self {
        self.default_to_current_conversation = Some(default_to_current_conversation);
        self
    }

    pub fn confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn response_url_enabled(mut self, response_url_enabled: bool) -> Self {
        self.response_url_enabled = Some(response_url_enabled);
        self
    }

    pub fn filter(mut self, filter: ConversationFilter) -> Self {
        self.filter = Some(filter);
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

/// A select menu of public channels. <https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChannelsSelectElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_url_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl Tagged for ChannelsSelectElement {
    const TAG: &'static str = "channels_select";
}

impl ChannelsSelectElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn initial_channel(mut self, initial_channel: impl Into<String>) -> Self {
        self.initial_channel = Some(initial_channel.into());
        self
    }

    pub fn confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn response_url_enabled(mut self, response_url_enabled: bool) -> Self {
        self.response_url_enabled = Some(response_url_enabled);
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

/// A multi-select menu with static options (either `options` or `option_groups`). <https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiStaticSelectElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<OptionObject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub option_groups: Vec<OptionGroup>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial_options: Vec<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_selected_items: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl Tagged for MultiStaticSelectElement {
    const TAG: &'static str = "multi_static_select";
}

impl MultiStaticSelectElement {
    pub fn new(options: Vec<OptionObject>) -> Self {
        Self {
            r#type: Tag::default(),
            action_id: None,
            options,
            option_groups: Vec::new(),
            initial_options: Vec::new(),
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
        }
    }

    pub fn from_option_groups(option_groups: Vec<OptionGroup>) -> Self {
        Self {
            option_groups,
            ..Self::new(Vec::new())
        }
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn option_groups(mut self, option_groups: Vec<OptionGroup>) -> Self {
        self.option_groups = option_groups;
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

    pub fn max_selected_items(mut self, max_selected_items: u32) -> Self {
        self.max_selected_items = Some(max_selected_items);
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

/// A multi-select menu that loads options from an external data source. <https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MultiExternalSelectElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_query_length: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial_options: Vec<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_selected_items: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl Tagged for MultiExternalSelectElement {
    const TAG: &'static str = "multi_external_select";
}

impl MultiExternalSelectElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn min_query_length(mut self, min_query_length: u32) -> Self {
        self.min_query_length = Some(min_query_length);
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

    pub fn max_selected_items(mut self, max_selected_items: u32) -> Self {
        self.max_selected_items = Some(max_selected_items);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<TextObject>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }
}

/// A multi-select menu of users. <https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MultiUsersSelectElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial_users: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_selected_items: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl Tagged for MultiUsersSelectElement {
    const TAG: &'static str = "multi_users_select";
}

impl MultiUsersSelectElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn initial_users(mut self, initial_users: Vec<String>) -> Self {
        self.initial_users = initial_users;
        self
    }

    pub fn confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn max_selected_items(mut self, max_selected_items: u32) -> Self {
        self.max_selected_items = Some(max_selected_items);
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

/// A multi-select menu of conversations. <https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MultiConversationsSelectElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial_conversations: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_to_current_conversation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_selected_items: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ConversationFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl Tagged for MultiConversationsSelectElement {
    const TAG: &'static str = "multi_conversations_select";
}

impl MultiConversationsSelectElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn initial_conversations(mut self, initial_conversations: Vec<String>) -> Self {
        self.initial_conversations = initial_conversations;
        self
    }

    pub fn default_to_current_conversation(
        mut self,
        default_to_current_conversation: bool,
    ) -> Self {
        self.default_to_current_conversation = Some(default_to_current_conversation);
        self
    }

    pub fn confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn max_selected_items(mut self, max_selected_items: u32) -> Self {
        self.max_selected_items = Some(max_selected_items);
        self
    }

    pub fn filter(mut self, filter: ConversationFilter) -> Self {
        self.filter = Some(filter);
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

/// A multi-select menu of public channels. <https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MultiChannelsSelectElement {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial_channels: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<ConfirmationDialog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_selected_items: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_load: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
}

impl Tagged for MultiChannelsSelectElement {
    const TAG: &'static str = "multi_channels_select";
}

impl MultiChannelsSelectElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn action_id(mut self, action_id: impl Into<String>) -> Self {
        self.action_id = Some(action_id.into());
        self
    }

    pub fn initial_channels(mut self, initial_channels: Vec<String>) -> Self {
        self.initial_channels = initial_channels;
        self
    }

    pub fn confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn max_selected_items(mut self, max_selected_items: u32) -> Self {
        self.max_selected_items = Some(max_selected_items);
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
