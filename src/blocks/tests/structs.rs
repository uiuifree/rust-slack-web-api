use super::*;

#[test]
fn plain_text() {
    assert_json(
        &PlainText::new("s"),
        r#"{"type": "plain_text", "text": "s"}"#,
    );
    assert_json(
        &PlainText::new("s").emoji(true),
        r#"{"type": "plain_text", "text": "s", "emoji": true}"#,
    );
}

#[test]
fn mrkdwn() {
    assert_json(&Mrkdwn::new("s"), r#"{"type": "mrkdwn", "text": "s"}"#);
    assert_json(
        &Mrkdwn::new("s").verbatim(true),
        r#"{"type": "mrkdwn", "text": "s", "verbatim": true}"#,
    );
}

#[test]
fn confirmation_dialog() {
    assert_json(
        &ConfirmationDialog::new(
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
        ),
        r#"{"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}"#,
    );
    assert_json(
        &ConfirmationDialog::new(
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
        )
        .style(ButtonStyle::Primary),
        r#"{"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}, "style": "primary"}"#,
    );
}

#[test]
fn option_object() {
    assert_json(
        &OptionObject::new(TextObject::from(PlainText::new("s")), "s"),
        r#"{"text": {"type": "plain_text", "text": "s"}, "value": "s"}"#,
    );
    assert_json(
        &OptionObject::new(TextObject::from(PlainText::new("s")), "s")
            .description(TextObject::from(PlainText::new("s")))
            .url("s"),
        r#"{"text": {"type": "plain_text", "text": "s"}, "value": "s", "description": {"type": "plain_text", "text": "s"}, "url": "s"}"#,
    );
}

#[test]
fn option_group() {
    assert_json(
        &OptionGroup::new(
            TextObject::from(PlainText::new("s")),
            vec![OptionObject::new(
                TextObject::from(PlainText::new("s")),
                "s",
            )],
        ),
        r#"{"label": {"type": "plain_text", "text": "s"}, "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
}

#[test]
fn conversation_filter() {
    assert_json(&ConversationFilter::new(), r#"{}"#);
    assert_json(
        &ConversationFilter::new()
            .include(vec![ConversationType::Im])
            .exclude_external_shared_channels(true)
            .exclude_bot_users(true),
        r#"{"include": ["im"], "exclude_external_shared_channels": true, "exclude_bot_users": true}"#,
    );
}

#[test]
fn dispatch_action_config() {
    assert_json(&DispatchActionConfig::new(), r#"{}"#);
    assert_json(
        &DispatchActionConfig::new().trigger_actions_on(vec![TriggerAction::OnEnterPressed]),
        r#"{"trigger_actions_on": ["on_enter_pressed"]}"#,
    );
}

#[test]
fn slack_file() {
    assert_json(&SlackFile::from_id("s"), r#"{"id": "s"}"#);
    assert_json(
        &SlackFile::from_id("s").url("s").id("s"),
        r#"{"id": "s", "url": "s"}"#,
    );
}

#[test]
fn slack_icon() {
    assert_json(&SlackIcon::new("s"), r#"{"type": "icon", "name": "s"}"#);
}

#[test]
fn workflow() {
    assert_json(
        &Workflow::new(Trigger::new("s")),
        r#"{"trigger": {"url": "s"}}"#,
    );
}

#[test]
fn trigger() {
    assert_json(&Trigger::new("s"), r#"{"url": "s"}"#);
    assert_json(
        &Trigger::new("s").customizable_input_parameters(vec![InputParameter::new("s", "s")]),
        r#"{"url": "s", "customizable_input_parameters": [{"name": "s", "value": "s"}]}"#,
    );
}

#[test]
fn input_parameter() {
    assert_json(
        &InputParameter::new("s", "s"),
        r#"{"name": "s", "value": "s"}"#,
    );
}

#[test]
fn button_element() {
    assert_json(
        &ButtonElement::new(TextObject::from(PlainText::new("s"))),
        r#"{"type": "button", "text": {"type": "plain_text", "text": "s"}}"#,
    );
    assert_json(
        &ButtonElement::new(TextObject::from(PlainText::new("s")))
            .action_id("s")
            .url("s")
            .value("s")
            .style(ButtonStyle::Primary)
            .confirm(ConfirmationDialog::new(
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
            ))
            .accessibility_label("s")
            .agent_prompt("s")
            .agent_prompt_display("s")
            .visible_to_user_ids(vec!["s".to_string()]),
        r#"{"type": "button", "text": {"type": "plain_text", "text": "s"}, "action_id": "s", "url": "s", "value": "s", "style": "primary", "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}, "accessibility_label": "s", "agent_prompt": "s", "agent_prompt_display": "s", "visible_to_user_ids": ["s"]}"#,
    );
}

#[test]
fn checkboxes_element() {
    assert_json(
        &CheckboxesElement::new(vec![OptionObject::new(
            TextObject::from(PlainText::new("s")),
            "s",
        )]),
        r#"{"type": "checkboxes", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    assert_json(
        &CheckboxesElement::new(vec![OptionObject::new(
            TextObject::from(PlainText::new("s")),
            "s",
        )])
        .action_id("s")
        .initial_options(vec![OptionObject::new(
            TextObject::from(PlainText::new("s")),
            "s",
        )])
        .confirm(ConfirmationDialog::new(
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
        ))
        .focus_on_load(true)
        .visible_to_user_ids(vec!["s".to_string()]),
        r#"{"type": "checkboxes", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}], "action_id": "s", "initial_options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}], "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}, "focus_on_load": true, "visible_to_user_ids": ["s"]}"#,
    );
}

#[test]
fn date_picker_element() {
    assert_json(&DatePickerElement::new(), r#"{"type": "datepicker"}"#);
    assert_json(
        &DatePickerElement::new()
            .action_id("s")
            .initial_date("s")
            .confirm(ConfirmationDialog::new(
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
            ))
            .focus_on_load(true)
            .placeholder(TextObject::from(PlainText::new("s"))),
        r#"{"type": "datepicker", "action_id": "s", "initial_date": "s", "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}, "focus_on_load": true, "placeholder": {"type": "plain_text", "text": "s"}}"#,
    );
}

#[test]
fn datetime_picker_element() {
    assert_json(
        &DatetimePickerElement::new(),
        r#"{"type": "datetimepicker"}"#,
    );
    assert_json(
        &DatetimePickerElement::new()
            .action_id("s")
            .initial_date_time(1)
            .confirm(ConfirmationDialog::new(
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
            ))
            .focus_on_load(true),
        r#"{"type": "datetimepicker", "action_id": "s", "initial_date_time": 1, "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}, "focus_on_load": true}"#,
    );
}

#[test]
fn email_input_element() {
    assert_json(&EmailInputElement::new(), r#"{"type": "email_text_input"}"#);
    assert_json(
        &EmailInputElement::new()
            .action_id("s")
            .initial_value("s")
            .dispatch_action_config(DispatchActionConfig::new())
            .focus_on_load(true)
            .placeholder(TextObject::from(PlainText::new("s"))),
        r#"{"type": "email_text_input", "action_id": "s", "initial_value": "s", "dispatch_action_config": {}, "focus_on_load": true, "placeholder": {"type": "plain_text", "text": "s"}}"#,
    );
}

#[test]
fn feedback_buttons_element() {
    assert_json(
        &FeedbackButtonsElement::new(
            FeedbackButton::new(TextObject::from(PlainText::new("s")), "s"),
            FeedbackButton::new(TextObject::from(PlainText::new("s")), "s"),
        ),
        r#"{"type": "feedback_buttons", "positive_button": {"text": {"type": "plain_text", "text": "s"}, "value": "s"}, "negative_button": {"text": {"type": "plain_text", "text": "s"}, "value": "s"}}"#,
    );
    assert_json(
        &FeedbackButtonsElement::new(
            FeedbackButton::new(TextObject::from(PlainText::new("s")), "s"),
            FeedbackButton::new(TextObject::from(PlainText::new("s")), "s"),
        )
        .action_id("s"),
        r#"{"type": "feedback_buttons", "positive_button": {"text": {"type": "plain_text", "text": "s"}, "value": "s"}, "negative_button": {"text": {"type": "plain_text", "text": "s"}, "value": "s"}, "action_id": "s"}"#,
    );
}

#[test]
fn feedback_button() {
    assert_json(
        &FeedbackButton::new(TextObject::from(PlainText::new("s")), "s"),
        r#"{"text": {"type": "plain_text", "text": "s"}, "value": "s"}"#,
    );
    assert_json(
        &FeedbackButton::new(TextObject::from(PlainText::new("s")), "s").accessibility_label("s"),
        r#"{"text": {"type": "plain_text", "text": "s"}, "value": "s", "accessibility_label": "s"}"#,
    );
}

#[test]
fn file_input_element() {
    assert_json(&FileInputElement::new(), r#"{"type": "file_input"}"#);
    assert_json(
        &FileInputElement::new()
            .action_id("s")
            .filetypes(vec!["s".to_string()])
            .max_files(1),
        r#"{"type": "file_input", "action_id": "s", "filetypes": ["s"], "max_files": 1}"#,
    );
}

#[test]
fn icon_button_element() {
    assert_json(
        &IconButtonElement::new("s", TextObject::from(PlainText::new("s"))),
        r#"{"type": "icon_button", "icon": "s", "text": {"type": "plain_text", "text": "s"}}"#,
    );
    assert_json(
        &IconButtonElement::new("s", TextObject::from(PlainText::new("s")))
            .action_id("s")
            .value("s")
            .confirm(ConfirmationDialog::new(
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
            ))
            .accessibility_label("s")
            .visible_to_user_ids(vec!["s".to_string()]),
        r#"{"type": "icon_button", "icon": "s", "text": {"type": "plain_text", "text": "s"}, "action_id": "s", "value": "s", "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}, "accessibility_label": "s", "visible_to_user_ids": ["s"]}"#,
    );
}

#[test]
fn image_element() {
    assert_json(
        &ImageElement::new("s", "s"),
        r#"{"type": "image", "alt_text": "s", "image_url": "s"}"#,
    );
    assert_json(
        &ImageElement::new("s", "s")
            .image_url("s")
            .slack_file(SlackFile::from_id("s")),
        r#"{"type": "image", "alt_text": "s", "image_url": "s", "slack_file": {"id": "s"}}"#,
    );
}

#[test]
fn number_input_element() {
    assert_json(
        &NumberInputElement::new(true),
        r#"{"type": "number_input", "is_decimal_allowed": true}"#,
    );
    assert_json(
        &NumberInputElement::new(true)
            .action_id("s")
            .initial_value("s")
            .min_value("s")
            .max_value("s")
            .dispatch_action_config(DispatchActionConfig::new())
            .focus_on_load(true)
            .placeholder(TextObject::from(PlainText::new("s"))),
        r#"{"type": "number_input", "is_decimal_allowed": true, "action_id": "s", "initial_value": "s", "min_value": "s", "max_value": "s", "dispatch_action_config": {}, "focus_on_load": true, "placeholder": {"type": "plain_text", "text": "s"}}"#,
    );
}

#[test]
fn overflow_element() {
    assert_json(
        &OverflowElement::new(vec![OptionObject::new(
            TextObject::from(PlainText::new("s")),
            "s",
        )]),
        r#"{"type": "overflow", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    assert_json(
        &OverflowElement::new(vec![OptionObject::new(
            TextObject::from(PlainText::new("s")),
            "s",
        )])
        .action_id("s")
        .confirm(ConfirmationDialog::new(
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
        )),
        r#"{"type": "overflow", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}], "action_id": "s", "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}}"#,
    );
}

#[test]
fn plain_text_input_element() {
    assert_json(
        &PlainTextInputElement::new(),
        r#"{"type": "plain_text_input"}"#,
    );
    assert_json(
        &PlainTextInputElement::new()
            .action_id("s")
            .initial_value("s")
            .multiline(true)
            .min_length(1)
            .max_length(1)
            .dispatch_action_config(DispatchActionConfig::new())
            .focus_on_load(true)
            .placeholder(TextObject::from(PlainText::new("s"))),
        r#"{"type": "plain_text_input", "action_id": "s", "initial_value": "s", "multiline": true, "min_length": 1, "max_length": 1, "dispatch_action_config": {}, "focus_on_load": true, "placeholder": {"type": "plain_text", "text": "s"}}"#,
    );
}

#[test]
fn radio_buttons_element() {
    assert_json(
        &RadioButtonsElement::new(vec![OptionObject::new(
            TextObject::from(PlainText::new("s")),
            "s",
        )]),
        r#"{"type": "radio_buttons", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    assert_json(
        &RadioButtonsElement::new(vec![OptionObject::new(
            TextObject::from(PlainText::new("s")),
            "s",
        )])
        .action_id("s")
        .initial_option(OptionObject::new(
            TextObject::from(PlainText::new("s")),
            "s",
        ))
        .confirm(ConfirmationDialog::new(
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
        ))
        .focus_on_load(true),
        r#"{"type": "radio_buttons", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}], "action_id": "s", "initial_option": {"text": {"type": "plain_text", "text": "s"}, "value": "s"}, "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}, "focus_on_load": true}"#,
    );
}

#[test]
fn rich_text_input_element() {
    assert_json(
        &RichTextInputElement::new("s"),
        r#"{"type": "rich_text_input", "action_id": "s"}"#,
    );
    assert_json(
        &RichTextInputElement::new("s")
            .initial_value(RichTextBlock::new(vec![RichTextObject::from(
                RichTextSection::new(vec![RichTextElement::from(AttachmentMentionElement::new(
                    "s",
                ))]),
            )]))
            .dispatch_action_config(DispatchActionConfig::new())
            .focus_on_load(true)
            .placeholder(TextObject::from(PlainText::new("s")))
            .min_lines(1)
            .max_lines(1),
        r#"{"type": "rich_text_input", "action_id": "s", "initial_value": {"type": "rich_text", "elements": [{"type": "rich_text_section", "elements": [{"type": "attachment_mention", "url": "s"}]}]}, "dispatch_action_config": {}, "focus_on_load": true, "placeholder": {"type": "plain_text", "text": "s"}, "min_lines": 1, "max_lines": 1}"#,
    );
}

#[test]
fn time_picker_element() {
    assert_json(&TimePickerElement::new(), r#"{"type": "timepicker"}"#);
    assert_json(
        &TimePickerElement::new()
            .action_id("s")
            .initial_time("s")
            .confirm(ConfirmationDialog::new(
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
            ))
            .focus_on_load(true)
            .placeholder(TextObject::from(PlainText::new("s")))
            .timezone("s"),
        r#"{"type": "timepicker", "action_id": "s", "initial_time": "s", "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}, "focus_on_load": true, "placeholder": {"type": "plain_text", "text": "s"}, "timezone": "s"}"#,
    );
}

#[test]
fn url_input_element() {
    assert_json(&UrlInputElement::new(), r#"{"type": "url_text_input"}"#);
    assert_json(
        &UrlInputElement::new()
            .action_id("s")
            .initial_value("s")
            .dispatch_action_config(DispatchActionConfig::new())
            .focus_on_load(true)
            .placeholder(TextObject::from(PlainText::new("s"))),
        r#"{"type": "url_text_input", "action_id": "s", "initial_value": "s", "dispatch_action_config": {}, "focus_on_load": true, "placeholder": {"type": "plain_text", "text": "s"}}"#,
    );
}

#[test]
fn workflow_button_element() {
    assert_json(
        &WorkflowButtonElement::new(
            TextObject::from(PlainText::new("s")),
            Workflow::new(Trigger::new("s")),
            "s",
        ),
        r#"{"type": "workflow_button", "text": {"type": "plain_text", "text": "s"}, "workflow": {"trigger": {"url": "s"}}, "action_id": "s"}"#,
    );
    assert_json(
        &WorkflowButtonElement::new(
            TextObject::from(PlainText::new("s")),
            Workflow::new(Trigger::new("s")),
            "s",
        )
        .style(ButtonStyle::Primary)
        .accessibility_label("s"),
        r#"{"type": "workflow_button", "text": {"type": "plain_text", "text": "s"}, "workflow": {"trigger": {"url": "s"}}, "action_id": "s", "style": "primary", "accessibility_label": "s"}"#,
    );
}

#[test]
fn url_source_element() {
    assert_json(
        &UrlSourceElement::new("s", "s"),
        r#"{"type": "url", "url": "s", "text": "s"}"#,
    );
}

#[test]
fn static_select_element() {
    assert_json(
        &StaticSelectElement::new(vec![OptionObject::new(
            TextObject::from(PlainText::new("s")),
            "s",
        )]),
        r#"{"type": "static_select", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    assert_json(
        &StaticSelectElement::new(vec![OptionObject::new(
            TextObject::from(PlainText::new("s")),
            "s",
        )])
        .action_id("s")
        .option_groups(vec![OptionGroup::new(
            TextObject::from(PlainText::new("s")),
            vec![OptionObject::new(
                TextObject::from(PlainText::new("s")),
                "s",
            )],
        )])
        .initial_option(OptionObject::new(
            TextObject::from(PlainText::new("s")),
            "s",
        ))
        .confirm(ConfirmationDialog::new(
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
        ))
        .focus_on_load(true)
        .placeholder(TextObject::from(PlainText::new("s"))),
        r#"{"type": "static_select", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}], "action_id": "s", "option_groups": [{"label": {"type": "plain_text", "text": "s"}, "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}], "initial_option": {"text": {"type": "plain_text", "text": "s"}, "value": "s"}, "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}, "focus_on_load": true, "placeholder": {"type": "plain_text", "text": "s"}}"#,
    );
}

#[test]
fn external_select_element() {
    assert_json(
        &ExternalSelectElement::new(),
        r#"{"type": "external_select"}"#,
    );
    assert_json(
        &ExternalSelectElement::new()
            .action_id("s")
            .initial_option(OptionObject::new(
                TextObject::from(PlainText::new("s")),
                "s",
            ))
            .min_query_length(1)
            .confirm(ConfirmationDialog::new(
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
            ))
            .focus_on_load(true)
            .placeholder(TextObject::from(PlainText::new("s"))),
        r#"{"type": "external_select", "action_id": "s", "initial_option": {"text": {"type": "plain_text", "text": "s"}, "value": "s"}, "min_query_length": 1, "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}, "focus_on_load": true, "placeholder": {"type": "plain_text", "text": "s"}}"#,
    );
}

#[test]
fn users_select_element() {
    assert_json(&UsersSelectElement::new(), r#"{"type": "users_select"}"#);
    assert_json(
        &UsersSelectElement::new()
            .action_id("s")
            .initial_user("s")
            .confirm(ConfirmationDialog::new(
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
            ))
            .focus_on_load(true)
            .placeholder(TextObject::from(PlainText::new("s"))),
        r#"{"type": "users_select", "action_id": "s", "initial_user": "s", "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}, "focus_on_load": true, "placeholder": {"type": "plain_text", "text": "s"}}"#,
    );
}

#[test]
fn conversations_select_element() {
    assert_json(
        &ConversationsSelectElement::new(),
        r#"{"type": "conversations_select"}"#,
    );
    assert_json(
        &ConversationsSelectElement::new()
            .action_id("s")
            .initial_conversation("s")
            .default_to_current_conversation(true)
            .confirm(ConfirmationDialog::new(
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
            ))
            .response_url_enabled(true)
            .filter(ConversationFilter::new())
            .focus_on_load(true)
            .placeholder(TextObject::from(PlainText::new("s"))),
        r#"{"type": "conversations_select", "action_id": "s", "initial_conversation": "s", "default_to_current_conversation": true, "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}, "response_url_enabled": true, "filter": {}, "focus_on_load": true, "placeholder": {"type": "plain_text", "text": "s"}}"#,
    );
}

#[test]
fn channels_select_element() {
    assert_json(
        &ChannelsSelectElement::new(),
        r#"{"type": "channels_select"}"#,
    );
    assert_json(
        &ChannelsSelectElement::new()
            .action_id("s")
            .initial_channel("s")
            .confirm(ConfirmationDialog::new(
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
            ))
            .response_url_enabled(true)
            .focus_on_load(true)
            .placeholder(TextObject::from(PlainText::new("s"))),
        r#"{"type": "channels_select", "action_id": "s", "initial_channel": "s", "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}, "response_url_enabled": true, "focus_on_load": true, "placeholder": {"type": "plain_text", "text": "s"}}"#,
    );
}

#[test]
fn multi_static_select_element() {
    assert_json(
        &MultiStaticSelectElement::new(vec![OptionObject::new(
            TextObject::from(PlainText::new("s")),
            "s",
        )]),
        r#"{"type": "multi_static_select", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}"#,
    );
    assert_json(
        &MultiStaticSelectElement::new(vec![OptionObject::new(
            TextObject::from(PlainText::new("s")),
            "s",
        )])
        .action_id("s")
        .option_groups(vec![OptionGroup::new(
            TextObject::from(PlainText::new("s")),
            vec![OptionObject::new(
                TextObject::from(PlainText::new("s")),
                "s",
            )],
        )])
        .initial_options(vec![OptionObject::new(
            TextObject::from(PlainText::new("s")),
            "s",
        )])
        .confirm(ConfirmationDialog::new(
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
            TextObject::from(PlainText::new("s")),
        ))
        .max_selected_items(1)
        .focus_on_load(true)
        .placeholder(TextObject::from(PlainText::new("s"))),
        r#"{"type": "multi_static_select", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}], "action_id": "s", "option_groups": [{"label": {"type": "plain_text", "text": "s"}, "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}], "initial_options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}], "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}, "max_selected_items": 1, "focus_on_load": true, "placeholder": {"type": "plain_text", "text": "s"}}"#,
    );
}

#[test]
fn multi_external_select_element() {
    assert_json(
        &MultiExternalSelectElement::new(),
        r#"{"type": "multi_external_select"}"#,
    );
    assert_json(
        &MultiExternalSelectElement::new()
            .action_id("s")
            .min_query_length(1)
            .initial_options(vec![OptionObject::new(
                TextObject::from(PlainText::new("s")),
                "s",
            )])
            .confirm(ConfirmationDialog::new(
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
            ))
            .max_selected_items(1)
            .placeholder(TextObject::from(PlainText::new("s"))),
        r#"{"type": "multi_external_select", "action_id": "s", "min_query_length": 1, "initial_options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}], "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}, "max_selected_items": 1, "placeholder": {"type": "plain_text", "text": "s"}}"#,
    );
}

#[test]
fn multi_users_select_element() {
    assert_json(
        &MultiUsersSelectElement::new(),
        r#"{"type": "multi_users_select"}"#,
    );
    assert_json(
        &MultiUsersSelectElement::new()
            .action_id("s")
            .initial_users(vec!["s".to_string()])
            .confirm(ConfirmationDialog::new(
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
            ))
            .max_selected_items(1)
            .focus_on_load(true)
            .placeholder(TextObject::from(PlainText::new("s"))),
        r#"{"type": "multi_users_select", "action_id": "s", "initial_users": ["s"], "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}, "max_selected_items": 1, "focus_on_load": true, "placeholder": {"type": "plain_text", "text": "s"}}"#,
    );
}

#[test]
fn multi_conversations_select_element() {
    assert_json(
        &MultiConversationsSelectElement::new(),
        r#"{"type": "multi_conversations_select"}"#,
    );
    assert_json(
        &MultiConversationsSelectElement::new()
            .action_id("s")
            .initial_conversations(vec!["s".to_string()])
            .default_to_current_conversation(true)
            .confirm(ConfirmationDialog::new(
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
            ))
            .max_selected_items(1)
            .filter(ConversationFilter::new())
            .focus_on_load(true)
            .placeholder(TextObject::from(PlainText::new("s"))),
        r#"{"type": "multi_conversations_select", "action_id": "s", "initial_conversations": ["s"], "default_to_current_conversation": true, "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}, "max_selected_items": 1, "filter": {}, "focus_on_load": true, "placeholder": {"type": "plain_text", "text": "s"}}"#,
    );
}

#[test]
fn multi_channels_select_element() {
    assert_json(
        &MultiChannelsSelectElement::new(),
        r#"{"type": "multi_channels_select"}"#,
    );
    assert_json(
        &MultiChannelsSelectElement::new()
            .action_id("s")
            .initial_channels(vec!["s".to_string()])
            .confirm(ConfirmationDialog::new(
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
                TextObject::from(PlainText::new("s")),
            ))
            .max_selected_items(1)
            .focus_on_load(true)
            .placeholder(TextObject::from(PlainText::new("s"))),
        r#"{"type": "multi_channels_select", "action_id": "s", "initial_channels": ["s"], "confirm": {"title": {"type": "plain_text", "text": "s"}, "text": {"type": "plain_text", "text": "s"}, "confirm": {"type": "plain_text", "text": "s"}, "deny": {"type": "plain_text", "text": "s"}}, "max_selected_items": 1, "focus_on_load": true, "placeholder": {"type": "plain_text", "text": "s"}}"#,
    );
}

#[test]
fn rich_text_block() {
    assert_json(
        &RichTextBlock::new(vec![RichTextObject::from(RichTextSection::new(vec![
            RichTextElement::from(AttachmentMentionElement::new("s")),
        ]))]),
        r#"{"type": "rich_text", "elements": [{"type": "rich_text_section", "elements": [{"type": "attachment_mention", "url": "s"}]}]}"#,
    );
    assert_json(
        &RichTextBlock::new(vec![RichTextObject::from(RichTextSection::new(vec![
            RichTextElement::from(AttachmentMentionElement::new("s")),
        ]))])
        .block_id("s"),
        r#"{"type": "rich_text", "elements": [{"type": "rich_text_section", "elements": [{"type": "attachment_mention", "url": "s"}]}], "block_id": "s"}"#,
    );
}

#[test]
fn rich_text_section() {
    assert_json(
        &RichTextSection::new(vec![RichTextElement::from(AttachmentMentionElement::new(
            "s",
        ))]),
        r#"{"type": "rich_text_section", "elements": [{"type": "attachment_mention", "url": "s"}]}"#,
    );
}

#[test]
fn rich_text_list() {
    assert_json(
        &RichTextList::new(
            RichTextListStyle::Bullet,
            vec![RichTextSection::new(vec![RichTextElement::from(
                AttachmentMentionElement::new("s"),
            )])],
        ),
        r#"{"type": "rich_text_list", "style": "bullet", "elements": [{"type": "rich_text_section", "elements": [{"type": "attachment_mention", "url": "s"}]}]}"#,
    );
    assert_json(
        &RichTextList::new(
            RichTextListStyle::Bullet,
            vec![RichTextSection::new(vec![RichTextElement::from(
                AttachmentMentionElement::new("s"),
            )])],
        )
        .indent(1)
        .offset(1)
        .border(1),
        r#"{"type": "rich_text_list", "style": "bullet", "elements": [{"type": "rich_text_section", "elements": [{"type": "attachment_mention", "url": "s"}]}], "indent": 1, "offset": 1, "border": 1}"#,
    );
}

#[test]
fn rich_text_preformatted() {
    assert_json(
        &RichTextPreformatted::new(vec![PreformattedElement::from(TextElement::new("s"))]),
        r#"{"type": "rich_text_preformatted", "elements": [{"type": "text", "text": "s"}]}"#,
    );
    assert_json(
        &RichTextPreformatted::new(vec![PreformattedElement::from(TextElement::new("s"))])
            .border(1)
            .language("s"),
        r#"{"type": "rich_text_preformatted", "elements": [{"type": "text", "text": "s"}], "border": 1, "language": "s"}"#,
    );
}

#[test]
fn rich_text_quote() {
    assert_json(
        &RichTextQuote::new(vec![RichTextElement::from(AttachmentMentionElement::new(
            "s",
        ))]),
        r#"{"type": "rich_text_quote", "elements": [{"type": "attachment_mention", "url": "s"}]}"#,
    );
    assert_json(
        &RichTextQuote::new(vec![RichTextElement::from(AttachmentMentionElement::new(
            "s",
        ))])
        .border(1),
        r#"{"type": "rich_text_quote", "elements": [{"type": "attachment_mention", "url": "s"}], "border": 1}"#,
    );
}

#[test]
fn rich_text_style() {
    assert_json(&RichTextStyle::new(), r#"{}"#);
    assert_json(
        &RichTextStyle::new()
            .bold(true)
            .italic(true)
            .strike(true)
            .highlight(true)
            .client_highlight(true)
            .underline(true)
            .unlink(true)
            .code(true),
        r#"{"bold": true, "italic": true, "strike": true, "highlight": true, "client_highlight": true, "underline": true, "unlink": true, "code": true}"#,
    );
}

#[test]
fn attachment_mention_element() {
    assert_json(
        &AttachmentMentionElement::new("s"),
        r#"{"type": "attachment_mention", "url": "s"}"#,
    );
    assert_json(
        &AttachmentMentionElement::new("s")
            .text("s")
            .app_id("s")
            .entity_id("s")
            .icon_url("s")
            .channel_id("s")
            .ts("s")
            .full_size_preview_enabled(true)
            .icon_name("s")
            .reference_object_type("s")
            .product_name("s")
            .style(RichTextStyle::new()),
        r#"{"type": "attachment_mention", "url": "s", "text": "s", "app_id": "s", "entity_id": "s", "icon_url": "s", "channel_id": "s", "ts": "s", "full_size_preview_enabled": true, "icon_name": "s", "reference_object_type": "s", "product_name": "s", "style": {}}"#,
    );
}

#[test]
fn broadcast_element() {
    assert_json(
        &BroadcastElement::new(BroadcastRange::Here),
        r#"{"type": "broadcast", "range": "here"}"#,
    );
    assert_json(
        &BroadcastElement::new(BroadcastRange::Here).style(RichTextStyle::new()),
        r#"{"type": "broadcast", "range": "here", "style": {}}"#,
    );
}

#[test]
fn canvas_element() {
    assert_json(
        &CanvasElement::new("s"),
        r#"{"type": "canvas", "file_id": "s"}"#,
    );
    assert_json(
        &CanvasElement::new("s")
            .label("s")
            .hide_title(true)
            .section_id("s")
            .style(RichTextStyle::new())
            .text("s")
            .url("s")
            .is_skill_invocation(true),
        r#"{"type": "canvas", "file_id": "s", "label": "s", "hide_title": true, "section_id": "s", "style": {}, "text": "s", "url": "s", "is_skill_invocation": true}"#,
    );
}

#[test]
fn canvas_user_mention_element() {
    assert_json(
        &CanvasUserMentionElement::new("s"),
        r#"{"type": "canvas_user_mention", "user_id": "s"}"#,
    );
    assert_json(
        &CanvasUserMentionElement::new("s")
            .thread_id("s")
            .style(RichTextStyle::new()),
        r#"{"type": "canvas_user_mention", "user_id": "s", "thread_id": "s", "style": {}}"#,
    );
}

#[test]
fn canvas_message_unfurl_element() {
    assert_json(
        &CanvasMessageUnfurlElement::new("s", "s"),
        r#"{"type": "canvas_message_unfurl", "root_message_ts": "s", "root_message_channel": "s"}"#,
    );
    assert_json(
        &CanvasMessageUnfurlElement::new("s", "s").style(RichTextStyle::new()),
        r#"{"type": "canvas_message_unfurl", "root_message_ts": "s", "root_message_channel": "s", "style": {}}"#,
    );
}

#[test]
fn channel_element() {
    assert_json(
        &ChannelElement::new("s"),
        r#"{"type": "channel", "channel_id": "s"}"#,
    );
    assert_json(
        &ChannelElement::new("s")
            .tab_id("s")
            .style(RichTextStyle::new())
            .from_llm(true),
        r#"{"type": "channel", "channel_id": "s", "tab_id": "s", "style": {}, "from_llm": true}"#,
    );
}

#[test]
fn citation_element() {
    assert_json(
        &CitationElement::new("s", "s", 1, CitationDetails::from(FileCitation::new())),
        r#"{"type": "citation", "url": "s", "text": "s", "index": 1, "details": {"citation_type": "file"}}"#,
    );
    assert_json(
        &CitationElement::new("s", "s", 1, CitationDetails::from(FileCitation::new()))
            .from_llm(true)
            .is_slack_url(true),
        r#"{"type": "citation", "url": "s", "text": "s", "index": 1, "details": {"citation_type": "file"}, "from_llm": true, "is_slack_url": true}"#,
    );
}

#[test]
fn file_citation() {
    assert_json(&FileCitation::new(), r#"{"citation_type": "file"}"#);
    assert_json(
        &FileCitation::new().descriptor("s").file_id("s"),
        r#"{"citation_type": "file", "descriptor": "s", "file_id": "s"}"#,
    );
}

#[test]
fn external_citation() {
    assert_json(&ExternalCitation::new(), r#"{"citation_type": "external"}"#);
    assert_json(
        &ExternalCitation::new().app_name("s").app_icon_url("s"),
        r#"{"citation_type": "external", "app_name": "s", "app_icon_url": "s"}"#,
    );
}

#[test]
fn web_citation() {
    assert_json(&WebCitation::new(), r#"{"citation_type": "web"}"#);
    assert_json(
        &WebCitation::new().display_name("s").title("s").snippet("s"),
        r#"{"citation_type": "web", "display_name": "s", "title": "s", "snippet": "s"}"#,
    );
}

#[test]
fn message_citation() {
    assert_json(&MessageCitation::new(), r#"{"citation_type": "message"}"#);
    assert_json(
        &MessageCitation::new().channel("s").message_ts("s"),
        r#"{"citation_type": "message", "channel": "s", "message_ts": "s"}"#,
    );
}

#[test]
fn memory_citation() {
    assert_json(&MemoryCitation::new(), r#"{"citation_type": "memory"}"#);
    assert_json(
        &MemoryCitation::new().memory_id("s"),
        r#"{"citation_type": "memory", "memory_id": "s"}"#,
    );
}

#[test]
fn color_element() {
    assert_json(
        &ColorElement::new("s"),
        r#"{"type": "color", "value": "s"}"#,
    );
    assert_json(
        &ColorElement::new("s").style(RichTextStyle::new()),
        r#"{"type": "color", "value": "s", "style": {}}"#,
    );
}

#[test]
fn date_element() {
    assert_json(
        &DateElement::new(1, "s"),
        r#"{"type": "date", "timestamp": 1, "format": "s"}"#,
    );
    assert_json(
        &DateElement::new(1, "s")
            .timezone("s")
            .url("s")
            .fallback("s")
            .style(RichTextStyle::new()),
        r#"{"type": "date", "timestamp": 1, "format": "s", "timezone": "s", "url": "s", "fallback": "s", "style": {}}"#,
    );
}

#[test]
fn emoji_element() {
    assert_json(&EmojiElement::new("s"), r#"{"type": "emoji", "name": "s"}"#);
    assert_json(
        &EmojiElement::new("s").unicode("s"),
        r#"{"type": "emoji", "name": "s", "unicode": "s"}"#,
    );
}

#[test]
fn file_element() {
    assert_json(
        &FileElement::new("s"),
        r#"{"type": "file", "file_id": "s"}"#,
    );
    assert_json(
        &FileElement::new("s")
            .style(RichTextStyle::new())
            .text("s")
            .url("s")
            .is_skill_invocation(true),
        r#"{"type": "file", "file_id": "s", "style": {}, "text": "s", "url": "s", "is_skill_invocation": true}"#,
    );
}

#[test]
fn link_element() {
    assert_json(&LinkElement::new("s"), r#"{"type": "link", "url": "s"}"#);
    assert_json(
        &LinkElement::new("s")
            .text("s")
            .r#unsafe(true)
            .from_llm(true)
            .is_slack_url(true)
            .truncated(true)
            .style(RichTextStyle::new()),
        r#"{"type": "link", "url": "s", "text": "s", "unsafe": true, "from_llm": true, "is_slack_url": true, "truncated": true, "style": {}}"#,
    );
}

#[test]
fn list_record_element() {
    assert_json(
        &ListRecordElement::new("s"),
        r#"{"type": "list_record", "file_id": "s"}"#,
    );
    assert_json(
        &ListRecordElement::new("s")
            .record_id("s")
            .view_id("s")
            .style(RichTextStyle::new())
            .text("s")
            .url("s"),
        r#"{"type": "list_record", "file_id": "s", "record_id": "s", "view_id": "s", "style": {}, "text": "s", "url": "s"}"#,
    );
}

#[test]
fn message_mention_element() {
    assert_json(
        &MessageMentionElement::new("s", "s"),
        r#"{"type": "message_mention", "channel_id": "s", "message_ts": "s"}"#,
    );
    assert_json(
        &MessageMentionElement::new("s", "s")
            .author_id("s")
            .thread_ts("s")
            .style(RichTextStyle::new())
            .text("s")
            .url("s"),
        r#"{"type": "message_mention", "channel_id": "s", "message_ts": "s", "author_id": "s", "thread_ts": "s", "style": {}, "text": "s", "url": "s"}"#,
    );
}

#[test]
fn salesforce_data_field_element() {
    assert_json(
        &SalesforceDataFieldElement::new("s"),
        r#"{"type": "salesforce_data_field", "salesforce_record_id": "s"}"#,
    );
    assert_json(
        &SalesforceDataFieldElement::new("s")
            .salesforce_field_label("s")
            .salesforce_field_api_name("s")
            .salesforce_include_field_label(true)
            .style(RichTextStyle::new()),
        r#"{"type": "salesforce_data_field", "salesforce_record_id": "s", "salesforce_field_label": "s", "salesforce_field_api_name": "s", "salesforce_include_field_label": true, "style": {}}"#,
    );
}

#[test]
fn tag_element() {
    assert_json(&TagElement::new("s"), r#"{"type": "tag", "text": "s"}"#);
    assert_json(
        &TagElement::new("s")
            .color(TagColor::Gray)
            .style(RichTextStyle::new()),
        r#"{"type": "tag", "text": "s", "color": "gray", "style": {}}"#,
    );
}

#[test]
fn team_element() {
    assert_json(
        &TeamElement::new("s"),
        r#"{"type": "team", "team_id": "s"}"#,
    );
    assert_json(
        &TeamElement::new("s").style(RichTextStyle::new()),
        r#"{"type": "team", "team_id": "s", "style": {}}"#,
    );
}

#[test]
fn text_element() {
    assert_json(&TextElement::new("s"), r#"{"type": "text", "text": "s"}"#);
    assert_json(
        &TextElement::new("s").style(RichTextStyle::new()),
        r#"{"type": "text", "text": "s", "style": {}}"#,
    );
}

#[test]
fn user_element() {
    assert_json(
        &UserElement::new("s"),
        r#"{"type": "user", "user_id": "s"}"#,
    );
    assert_json(
        &UserElement::new("s")
            .style(RichTextStyle::new())
            .from_llm(true),
        r#"{"type": "user", "user_id": "s", "style": {}, "from_llm": true}"#,
    );
}

#[test]
fn usergroup_element() {
    assert_json(
        &UsergroupElement::new("s"),
        r#"{"type": "usergroup", "usergroup_id": "s"}"#,
    );
    assert_json(
        &UsergroupElement::new("s").style(RichTextStyle::new()),
        r#"{"type": "usergroup", "usergroup_id": "s", "style": {}}"#,
    );
}

#[test]
fn work_object_mention_element() {
    assert_json(
        &WorkObjectMentionElement::new("s", "s", "s", "s"),
        r#"{"type": "work_object_mention", "entity_id": "s", "app_id": "s", "text": "s", "url": "s"}"#,
    );
    assert_json(
        &WorkObjectMentionElement::new("s", "s", "s", "s")
            .icon_url("s")
            .full_size_preview_enabled(true)
            .product_name("s")
            .style(RichTextStyle::new()),
        r#"{"type": "work_object_mention", "entity_id": "s", "app_id": "s", "text": "s", "url": "s", "icon_url": "s", "full_size_preview_enabled": true, "product_name": "s", "style": {}}"#,
    );
}

#[test]
fn workflow_mention_element() {
    assert_json(
        &WorkflowMentionElement::new("s", "s", "s"),
        r#"{"type": "workflow_mention", "workflow_id": "s", "function_trigger_id": "s", "text": "s"}"#,
    );
    assert_json(
        &WorkflowMentionElement::new("s", "s", "s")
            .url("s")
            .channel_id("s")
            .ts("s")
            .style(RichTextStyle::new()),
        r#"{"type": "workflow_mention", "workflow_id": "s", "function_trigger_id": "s", "text": "s", "url": "s", "channel_id": "s", "ts": "s", "style": {}}"#,
    );
}

#[test]
fn actions_block() {
    assert_json(
        &ActionsBlock::new(vec![ActionsElement::from(ButtonElement::new(
            TextObject::from(PlainText::new("s")),
        ))]),
        r#"{"type": "actions", "elements": [{"type": "button", "text": {"type": "plain_text", "text": "s"}}]}"#,
    );
    assert_json(
        &ActionsBlock::new(vec![ActionsElement::from(ButtonElement::new(
            TextObject::from(PlainText::new("s")),
        ))])
        .block_id("s"),
        r#"{"type": "actions", "elements": [{"type": "button", "text": {"type": "plain_text", "text": "s"}}], "block_id": "s"}"#,
    );
}

#[test]
fn alert_block() {
    assert_json(
        &AlertBlock::new(TextObject::from(PlainText::new("s"))),
        r#"{"type": "alert", "text": {"type": "plain_text", "text": "s"}}"#,
    );
    assert_json(
        &AlertBlock::new(TextObject::from(PlainText::new("s")))
            .level(AlertLevel::Default)
            .block_id("s"),
        r#"{"type": "alert", "text": {"type": "plain_text", "text": "s"}, "level": "default", "block_id": "s"}"#,
    );
}

#[test]
fn card_block() {
    assert_json(&CardBlock::new(), r#"{"type": "card"}"#);
    assert_json(
        &CardBlock::new()
            .block_id("s")
            .hero_image(ImageElement::new("s", "s"))
            .icon(ImageElement::new("s", "s"))
            .title(TextObject::from(PlainText::new("s")))
            .subtitle(TextObject::from(PlainText::new("s")))
            .body(TextObject::from(PlainText::new("s")))
            .actions(vec![ButtonElement::new(TextObject::from(PlainText::new(
                "s",
            )))])
            .slack_icon(SlackIcon::new("s"))
            .subtext(TextObject::from(PlainText::new("s"))),
        r#"{"type": "card", "block_id": "s", "hero_image": {"type": "image", "alt_text": "s", "image_url": "s"}, "icon": {"type": "image", "alt_text": "s", "image_url": "s"}, "title": {"type": "plain_text", "text": "s"}, "subtitle": {"type": "plain_text", "text": "s"}, "body": {"type": "plain_text", "text": "s"}, "actions": [{"type": "button", "text": {"type": "plain_text", "text": "s"}}], "slack_icon": {"type": "icon", "name": "s"}, "subtext": {"type": "plain_text", "text": "s"}}"#,
    );
}

#[test]
fn carousel_block() {
    assert_json(
        &CarouselBlock::new(vec![CardBlock::new()]),
        r#"{"type": "carousel", "elements": [{"type": "card"}]}"#,
    );
    assert_json(
        &CarouselBlock::new(vec![CardBlock::new()]).block_id("s"),
        r#"{"type": "carousel", "elements": [{"type": "card"}], "block_id": "s"}"#,
    );
}

#[test]
fn container_block() {
    assert_json(
        &ContainerBlock::new(
            TextObject::from(PlainText::new("s")),
            vec![Block::from(ActionsBlock::new(vec![ActionsElement::from(
                ButtonElement::new(TextObject::from(PlainText::new("s"))),
            )]))],
        ),
        r#"{"type": "container", "title": {"type": "plain_text", "text": "s"}, "child_blocks": [{"type": "actions", "elements": [{"type": "button", "text": {"type": "plain_text", "text": "s"}}]}]}"#,
    );
    assert_json(
        &ContainerBlock::new(
            TextObject::from(PlainText::new("s")),
            vec![Block::from(ActionsBlock::new(vec![ActionsElement::from(
                ButtonElement::new(TextObject::from(PlainText::new("s"))),
            )]))],
        )
        .rich_text_title(RichTextBlock::new(vec![RichTextObject::from(
            RichTextSection::new(vec![RichTextElement::from(AttachmentMentionElement::new(
                "s",
            ))]),
        )]))
        .subtitle(TextObject::from(PlainText::new("s")))
        .block_id("s")
        .width(ContainerWidth::Narrow)
        .icon(ImageElement::new("s", "s"))
        .is_collapsible(true)
        .default_collapsed(true)
        .has_header_divider(true),
        r#"{"type": "container", "title": {"type": "plain_text", "text": "s"}, "child_blocks": [{"type": "actions", "elements": [{"type": "button", "text": {"type": "plain_text", "text": "s"}}]}], "rich_text_title": {"type": "rich_text", "elements": [{"type": "rich_text_section", "elements": [{"type": "attachment_mention", "url": "s"}]}]}, "subtitle": {"type": "plain_text", "text": "s"}, "block_id": "s", "width": "narrow", "icon": {"type": "image", "alt_text": "s", "image_url": "s"}, "is_collapsible": true, "default_collapsed": true, "has_header_divider": true}"#,
    );
}

#[test]
fn context_block() {
    assert_json(
        &ContextBlock::new(vec![ContextElement::from(ImageElement::new("s", "s"))]),
        r#"{"type": "context", "elements": [{"type": "image", "alt_text": "s", "image_url": "s"}]}"#,
    );
    assert_json(
        &ContextBlock::new(vec![ContextElement::from(ImageElement::new("s", "s"))]).block_id("s"),
        r#"{"type": "context", "elements": [{"type": "image", "alt_text": "s", "image_url": "s"}], "block_id": "s"}"#,
    );
}

#[test]
fn context_actions_block() {
    assert_json(
        &ContextActionsBlock::new(vec![ContextActionsElement::from(
            FeedbackButtonsElement::new(
                FeedbackButton::new(TextObject::from(PlainText::new("s")), "s"),
                FeedbackButton::new(TextObject::from(PlainText::new("s")), "s"),
            ),
        )]),
        r#"{"type": "context_actions", "elements": [{"type": "feedback_buttons", "positive_button": {"text": {"type": "plain_text", "text": "s"}, "value": "s"}, "negative_button": {"text": {"type": "plain_text", "text": "s"}, "value": "s"}}]}"#,
    );
    assert_json(
        &ContextActionsBlock::new(vec![ContextActionsElement::from(
            FeedbackButtonsElement::new(
                FeedbackButton::new(TextObject::from(PlainText::new("s")), "s"),
                FeedbackButton::new(TextObject::from(PlainText::new("s")), "s"),
            ),
        )])
        .block_id("s"),
        r#"{"type": "context_actions", "elements": [{"type": "feedback_buttons", "positive_button": {"text": {"type": "plain_text", "text": "s"}, "value": "s"}, "negative_button": {"text": {"type": "plain_text", "text": "s"}, "value": "s"}}], "block_id": "s"}"#,
    );
}

#[test]
fn divider_block() {
    assert_json(&DividerBlock::new(), r#"{"type": "divider"}"#);
    assert_json(
        &DividerBlock::new().block_id("s"),
        r#"{"type": "divider", "block_id": "s"}"#,
    );
}

#[test]
fn file_block() {
    assert_json(
        &FileBlock::new("s", "s"),
        r#"{"type": "file", "external_id": "s", "source": "s"}"#,
    );
    assert_json(
        &FileBlock::new("s", "s").block_id("s"),
        r#"{"type": "file", "external_id": "s", "source": "s", "block_id": "s"}"#,
    );
}

#[test]
fn header_block() {
    assert_json(
        &HeaderBlock::new(TextObject::from(PlainText::new("s"))),
        r#"{"type": "header", "text": {"type": "plain_text", "text": "s"}}"#,
    );
    assert_json(
        &HeaderBlock::new(TextObject::from(PlainText::new("s")))
            .block_id("s")
            .level(1),
        r#"{"type": "header", "text": {"type": "plain_text", "text": "s"}, "block_id": "s", "level": 1}"#,
    );
}

#[test]
fn image_block() {
    assert_json(
        &ImageBlock::new("s", "s"),
        r#"{"type": "image", "alt_text": "s", "image_url": "s"}"#,
    );
    assert_json(
        &ImageBlock::new("s", "s")
            .image_url("s")
            .slack_file(SlackFile::from_id("s"))
            .title(TextObject::from(PlainText::new("s")))
            .block_id("s"),
        r#"{"type": "image", "alt_text": "s", "image_url": "s", "slack_file": {"id": "s"}, "title": {"type": "plain_text", "text": "s"}, "block_id": "s"}"#,
    );
}

#[test]
fn input_block() {
    assert_json(
        &InputBlock::new(
            TextObject::from(PlainText::new("s")),
            InputElement::from(CheckboxesElement::new(vec![OptionObject::new(
                TextObject::from(PlainText::new("s")),
                "s",
            )])),
        ),
        r#"{"type": "input", "label": {"type": "plain_text", "text": "s"}, "element": {"type": "checkboxes", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}}"#,
    );
    assert_json(
        &InputBlock::new(
            TextObject::from(PlainText::new("s")),
            InputElement::from(CheckboxesElement::new(vec![OptionObject::new(
                TextObject::from(PlainText::new("s")),
                "s",
            )])),
        )
        .dispatch_action(true)
        .block_id("s")
        .hint(TextObject::from(PlainText::new("s")))
        .optional(true),
        r#"{"type": "input", "label": {"type": "plain_text", "text": "s"}, "element": {"type": "checkboxes", "options": [{"text": {"type": "plain_text", "text": "s"}, "value": "s"}]}, "dispatch_action": true, "block_id": "s", "hint": {"type": "plain_text", "text": "s"}, "optional": true}"#,
    );
}

#[test]
fn markdown_block() {
    assert_json(
        &MarkdownBlock::new("s"),
        r#"{"type": "markdown", "text": "s"}"#,
    );
    assert_json(
        &MarkdownBlock::new("s").block_id("s"),
        r#"{"type": "markdown", "text": "s", "block_id": "s"}"#,
    );
}

#[test]
fn plan_block() {
    assert_json(
        &PlanBlock::new("s", vec![PlanTask::new("s", "s", TaskStatus::Pending)]),
        r#"{"type": "plan", "title": "s", "tasks": [{"task_id": "s", "title": "s", "status": "pending"}]}"#,
    );
    assert_json(
        &PlanBlock::new("s", vec![PlanTask::new("s", "s", TaskStatus::Pending)]).block_id("s"),
        r#"{"type": "plan", "title": "s", "tasks": [{"task_id": "s", "title": "s", "status": "pending"}], "block_id": "s"}"#,
    );
}

#[test]
fn plan_task() {
    assert_json(
        &PlanTask::new("s", "s", TaskStatus::Pending),
        r#"{"task_id": "s", "title": "s", "status": "pending"}"#,
    );
    assert_json(
        &PlanTask::new("s", "s", TaskStatus::Pending)
            .details(RichTextBlock::new(vec![RichTextObject::from(
                RichTextSection::new(vec![RichTextElement::from(AttachmentMentionElement::new(
                    "s",
                ))]),
            )]))
            .output(RichTextBlock::new(vec![RichTextObject::from(
                RichTextSection::new(vec![RichTextElement::from(AttachmentMentionElement::new(
                    "s",
                ))]),
            )]))
            .sources(vec![UrlSourceElement::new("s", "s")]),
        r#"{"task_id": "s", "title": "s", "status": "pending", "details": {"type": "rich_text", "elements": [{"type": "rich_text_section", "elements": [{"type": "attachment_mention", "url": "s"}]}]}, "output": {"type": "rich_text", "elements": [{"type": "rich_text_section", "elements": [{"type": "attachment_mention", "url": "s"}]}]}, "sources": [{"type": "url", "url": "s", "text": "s"}]}"#,
    );
}

#[test]
fn section_block() {
    assert_json(&SectionBlock::new(), r#"{"type": "section"}"#);
    assert_json(
        &SectionBlock::new()
            .text(TextObject::from(PlainText::new("s")))
            .block_id("s")
            .fields(vec![TextObject::from(PlainText::new("s"))])
            .accessory(SectionAccessory::from(ButtonElement::new(
                TextObject::from(PlainText::new("s")),
            )))
            .expand(true),
        r#"{"type": "section", "text": {"type": "plain_text", "text": "s"}, "block_id": "s", "fields": [{"type": "plain_text", "text": "s"}], "accessory": {"type": "button", "text": {"type": "plain_text", "text": "s"}}, "expand": true}"#,
    );
}

#[test]
fn task_card_block() {
    assert_json(
        &TaskCardBlock::new("s", "s", TaskStatus::Pending),
        r#"{"type": "task_card", "task_id": "s", "title": "s", "status": "pending"}"#,
    );
    assert_json(
        &TaskCardBlock::new("s", "s", TaskStatus::Pending)
            .details(RichTextBlock::new(vec![RichTextObject::from(
                RichTextSection::new(vec![RichTextElement::from(AttachmentMentionElement::new(
                    "s",
                ))]),
            )]))
            .output(RichTextBlock::new(vec![RichTextObject::from(
                RichTextSection::new(vec![RichTextElement::from(AttachmentMentionElement::new(
                    "s",
                ))]),
            )]))
            .sources(vec![UrlSourceElement::new("s", "s")])
            .block_id("s")
            .icon(SlackIcon::new("s"))
            .hide_title(true),
        r#"{"type": "task_card", "task_id": "s", "title": "s", "status": "pending", "details": {"type": "rich_text", "elements": [{"type": "rich_text_section", "elements": [{"type": "attachment_mention", "url": "s"}]}]}, "output": {"type": "rich_text", "elements": [{"type": "rich_text_section", "elements": [{"type": "attachment_mention", "url": "s"}]}]}, "sources": [{"type": "url", "url": "s", "text": "s"}], "block_id": "s", "icon": {"type": "icon", "name": "s"}, "hide_title": true}"#,
    );
}

#[test]
fn video_block() {
    assert_json(
        &VideoBlock::new("s", TextObject::from(PlainText::new("s")), "s", "s"),
        r#"{"type": "video", "alt_text": "s", "title": {"type": "plain_text", "text": "s"}, "thumbnail_url": "s", "video_url": "s"}"#,
    );
    assert_json(
        &VideoBlock::new("s", TextObject::from(PlainText::new("s")), "s", "s")
            .author_name("s")
            .block_id("s")
            .description(TextObject::from(PlainText::new("s")))
            .provider_icon_url("s")
            .provider_name("s")
            .title_url("s"),
        r#"{"type": "video", "alt_text": "s", "title": {"type": "plain_text", "text": "s"}, "thumbnail_url": "s", "video_url": "s", "author_name": "s", "block_id": "s", "description": {"type": "plain_text", "text": "s"}, "provider_icon_url": "s", "provider_name": "s", "title_url": "s"}"#,
    );
}

#[test]
fn table_block() {
    assert_json(
        &TableBlock::new(vec![vec![TableCell::from(RawTextCell::new("s"))]]),
        r#"{"type": "table", "rows": [[{"type": "raw_text", "text": "s"}]]}"#,
    );
    assert_json(
        &TableBlock::new(vec![vec![TableCell::from(RawTextCell::new("s"))]])
            .block_id("s")
            .column_settings(vec![Some(ColumnSetting::new())]),
        r#"{"type": "table", "rows": [[{"type": "raw_text", "text": "s"}]], "block_id": "s", "column_settings": [{}]}"#,
    );
}

#[test]
fn column_setting() {
    assert_json(&ColumnSetting::new(), r#"{}"#);
    assert_json(
        &ColumnSetting::new()
            .align(ColumnAlign::Left)
            .is_wrapped(true),
        r#"{"align": "left", "is_wrapped": true}"#,
    );
}

#[test]
fn data_table_block() {
    assert_json(
        &DataTableBlock::new(vec![vec![DataTableCell::from(RawTextCell::new("s"))]], "s"),
        r#"{"type": "data_table", "rows": [[{"type": "raw_text", "text": "s"}]], "caption": "s"}"#,
    );
    assert_json(
        &DataTableBlock::new(vec![vec![DataTableCell::from(RawTextCell::new("s"))]], "s")
            .block_id("s")
            .page_size(1)
            .row_header_column_index(1),
        r#"{"type": "data_table", "rows": [[{"type": "raw_text", "text": "s"}]], "caption": "s", "block_id": "s", "page_size": 1, "row_header_column_index": 1}"#,
    );
}

#[test]
fn raw_text_cell() {
    assert_json(
        &RawTextCell::new("s"),
        r#"{"type": "raw_text", "text": "s"}"#,
    );
}

#[test]
fn raw_number_cell() {
    assert_json(
        &RawNumberCell::new(1.5),
        r#"{"type": "raw_number", "value": 1.5}"#,
    );
    assert_json(
        &RawNumberCell::new(1.5).text("s"),
        r#"{"type": "raw_number", "value": 1.5, "text": "s"}"#,
    );
}

#[test]
fn action_cell() {
    assert_json(
        &ActionCell::new(ButtonElement::new(TextObject::from(PlainText::new("s")))),
        r#"{"type": "action_cell", "element": {"type": "button", "text": {"type": "plain_text", "text": "s"}}}"#,
    );
    assert_json(
        &ActionCell::new(ButtonElement::new(TextObject::from(PlainText::new("s"))))
            .fallback(ActionCellFallback::from(RawTextCell::new("s"))),
        r#"{"type": "action_cell", "element": {"type": "button", "text": {"type": "plain_text", "text": "s"}}, "fallback": {"type": "raw_text", "text": "s"}}"#,
    );
}

#[test]
fn data_visualization_block() {
    assert_json(
        &DataVisualizationBlock::new(
            "s",
            Chart::from(PieChart::new(vec![Segment::new("s", 1.5)])),
        ),
        r#"{"type": "data_visualization", "title": "s", "chart": {"type": "pie", "segments": [{"label": "s", "value": 1.5}]}}"#,
    );
    assert_json(
        &DataVisualizationBlock::new(
            "s",
            Chart::from(PieChart::new(vec![Segment::new("s", 1.5)])),
        )
        .block_id("s"),
        r#"{"type": "data_visualization", "title": "s", "chart": {"type": "pie", "segments": [{"label": "s", "value": 1.5}]}, "block_id": "s"}"#,
    );
}

#[test]
fn pie_chart() {
    assert_json(
        &PieChart::new(vec![Segment::new("s", 1.5)]),
        r#"{"type": "pie", "segments": [{"label": "s", "value": 1.5}]}"#,
    );
}

#[test]
fn bar_chart() {
    assert_json(
        &BarChart::new(
            vec![DataSeries::new("s", vec![DataPoint::new("s", 1.5)])],
            AxisConfig::new(vec!["s".to_string()]),
        ),
        r#"{"type": "bar", "series": [{"name": "s", "data": [{"label": "s", "value": 1.5}]}], "axis_config": {"categories": ["s"]}}"#,
    );
}

#[test]
fn area_chart() {
    assert_json(
        &AreaChart::new(
            vec![DataSeries::new("s", vec![DataPoint::new("s", 1.5)])],
            AxisConfig::new(vec!["s".to_string()]),
        ),
        r#"{"type": "area", "series": [{"name": "s", "data": [{"label": "s", "value": 1.5}]}], "axis_config": {"categories": ["s"]}}"#,
    );
}

#[test]
fn line_chart() {
    assert_json(
        &LineChart::new(
            vec![DataSeries::new("s", vec![DataPoint::new("s", 1.5)])],
            AxisConfig::new(vec!["s".to_string()]),
        ),
        r#"{"type": "line", "series": [{"name": "s", "data": [{"label": "s", "value": 1.5}]}], "axis_config": {"categories": ["s"]}}"#,
    );
}

#[test]
fn segment() {
    assert_json(&Segment::new("s", 1.5), r#"{"label": "s", "value": 1.5}"#);
}

#[test]
fn data_series() {
    assert_json(
        &DataSeries::new("s", vec![DataPoint::new("s", 1.5)]),
        r#"{"name": "s", "data": [{"label": "s", "value": 1.5}]}"#,
    );
}

#[test]
fn data_point() {
    assert_json(&DataPoint::new("s", 1.5), r#"{"label": "s", "value": 1.5}"#);
}

#[test]
fn axis_config() {
    assert_json(
        &AxisConfig::new(vec!["s".to_string()]),
        r#"{"categories": ["s"]}"#,
    );
    assert_json(
        &AxisConfig::new(vec!["s".to_string()])
            .x_label("s")
            .y_label("s"),
        r#"{"categories": ["s"], "x_label": "s", "y_label": "s"}"#,
    );
}

#[test]
fn attachment() {
    assert_json(&Attachment::new(), r#"{}"#);
    assert_json(
        &Attachment::new()
            .id(1)
            .fallback("s")
            .color("s")
            .pretext("s")
            .author_name("s")
            .author_link("s")
            .author_icon("s")
            .title("s")
            .title_link("s")
            .text("s")
            .fields(vec![AttachmentField::new("s", "s")])
            .image_url("s")
            .thumb_url("s")
            .thumb_width(1)
            .thumb_height(1)
            .service_name("s")
            .footer("s")
            .footer_icon("s")
            .ts(AttachmentTs::Number(1.into()))
            .mrkdwn_in(vec!["s".to_string()])
            .blocks(vec![Block::from(ActionsBlock::new(vec![
                ActionsElement::from(ButtonElement::new(TextObject::from(PlainText::new("s")))),
            ]))]),
        r#"{"id": 1, "fallback": "s", "color": "s", "pretext": "s", "author_name": "s", "author_link": "s", "author_icon": "s", "title": "s", "title_link": "s", "text": "s", "fields": [{"title": "s", "value": "s"}], "image_url": "s", "thumb_url": "s", "thumb_width": 1, "thumb_height": 1, "service_name": "s", "footer": "s", "footer_icon": "s", "ts": 1, "mrkdwn_in": ["s"], "blocks": [{"type": "actions", "elements": [{"type": "button", "text": {"type": "plain_text", "text": "s"}}]}]}"#,
    );
}

#[test]
fn attachment_field() {
    assert_json(
        &AttachmentField::new("s", "s"),
        r#"{"title": "s", "value": "s"}"#,
    );
    assert_json(
        &AttachmentField::new("s", "s").short(true),
        r#"{"title": "s", "value": "s", "short": true}"#,
    );
}

#[test]
fn view() {
    assert_json(&View::home(Vec::new()), r#"{"type": "home", "blocks": []}"#);
    assert_json(
        &View::home(Vec::new())
            .title(TextObject::from(PlainText::new("s")))
            .close(TextObject::from(PlainText::new("s")))
            .submit(TextObject::from(PlainText::new("s")))
            .private_metadata("s")
            .callback_id("s")
            .clear_on_close(true)
            .notify_on_close(true)
            .external_id("s")
            .submit_disabled(true),
        r#"{"type": "home", "blocks": [], "title": {"type": "plain_text", "text": "s"}, "close": {"type": "plain_text", "text": "s"}, "submit": {"type": "plain_text", "text": "s"}, "private_metadata": "s", "callback_id": "s", "clear_on_close": true, "notify_on_close": true, "external_id": "s", "submit_disabled": true}"#,
    );
}
