//! Web API リファレンスのレスポンス例に入っている view・attachments・blocks を読み書きする。

use super::*;

// m/chat.appendstream.md#7 $.blocks
#[test]
fn chat_appendstream_blocks() {
    roundtrip::<Vec<Block>>(
        r#"[
        {
            "type": "section",
            "text": {
                "type": "plain_text",
                "text": "Sandra's plan outline"
            }
        }
    ]"#,
    );
}

// m/chat.postmessage.md#5 $.message.attachments
#[test]
fn chat_postmessage_attachments() {
    roundtrip::<Vec<Attachment>>(
        r#"[
        {
            "text": "This is an attachment",
            "id": 1,
            "fallback": "This is an attachment's fallback"
        }
    ]"#,
    );
}

// m/conversations.history.md#6 $.messages[1].attachments
#[test]
fn conversations_history_attachments() {
    roundtrip::<Vec<Attachment>>(
        r#"[
        {
            "service_name": "Leg end nary a laugh, Ink.",
            "text": "This is likely a pun about the weather.",
            "fallback": "We're withholding a pun from you",
            "thumb_url": "https://badpuns.example.com/puns/123.png",
            "thumb_width": 1920,
            "thumb_height": 700,
            "id": 1
        }
    ]"#,
    );
}

// m/stars.list.md#5 $.items[0].message.attachments
#[test]
fn stars_list_attachments() {
    roundtrip::<Vec<Attachment>>(
        r#"[
        {
            "color": "ecb438",
            "ts": 1655762568,
            "id": 1,
            "fallback": "some text",
            "text": "some text",
            "pretext": "*chat.postMessage*",
            "mrkdwn_in": [
                "pretext",
                "text"
            ]
        }
    ]"#,
    );
}

// m/views.open.md#5 $.view
#[test]
fn views_open_view() {
    roundtrip::<View>(
        r#"{
        "id": "VMHU10V25",
        "team_id": "T8N4K1JN",
        "type": "modal",
        "title": {
            "type": "plain_text",
            "text": "Quite a plain modal"
        },
        "submit": {
            "type": "plain_text",
            "text": "Create"
        },
        "blocks": [
            {
                "type": "input",
                "block_id": "a_block_id",
                "label": {
                    "type": "plain_text",
                    "text": "A simple label",
                    "emoji": true
                },
                "optional": false,
                "element": {
                    "type": "plain_text_input",
                    "action_id": "an_action_id"
                }
            }
        ],
        "private_metadata": "Shh it is a secret",
        "callback_id": "identify_your_modals",
        "external_id": "",
        "state": {
            "values": {}
        },
        "hash": "156772938.1827394",
        "clear_on_close": false,
        "notify_on_close": false,
        "root_view_id": "VMHU10V25",
        "app_id": "AA4928AQ",
        "bot_id": "BA13894H"
    }"#,
    );
}

// m/views.open.md#5 $.view.blocks
#[test]
fn views_open_blocks() {
    roundtrip::<Vec<Block>>(
        r#"[
        {
            "type": "input",
            "block_id": "a_block_id",
            "label": {
                "type": "plain_text",
                "text": "A simple label",
                "emoji": true
            },
            "optional": false,
            "element": {
                "type": "plain_text_input",
                "action_id": "an_action_id"
            }
        }
    ]"#,
    );
}

// m/views.publish.md#5 $.view
#[test]
fn views_publish_view() {
    roundtrip::<View>(
        r#"{
        "id": "VMHU10V25",
        "team_id": "T8N4K1JN",
        "type": "home",
        "close": null,
        "submit": null,
        "blocks": [
            {
                "type": "section",
                "block_id": "2WGp9",
                "text": {
                    "type": "mrkdwn",
                    "text": "A simple section with some sample sentence.",
                    "verbatim": false
                }
            }
        ],
        "private_metadata": "Shh it is a secret",
        "callback_id": "identify_your_home_tab",
        "state": {
            "values": {}
        },
        "hash": "156772938.1827394",
        "clear_on_close": false,
        "notify_on_close": false,
        "root_view_id": "VMHU10V25",
        "previous_view_id": null,
        "app_id": "AA4928AQ",
        "external_id": "",
        "bot_id": "BA13894H"
    }"#,
    );
}

// m/views.publish.md#5 $.view.blocks
#[test]
fn views_publish_blocks() {
    roundtrip::<Vec<Block>>(
        r#"[
        {
            "type": "section",
            "block_id": "2WGp9",
            "text": {
                "type": "mrkdwn",
                "text": "A simple section with some sample sentence.",
                "verbatim": false
            }
        }
    ]"#,
    );
}

// m/views.push.md#5 $.view
#[test]
fn views_push_view() {
    roundtrip::<View>(
        r#"{
        "id": "VNM522E2U",
        "team_id": "T9M4RL1JM",
        "type": "modal",
        "title": {
            "type": "plain_text",
            "text": "Pushed Modal",
            "emoji": true
        },
        "close": {
            "type": "plain_text",
            "text": "Back",
            "emoji": true
        },
        "submit": {
            "type": "plain_text",
            "text": "Save",
            "emoji": true
        },
        "blocks": [
            {
                "type": "input",
                "block_id": "edit_details",
                "element": {
                    "type": "plain_text_input",
                    "action_id": "detail_input"
                },
                "label": {
                    "type": "plain_text",
                    "text": "Edit details"
                }
            }
        ],
        "private_metadata": "",
        "callback_id": "view_4",
        "external_id": "",
        "state": {
            "values": {}
        },
        "hash": "1569362015.55b5e41b",
        "clear_on_close": true,
        "notify_on_close": false,
        "root_view_id": "VNN729E3U",
        "previous_view_id": null,
        "app_id": "AAD3351BQ",
        "bot_id": "BADF7A34H"
    }"#,
    );
}

// m/views.push.md#5 $.view.blocks
#[test]
fn views_push_blocks() {
    roundtrip::<Vec<Block>>(
        r#"[
        {
            "type": "input",
            "block_id": "edit_details",
            "element": {
                "type": "plain_text_input",
                "action_id": "detail_input"
            },
            "label": {
                "type": "plain_text",
                "text": "Edit details"
            }
        }
    ]"#,
    );
}

// m/views.update.md#5 $.view
#[test]
fn views_update_view() {
    roundtrip::<View>(
        r#"{
        "id": "VNM522E2U",
        "team_id": "T9M4RL1JM",
        "type": "modal",
        "title": {
            "type": "plain_text",
            "text": "Updated Modal",
            "emoji": true
        },
        "close": {
            "type": "plain_text",
            "text": "Close",
            "emoji": true
        },
        "submit": null,
        "blocks": [
            {
                "type": "section",
                "block_id": "s_block",
                "text": {
                    "type": "plain_text",
                    "text": "I am but an updated modal",
                    "emoji": true
                },
                "accessory": {
                    "type": "button",
                    "action_id": "button_4",
                    "text": {
                        "type": "plain_text",
                        "text": "Click me"
                    }
                }
            }
        ],
        "private_metadata": "",
        "callback_id": "view_2",
        "external_id": "",
        "state": {
            "values": {}
        },
        "hash": "1569262015.55b5e41b",
        "clear_on_close": true,
        "notify_on_close": false,
        "root_view_id": "VNN729E3U",
        "previous_view_id": null,
        "app_id": "AAD3351BQ",
        "bot_id": "BADF7A34H"
    }"#,
    );
}

// m/views.update.md#5 $.view.blocks
#[test]
fn views_update_blocks() {
    roundtrip::<Vec<Block>>(
        r#"[
        {
            "type": "section",
            "block_id": "s_block",
            "text": {
                "type": "plain_text",
                "text": "I am but an updated modal",
                "emoji": true
            },
            "accessory": {
                "type": "button",
                "action_id": "button_4",
                "text": {
                    "type": "plain_text",
                    "text": "Click me"
                }
            }
        }
    ]"#,
    );
}

// obj/channel-object.md#0 $.channel.latest.attachments
#[test]
fn channel_object_attachments() {
    roundtrip::<Vec<Attachment>>(
        r#"[
        {
            "text": "Don't get too attached",
            "id": 1,
            "fallback": "This is an attachment fallback"
        }
    ]"#,
    );
}

// obj/message.md#30 $.blocks
#[test]
fn message_blocks() {
    roundtrip::<Vec<Block>>(
        r#"[
        {
            "type": "header",
            "text": {
                "type": "plain_text",
                "text": "New request"
            }
        },
        {
            "type": "section",
            "fields": [
                {
                    "type": "mrkdwn",
                    "text": "*Type:*\nPaid Time Off"
                },
                {
                    "type": "mrkdwn",
                    "text": "*Created by:*\n<example.com|Fred Enriquez>"
                }
            ]
        },
        {
            "type": "section",
            "fields": [
                {
                    "type": "mrkdwn",
                    "text": "*When:*\nAug 10 - Aug 13"
                }
            ]
        },
        {
            "type": "section",
            "text": {
                "type": "mrkdwn",
                "text": "<https://example.com|View request>"
            }
        }
    ]"#,
    );
}

// m/slacklists.items.list.md#9
#[test]
fn slacklists_items_list_rich_text() {
    roundtrip::<RichTextBlock>(
        r#"{
    "type": "rich_text",
    "block_id": "08jc0",
    "elements": [
        {
            "type": "rich_text_section",
            "elements": [
                {
                    "text": "Onboard new hire",
                    "type": "text"
                }
            ]
        }
    ]
}"#,
    );
}

// m/slacklists.items.create.md#27
#[test]
fn slacklists_items_create_rich_text() {
    roundtrip::<RichTextBlock>(
        r#"{
    "type": "rich_text",
    "block_id": "k0zIi",
    "elements": [
        {
            "type": "rich_text_section",
            "elements": [
                {
                    "text": "Fix bug",
                    "type": "text"
                }
            ]
        }
    ]
}"#,
    );
}

// m/calls.add.md: Block Kit リファレンスに無い call ブロック
#[test]
fn calls_add_unknown_block() {
    assert_unknown::<Block>(r#"{"type": "call", "call_id": "R0E69JAIF"}"#, |b| {
        matches!(b, Block::Unknown(_))
    });
}

// m/admin.workflows.search.md: リファレンスに無い workflowtoken 要素だけが Unknown になる
#[test]
fn admin_workflows_search_unknown_inline_element() {
    let json = r#"{
        "type": "rich_text",
        "elements": [{
            "type": "rich_text_section",
            "elements": [
                {"text": "Hello ", "type": "text"},
                {"id": "{{inputs.Ft014FQ980RZ__user_id}}", "type": "workflowtoken", "property": "", "data_type": "slack#/reference/objects/user-object_id"}
            ]
        }]
    }"#;
    let block: RichTextBlock = serde_json::from_str(json).unwrap();
    let RichTextObject::Section(section) = &block.elements[0] else {
        panic!("{block:?}")
    };
    assert!(matches!(section.elements[0], RichTextElement::Text(_)));
    assert!(matches!(section.elements[1], RichTextElement::Unknown(_)));
    assert_eq!(
        serde_json::to_value(&block).unwrap(),
        serde_json::from_str::<Value>(json).unwrap()
    );
}
