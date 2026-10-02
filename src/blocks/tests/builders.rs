//! 組み立て用の API で作った値が、Slack に送る JSON になることを確かめる。

use serde_json::json;

use super::*;

#[test]
fn section_with_button() {
    let block = Block::from(
        SectionBlock::new()
            .text(TextObject::mrkdwn("hi"))
            .block_id("b1")
            .accessory(
                ButtonElement::new(TextObject::plain("押す"))
                    .action_id("a1")
                    .style(ButtonStyle::Primary),
            ),
    );
    assert_json(
        &block,
        r#"{
            "type": "section",
            "text": {"type": "mrkdwn", "text": "hi"},
            "block_id": "b1",
            "accessory": {
                "type": "button",
                "text": {"type": "plain_text", "text": "押す"},
                "action_id": "a1",
                "style": "primary"
            }
        }"#,
    );
}

#[test]
fn modal_view() {
    let view = View::modal(
        TextObject::plain("申請"),
        vec![
            InputBlock::new(
                TextObject::plain("理由"),
                PlainTextInputElement::new()
                    .action_id("reason")
                    .multiline(true),
            )
            .block_id("reason_block")
            .into(),
            ContextBlock::new(vec![
                ImageElement::new("https://example.com/a.png", "icon").into(),
                Mrkdwn::new("*注意*").verbatim(true).into(),
            ])
            .into(),
        ],
    )
    .submit(TextObject::plain("送信"))
    .callback_id("request");
    assert_json(
        &view,
        r#"{
            "type": "modal",
            "title": {"type": "plain_text", "text": "申請"},
            "blocks": [
                {
                    "type": "input",
                    "label": {"type": "plain_text", "text": "理由"},
                    "element": {"type": "plain_text_input", "action_id": "reason", "multiline": true},
                    "block_id": "reason_block"
                },
                {
                    "type": "context",
                    "elements": [
                        {"type": "image", "alt_text": "icon", "image_url": "https://example.com/a.png"},
                        {"type": "mrkdwn", "text": "*注意*", "verbatim": true}
                    ]
                }
            ],
            "submit": {"type": "plain_text", "text": "送信"},
            "callback_id": "request"
        }"#,
    );
}

#[test]
fn view_constructors() {
    assert_json(
        &View::workflow_step(vec![DividerBlock::new().into()]).submit_disabled(true),
        r#"{"type": "workflow_step", "blocks": [{"type": "divider"}], "submit_disabled": true}"#,
    );
    assert_json(&ViewType::Other("profile".into()), r#""profile""#);
}

#[test]
fn rich_text_message() {
    let block = RichTextBlock::new(vec![
        RichTextSection::new(vec![
            TextElement::new("こんにちは ")
                .style(RichTextStyle::new().bold(true))
                .into(),
            UserElement::new("U123").into(),
        ])
        .into(),
        RichTextList::new(
            RichTextListStyle::Ordered,
            vec![RichTextSection::new(vec![LinkElement::new(
                "https://slack.com",
            )
            .into()])],
        )
        .indent(1)
        .into(),
    ]);
    assert_json(
        &block,
        r#"{
            "type": "rich_text",
            "elements": [
                {
                    "type": "rich_text_section",
                    "elements": [
                        {"type": "text", "text": "こんにちは ", "style": {"bold": true}},
                        {"type": "user", "user_id": "U123"}
                    ]
                },
                {
                    "type": "rich_text_list",
                    "style": "ordered",
                    "elements": [
                        {"type": "rich_text_section", "elements": [{"type": "link", "url": "https://slack.com"}]}
                    ],
                    "indent": 1
                }
            ]
        }"#,
    );
}

#[test]
fn select_from_option_groups() {
    let groups = vec![OptionGroup::new(
        TextObject::plain("G"),
        vec![OptionObject::new(TextObject::plain("A"), "a")],
    )];
    let expected = r#"{
        "type": "%s",
        "option_groups": [
            {
                "label": {"type": "plain_text", "text": "G"},
                "options": [{"text": {"type": "plain_text", "text": "A"}, "value": "a"}]
            }
        ]
    }"#;
    assert_json(
        &StaticSelectElement::from_option_groups(groups.clone()),
        &expected.replace("%s", "static_select"),
    );
    assert_json(
        &MultiStaticSelectElement::from_option_groups(groups),
        &expected.replace("%s", "multi_static_select"),
    );
}

#[test]
fn slack_file_constructors() {
    assert_json(
        &SlackFile::from_url("https://files.slack.com/x.png"),
        r#"{"url": "https://files.slack.com/x.png"}"#,
    );
    assert_json(&SlackFile::from_id("F123"), r#"{"id": "F123"}"#);
    assert_json(
        &ImageBlock::from_slack_file(SlackFile::from_id("F123"), "猫"),
        r#"{"type": "image", "alt_text": "猫", "slack_file": {"id": "F123"}}"#,
    );
    assert_json(
        &ImageElement::from_slack_file(SlackFile::from_id("F123"), "猫"),
        r#"{"type": "image", "alt_text": "猫", "slack_file": {"id": "F123"}}"#,
    );
}

#[test]
fn table_with_skipped_column_setting() {
    let table = TableBlock::new(vec![vec![
        RawTextCell::new("a").into(),
        RawNumberCell::new(2.0).into(),
    ]])
    .column_settings(vec![
        None,
        Some(ColumnSetting::new().align(ColumnAlign::Right)),
    ]);
    assert_json(
        &table,
        r#"{
            "type": "table",
            "rows": [[{"type": "raw_text", "text": "a"}, {"type": "raw_number", "value": 2.0}]],
            "column_settings": [null, {"align": "right"}]
        }"#,
    );
}

#[test]
fn attachment_with_blocks() {
    let attachment = Attachment::new()
        .color("good")
        .fields(vec![AttachmentField::new("優先度", "高").short(true)])
        .ts(AttachmentTs::String("1503435956.000247".into()))
        .blocks(vec![DividerBlock::new().into()]);
    assert_json(
        &attachment,
        r#"{
            "color": "good",
            "fields": [{"title": "優先度", "value": "高", "short": true}],
            "ts": "1503435956.000247",
            "blocks": [{"type": "divider"}]
        }"#,
    );
}

#[test]
fn message_metadata() {
    let metadata = MessageMetadata::new(
        "task_created",
        json!({"id": "11223", "title": "Redesign Homepage"}),
    );
    assert_json(
        &metadata,
        r#"{"event_type": "task_created", "event_payload": {"id": "11223", "title": "Redesign Homepage"}}"#,
    );
}

#[test]
fn empty_vectors_are_omitted() {
    let button = ButtonElement::new(TextObject::plain("x")).visible_to_user_ids(Vec::new());
    assert_json(
        &button,
        r#"{"type": "button", "text": {"type": "plain_text", "text": "x"}}"#,
    );
}

#[test]
fn wrong_type_is_rejected() {
    let error = serde_json::from_str::<ButtonElement>(
        r#"{"type": "overflow", "text": {"type": "plain_text", "text": "x"}}"#,
    )
    .unwrap_err();
    assert!(
        error.to_string().contains(r#"expected "button""#),
        "{error}"
    );
    assert!(serde_json::from_str::<ButtonElement>(
        r#"{"type": 1, "text": {"type": "plain_text", "text": "x"}}"#
    )
    .is_err());
    assert!(serde_json::from_str::<ButtonElement>(
        r#"{"text": {"type": "plain_text", "text": "x"}}"#
    )
    .is_err());
    // 壊れた JSON は Unknown にせずエラーにする
    assert!(serde_json::from_str::<Block>(r#"{"type": "divider""#).is_err());
}

#[test]
fn debug_shows_type() {
    assert_eq!(
        format!("{:?}", DividerBlock::new()),
        "DividerBlock { type: divider, block_id: None }"
    );
}

#[test]
fn unknown_inside_known_block_is_kept() {
    // 既知のブロックの中に未知の要素があっても、その要素だけが Unknown になり全体は読める。
    let json = r#"{
        "type": "actions",
        "elements": [
            {"type": "button", "text": {"type": "plain_text", "text": "x"}},
            {"type": "future_element", "foo": [1, 2]}
        ]
    }"#;
    let block: Block = serde_json::from_str(json).unwrap();
    let Block::Actions(actions) = &block else {
        panic!("{block:?}")
    };
    assert!(matches!(actions.elements[0], ActionsElement::Button(_)));
    assert!(matches!(actions.elements[1], ActionsElement::Unknown(_)));
    assert_eq!(
        serde_json::to_value(&block).unwrap(),
        serde_json::from_str::<Value>(json).unwrap()
    );
}
