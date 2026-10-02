//! docs.slack.dev の Block Kit リファレンスに載っている JSON 例をそのまま読み書きする。
//! テスト名は「ページ名_ページ内で何番目のコード例か」。次の例は使っていない。
//! - JSON として壊れている（末尾カンマ）: checkboxes-element 1、canvas-message-unfurl-element 1、
//!   data-visualization-block 2（棒グラフ）
//! - 値の例ではなく JSON Schema の断片: data-table-block 1〜3（raw_text / raw_number / action_cell）、
//!   citation-element 1〜5（details の各種類）

use super::*;

#[test]
fn actions_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "actions",
                "block_id": "actions1",
                "elements": [
                    {
                        "type": "static_select",
                        "placeholder": {
                            "type": "plain_text",
                            "text": "Which witch is the witchiest witch?"
                        },
                        "action_id": "select_2",
                        "options": [
                            {
                                "text": {
                                    "type": "plain_text",
                                    "text": "Matilda"
                                },
                                "value": "matilda"
                            },
                            {
                                "text": {
                                    "type": "plain_text",
                                    "text": "Glinda"
                                },
                                "value": "glinda"
                            },
                            {
                                "text": {
                                    "type": "plain_text",
                                    "text": "Granny Weatherwax"
                                },
                                "value": "grannyWeatherwax"
                            },
                            {
                                "text": {
                                    "type": "plain_text",
                                    "text": "Hermione"
                                },
                                "value": "hermione"
                            }
                        ]
                    },
                    {
                        "type": "button",
                        "text": {
                            "type": "plain_text",
                            "text": "Cancel"
                        },
                        "value": "cancel",
                        "action_id": "button_1"
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn actions_block_2() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "actions",
                "block_id": "actionblock789",
                "elements": [
                    {
                        "type": "datepicker",
                        "action_id": "datepicker123",
                        "initial_date": "1990-04-28",
                        "placeholder": {
                            "type": "plain_text",
                            "text": "Select a date"
                        }
                    },
                    {
                        "type": "overflow",
                        "options": [
                            {
                                "text": {
                                    "type": "plain_text",
                                    "text": "*this is plain_text text*"
                                },
                                "value": "value-0"
                            },
                            {
                                "text": {
                                    "type": "plain_text",
                                    "text": "*this is plain_text text*"
                                },
                                "value": "value-1"
                            },
                            {
                                "text": {
                                    "type": "plain_text",
                                    "text": "*this is plain_text text*"
                                },
                                "value": "value-2"
                            },
                            {
                                "text": {
                                    "type": "plain_text",
                                    "text": "*this is plain_text text*"
                                },
                                "value": "value-3"
                            },
                            {
                                "text": {
                                    "type": "plain_text",
                                    "text": "*this is plain_text text*"
                                },
                                "value": "value-4"
                            }
                        ],
                        "action_id": "overflow"
                    },
                    {
                        "type": "button",
                        "text": {
                            "type": "plain_text",
                            "text": "Click Me"
                        },
                        "value": "click_me_123",
                        "action_id": "button"
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn alert_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "alert",
                "text": {
                    "type": "mrkdwn",
                    "text": "The work is mysterious and important.",
                    "verbatim": false
                },
                "level": "info"
            }
        ]
    }"#,
    );
}

#[test]
fn attachment_mention_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "attachment_mention",
                                "url": "https://example.com/attachment"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn broadcast_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "broadcast",
                                "range": "everyone"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn button_element_1() {
    roundtrip::<ButtonElement>(
        r#"{
        "type": "button",
        "text": {
            "type": "plain_text",
            "text": "Click Me"
        },
        "value": "click_me_123",
        "action_id": "button"
    }"#,
    );
}

#[test]
fn button_element_2() {
    roundtrip::<ButtonElement>(
        r#"{
        "type": "button",
        "text": {
            "type": "plain_text",
            "text": "Save"
        },
        "style": "primary",
        "value": "click_me_123",
        "action_id": "button"
    }"#,
    );
}

#[test]
fn button_element_3() {
    roundtrip::<ButtonElement>(
        r#"{
        "type": "button",
        "text": {
            "type": "plain_text",
            "text": "Link Button"
        },
        "url": "https://docs.slack.dev/block-kit"
    }"#,
    );
}

#[test]
fn button_element_4() {
    roundtrip::<ButtonElement>(
        r#"{
        "type": "button",
        "text": {
            "type": "plain_text",
            "text": "Summarize with Slackbot"
        },
        "action_id": "summarize_incident",
        "agent_prompt": "Summarize this incident and suggest next steps.",
        "agent_prompt_display": "Summarize this incident"
    }"#,
    );
}

#[test]
fn button_element_5() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": "This is a section block with a button."
                },
                "accessory": {
                    "type": "button",
                    "text": {
                        "type": "plain_text",
                        "text": "Click Me"
                    },
                    "value": "click_me_123",
                    "action_id": "button"
                }
            },
            {
                "type": "actions",
                "block_id": "actionblock789",
                "elements": [
                    {
                        "type": "button",
                        "text": {
                            "type": "plain_text",
                            "text": "Primary Button"
                        },
                        "style": "primary",
                        "value": "click_me_456"
                    },
                    {
                        "type": "button",
                        "text": {
                            "type": "plain_text",
                            "text": "Link Button"
                        },
                        "url": "https://api.slack.com/block-kit"
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn canvas_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "canvas",
                                "file_id": "F123ABC456"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn canvas_user_mention_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "canvas_user_mention",
                                "user_id": "U123ABC456"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn card_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "card",
                "icon": {
                    "type": "image",
                    "image_url": "https://picsum.photos/36/36",
                    "alt_text": "Icon"
                },
                "title": {
                    "type": "mrkdwn",
                    "text": "Lumon Industries",
                    "verbatim": false
                },
                "subtitle": {
                    "type": "mrkdwn",
                    "text": "Committed to work-life balance",
                    "verbatim": false
                },
                "hero_image": {
                    "type": "image",
                    "image_url": "https://picsum.photos/400/300",
                    "alt_text": "Sample hero image"
                },
                "body": {
                    "type": "mrkdwn",
                    "text": "Please enjoy each card equally.",
                    "verbatim": false
                },
                "actions": [
                    {
                        "type": "button",
                        "text": {
                            "type": "plain_text",
                            "text": "Action Button",
                            "emoji": false
                        },
                        "action_id": "button_action"
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn carousel_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "carousel",
                "elements": [
                    {
                        "type": "card",
                        "block_id": "carousel-card-1",
                        "icon": {
                            "type": "image",
                            "image_url": "https://picsum.photos/36/36",
                            "alt_text": "Icon"
                        },
                        "title": {
                            "type": "mrkdwn",
                            "text": "MDR",
                            "verbatim": false
                        },
                        "subtitle": {
                            "type": "mrkdwn",
                            "text": "Refining data files",
                            "verbatim": false
                        },
                        "hero_image": {
                            "type": "image",
                            "image_url": "https://picsum.photos/400/300",
                            "alt_text": "Sample hero image"
                        },
                        "body": {
                            "type": "mrkdwn",
                            "text": "Blue badge required to gain access.",
                            "verbatim": false
                        },
                        "actions": [
                            {
                                "type": "button",
                                "text": {
                                    "type": "plain_text",
                                    "text": "Action Button",
                                    "emoji": false
                                },
                                "action_id": "button_action_1"
                            }
                        ]
                    },
                    {
                        "type": "card",
                        "block_id": "carousel-card-2",
                        "icon": {
                            "type": "image",
                            "image_url": "https://picsum.photos/36/36",
                            "alt_text": "Icon"
                        },
                        "title": {
                            "type": "mrkdwn",
                            "text": "O&D",
                            "verbatim": false
                        },
                        "subtitle": {
                            "type": "mrkdwn",
                            "text": "Storage, maintenance, and rotation of art pieces",
                            "verbatim": false
                        },
                        "hero_image": {
                            "type": "image",
                            "image_url": "https://picsum.photos/400/300",
                            "alt_text": "Sample hero image"
                        },
                        "body": {
                            "type": "mrkdwn",
                            "text": "Green badge required to gain access.",
                            "verbatim": false
                        },
                        "actions": [
                            {
                                "type": "button",
                                "text": {
                                    "type": "plain_text",
                                    "text": "Action Button",
                                    "emoji": false
                                },
                                "action_id": "button_action_2"
                            }
                        ]
                    },
                    {
                        "type": "card",
                        "block_id": "carousel-card-3",
                        "icon": {
                            "type": "image",
                            "image_url": "https://picsum.photos/36/36",
                            "alt_text": "Icon"
                        },
                        "title": {
                            "type": "mrkdwn",
                            "text": "Wellness Center",
                            "verbatim": false
                        },
                        "subtitle": {
                            "type": "mrkdwn",
                            "text": "Wellness sessions",
                            "verbatim": false
                        },
                        "hero_image": {
                            "type": "image",
                            "image_url": "https://picsum.photos/400/300",
                            "alt_text": "Sample hero image"
                        },
                        "body": {
                            "type": "mrkdwn",
                            "text": "Please take a seat in the waiting room until called.",
                            "verbatim": false
                        },
                        "actions": [
                            {
                                "type": "button",
                                "text": {
                                    "type": "plain_text",
                                    "text": "Action Button",
                                    "emoji": false
                                },
                                "action_id": "button_action_3"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn channel_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "channel",
                                "channel_id": "C123ABC456"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn citation_element_6() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "citation",
                                "url": "https://example.com/source",
                                "text": "Source title",
                                "index": 1,
                                "details": {
                                    "citation_type": "file",
                                    "descriptor": "Team canvas",
                                    "file_id": "F0ABCDEFG12"
                                }
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn color_element_1() {
    roundtrip_blocks(
        r##"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "color",
                                "value": "#F405B3"
                            }
                        ]
                    }
                ]
            }
        ]
    }"##,
    );
}

#[test]
fn confirmation_dialog_object_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "actions",
                "elements": [
                    {
                        "type": "button",
                        "text": {
                            "type": "plain_text",
                            "emoji": true,
                            "text": "Approve"
                        },
                        "confirm": {
                            "title": {
                                "type": "plain_text",
                                "text": "Are you sure?"
                            },
                            "text": {
                                "type": "mrkdwn",
                                "text": "Would you not prefer a good game of _chess_?"
                            },
                            "confirm": {
                                "type": "plain_text",
                                "text": "Do it"
                            },
                            "deny": {
                                "type": "plain_text",
                                "text": "Stop, I changed my mind!"
                            }
                        },
                        "style": "primary",
                        "value": "click_me_123"
                    },
                    {
                        "type": "button",
                        "text": {
                            "type": "plain_text",
                            "emoji": true,
                            "text": "Deny"
                        },
                        "style": "danger",
                        "value": "click_me_123"
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn container_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "container",
                "block_id": "bkb_container_bulk_update",
                "title": {
                    "type": "plain_text",
                    "text": "Bulk update: 2 records selected"
                },
                "subtitle": {
                    "type": "plain_text",
                    "text": "Review changes before confirming"
                },
                "is_collapsible": true,
                "child_blocks": [
                    {
                        "type": "section",
                        "block_id": "record-row-1",
                        "text": {
                            "type": "mrkdwn",
                            "text": "*DCW-1024*\nStatus: Open → Closed\nAssignee: @princessdonut → @carl"
                        }
                    },
                    {
                        "type": "divider",
                        "block_id": "bulk-div-1"
                    },
                    {
                        "type": "section",
                        "block_id": "record-row-2",
                        "text": {
                            "type": "mrkdwn",
                            "text": "*DCW-1025*\nStatus: In Progress → Closed\nAssignee: @mordecai → @carl"
                        }
                    },
                    {
                        "type": "divider",
                        "block_id": "bulk-div-2"
                    },
                    {
                        "type": "context",
                        "block_id": "bulk-status-bar",
                        "elements": [
                            {
                                "type": "mrkdwn",
                                "text": ":white_check_mark: 2 records will be updated • Status → Closed • Assignee → @carl"
                            }
                        ]
                    },
                    {
                        "type": "actions",
                        "block_id": "bulk-actions",
                        "elements": [
                            {
                                "type": "button",
                                "text": {
                                    "type": "plain_text",
                                    "text": "Confirm All",
                                    "emoji": true
                                },
                                "style": "primary",
                                "action_id": "bulk_confirm"
                            },
                            {
                                "type": "button",
                                "text": {
                                    "type": "plain_text",
                                    "text": "Cancel",
                                    "emoji": true
                                },
                                "action_id": "bulk_cancel"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn context_actions_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "context_actions",
                "elements": [
                    {
                        "type": "feedback_buttons",
                        "action_id": "feedback_buttons_1",
                        "positive_button": {
                            "text": {
                                "type": "plain_text",
                                "text": "👍"
                            },
                            "value": "positive_feedback"
                        },
                        "negative_button": {
                            "text": {
                                "type": "plain_text",
                                "text": "👎"
                            },
                            "value": "negative_feedback"
                        }
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn context_actions_block_2() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "context_actions",
                "elements": [
                    {
                        "type": "icon_button",
                        "icon": "trash",
                        "text": {
                            "type": "plain_text",
                            "text": "Delete"
                        },
                        "action_id": "delete_button_1",
                        "value": "delete_item"
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn context_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "context",
                "elements": [
                    {
                        "type": "image",
                        "image_url": "https://image.freepik.com/free-photo/red-drawing-pin_1156-445.jpg",
                        "alt_text": "images"
                    },
                    {
                        "type": "mrkdwn",
                        "text": "Location: **Dogpatch**"
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn conversation_filter_object_1() {
    roundtrip::<View>(
        r#"{
        "title": {
            "type": "plain_text",
            "text": "My App",
            "emoji": true
        },
        "submit": {
            "type": "plain_text",
            "text": "Submit",
            "emoji": true
        },
        "type": "modal",
        "close": {
            "type": "plain_text",
            "text": "Cancel",
            "emoji": true
        },
        "blocks": [
            {
                "type": "input",
                "element": {
                    "type": "conversations_select",
                    "placeholder": {
                        "type": "plain_text",
                        "text": "Select a conversation",
                        "emoji": true
                    },
                    "filter": {
                        "include": [
                            "public",
                            "mpim"
                        ],
                        "exclude_bot_users": true
                    }
                },
                "label": {
                    "type": "plain_text",
                    "text": "Choose the conversation to publish your result to:",
                    "emoji": true
                }
            }
        ]
    }"#,
    );
}

#[test]
fn data_table_block_4() {
    roundtrip::<ActionCell>(
        r#"{
        "type": "action_cell",
        "element": {
            "type": "button",
            "text": {
                "type": "plain_text",
                "text": "Mark done"
            },
            "action_id": "mark_done",
            "value": "task_123"
        },
        "fallback": {
            "type": "raw_text",
            "text": "Open"
        }
    }"#,
    );
}

#[test]
fn data_table_block_5() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "data_table",
                "caption": "A Fabulous Table",
                "rows": [
                    [
                        {
                            "type": "raw_text",
                            "text": "Name"
                        },
                        {
                            "type": "raw_text",
                            "text": "Department"
                        },
                        {
                            "type": "raw_text",
                            "text": "Badge"
                        }
                    ],
                    [
                        {
                            "type": "raw_text",
                            "text": "Data Refinement Department"
                        },
                        {
                            "type": "raw_text",
                            "text": "MDR"
                        },
                        {
                            "type": "rich_text",
                            "elements": [
                                {
                                    "type": "rich_text_section",
                                    "elements": [
                                        {
                                            "type": "text",
                                            "text": "Blue",
                                            "style": {
                                                "bold": true
                                            }
                                        }
                                    ]
                                }
                            ]
                        }
                    ],
                    [
                        {
                            "type": "raw_text",
                            "text": "Art Sourcing Department"
                        },
                        {
                            "type": "raw_text",
                            "text": "O&D"
                        },
                        {
                            "type": "rich_text",
                            "elements": [
                                {
                                    "type": "rich_text_section",
                                    "elements": [
                                        {
                                            "type": "text",
                                            "text": "Green"
                                        },
                                        {
                                            "type": "text",
                                            "text": "review",
                                            "style": {
                                                "italic": true
                                            }
                                        }
                                    ]
                                }
                            ]
                        }
                    ],
                    [
                        {
                            "type": "raw_text",
                            "text": "Wellness Department"
                        },
                        {
                            "type": "raw_text",
                            "text": "Wellness Center"
                        },
                        {
                            "type": "rich_text",
                            "elements": [
                                {
                                    "type": "rich_text_section",
                                    "elements": [
                                        {
                                            "type": "text",
                                            "text": "Limited",
                                            "style": {
                                                "bold": true
                                            }
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn data_visualization_block_1() {
    roundtrip::<DataVisualizationBlock>(
        r#"{
        "type": "data_visualization",
        "title": "My Favorite Candy Bars",
        "chart": {
            "type": "pie",
            "segments": [
                {
                    "label": "Kit Kat",
                    "value": 45
                },
                {
                    "label": "Twix",
                    "value": 28
                },
                {
                    "label": "Crunch",
                    "value": 18
                },
                {
                    "label": "Milky Way",
                    "value": 9
                }
            ]
        }
    }"#,
    );
}

#[test]
fn data_visualization_block_3() {
    roundtrip::<DataVisualizationBlock>(
        r#"{
        "type": "data_visualization",
        "title": "Daily Active Users",
        "chart": {
            "type": "area",
            "series": [
                {
                    "name": "Pied Piper Free Tier",
                    "data": [
                        {
                            "label": "Mon",
                            "value": 12000
                        },
                        {
                            "label": "Tues",
                            "value": 13500
                        },
                        {
                            "label": "Wed",
                            "value": 15200
                        },
                        {
                            "label": "Thurs",
                            "value": 14800
                        },
                        {
                            "label": "Fri",
                            "value": 16400
                        }
                    ]
                },
                {
                    "name": "Pied Piper Paid Tier",
                    "data": [
                        {
                            "label": "Mon",
                            "value": 4500
                        },
                        {
                            "label": "Tues",
                            "value": 4800
                        },
                        {
                            "label": "Wed",
                            "value": 5100
                        },
                        {
                            "label": "Thurs",
                            "value": 5600
                        },
                        {
                            "label": "Fri",
                            "value": 6200
                        }
                    ]
                }
            ],
            "axis_config": {
                "categories": [
                    "Mon",
                    "Tues",
                    "Wed",
                    "Thurs",
                    "Fri"
                ],
                "x_label": "Day",
                "y_label": "Users"
            }
        }
    }"#,
    );
}

#[test]
fn data_visualization_block_4() {
    roundtrip::<DataVisualizationBlock>(
        r#"{
        "type": "data_visualization",
        "title": "Weekly Paper Sales",
        "chart": {
            "type": "line",
            "series": [
                {
                    "name": "Dunder Mifflin Infinity Website",
                    "data": [
                        {
                            "label": "Week 1",
                            "value": 32000
                        },
                        {
                            "label": "Week 2",
                            "value": 35000
                        },
                        {
                            "label": "Week 3",
                            "value": 29000
                        },
                        {
                            "label": "Week 4",
                            "value": 41000
                        },
                        {
                            "label": "Week 5",
                            "value": 45000
                        }
                    ]
                },
                {
                    "name": "Dunder Mifflin In-store",
                    "data": [
                        {
                            "label": "Week 1",
                            "value": 32000
                        },
                        {
                            "label": "Week 2",
                            "value": 35000
                        },
                        {
                            "label": "Week 3",
                            "value": 29000
                        },
                        {
                            "label": "Week 4",
                            "value": 41000
                        },
                        {
                            "label": "Week 5",
                            "value": 45000
                        }
                    ]
                }
            ],
            "axis_config": {
                "categories": [
                    "Week 1",
                    "Week 2",
                    "Week 3",
                    "Week 4",
                    "Week 5"
                ],
                "x_label": "Week",
                "y_label": "Paper Sales (USD)"
            }
        }
    }"#,
    );
}

#[test]
fn date_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "date",
                                "timestamp": 1720710212,
                                "format": "{date_num} at {time}",
                                "fallback": "timey"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn date_picker_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "block_id": "section1234",
                "text": {
                    "type": "mrkdwn",
                    "text": "Pick a date for the deadline."
                },
                "accessory": {
                    "type": "datepicker",
                    "action_id": "datepicker123",
                    "initial_date": "1990-04-28",
                    "placeholder": {
                        "type": "plain_text",
                        "text": "Select a date"
                    }
                }
            }
        ]
    }"#,
    );
}

#[test]
fn datetime_picker_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "input",
                "element": {
                    "type": "datetimepicker",
                    "action_id": "datetimepicker-action"
                },
                "hint": {
                    "type": "plain_text",
                    "text": "This is some hint text",
                    "emoji": true
                },
                "label": {
                    "type": "plain_text",
                    "text": "Start date",
                    "emoji": true
                }
            }
        ]
    }"#,
    );
}

#[test]
fn dispatch_action_configuration_object_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "input",
                "dispatch_action": true,
                "element": {
                    "type": "plain_text_input",
                    "multiline": true,
                    "dispatch_action_config": {
                        "trigger_actions_on": [
                            "on_character_entered"
                        ]
                    }
                },
                "label": {
                    "type": "plain_text",
                    "text": "This is a multiline plain-text input",
                    "emoji": true
                }
            }
        ]
    }"#,
    );
}

#[test]
fn divider_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "divider"
            }
        ]
    }"#,
    );
}

#[test]
fn email_input_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "input",
                "block_id": "input123",
                "label": {
                    "type": "plain_text",
                    "text": "Email Address"
                },
                "element": {
                    "type": "email_text_input",
                    "action_id": "email_text_input-action",
                    "placeholder": {
                        "type": "plain_text",
                        "text": "Enter an email"
                    }
                }
            }
        ]
    }"#,
    );
}

#[test]
fn emoji_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "emoji",
                                "name": "basketball"
                            },
                            {
                                "type": "text",
                                "text": " "
                            },
                            {
                                "type": "emoji",
                                "name": "snowboarder"
                            },
                            {
                                "type": "text",
                                "text": " "
                            },
                            {
                                "type": "emoji",
                                "name": "checkered_flag"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn feedback_buttons_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "context_actions",
                "elements": [
                    {
                        "type": "feedback_buttons",
                        "action_id": "feedback_buttons_1",
                        "positive_button": {
                            "text": {
                                "type": "plain_text",
                                "text": "Good"
                            },
                            "value": "positive_feedback",
                            "accessibility_label": "Mark this response as good"
                        },
                        "negative_button": {
                            "text": {
                                "type": "plain_text",
                                "text": "Bad"
                            },
                            "value": "negative_feedback",
                            "accessibility_label": "Mark this response as bad"
                        }
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn file_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "file",
                "external_id": "ABCD1",
                "source": "remote"
            }
        ]
    }"#,
    );
}

#[test]
fn file_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "file",
                                "file_id": "F123ABC456"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn file_input_element_1() {
    roundtrip::<View>(
        r#"{
        "title": {
            "type": "plain_text",
            "text": "My App",
            "emoji": true
        },
        "submit": {
            "type": "plain_text",
            "text": "Submit",
            "emoji": true
        },
        "type": "modal",
        "close": {
            "type": "plain_text",
            "text": "Cancel",
            "emoji": true
        },
        "blocks": [
            {
                "type": "input",
                "block_id": "input_block_id",
                "label": {
                    "type": "plain_text",
                    "text": "Upload Files"
                },
                "element": {
                    "type": "file_input",
                    "action_id": "file_input_action_id_1",
                    "filetypes": [
                        "jpg",
                        "png"
                    ],
                    "max_files": 5
                }
            }
        ]
    }"#,
    );
}

#[test]
fn header_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "header",
                "text": {
                    "type": "plain_text",
                    "text": "A Heartfelt Header"
                }
            }
        ]
    }"#,
    );
}

#[test]
fn icon_button_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "context_actions",
                "elements": [
                    {
                        "type": "icon_button",
                        "icon": "trash",
                        "text": {
                            "type": "plain_text",
                            "text": "Delete"
                        },
                        "action_id": "delete_button",
                        "value": "delete_item"
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn image_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "image",
                "title": {
                    "type": "plain_text",
                    "text": "Please enjoy this photo of a kitten"
                },
                "block_id": "image4",
                "image_url": "http://placekitten.com/500/500",
                "alt_text": "An incredibly cute kitten."
            }
        ]
    }"#,
    );
}

#[test]
fn image_block_2() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "image",
                "title": {
                    "type": "plain_text",
                    "text": "Please enjoy this photo of a kitten"
                },
                "block_id": "image4",
                "slack_file": {
                    "url": "https://files.slack.com/files-pri/T0123456-F0123456/xyz.png"
                },
                "alt_text": "An incredibly cute kitten."
            }
        ]
    }"#,
    );
}

#[test]
fn image_block_3() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "image",
                "title": {
                    "type": "plain_text",
                    "text": "Please enjoy this photo of a kitten"
                },
                "block_id": "image4",
                "slack_file": {
                    "id": "F0123456"
                },
                "alt_text": "An incredibly cute kitten."
            }
        ]
    }"#,
    );
}

#[test]
fn image_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "block_id": "section567",
                "text": {
                    "type": "mrkdwn",
                    "text": "This is a section block with an accessory image."
                },
                "accessory": {
                    "type": "image",
                    "image_url": "https://pbs.twimg.com/profile_images/625633822235693056/lNGUneLX_400x400.jpg",
                    "alt_text": "cute cat"
                }
            }
        ]
    }"#,
    );
}

#[test]
fn image_element_2() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "block_id": "section567",
                "text": {
                    "type": "mrkdwn",
                    "text": "This is a section block with an accessory image."
                },
                "accessory": {
                    "type": "image",
                    "slack_file": {
                        "url": "https://files.slack.com/files-pri/T0123456-F0123456/xyz.png"
                    },
                    "alt_text": "Slack file object."
                }
            }
        ]
    }"#,
    );
}

#[test]
fn image_element_3() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "block_id": "section567",
                "text": {
                    "type": "mrkdwn",
                    "text": "This is a section block with an accessory image."
                },
                "accessory": {
                    "type": "image",
                    "slack_file": {
                        "id": "F01234567"
                    },
                    "alt_text": "Slack file object."
                }
            }
        ]
    }"#,
    );
}

#[test]
fn input_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "input",
                "element": {
                    "type": "plain_text_input"
                },
                "label": {
                    "type": "plain_text",
                    "text": "Label",
                    "emoji": true
                }
            }
        ]
    }"#,
    );
}

#[test]
fn link_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "link",
                                "url": "https://docs.slack.dev"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn list_record_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "list_record",
                                "file_id": "F123ABC456"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn markdown_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "markdown",
                "text": "**Lots of information here!!**"
            }
        ]
    }"#,
    );
}

#[test]
fn message_mention_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "message_mention",
                                "channel_id": "C123ABC456",
                                "message_ts": "1720710212.123456"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn multi_select_menu_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "block_id": "section678",
                "text": {
                    "type": "mrkdwn",
                    "text": "Pick items from the list"
                },
                "accessory": {
                    "action_id": "text1234",
                    "type": "multi_static_select",
                    "placeholder": {
                        "type": "plain_text",
                        "text": "Select items"
                    },
                    "options": [
                        {
                            "text": {
                                "type": "plain_text",
                                "text": "*this is plain_text text*"
                            },
                            "value": "value-0"
                        },
                        {
                            "text": {
                                "type": "plain_text",
                                "text": "*this is plain_text text*"
                            },
                            "value": "value-1"
                        },
                        {
                            "text": {
                                "type": "plain_text",
                                "text": "*this is plain_text text*"
                            },
                            "value": "value-2"
                        }
                    ]
                }
            }
        ]
    }"#,
    );
}

#[test]
fn multi_select_menu_element_2() {
    roundtrip::<Vec<OptionObject>>(
        &serde_json::from_str::<Value>(
            r#"{
        "options": [
            {
                "text": {
                    "type": "plain_text",
                    "text": "*this is plain_text text*"
                },
                "value": "value-0"
            },
            {
                "text": {
                    "type": "plain_text",
                    "text": "*this is plain_text text*"
                },
                "value": "value-1"
            },
            {
                "text": {
                    "type": "plain_text",
                    "text": "*this is plain_text text*"
                },
                "value": "value-2"
            }
        ]
    }"#,
        )
        .unwrap()["options"]
            .to_string(),
    );
}

#[test]
fn multi_select_menu_element_3() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "block_id": "section678",
                "text": {
                    "type": "mrkdwn",
                    "text": "Pick items from the list"
                },
                "accessory": {
                    "action_id": "text1234",
                    "type": "multi_external_select",
                    "placeholder": {
                        "type": "plain_text",
                        "text": "Select items"
                    },
                    "min_query_length": 3
                }
            }
        ]
    }"#,
    );
}

#[test]
fn multi_select_menu_element_4() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "block_id": "section678",
                "text": {
                    "type": "mrkdwn",
                    "text": "Pick users from the list"
                },
                "accessory": {
                    "action_id": "text1234",
                    "type": "multi_users_select",
                    "placeholder": {
                        "type": "plain_text",
                        "text": "Select users"
                    }
                }
            }
        ]
    }"#,
    );
}

#[test]
fn multi_select_menu_element_5() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "block_id": "section678",
                "text": {
                    "type": "mrkdwn",
                    "text": "Pick conversations from the list"
                },
                "accessory": {
                    "action_id": "text1234",
                    "type": "multi_conversations_select",
                    "placeholder": {
                        "type": "plain_text",
                        "text": "Select conversations"
                    }
                }
            }
        ]
    }"#,
    );
}

#[test]
fn multi_select_menu_element_6() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "block_id": "section678",
                "text": {
                    "type": "mrkdwn",
                    "text": "Pick channels from the list"
                },
                "accessory": {
                    "action_id": "text1234",
                    "type": "multi_channels_select",
                    "placeholder": {
                        "type": "plain_text",
                        "text": "Select channels"
                    }
                }
            }
        ]
    }"#,
    );
}

#[test]
fn number_input_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "input",
                "element": {
                    "type": "number_input",
                    "is_decimal_allowed": false,
                    "action_id": "number_input-action"
                },
                "label": {
                    "type": "plain_text",
                    "text": "Label",
                    "emoji": true
                }
            }
        ]
    }"#,
    );
}

#[test]
fn option_group_object_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": ":mag: Search results for *Cata*"
                }
            },
            {
                "type": "divider"
            },
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": "*<fakeLink.toYourApp.com|Use Case Catalogue>*\nUse Case Catalogue for the following departments/roles..."
                },
                "accessory": {
                    "type": "static_select",
                    "placeholder": {
                        "type": "plain_text",
                        "emoji": true,
                        "text": "Manage"
                    },
                    "option_groups": [
                        {
                            "label": {
                                "type": "plain_text",
                                "text": "Group 1"
                            },
                            "options": [
                                {
                                    "text": {
                                        "type": "plain_text",
                                        "text": "*this is plain_text text*"
                                    },
                                    "value": "value-0"
                                },
                                {
                                    "text": {
                                        "type": "plain_text",
                                        "text": "*this is plain_text text*"
                                    },
                                    "value": "value-1"
                                },
                                {
                                    "text": {
                                        "type": "plain_text",
                                        "text": "*this is plain_text text*"
                                    },
                                    "value": "value-2"
                                }
                            ]
                        },
                        {
                            "label": {
                                "type": "plain_text",
                                "text": "Group 2"
                            },
                            "options": [
                                {
                                    "text": {
                                        "type": "plain_text",
                                        "text": "*this is plain_text text*"
                                    },
                                    "value": "value-3"
                                }
                            ]
                        }
                    ]
                }
            }
        ]
    }"#,
    );
}

#[test]
fn option_object_1() {
    roundtrip::<OptionObject>(
        r#"{
        "text": {
            "type": "plain_text",
            "emoji": true,
            "text": "Save it"
        },
        "value": "value-2"
    }"#,
    );
}

#[test]
fn option_object_2() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": ":mag: Search results for *Cata*"
                }
            },
            {
                "type": "divider"
            },
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": "*<fakeLink.toYourApp.com|Use Case Catalogue>*\nUse Case Catalogue for the following departments/roles..."
                },
                "accessory": {
                    "type": "static_select",
                    "placeholder": {
                        "type": "plain_text",
                        "emoji": true,
                        "text": "Manage"
                    },
                    "options": [
                        {
                            "text": {
                                "type": "plain_text",
                                "emoji": true,
                                "text": "Edit it"
                            },
                            "value": "value-0"
                        },
                        {
                            "text": {
                                "type": "plain_text",
                                "emoji": true,
                                "text": "Read it"
                            },
                            "value": "value-1"
                        },
                        {
                            "text": {
                                "type": "plain_text",
                                "emoji": true,
                                "text": "Save it"
                            },
                            "value": "value-2"
                        }
                    ]
                }
            }
        ]
    }"#,
    );
}

#[test]
fn overflow_menu_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "block_id": "section 890",
                "text": {
                    "type": "mrkdwn",
                    "text": "This is a section block with an overflow menu."
                },
                "accessory": {
                    "type": "overflow",
                    "options": [
                        {
                            "text": {
                                "type": "plain_text",
                                "text": "*this is plain_text text*"
                            },
                            "value": "value-0"
                        },
                        {
                            "text": {
                                "type": "plain_text",
                                "text": "*this is plain_text text*"
                            },
                            "value": "value-1"
                        },
                        {
                            "text": {
                                "type": "plain_text",
                                "text": "*this is plain_text text*"
                            },
                            "value": "value-2"
                        },
                        {
                            "text": {
                                "type": "plain_text",
                                "text": "*this is plain_text text*"
                            },
                            "value": "value-3"
                        },
                        {
                            "text": {
                                "type": "plain_text",
                                "text": "*this is plain_text text*"
                            },
                            "value": "value-4"
                        }
                    ],
                    "action_id": "overflow"
                }
            }
        ]
    }"#,
    );
}

#[test]
fn plain_text_input_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "input",
                "element": {
                    "type": "plain_text_input",
                    "action_id": "plain_text_input-action"
                },
                "label": {
                    "type": "plain_text",
                    "text": "Label",
                    "emoji": true
                }
            }
        ]
    }"#,
    );
}

#[test]
fn plan_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "plan",
                "title": "Thinking completed",
                "tasks": [
                    {
                        "task_id": "call_001",
                        "title": "Fetched user profile information",
                        "status": "in_progress",
                        "details": {
                            "type": "rich_text",
                            "block_id": "viMWO",
                            "elements": [
                                {
                                    "type": "rich_text_section",
                                    "elements": [
                                        {
                                            "type": "text",
                                            "text": "Searched database..."
                                        }
                                    ]
                                }
                            ]
                        },
                        "output": {
                            "type": "rich_text",
                            "block_id": "viMWO",
                            "elements": [
                                {
                                    "type": "rich_text_section",
                                    "elements": [
                                        {
                                            "type": "text",
                                            "text": "Profile data loaded"
                                        }
                                    ]
                                }
                            ]
                        }
                    },
                    {
                        "task_id": "call_002",
                        "title": "Checked user permissions",
                        "status": "pending"
                    },
                    {
                        "task_id": "call_003",
                        "title": "Generated comprehensive user report",
                        "status": "complete",
                        "output": {
                            "type": "rich_text",
                            "block_id": "crsk",
                            "elements": [
                                {
                                    "type": "rich_text_section",
                                    "elements": [
                                        {
                                            "type": "text",
                                            "text": "15 data points compiled"
                                        }
                                    ]
                                }
                            ]
                        }
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn radio_button_group_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "plain_text",
                    "text": "Check out these rad radio buttons"
                },
                "accessory": {
                    "type": "radio_buttons",
                    "action_id": "this_is_an_action_id",
                    "initial_option": {
                        "value": "A1",
                        "text": {
                            "type": "plain_text",
                            "text": "Radio 1"
                        }
                    },
                    "options": [
                        {
                            "value": "A1",
                            "text": {
                                "type": "plain_text",
                                "text": "Radio 1"
                            }
                        },
                        {
                            "value": "A2",
                            "text": {
                                "type": "plain_text",
                                "text": "Radio 2"
                            }
                        }
                    ]
                }
            }
        ]
    }"#,
    );
}

#[test]
fn rich_text_input_element_1() {
    roundtrip::<View>(
        r#"{
        "blocks": [
            {
                "type": "input",
                "element": {
                    "type": "rich_text_input",
                    "action_id": "rich_text_input-action",
                    "dispatch_action_config": {
                        "trigger_actions_on": [
                            "on_character_entered"
                        ]
                    },
                    "focus_on_load": true,
                    "placeholder": {
                        "type": "plain_text",
                        "text": "Enter text"
                    }
                },
                "label": {
                    "type": "plain_text",
                    "text": "Label",
                    "emoji": true
                }
            }
        ],
        "type": "home"
    }"#,
    );
}

#[test]
fn rich_text_list_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "block_id": "block1",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "text",
                                "text": "My favorite Slack features (in no particular order):"
                            }
                        ]
                    },
                    {
                        "type": "rich_text_list",
                        "elements": [
                            {
                                "type": "rich_text_section",
                                "elements": [
                                    {
                                        "type": "text",
                                        "text": "Huddles"
                                    }
                                ]
                            },
                            {
                                "type": "rich_text_section",
                                "elements": [
                                    {
                                        "type": "text",
                                        "text": "Canvas"
                                    }
                                ]
                            },
                            {
                                "type": "rich_text_section",
                                "elements": [
                                    {
                                        "type": "text",
                                        "text": "Developing with Block Kit"
                                    }
                                ]
                            }
                        ],
                        "style": "bullet",
                        "indent": 0,
                        "border": 1
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn rich_text_list_element_2() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "block_id": "block1",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "text",
                                "text": "Breakfast foods I enjoy:"
                            }
                        ]
                    },
                    {
                        "type": "rich_text_list",
                        "style": "bullet",
                        "elements": [
                            {
                                "type": "rich_text_section",
                                "elements": [
                                    {
                                        "type": "text",
                                        "text": "Hashbrowns"
                                    }
                                ]
                            },
                            {
                                "type": "rich_text_section",
                                "elements": [
                                    {
                                        "type": "text",
                                        "text": "Eggs"
                                    }
                                ]
                            }
                        ]
                    },
                    {
                        "type": "rich_text_list",
                        "style": "bullet",
                        "indent": 1,
                        "elements": [
                            {
                                "type": "rich_text_section",
                                "elements": [
                                    {
                                        "type": "text",
                                        "text": "Scrambled"
                                    }
                                ]
                            },
                            {
                                "type": "rich_text_section",
                                "elements": [
                                    {
                                        "type": "text",
                                        "text": "Over easy"
                                    }
                                ]
                            }
                        ]
                    },
                    {
                        "type": "rich_text_list",
                        "style": "bullet",
                        "elements": [
                            {
                                "type": "rich_text_section",
                                "elements": [
                                    {
                                        "type": "text",
                                        "text": "Pancakes, extra syrup"
                                    }
                                ]
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn rich_text_preformatted_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_preformatted",
                        "elements": [
                            {
                                "type": "text",
                                "text": "{\n  \"object\": {\n    \"description\": \"this is an example of a json object\"\n  }\n}"
                            }
                        ],
                        "border": 0
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn rich_text_quote_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "block_id": "Vrzsu",
                "elements": [
                    {
                        "type": "rich_text_quote",
                        "elements": [
                            {
                                "type": "text",
                                "text": "What we need is good examples in our documentation."
                            }
                        ]
                    },
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "text",
                                "text": "Yes - I completely agree, Luke!"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn rich_text_section_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "text",
                                "text": "Hello there, I am a basic rich text block!"
                            }
                        ]
                    }
                ]
            },
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "text",
                                "text": "Hello there, "
                            },
                            {
                                "type": "text",
                                "text": "I am a bold rich text block!",
                                "style": {
                                    "bold": true
                                }
                            }
                        ]
                    }
                ]
            },
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "text",
                                "text": "Hello there, "
                            },
                            {
                                "type": "text",
                                "text": "I am an italic rich text block!",
                                "style": {
                                    "italic": true
                                }
                            }
                        ]
                    }
                ]
            },
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "text",
                                "text": "Hello there, "
                            },
                            {
                                "type": "text",
                                "text": "I am a strikethrough rich text block!",
                                "style": {
                                    "strike": true
                                }
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn salesforce_data_field_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "salesforce_data_field",
                                "salesforce_record_id": "001ABC456DEF789"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn section_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": "A message *with some bold text* and _some italicized text_."
                }
            }
        ]
    }"#,
    );
}

#[test]
fn section_block_2() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "text": {
                    "text": "A message *with some bold text* and _some italicized text_.",
                    "type": "mrkdwn"
                },
                "fields": [
                    {
                        "type": "mrkdwn",
                        "text": "High"
                    },
                    {
                        "type": "plain_text",
                        "emoji": true,
                        "text": "Silly"
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn section_block_3() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "text": {
                    "text": "*Haley* has requested you set a deadline for finding a house",
                    "type": "mrkdwn"
                },
                "accessory": {
                    "type": "datepicker",
                    "action_id": "datepicker123",
                    "initial_date": "1990-04-28",
                    "placeholder": {
                        "type": "plain_text",
                        "text": "Select a date"
                    }
                }
            }
        ]
    }"#,
    );
}

#[test]
fn select_menu_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "block_id": "section678",
                "text": {
                    "type": "mrkdwn",
                    "text": "Pick an item from the dropdown list"
                },
                "accessory": {
                    "action_id": "text1234",
                    "type": "static_select",
                    "placeholder": {
                        "type": "plain_text",
                        "text": "Select an item"
                    },
                    "options": [
                        {
                            "text": {
                                "type": "plain_text",
                                "text": "*this is plain_text text*"
                            },
                            "value": "value-0"
                        },
                        {
                            "text": {
                                "type": "plain_text",
                                "text": "*this is plain_text text*"
                            },
                            "value": "value-1"
                        },
                        {
                            "text": {
                                "type": "plain_text",
                                "text": "*this is plain_text text*"
                            },
                            "value": "value-2"
                        }
                    ]
                }
            }
        ]
    }"#,
    );
}

#[test]
fn select_menu_element_2() {
    roundtrip::<Vec<OptionObject>>(
        &serde_json::from_str::<Value>(
            r#"{
        "options": [
            {
                "text": {
                    "type": "plain_text",
                    "text": "*this is plain_text text*"
                },
                "value": "value-0"
            },
            {
                "text": {
                    "type": "plain_text",
                    "text": "*this is plain_text text*"
                },
                "value": "value-1"
            },
            {
                "text": {
                    "type": "plain_text",
                    "text": "*this is plain_text text*"
                },
                "value": "value-2"
            }
        ]
    }"#,
        )
        .unwrap()["options"]
            .to_string(),
    );
}

#[test]
fn select_menu_element_3() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "block_id": "section678",
                "text": {
                    "type": "mrkdwn",
                    "text": "Pick an item from the dropdown list"
                },
                "accessory": {
                    "action_id": "text1234",
                    "type": "external_select",
                    "placeholder": {
                        "type": "plain_text",
                        "text": "Select an item"
                    },
                    "min_query_length": 3
                }
            }
        ]
    }"#,
    );
}

#[test]
fn select_menu_element_4() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "block_id": "section678",
                "text": {
                    "type": "mrkdwn",
                    "text": "Pick a user from the dropdown list"
                },
                "accessory": {
                    "action_id": "text1234",
                    "type": "users_select",
                    "placeholder": {
                        "type": "plain_text",
                        "text": "Select an item"
                    }
                }
            }
        ]
    }"#,
    );
}

#[test]
fn select_menu_element_5() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "block_id": "section678",
                "text": {
                    "type": "mrkdwn",
                    "text": "Pick a conversation from the dropdown list"
                },
                "accessory": {
                    "action_id": "text1234",
                    "type": "conversations_select",
                    "placeholder": {
                        "type": "plain_text",
                        "text": "Select an item"
                    }
                }
            }
        ]
    }"#,
    );
}

#[test]
fn select_menu_element_6() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "block_id": "section678",
                "text": {
                    "type": "mrkdwn",
                    "text": "Pick a channel from the dropdown list"
                },
                "accessory": {
                    "action_id": "text1234",
                    "type": "channels_select",
                    "placeholder": {
                        "type": "plain_text",
                        "text": "Select an item"
                    }
                }
            }
        ]
    }"#,
    );
}

#[test]
fn slack_file_object_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "image",
                "title": {
                    "type": "plain_text",
                    "text": "Please enjoy this photo of a kitten"
                },
                "block_id": "image4",
                "slack_file": {
                    "url": "https://files.slack.com/files-pri/T0123456-F0123456/xyz.png"
                },
                "alt_text": "An incredibly cute kitten."
            }
        ]
    }"#,
    );
}

#[test]
fn slack_file_object_2() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "image",
                "title": {
                    "type": "plain_text",
                    "text": "Please enjoy this photo of a kitten"
                },
                "block_id": "image4",
                "slack_file": {
                    "id": "F0123456"
                },
                "alt_text": "An incredibly cute kitten."
            }
        ]
    }"#,
    );
}

#[test]
fn table_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "table",
                "column_settings": [
                    {
                        "is_wrapped": true
                    },
                    {
                        "align": "right"
                    }
                ],
                "rows": [
                    [
                        {
                            "type": "raw_text",
                            "text": "Header A"
                        },
                        {
                            "type": "raw_text",
                            "text": "Header B"
                        }
                    ],
                    [
                        {
                            "type": "raw_text",
                            "text": "Data 1A"
                        },
                        {
                            "type": "rich_text",
                            "elements": [
                                {
                                    "type": "rich_text_section",
                                    "elements": [
                                        {
                                            "text": "Data 1B",
                                            "type": "link",
                                            "url": "https://slack.com"
                                        }
                                    ]
                                }
                            ]
                        }
                    ],
                    [
                        {
                            "type": "raw_text",
                            "text": "Data 2A"
                        },
                        {
                            "type": "rich_text",
                            "elements": [
                                {
                                    "type": "rich_text_section",
                                    "elements": [
                                        {
                                            "text": "Data 2B",
                                            "type": "link",
                                            "url": "https://slack.com"
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn tag_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "tag",
                                "text": "In progress"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn task_card_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "task_card",
                "task_id": "task_1",
                "title": "Fetching weather data",
                "status": "in_progress",
                "output": {
                    "type": "rich_text",
                    "elements": [
                        {
                            "type": "rich_text_section",
                            "elements": [
                                {
                                    "type": "text",
                                    "text": "Found weather data for Chicago from 2 sources"
                                }
                            ]
                        }
                    ]
                },
                "sources": [
                    {
                        "type": "url",
                        "url": "https://weather.com/",
                        "text": "weather.com"
                    },
                    {
                        "type": "url",
                        "url": "https://www.accuweather.com/",
                        "text": "accuweather.com"
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn team_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "team",
                                "team_id": "T123ABC456"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn text_element_1() {
    roundtrip::<RichTextBlock>(
        r#"{
        "type": "rich_text",
        "elements": [
            {
                "type": "rich_text_section",
                "elements": [
                    {
                        "type": "text",
                        "text": "Hello there, "
                    },
                    {
                        "type": "text",
                        "text": "I am a bold rich text block!",
                        "style": {
                            "bold": true
                        }
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn text_object_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": "A message *with some bold text* and _some italicized text_."
                }
            }
        ]
    }"#,
    );
}

#[test]
fn time_picker_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "block_id": "section1234",
                "text": {
                    "type": "mrkdwn",
                    "text": "Pick a date for the deadline."
                },
                "accessory": {
                    "type": "timepicker",
                    "timezone": "America/Los_Angeles",
                    "action_id": "timepicker123",
                    "initial_time": "11:40",
                    "placeholder": {
                        "type": "plain_text",
                        "text": "Select a time"
                    }
                }
            }
        ]
    }"#,
    );
}

#[test]
fn trigger_object_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "text": {
                    "text": "A message *with some bold text* and _some italicized text_.",
                    "type": "mrkdwn"
                },
                "accessory": {
                    "type": "workflow_button",
                    "text": {
                        "type": "plain_text",
                        "text": "Run Workflow"
                    },
                    "action_id": "workflowbutton123",
                    "workflow": {
                        "trigger": {
                            "url": "https://slack.com/shortcuts/Ft0123ABC456/xyz...zyx",
                            "customizable_input_parameters": [
                                {
                                    "name": "input_parameter_a",
                                    "value": "Value for input param A"
                                },
                                {
                                    "name": "input_parameter_b",
                                    "value": "Value for input param B"
                                }
                            ]
                        }
                    }
                }
            }
        ]
    }"#,
    );
}

#[test]
fn url_input_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "input",
                "element": {
                    "type": "url_text_input",
                    "action_id": "url_text_input-action"
                },
                "label": {
                    "type": "plain_text",
                    "text": "Label",
                    "emoji": true
                }
            }
        ]
    }"#,
    );
}

#[test]
fn url_source_element_1() {
    roundtrip::<UrlSourceElement>(
        r#"{
        "type": "url",
        "url": "https://docs.slack.dev/",
        "text": "Slack API docs"
    }"#,
    );
}

#[test]
fn url_source_element_2() {
    roundtrip::<TaskCardBlock>(
        r#"{
        "type": "task_card",
        "task_id": "task_1",
        "title": "Scientific findings",
        "status": "complete",
        "sources": [
            {
                "type": "url",
                "url": "https://docs.example.com/",
                "text": "Tracy's delightful docs"
            },
            {
                "type": "url",
                "url": "https://research.example.com/",
                "text": "Haley's resourceful research"
            }
        ]
    }"#,
    );
}

#[test]
fn user_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "user",
                                "user_id": "U123ABC456"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn usergroup_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "usergroup",
                                "usergroup_id": "G123ABC456"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn video_block_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "video",
                "title": {
                    "type": "plain_text",
                    "text": "Use the Events API to create a dynamic App Home",
                    "emoji": true
                },
                "title_url": "https://www.youtube.com/watch?v=8876OZV_Yy0",
                "description": {
                    "type": "plain_text",
                    "text": "Slack sure is nifty!",
                    "emoji": true
                },
                "video_url": "https://www.youtube.com/embed/8876OZV_Yy0?feature=oembed&autoplay=1",
                "alt_text": "Use the Events API to create a dynamic App Home",
                "thumbnail_url": "https://i.ytimg.com/vi/8876OZV_Yy0/hqdefault.jpg"
            }
        ]
    }"#,
    );
}

#[test]
fn work_object_mention_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "work_object_mention",
                                "entity_id": "E123ABC456",
                                "app_id": "A123ABC456",
                                "text": "Work object",
                                "url": "https://example.com/work-object"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn workflow_button_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "text": {
                    "text": "A message *with some bold text* and _some italicized text_.",
                    "type": "mrkdwn"
                },
                "accessory": {
                    "type": "workflow_button",
                    "text": {
                        "type": "plain_text",
                        "text": "Run Workflow"
                    },
                    "action_id": "workflowbutton123",
                    "workflow": {
                        "trigger": {
                            "url": "https://slack.com/shortcuts/Ft0123ABC456/xyz...zyx",
                            "customizable_input_parameters": [
                                {
                                    "name": "input_parameter_a",
                                    "value": "Value for input param A"
                                },
                                {
                                    "name": "input_parameter_b",
                                    "value": "Value for input param B"
                                }
                            ]
                        }
                    }
                }
            }
        ]
    }"#,
    );
}

#[test]
fn workflow_mention_element_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "workflow_mention",
                                "workflow_id": "Wf123ABC456",
                                "function_trigger_id": "Ft123ABC456",
                                "text": "Run workflow"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#,
    );
}

#[test]
fn workflow_object_1() {
    roundtrip_blocks(
        r#"{
        "blocks": [
            {
                "type": "section",
                "text": {
                    "text": "A message *with some bold text* and _some italicized text_.",
                    "type": "mrkdwn"
                },
                "accessory": {
                    "type": "workflow_button",
                    "text": {
                        "type": "plain_text",
                        "text": "Run Workflow"
                    },
                    "action_id": "workflowbutton123",
                    "workflow": {
                        "trigger": {
                            "url": "https://slack.com/shortcuts/Ft0123ABC456/xyz...zyx",
                            "customizable_input_parameters": [
                                {
                                    "name": "input_parameter_a",
                                    "value": "Value for input param A"
                                },
                                {
                                    "name": "input_parameter_b",
                                    "value": "Value for input param B"
                                }
                            ]
                        }
                    }
                }
            }
        ]
    }"#,
    );
}
