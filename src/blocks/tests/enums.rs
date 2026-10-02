use super::*;

#[test]
fn text_object() {
    let value = PlainText::new("s");
    assert_eq!(
        TextObject::from(value.clone()),
        TextObject::PlainText(value.clone())
    );
    assert_json(
        &TextObject::PlainText(value),
        r#"{"type": "plain_text", "text": "s"}"#,
    );
    let value = Mrkdwn::new("s");
    assert_eq!(
        TextObject::from(value.clone()),
        TextObject::Mrkdwn(value.clone())
    );
    assert_json(
        &TextObject::Mrkdwn(value),
        r#"{"type": "mrkdwn", "text": "s"}"#,
    );
    assert_unknown::<TextObject>(r#"{"type": "future_kind", "payload": {"n": 1}}"#, |v| {
        matches!(v, TextObject::Unknown(_))
    });
    assert_unknown::<TextObject>(r#"{"payload": 1}"#, |v| matches!(v, TextObject::Unknown(_)));
    assert!(serde_json::from_str::<TextObject>(r#"{"type": "plain_text", "text": true}"#).is_err());
    assert!(serde_json::from_str::<TextObject>(r#"{"type": "x""#).is_err());
}

#[test]
fn actions_element() {
    let value = ButtonElement::new(TextObject::from(PlainText::new("s")));
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::Button(value.clone())
    );
    assert_json(
        &ActionsElement::Button(value),
        r#"{"type": "button", "text": {"type": "plain_text", "text": "s"}}"#,
    );
    let value = CheckboxesElement::new(vec![OptionObject::new(
        TextObject::from(PlainText::new("s")),
        "s",
    )]);
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::Checkboxes(value.clone())
    );
    assert_json(
        &ActionsElement::Checkboxes(value),
        r#"{"type": "checkboxes", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    let value = DatePickerElement::new();
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::DatePicker(value.clone())
    );
    assert_json(
        &ActionsElement::DatePicker(value),
        r#"{"type": "datepicker"}"#,
    );
    let value = DatetimePickerElement::new();
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::DatetimePicker(value.clone())
    );
    assert_json(
        &ActionsElement::DatetimePicker(value),
        r#"{"type": "datetimepicker"}"#,
    );
    let value = MultiStaticSelectElement::new(vec![OptionObject::new(
        TextObject::from(PlainText::new("s")),
        "s",
    )]);
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::MultiStaticSelect(value.clone())
    );
    assert_json(
        &ActionsElement::MultiStaticSelect(value),
        r#"{"type": "multi_static_select", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    let value = MultiExternalSelectElement::new();
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::MultiExternalSelect(value.clone())
    );
    assert_json(
        &ActionsElement::MultiExternalSelect(value),
        r#"{"type": "multi_external_select"}"#,
    );
    let value = MultiUsersSelectElement::new();
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::MultiUsersSelect(value.clone())
    );
    assert_json(
        &ActionsElement::MultiUsersSelect(value),
        r#"{"type": "multi_users_select"}"#,
    );
    let value = MultiConversationsSelectElement::new();
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::MultiConversationsSelect(value.clone())
    );
    assert_json(
        &ActionsElement::MultiConversationsSelect(value),
        r#"{"type": "multi_conversations_select"}"#,
    );
    let value = MultiChannelsSelectElement::new();
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::MultiChannelsSelect(value.clone())
    );
    assert_json(
        &ActionsElement::MultiChannelsSelect(value),
        r#"{"type": "multi_channels_select"}"#,
    );
    let value = OverflowElement::new(vec![OptionObject::new(
        TextObject::from(PlainText::new("s")),
        "s",
    )]);
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::Overflow(value.clone())
    );
    assert_json(
        &ActionsElement::Overflow(value),
        r#"{"type": "overflow", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    let value = RadioButtonsElement::new(vec![OptionObject::new(
        TextObject::from(PlainText::new("s")),
        "s",
    )]);
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::RadioButtons(value.clone())
    );
    assert_json(
        &ActionsElement::RadioButtons(value),
        r#"{"type": "radio_buttons", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    let value = RichTextInputElement::new("s");
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::RichTextInput(value.clone())
    );
    assert_json(
        &ActionsElement::RichTextInput(value),
        r#"{"type": "rich_text_input", "action_id": "s"}"#,
    );
    let value = StaticSelectElement::new(vec![OptionObject::new(
        TextObject::from(PlainText::new("s")),
        "s",
    )]);
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::StaticSelect(value.clone())
    );
    assert_json(
        &ActionsElement::StaticSelect(value),
        r#"{"type": "static_select", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    let value = ExternalSelectElement::new();
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::ExternalSelect(value.clone())
    );
    assert_json(
        &ActionsElement::ExternalSelect(value),
        r#"{"type": "external_select"}"#,
    );
    let value = UsersSelectElement::new();
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::UsersSelect(value.clone())
    );
    assert_json(
        &ActionsElement::UsersSelect(value),
        r#"{"type": "users_select"}"#,
    );
    let value = ConversationsSelectElement::new();
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::ConversationsSelect(value.clone())
    );
    assert_json(
        &ActionsElement::ConversationsSelect(value),
        r#"{"type": "conversations_select"}"#,
    );
    let value = ChannelsSelectElement::new();
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::ChannelsSelect(value.clone())
    );
    assert_json(
        &ActionsElement::ChannelsSelect(value),
        r#"{"type": "channels_select"}"#,
    );
    let value = TimePickerElement::new();
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::TimePicker(value.clone())
    );
    assert_json(
        &ActionsElement::TimePicker(value),
        r#"{"type": "timepicker"}"#,
    );
    let value = WorkflowButtonElement::new(
        TextObject::from(PlainText::new("s")),
        Workflow::new(Trigger::new("s")),
        "s",
    );
    assert_eq!(
        ActionsElement::from(value.clone()),
        ActionsElement::WorkflowButton(value.clone())
    );
    assert_json(
        &ActionsElement::WorkflowButton(value),
        r#"{"type": "workflow_button", "text": {"type": "plain_text", "text": "s"}, "workflow": {"trigger": {"url": "s"}}, "action_id": "s"}"#,
    );
    assert_unknown::<ActionsElement>(r#"{"type": "future_kind", "payload": {"n": 1}}"#, |v| {
        matches!(v, ActionsElement::Unknown(_))
    });
    assert_unknown::<ActionsElement>(r#"{"payload": 1}"#, |v| {
        matches!(v, ActionsElement::Unknown(_))
    });
    assert!(
        serde_json::from_str::<ActionsElement>(r#"{"type": "button", "action_id": true}"#).is_err()
    );
    assert!(serde_json::from_str::<ActionsElement>(r#"{"type": "x""#).is_err());
}

#[test]
fn section_accessory() {
    let value = ButtonElement::new(TextObject::from(PlainText::new("s")));
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::Button(value.clone())
    );
    assert_json(
        &SectionAccessory::Button(value),
        r#"{"type": "button", "text": {"type": "plain_text", "text": "s"}}"#,
    );
    let value = CheckboxesElement::new(vec![OptionObject::new(
        TextObject::from(PlainText::new("s")),
        "s",
    )]);
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::Checkboxes(value.clone())
    );
    assert_json(
        &SectionAccessory::Checkboxes(value),
        r#"{"type": "checkboxes", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    let value = DatePickerElement::new();
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::DatePicker(value.clone())
    );
    assert_json(
        &SectionAccessory::DatePicker(value),
        r#"{"type": "datepicker"}"#,
    );
    let value = ImageElement::new("s", "s");
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::Image(value.clone())
    );
    assert_json(
        &SectionAccessory::Image(value),
        r#"{"type": "image", "alt_text": "s", "image_url": "s"}"#,
    );
    let value = MultiStaticSelectElement::new(vec![OptionObject::new(
        TextObject::from(PlainText::new("s")),
        "s",
    )]);
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::MultiStaticSelect(value.clone())
    );
    assert_json(
        &SectionAccessory::MultiStaticSelect(value),
        r#"{"type": "multi_static_select", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    let value = MultiExternalSelectElement::new();
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::MultiExternalSelect(value.clone())
    );
    assert_json(
        &SectionAccessory::MultiExternalSelect(value),
        r#"{"type": "multi_external_select"}"#,
    );
    let value = MultiUsersSelectElement::new();
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::MultiUsersSelect(value.clone())
    );
    assert_json(
        &SectionAccessory::MultiUsersSelect(value),
        r#"{"type": "multi_users_select"}"#,
    );
    let value = MultiConversationsSelectElement::new();
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::MultiConversationsSelect(value.clone())
    );
    assert_json(
        &SectionAccessory::MultiConversationsSelect(value),
        r#"{"type": "multi_conversations_select"}"#,
    );
    let value = MultiChannelsSelectElement::new();
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::MultiChannelsSelect(value.clone())
    );
    assert_json(
        &SectionAccessory::MultiChannelsSelect(value),
        r#"{"type": "multi_channels_select"}"#,
    );
    let value = OverflowElement::new(vec![OptionObject::new(
        TextObject::from(PlainText::new("s")),
        "s",
    )]);
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::Overflow(value.clone())
    );
    assert_json(
        &SectionAccessory::Overflow(value),
        r#"{"type": "overflow", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    let value = RadioButtonsElement::new(vec![OptionObject::new(
        TextObject::from(PlainText::new("s")),
        "s",
    )]);
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::RadioButtons(value.clone())
    );
    assert_json(
        &SectionAccessory::RadioButtons(value),
        r#"{"type": "radio_buttons", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    let value = StaticSelectElement::new(vec![OptionObject::new(
        TextObject::from(PlainText::new("s")),
        "s",
    )]);
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::StaticSelect(value.clone())
    );
    assert_json(
        &SectionAccessory::StaticSelect(value),
        r#"{"type": "static_select", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    let value = ExternalSelectElement::new();
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::ExternalSelect(value.clone())
    );
    assert_json(
        &SectionAccessory::ExternalSelect(value),
        r#"{"type": "external_select"}"#,
    );
    let value = UsersSelectElement::new();
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::UsersSelect(value.clone())
    );
    assert_json(
        &SectionAccessory::UsersSelect(value),
        r#"{"type": "users_select"}"#,
    );
    let value = ConversationsSelectElement::new();
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::ConversationsSelect(value.clone())
    );
    assert_json(
        &SectionAccessory::ConversationsSelect(value),
        r#"{"type": "conversations_select"}"#,
    );
    let value = ChannelsSelectElement::new();
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::ChannelsSelect(value.clone())
    );
    assert_json(
        &SectionAccessory::ChannelsSelect(value),
        r#"{"type": "channels_select"}"#,
    );
    let value = TimePickerElement::new();
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::TimePicker(value.clone())
    );
    assert_json(
        &SectionAccessory::TimePicker(value),
        r#"{"type": "timepicker"}"#,
    );
    let value = WorkflowButtonElement::new(
        TextObject::from(PlainText::new("s")),
        Workflow::new(Trigger::new("s")),
        "s",
    );
    assert_eq!(
        SectionAccessory::from(value.clone()),
        SectionAccessory::WorkflowButton(value.clone())
    );
    assert_json(
        &SectionAccessory::WorkflowButton(value),
        r#"{"type": "workflow_button", "text": {"type": "plain_text", "text": "s"}, "workflow": {"trigger": {"url": "s"}}, "action_id": "s"}"#,
    );
    assert_unknown::<SectionAccessory>(r#"{"type": "future_kind", "payload": {"n": 1}}"#, |v| {
        matches!(v, SectionAccessory::Unknown(_))
    });
    assert_unknown::<SectionAccessory>(r#"{"payload": 1}"#, |v| {
        matches!(v, SectionAccessory::Unknown(_))
    });
    assert!(
        serde_json::from_str::<SectionAccessory>(r#"{"type": "button", "action_id": true}"#)
            .is_err()
    );
    assert!(serde_json::from_str::<SectionAccessory>(r#"{"type": "x""#).is_err());
}

#[test]
fn input_element() {
    let value = CheckboxesElement::new(vec![OptionObject::new(
        TextObject::from(PlainText::new("s")),
        "s",
    )]);
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::Checkboxes(value.clone())
    );
    assert_json(
        &InputElement::Checkboxes(value),
        r#"{"type": "checkboxes", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    let value = DatePickerElement::new();
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::DatePicker(value.clone())
    );
    assert_json(
        &InputElement::DatePicker(value),
        r#"{"type": "datepicker"}"#,
    );
    let value = DatetimePickerElement::new();
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::DatetimePicker(value.clone())
    );
    assert_json(
        &InputElement::DatetimePicker(value),
        r#"{"type": "datetimepicker"}"#,
    );
    let value = EmailInputElement::new();
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::EmailInput(value.clone())
    );
    assert_json(
        &InputElement::EmailInput(value),
        r#"{"type": "email_text_input"}"#,
    );
    let value = FileInputElement::new();
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::FileInput(value.clone())
    );
    assert_json(&InputElement::FileInput(value), r#"{"type": "file_input"}"#);
    let value = MultiStaticSelectElement::new(vec![OptionObject::new(
        TextObject::from(PlainText::new("s")),
        "s",
    )]);
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::MultiStaticSelect(value.clone())
    );
    assert_json(
        &InputElement::MultiStaticSelect(value),
        r#"{"type": "multi_static_select", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    let value = MultiExternalSelectElement::new();
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::MultiExternalSelect(value.clone())
    );
    assert_json(
        &InputElement::MultiExternalSelect(value),
        r#"{"type": "multi_external_select"}"#,
    );
    let value = MultiUsersSelectElement::new();
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::MultiUsersSelect(value.clone())
    );
    assert_json(
        &InputElement::MultiUsersSelect(value),
        r#"{"type": "multi_users_select"}"#,
    );
    let value = MultiConversationsSelectElement::new();
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::MultiConversationsSelect(value.clone())
    );
    assert_json(
        &InputElement::MultiConversationsSelect(value),
        r#"{"type": "multi_conversations_select"}"#,
    );
    let value = MultiChannelsSelectElement::new();
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::MultiChannelsSelect(value.clone())
    );
    assert_json(
        &InputElement::MultiChannelsSelect(value),
        r#"{"type": "multi_channels_select"}"#,
    );
    let value = NumberInputElement::new(true);
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::NumberInput(value.clone())
    );
    assert_json(
        &InputElement::NumberInput(value),
        r#"{"type": "number_input", "is_decimal_allowed": true}"#,
    );
    let value = PlainTextInputElement::new();
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::PlainTextInput(value.clone())
    );
    assert_json(
        &InputElement::PlainTextInput(value),
        r#"{"type": "plain_text_input"}"#,
    );
    let value = RadioButtonsElement::new(vec![OptionObject::new(
        TextObject::from(PlainText::new("s")),
        "s",
    )]);
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::RadioButtons(value.clone())
    );
    assert_json(
        &InputElement::RadioButtons(value),
        r#"{"type": "radio_buttons", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    let value = RichTextInputElement::new("s");
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::RichTextInput(value.clone())
    );
    assert_json(
        &InputElement::RichTextInput(value),
        r#"{"type": "rich_text_input", "action_id": "s"}"#,
    );
    let value = StaticSelectElement::new(vec![OptionObject::new(
        TextObject::from(PlainText::new("s")),
        "s",
    )]);
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::StaticSelect(value.clone())
    );
    assert_json(
        &InputElement::StaticSelect(value),
        r#"{"type": "static_select", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    let value = ExternalSelectElement::new();
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::ExternalSelect(value.clone())
    );
    assert_json(
        &InputElement::ExternalSelect(value),
        r#"{"type": "external_select"}"#,
    );
    let value = UsersSelectElement::new();
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::UsersSelect(value.clone())
    );
    assert_json(
        &InputElement::UsersSelect(value),
        r#"{"type": "users_select"}"#,
    );
    let value = ConversationsSelectElement::new();
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::ConversationsSelect(value.clone())
    );
    assert_json(
        &InputElement::ConversationsSelect(value),
        r#"{"type": "conversations_select"}"#,
    );
    let value = ChannelsSelectElement::new();
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::ChannelsSelect(value.clone())
    );
    assert_json(
        &InputElement::ChannelsSelect(value),
        r#"{"type": "channels_select"}"#,
    );
    let value = TimePickerElement::new();
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::TimePicker(value.clone())
    );
    assert_json(
        &InputElement::TimePicker(value),
        r#"{"type": "timepicker"}"#,
    );
    let value = UrlInputElement::new();
    assert_eq!(
        InputElement::from(value.clone()),
        InputElement::UrlInput(value.clone())
    );
    assert_json(
        &InputElement::UrlInput(value),
        r#"{"type": "url_text_input"}"#,
    );
    assert_unknown::<InputElement>(r#"{"type": "future_kind", "payload": {"n": 1}}"#, |v| {
        matches!(v, InputElement::Unknown(_))
    });
    assert_unknown::<InputElement>(r#"{"payload": 1}"#, |v| {
        matches!(v, InputElement::Unknown(_))
    });
    assert!(
        serde_json::from_str::<InputElement>(r#"{"type": "checkboxes", "action_id": true}"#)
            .is_err()
    );
    assert!(serde_json::from_str::<InputElement>(r#"{"type": "x""#).is_err());
}

#[test]
fn context_element() {
    let value = ImageElement::new("s", "s");
    assert_eq!(
        ContextElement::from(value.clone()),
        ContextElement::Image(value.clone())
    );
    assert_json(
        &ContextElement::Image(value),
        r#"{"type": "image", "alt_text": "s", "image_url": "s"}"#,
    );
    let value = PlainText::new("s");
    assert_eq!(
        ContextElement::from(value.clone()),
        ContextElement::PlainText(value.clone())
    );
    assert_json(
        &ContextElement::PlainText(value),
        r#"{"type": "plain_text", "text": "s"}"#,
    );
    let value = Mrkdwn::new("s");
    assert_eq!(
        ContextElement::from(value.clone()),
        ContextElement::Mrkdwn(value.clone())
    );
    assert_json(
        &ContextElement::Mrkdwn(value),
        r#"{"type": "mrkdwn", "text": "s"}"#,
    );
    assert_unknown::<ContextElement>(r#"{"type": "future_kind", "payload": {"n": 1}}"#, |v| {
        matches!(v, ContextElement::Unknown(_))
    });
    assert_unknown::<ContextElement>(r#"{"payload": 1}"#, |v| {
        matches!(v, ContextElement::Unknown(_))
    });
    assert!(
        serde_json::from_str::<ContextElement>(r#"{"type": "image", "alt_text": true}"#).is_err()
    );
    assert!(serde_json::from_str::<ContextElement>(r#"{"type": "x""#).is_err());
}

#[test]
fn context_actions_element() {
    let value = FeedbackButtonsElement::new(
        FeedbackButton::new(TextObject::from(PlainText::new("s")), "s"),
        FeedbackButton::new(TextObject::from(PlainText::new("s")), "s"),
    );
    assert_eq!(
        ContextActionsElement::from(value.clone()),
        ContextActionsElement::FeedbackButtons(value.clone())
    );
    assert_json(
        &ContextActionsElement::FeedbackButtons(value),
        r#"{"type": "feedback_buttons", "positive_button": {"text": {"type": "plain_text", "text": "s"}, "value": "s"}, "negative_button": {"text": {"type": "plain_text", "text": "s"}, "value": "s"}}"#,
    );
    let value = IconButtonElement::new("s", TextObject::from(PlainText::new("s")));
    assert_eq!(
        ContextActionsElement::from(value.clone()),
        ContextActionsElement::IconButton(value.clone())
    );
    assert_json(
        &ContextActionsElement::IconButton(value),
        r#"{"type": "icon_button", "icon": "s", "text": {"type": "plain_text", "text": "s"}}"#,
    );
    assert_unknown::<ContextActionsElement>(
        r#"{"type": "future_kind", "payload": {"n": 1}}"#,
        |v| matches!(v, ContextActionsElement::Unknown(_)),
    );
    assert_unknown::<ContextActionsElement>(r#"{"payload": 1}"#, |v| {
        matches!(v, ContextActionsElement::Unknown(_))
    });
    assert!(serde_json::from_str::<ContextActionsElement>(
        r#"{"type": "feedback_buttons", "positive_button": true}"#
    )
    .is_err());
    assert!(serde_json::from_str::<ContextActionsElement>(r#"{"type": "x""#).is_err());
}

#[test]
fn rich_text_object() {
    let value = RichTextSection::new(vec![RichTextElement::from(AttachmentMentionElement::new(
        "s",
    ))]);
    assert_eq!(
        RichTextObject::from(value.clone()),
        RichTextObject::Section(value.clone())
    );
    assert_json(
        &RichTextObject::Section(value),
        r#"{"type": "rich_text_section", "elements": [{"type": "attachment_mention", "url": "s"}]}"#,
    );
    let value = RichTextList::new(
        RichTextListStyle::Bullet,
        vec![RichTextSection::new(vec![RichTextElement::from(
            AttachmentMentionElement::new("s"),
        )])],
    );
    assert_eq!(
        RichTextObject::from(value.clone()),
        RichTextObject::List(value.clone())
    );
    assert_json(
        &RichTextObject::List(value),
        r#"{"type": "rich_text_list", "style": "bullet", "elements": [{"type": "rich_text_section", "elements": [{"type": "attachment_mention", "url": "s"}]}]}"#,
    );
    let value = RichTextPreformatted::new(vec![PreformattedElement::from(TextElement::new("s"))]);
    assert_eq!(
        RichTextObject::from(value.clone()),
        RichTextObject::Preformatted(value.clone())
    );
    assert_json(
        &RichTextObject::Preformatted(value),
        r#"{"type": "rich_text_preformatted", "elements": [{"type": "text", "text": "s"}]}"#,
    );
    let value = RichTextQuote::new(vec![RichTextElement::from(AttachmentMentionElement::new(
        "s",
    ))]);
    assert_eq!(
        RichTextObject::from(value.clone()),
        RichTextObject::Quote(value.clone())
    );
    assert_json(
        &RichTextObject::Quote(value),
        r#"{"type": "rich_text_quote", "elements": [{"type": "attachment_mention", "url": "s"}]}"#,
    );
    assert_unknown::<RichTextObject>(r#"{"type": "future_kind", "payload": {"n": 1}}"#, |v| {
        matches!(v, RichTextObject::Unknown(_))
    });
    assert_unknown::<RichTextObject>(r#"{"payload": 1}"#, |v| {
        matches!(v, RichTextObject::Unknown(_))
    });
    assert!(serde_json::from_str::<RichTextObject>(
        r#"{"type": "rich_text_section", "elements": true}"#
    )
    .is_err());
    assert!(serde_json::from_str::<RichTextObject>(r#"{"type": "x""#).is_err());
}

#[test]
fn preformatted_element() {
    let value = TextElement::new("s");
    assert_eq!(
        PreformattedElement::from(value.clone()),
        PreformattedElement::Text(value.clone())
    );
    assert_json(
        &PreformattedElement::Text(value),
        r#"{"type": "text", "text": "s"}"#,
    );
    let value = LinkElement::new("s");
    assert_eq!(
        PreformattedElement::from(value.clone()),
        PreformattedElement::Link(value.clone())
    );
    assert_json(
        &PreformattedElement::Link(value),
        r#"{"type": "link", "url": "s"}"#,
    );
    assert_unknown::<PreformattedElement>(r#"{"type": "future_kind", "payload": {"n": 1}}"#, |v| {
        matches!(v, PreformattedElement::Unknown(_))
    });
    assert_unknown::<PreformattedElement>(r#"{"payload": 1}"#, |v| {
        matches!(v, PreformattedElement::Unknown(_))
    });
    assert!(
        serde_json::from_str::<PreformattedElement>(r#"{"type": "text", "text": true}"#).is_err()
    );
    assert!(serde_json::from_str::<PreformattedElement>(r#"{"type": "x""#).is_err());
}

#[test]
fn rich_text_element() {
    let value = AttachmentMentionElement::new("s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::AttachmentMention(value.clone())
    );
    assert_json(
        &RichTextElement::AttachmentMention(value),
        r#"{"type": "attachment_mention", "url": "s"}"#,
    );
    let value = BroadcastElement::new(BroadcastRange::Here);
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::Broadcast(value.clone())
    );
    assert_json(
        &RichTextElement::Broadcast(value),
        r#"{"type": "broadcast", "range": "here"}"#,
    );
    let value = CanvasElement::new("s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::Canvas(value.clone())
    );
    assert_json(
        &RichTextElement::Canvas(value),
        r#"{"type": "canvas", "file_id": "s"}"#,
    );
    let value = CanvasUserMentionElement::new("s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::CanvasUserMention(value.clone())
    );
    assert_json(
        &RichTextElement::CanvasUserMention(value),
        r#"{"type": "canvas_user_mention", "user_id": "s"}"#,
    );
    let value = CanvasMessageUnfurlElement::new("s", "s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::CanvasMessageUnfurl(value.clone())
    );
    assert_json(
        &RichTextElement::CanvasMessageUnfurl(value),
        r#"{"type": "canvas_message_unfurl", "root_message_ts": "s", "root_message_channel": "s"}"#,
    );
    let value = ChannelElement::new("s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::Channel(value.clone())
    );
    assert_json(
        &RichTextElement::Channel(value),
        r#"{"type": "channel", "channel_id": "s"}"#,
    );
    let value = CitationElement::new("s", "s", 1, CitationDetails::from(FileCitation::new()));
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::Citation(value.clone())
    );
    assert_json(
        &RichTextElement::Citation(value),
        r#"{"type": "citation", "url": "s", "text": "s", "index": 1, "details": {"citation_type": "file"}}"#,
    );
    let value = ColorElement::new("s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::Color(value.clone())
    );
    assert_json(
        &RichTextElement::Color(value),
        r#"{"type": "color", "value": "s"}"#,
    );
    let value = DateElement::new(1, "s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::Date(value.clone())
    );
    assert_json(
        &RichTextElement::Date(value),
        r#"{"type": "date", "timestamp": 1, "format": "s"}"#,
    );
    let value = EmojiElement::new("s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::Emoji(value.clone())
    );
    assert_json(
        &RichTextElement::Emoji(value),
        r#"{"type": "emoji", "name": "s"}"#,
    );
    let value = FileElement::new("s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::File(value.clone())
    );
    assert_json(
        &RichTextElement::File(value),
        r#"{"type": "file", "file_id": "s"}"#,
    );
    let value = LinkElement::new("s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::Link(value.clone())
    );
    assert_json(
        &RichTextElement::Link(value),
        r#"{"type": "link", "url": "s"}"#,
    );
    let value = ListRecordElement::new("s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::ListRecord(value.clone())
    );
    assert_json(
        &RichTextElement::ListRecord(value),
        r#"{"type": "list_record", "file_id": "s"}"#,
    );
    let value = MessageMentionElement::new("s", "s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::MessageMention(value.clone())
    );
    assert_json(
        &RichTextElement::MessageMention(value),
        r#"{"type": "message_mention", "channel_id": "s", "message_ts": "s"}"#,
    );
    let value = SalesforceDataFieldElement::new("s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::SalesforceDataField(value.clone())
    );
    assert_json(
        &RichTextElement::SalesforceDataField(value),
        r#"{"type": "salesforce_data_field", "salesforce_record_id": "s"}"#,
    );
    let value = TagElement::new("s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::Tag(value.clone())
    );
    assert_json(
        &RichTextElement::Tag(value),
        r#"{"type": "tag", "text": "s"}"#,
    );
    let value = TeamElement::new("s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::Team(value.clone())
    );
    assert_json(
        &RichTextElement::Team(value),
        r#"{"type": "team", "team_id": "s"}"#,
    );
    let value = TextElement::new("s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::Text(value.clone())
    );
    assert_json(
        &RichTextElement::Text(value),
        r#"{"type": "text", "text": "s"}"#,
    );
    let value = UserElement::new("s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::User(value.clone())
    );
    assert_json(
        &RichTextElement::User(value),
        r#"{"type": "user", "user_id": "s"}"#,
    );
    let value = UsergroupElement::new("s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::Usergroup(value.clone())
    );
    assert_json(
        &RichTextElement::Usergroup(value),
        r#"{"type": "usergroup", "usergroup_id": "s"}"#,
    );
    let value = WorkObjectMentionElement::new("s", "s", "s", "s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::WorkObjectMention(value.clone())
    );
    assert_json(
        &RichTextElement::WorkObjectMention(value),
        r#"{"type": "work_object_mention", "entity_id": "s", "app_id": "s", "text": "s", "url": "s"}"#,
    );
    let value = WorkflowMentionElement::new("s", "s", "s");
    assert_eq!(
        RichTextElement::from(value.clone()),
        RichTextElement::WorkflowMention(value.clone())
    );
    assert_json(
        &RichTextElement::WorkflowMention(value),
        r#"{"type": "workflow_mention", "workflow_id": "s", "function_trigger_id": "s", "text": "s"}"#,
    );
    assert_unknown::<RichTextElement>(r#"{"type": "future_kind", "payload": {"n": 1}}"#, |v| {
        matches!(v, RichTextElement::Unknown(_))
    });
    assert_unknown::<RichTextElement>(r#"{"payload": 1}"#, |v| {
        matches!(v, RichTextElement::Unknown(_))
    });
    assert!(serde_json::from_str::<RichTextElement>(
        r#"{"type": "attachment_mention", "url": true}"#
    )
    .is_err());
    assert!(serde_json::from_str::<RichTextElement>(r#"{"type": "x""#).is_err());
}

#[test]
fn citation_details() {
    let value = FileCitation::new();
    assert_eq!(
        CitationDetails::from(value.clone()),
        CitationDetails::File(value.clone())
    );
    assert_json(
        &CitationDetails::File(value),
        r#"{"citation_type": "file"}"#,
    );
    let value = ExternalCitation::new();
    assert_eq!(
        CitationDetails::from(value.clone()),
        CitationDetails::External(value.clone())
    );
    assert_json(
        &CitationDetails::External(value),
        r#"{"citation_type": "external"}"#,
    );
    let value = WebCitation::new();
    assert_eq!(
        CitationDetails::from(value.clone()),
        CitationDetails::Web(value.clone())
    );
    assert_json(&CitationDetails::Web(value), r#"{"citation_type": "web"}"#);
    let value = MessageCitation::new();
    assert_eq!(
        CitationDetails::from(value.clone()),
        CitationDetails::Message(value.clone())
    );
    assert_json(
        &CitationDetails::Message(value),
        r#"{"citation_type": "message"}"#,
    );
    let value = MemoryCitation::new();
    assert_eq!(
        CitationDetails::from(value.clone()),
        CitationDetails::Memory(value.clone())
    );
    assert_json(
        &CitationDetails::Memory(value),
        r#"{"citation_type": "memory"}"#,
    );
    assert_unknown::<CitationDetails>(
        r#"{"citation_type": "future_kind", "payload": {"n": 1}}"#,
        |v| matches!(v, CitationDetails::Unknown(_)),
    );
    assert_unknown::<CitationDetails>(r#"{"payload": 1}"#, |v| {
        matches!(v, CitationDetails::Unknown(_))
    });
    assert!(serde_json::from_str::<CitationDetails>(
        r#"{"citation_type": "file", "descriptor": true}"#
    )
    .is_err());
    assert!(serde_json::from_str::<CitationDetails>(r#"{"citation_type": "x""#).is_err());
}

#[test]
fn block() {
    let value = ActionsBlock::new(vec![ActionsElement::from(ButtonElement::new(
        TextObject::from(PlainText::new("s")),
    ))]);
    assert_eq!(Block::from(value.clone()), Block::Actions(value.clone()));
    assert_json(
        &Block::Actions(value),
        r#"{"type": "actions", "elements": [{"type": "button", "text": {"type": "plain_text", "text": "s"}}]}"#,
    );
    let value = AlertBlock::new(TextObject::from(PlainText::new("s")));
    assert_eq!(Block::from(value.clone()), Block::Alert(value.clone()));
    assert_json(
        &Block::Alert(value),
        r#"{"type": "alert", "text": {"type": "plain_text", "text": "s"}}"#,
    );
    let value = CardBlock::new();
    assert_eq!(Block::from(value.clone()), Block::Card(value.clone()));
    assert_json(&Block::Card(value), r#"{"type": "card"}"#);
    let value = CarouselBlock::new(vec![CardBlock::new()]);
    assert_eq!(Block::from(value.clone()), Block::Carousel(value.clone()));
    assert_json(
        &Block::Carousel(value),
        r#"{"type": "carousel", "elements": [{"type": "card"}]}"#,
    );
    let value = ContainerBlock::new(
        TextObject::from(PlainText::new("s")),
        vec![Block::from(ActionsBlock::new(vec![ActionsElement::from(
            ButtonElement::new(TextObject::from(PlainText::new("s"))),
        )]))],
    );
    assert_eq!(Block::from(value.clone()), Block::Container(value.clone()));
    assert_json(
        &Block::Container(value),
        r#"{"type": "container", "title": {"type": "plain_text", "text": "s"}, "child_blocks": [{"type": "actions", "elements": [{"type": "button", "text": {"type": "plain_text", "text": "s"}}]}]}"#,
    );
    let value = ContextBlock::new(vec![ContextElement::from(ImageElement::new("s", "s"))]);
    assert_eq!(Block::from(value.clone()), Block::Context(value.clone()));
    assert_json(
        &Block::Context(value),
        r#"{"type": "context", "elements": [{"type": "image", "alt_text": "s", "image_url": "s"}]}"#,
    );
    let value = ContextActionsBlock::new(vec![ContextActionsElement::from(
        FeedbackButtonsElement::new(
            FeedbackButton::new(TextObject::from(PlainText::new("s")), "s"),
            FeedbackButton::new(TextObject::from(PlainText::new("s")), "s"),
        ),
    )]);
    assert_eq!(
        Block::from(value.clone()),
        Block::ContextActions(value.clone())
    );
    assert_json(
        &Block::ContextActions(value),
        r#"{"type": "context_actions", "elements": [{"type": "feedback_buttons", "positive_button": {"text": {"type": "plain_text", "text": "s"}, "value": "s"}, "negative_button": {"text": {"type": "plain_text", "text": "s"}, "value": "s"}}]}"#,
    );
    let value = DataTableBlock::new(vec![vec![DataTableCell::from(RawTextCell::new("s"))]], "s");
    assert_eq!(Block::from(value.clone()), Block::DataTable(value.clone()));
    assert_json(
        &Block::DataTable(value),
        r#"{"type": "data_table", "rows": [[{"type": "raw_text", "text": "s"}]], "caption": "s"}"#,
    );
    let value = DataVisualizationBlock::new(
        "s",
        Chart::from(PieChart::new(vec![Segment::new("s", 1.5)])),
    );
    assert_eq!(
        Block::from(value.clone()),
        Block::DataVisualization(value.clone())
    );
    assert_json(
        &Block::DataVisualization(value),
        r#"{"type": "data_visualization", "title": "s", "chart": {"type": "pie", "segments": [{"label": "s", "value": 1.5}]}}"#,
    );
    let value = DividerBlock::new();
    assert_eq!(Block::from(value.clone()), Block::Divider(value.clone()));
    assert_json(&Block::Divider(value), r#"{"type": "divider"}"#);
    let value = FileBlock::new("s", "s");
    assert_eq!(Block::from(value.clone()), Block::File(value.clone()));
    assert_json(
        &Block::File(value),
        r#"{"type": "file", "external_id": "s", "source": "s"}"#,
    );
    let value = HeaderBlock::new(TextObject::from(PlainText::new("s")));
    assert_eq!(Block::from(value.clone()), Block::Header(value.clone()));
    assert_json(
        &Block::Header(value),
        r#"{"type": "header", "text": {"type": "plain_text", "text": "s"}}"#,
    );
    let value = ImageBlock::new("s", "s");
    assert_eq!(Block::from(value.clone()), Block::Image(value.clone()));
    assert_json(
        &Block::Image(value),
        r#"{"type": "image", "alt_text": "s", "image_url": "s"}"#,
    );
    let value = InputBlock::new(
        TextObject::from(PlainText::new("s")),
        InputElement::from(CheckboxesElement::new(vec![OptionObject::new(
            TextObject::from(PlainText::new("s")),
            "s",
        )])),
    );
    assert_eq!(Block::from(value.clone()), Block::Input(value.clone()));
    assert_json(
        &Block::Input(value),
        r#"{"type": "input", "label": {"type": "plain_text", "text": "s"}, "element": {"type": "checkboxes", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}}"#,
    );
    let value = MarkdownBlock::new("s");
    assert_eq!(Block::from(value.clone()), Block::Markdown(value.clone()));
    assert_json(
        &Block::Markdown(value),
        r#"{"type": "markdown", "text": "s"}"#,
    );
    let value = PlanBlock::new("s", vec![PlanTask::new("s", "s", TaskStatus::Pending)]);
    assert_eq!(Block::from(value.clone()), Block::Plan(value.clone()));
    assert_json(
        &Block::Plan(value),
        r#"{"type": "plan", "title": "s", "tasks": [{"task_id": "s", "title": "s", "status": "pending"}]}"#,
    );
    let value = RichTextBlock::new(vec![RichTextObject::from(RichTextSection::new(vec![
        RichTextElement::from(AttachmentMentionElement::new("s")),
    ]))]);
    assert_eq!(Block::from(value.clone()), Block::RichText(value.clone()));
    assert_json(
        &Block::RichText(value),
        r#"{"type": "rich_text", "elements": [{"type": "rich_text_section", "elements": [{"type": "attachment_mention", "url": "s"}]}]}"#,
    );
    let value = SectionBlock::new();
    assert_eq!(Block::from(value.clone()), Block::Section(value.clone()));
    assert_json(&Block::Section(value), r#"{"type": "section"}"#);
    let value = TableBlock::new(vec![vec![TableCell::from(RawTextCell::new("s"))]]);
    assert_eq!(Block::from(value.clone()), Block::Table(value.clone()));
    assert_json(
        &Block::Table(value),
        r#"{"type": "table", "rows": [[{"type": "raw_text", "text": "s"}]]}"#,
    );
    let value = TaskCardBlock::new("s", "s", TaskStatus::Pending);
    assert_eq!(Block::from(value.clone()), Block::TaskCard(value.clone()));
    assert_json(
        &Block::TaskCard(value),
        r#"{"type": "task_card", "task_id": "s", "title": "s", "status": "pending"}"#,
    );
    let value = VideoBlock::new("s", TextObject::from(PlainText::new("s")), "s", "s");
    assert_eq!(Block::from(value.clone()), Block::Video(value.clone()));
    assert_json(
        &Block::Video(value),
        r#"{"type": "video", "alt_text": "s", "title": {"type": "plain_text", "text": "s"}, "thumbnail_url": "s", "video_url": "s"}"#,
    );
    assert_unknown::<Block>(r#"{"type": "future_kind", "payload": {"n": 1}}"#, |v| {
        matches!(v, Block::Unknown(_))
    });
    assert_unknown::<Block>(r#"{"payload": 1}"#, |v| matches!(v, Block::Unknown(_)));
    assert!(serde_json::from_str::<Block>(r#"{"type": "actions", "elements": true}"#).is_err());
    assert!(serde_json::from_str::<Block>(r#"{"type": "x""#).is_err());
}

#[test]
fn table_cell() {
    let value = RawTextCell::new("s");
    assert_eq!(
        TableCell::from(value.clone()),
        TableCell::RawText(value.clone())
    );
    assert_json(
        &TableCell::RawText(value),
        r#"{"type": "raw_text", "text": "s"}"#,
    );
    let value = RawNumberCell::new(1.5);
    assert_eq!(
        TableCell::from(value.clone()),
        TableCell::RawNumber(value.clone())
    );
    assert_json(
        &TableCell::RawNumber(value),
        r#"{"type": "raw_number", "value": 1.5}"#,
    );
    let value = RichTextBlock::new(vec![RichTextObject::from(RichTextSection::new(vec![
        RichTextElement::from(AttachmentMentionElement::new("s")),
    ]))]);
    assert_eq!(
        TableCell::from(value.clone()),
        TableCell::RichText(value.clone())
    );
    assert_json(
        &TableCell::RichText(value),
        r#"{"type": "rich_text", "elements": [{"type": "rich_text_section", "elements": [{"type": "attachment_mention", "url": "s"}]}]}"#,
    );
    assert_unknown::<TableCell>(r#"{"type": "future_kind", "payload": {"n": 1}}"#, |v| {
        matches!(v, TableCell::Unknown(_))
    });
    assert_unknown::<TableCell>(r#"{"payload": 1}"#, |v| matches!(v, TableCell::Unknown(_)));
    assert!(serde_json::from_str::<TableCell>(r#"{"type": "raw_text", "text": true}"#).is_err());
    assert!(serde_json::from_str::<TableCell>(r#"{"type": "x""#).is_err());
}

#[test]
fn data_table_cell() {
    let value = RawTextCell::new("s");
    assert_eq!(
        DataTableCell::from(value.clone()),
        DataTableCell::RawText(value.clone())
    );
    assert_json(
        &DataTableCell::RawText(value),
        r#"{"type": "raw_text", "text": "s"}"#,
    );
    let value = RawNumberCell::new(1.5);
    assert_eq!(
        DataTableCell::from(value.clone()),
        DataTableCell::RawNumber(value.clone())
    );
    assert_json(
        &DataTableCell::RawNumber(value),
        r#"{"type": "raw_number", "value": 1.5}"#,
    );
    let value = RichTextBlock::new(vec![RichTextObject::from(RichTextSection::new(vec![
        RichTextElement::from(AttachmentMentionElement::new("s")),
    ]))]);
    assert_eq!(
        DataTableCell::from(value.clone()),
        DataTableCell::RichText(value.clone())
    );
    assert_json(
        &DataTableCell::RichText(value),
        r#"{"type": "rich_text", "elements": [{"type": "rich_text_section", "elements": [{"type": "attachment_mention", "url": "s"}]}]}"#,
    );
    let value = ActionCell::new(ButtonElement::new(TextObject::from(PlainText::new("s"))));
    assert_eq!(
        DataTableCell::from(value.clone()),
        DataTableCell::ActionCell(value.clone())
    );
    assert_json(
        &DataTableCell::ActionCell(value),
        r#"{"type": "action_cell", "element": {"type": "button", "text": {"type": "plain_text", "text": "s"}}}"#,
    );
    assert_unknown::<DataTableCell>(r#"{"type": "future_kind", "payload": {"n": 1}}"#, |v| {
        matches!(v, DataTableCell::Unknown(_))
    });
    assert_unknown::<DataTableCell>(r#"{"payload": 1}"#, |v| {
        matches!(v, DataTableCell::Unknown(_))
    });
    assert!(
        serde_json::from_str::<DataTableCell>(r#"{"type": "raw_text", "text": true}"#).is_err()
    );
    assert!(serde_json::from_str::<DataTableCell>(r#"{"type": "x""#).is_err());
}

#[test]
fn action_cell_fallback() {
    let value = RawTextCell::new("s");
    assert_eq!(
        ActionCellFallback::from(value.clone()),
        ActionCellFallback::RawText(value.clone())
    );
    assert_json(
        &ActionCellFallback::RawText(value),
        r#"{"type": "raw_text", "text": "s"}"#,
    );
    let value = RawNumberCell::new(1.5);
    assert_eq!(
        ActionCellFallback::from(value.clone()),
        ActionCellFallback::RawNumber(value.clone())
    );
    assert_json(
        &ActionCellFallback::RawNumber(value),
        r#"{"type": "raw_number", "value": 1.5}"#,
    );
    assert_unknown::<ActionCellFallback>(r#"{"type": "future_kind", "payload": {"n": 1}}"#, |v| {
        matches!(v, ActionCellFallback::Unknown(_))
    });
    assert_unknown::<ActionCellFallback>(r#"{"payload": 1}"#, |v| {
        matches!(v, ActionCellFallback::Unknown(_))
    });
    assert!(
        serde_json::from_str::<ActionCellFallback>(r#"{"type": "raw_text", "text": true}"#)
            .is_err()
    );
    assert!(serde_json::from_str::<ActionCellFallback>(r#"{"type": "x""#).is_err());
}

#[test]
fn chart() {
    let value = PieChart::new(vec![Segment::new("s", 1.5)]);
    assert_eq!(Chart::from(value.clone()), Chart::Pie(value.clone()));
    assert_json(
        &Chart::Pie(value),
        r#"{"type": "pie", "segments": [{"label": "s", "value": 1.5}]}"#,
    );
    let value = BarChart::new(
        vec![DataSeries::new("s", vec![DataPoint::new("s", 1.5)])],
        AxisConfig::new(vec!["s".to_string()]),
    );
    assert_eq!(Chart::from(value.clone()), Chart::Bar(value.clone()));
    assert_json(
        &Chart::Bar(value),
        r#"{"type": "bar", "series": [{"name": "s", "data": [{"label": "s", "value": 1.5}]}], "axis_config": {"categories": ["s"]}}"#,
    );
    let value = AreaChart::new(
        vec![DataSeries::new("s", vec![DataPoint::new("s", 1.5)])],
        AxisConfig::new(vec!["s".to_string()]),
    );
    assert_eq!(Chart::from(value.clone()), Chart::Area(value.clone()));
    assert_json(
        &Chart::Area(value),
        r#"{"type": "area", "series": [{"name": "s", "data": [{"label": "s", "value": 1.5}]}], "axis_config": {"categories": ["s"]}}"#,
    );
    let value = LineChart::new(
        vec![DataSeries::new("s", vec![DataPoint::new("s", 1.5)])],
        AxisConfig::new(vec!["s".to_string()]),
    );
    assert_eq!(Chart::from(value.clone()), Chart::Line(value.clone()));
    assert_json(
        &Chart::Line(value),
        r#"{"type": "line", "series": [{"name": "s", "data": [{"label": "s", "value": 1.5}]}], "axis_config": {"categories": ["s"]}}"#,
    );
    assert_unknown::<Chart>(r#"{"type": "future_kind", "payload": {"n": 1}}"#, |v| {
        matches!(v, Chart::Unknown(_))
    });
    assert_unknown::<Chart>(r#"{"payload": 1}"#, |v| matches!(v, Chart::Unknown(_)));
    assert!(serde_json::from_str::<Chart>(r#"{"type": "pie", "segments": true}"#).is_err());
    assert!(serde_json::from_str::<Chart>(r#"{"type": "x""#).is_err());
}

#[test]
fn button_style() {
    assert_json(&ButtonStyle::Primary, r#""primary""#);
    assert_json(&ButtonStyle::Danger, r#""danger""#);
    assert_json(
        &ButtonStyle::Other("future_value".into()),
        r#""future_value""#,
    );
}

#[test]
fn conversation_type() {
    assert_json(&ConversationType::Im, r#""im""#);
    assert_json(&ConversationType::Mpim, r#""mpim""#);
    assert_json(&ConversationType::Private, r#""private""#);
    assert_json(&ConversationType::Public, r#""public""#);
    assert_json(
        &ConversationType::Other("future_value".into()),
        r#""future_value""#,
    );
}

#[test]
fn trigger_action() {
    assert_json(&TriggerAction::OnEnterPressed, r#""on_enter_pressed""#);
    assert_json(
        &TriggerAction::OnCharacterEntered,
        r#""on_character_entered""#,
    );
    assert_json(
        &TriggerAction::Other("future_value".into()),
        r#""future_value""#,
    );
}

#[test]
fn rich_text_list_style() {
    assert_json(&RichTextListStyle::Bullet, r#""bullet""#);
    assert_json(&RichTextListStyle::Ordered, r#""ordered""#);
    assert_json(
        &RichTextListStyle::Other("future_value".into()),
        r#""future_value""#,
    );
}

#[test]
fn broadcast_range() {
    assert_json(&BroadcastRange::Here, r#""here""#);
    assert_json(&BroadcastRange::Channel, r#""channel""#);
    assert_json(&BroadcastRange::Everyone, r#""everyone""#);
    assert_json(
        &BroadcastRange::Other("future_value".into()),
        r#""future_value""#,
    );
}

#[test]
fn tag_color() {
    assert_json(&TagColor::Gray, r#""gray""#);
    assert_json(&TagColor::Brown, r#""brown""#);
    assert_json(&TagColor::Purple, r#""purple""#);
    assert_json(&TagColor::Indigo, r#""indigo""#);
    assert_json(&TagColor::Blue, r#""blue""#);
    assert_json(&TagColor::Green, r#""green""#);
    assert_json(&TagColor::Yellow, r#""yellow""#);
    assert_json(&TagColor::Orange, r#""orange""#);
    assert_json(&TagColor::Red, r#""red""#);
    assert_json(&TagColor::Other("future_value".into()), r#""future_value""#);
}

#[test]
fn alert_level() {
    assert_json(&AlertLevel::Default, r#""default""#);
    assert_json(&AlertLevel::Info, r#""info""#);
    assert_json(&AlertLevel::Warning, r#""warning""#);
    assert_json(&AlertLevel::Error, r#""error""#);
    assert_json(&AlertLevel::Success, r#""success""#);
    assert_json(
        &AlertLevel::Other("future_value".into()),
        r#""future_value""#,
    );
}

#[test]
fn container_width() {
    assert_json(&ContainerWidth::Narrow, r#""narrow""#);
    assert_json(&ContainerWidth::Standard, r#""standard""#);
    assert_json(&ContainerWidth::Wide, r#""wide""#);
    assert_json(&ContainerWidth::Full, r#""full""#);
    assert_json(
        &ContainerWidth::Other("future_value".into()),
        r#""future_value""#,
    );
}

#[test]
fn task_status() {
    assert_json(&TaskStatus::Pending, r#""pending""#);
    assert_json(&TaskStatus::InProgress, r#""in_progress""#);
    assert_json(&TaskStatus::Complete, r#""complete""#);
    assert_json(&TaskStatus::Error, r#""error""#);
    assert_json(
        &TaskStatus::Other("future_value".into()),
        r#""future_value""#,
    );
}

#[test]
fn column_align() {
    assert_json(&ColumnAlign::Left, r#""left""#);
    assert_json(&ColumnAlign::Center, r#""center""#);
    assert_json(&ColumnAlign::Right, r#""right""#);
    assert_json(
        &ColumnAlign::Other("future_value".into()),
        r#""future_value""#,
    );
}

#[test]
fn view_type() {
    assert_json(&ViewType::Modal, r#""modal""#);
    assert_json(&ViewType::Home, r#""home""#);
    assert_json(&ViewType::WorkflowStep, r#""workflow_step""#);
    assert_json(&ViewType::Other("future_value".into()), r#""future_value""#);
}
