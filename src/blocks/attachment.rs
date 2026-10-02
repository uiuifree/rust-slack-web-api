use serde::{Deserialize, Serialize};

use super::block::Block;

/// The `ts` of an attachment. Sent as a number, but returned as a string (e.g. `"1503435956.000247"`) in places such as message unfurls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttachmentTs {
    Number(serde_json::Number),
    String(String),
}

/// A legacy secondary message attachment (can also hold `blocks`). <https://docs.slack.dev/legacy/legacy-messaging/legacy-secondary-message-attachments>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pretext: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<AttachmentField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<AttachmentTs>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mrkdwn_in: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<Block>,
}

impl Attachment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn fallback(mut self, fallback: impl Into<String>) -> Self {
        self.fallback = Some(fallback.into());
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn pretext(mut self, pretext: impl Into<String>) -> Self {
        self.pretext = Some(pretext.into());
        self
    }

    pub fn author_name(mut self, author_name: impl Into<String>) -> Self {
        self.author_name = Some(author_name.into());
        self
    }

    pub fn author_link(mut self, author_link: impl Into<String>) -> Self {
        self.author_link = Some(author_link.into());
        self
    }

    pub fn author_icon(mut self, author_icon: impl Into<String>) -> Self {
        self.author_icon = Some(author_icon.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn title_link(mut self, title_link: impl Into<String>) -> Self {
        self.title_link = Some(title_link.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn fields(mut self, fields: Vec<AttachmentField>) -> Self {
        self.fields = fields;
        self
    }

    pub fn image_url(mut self, image_url: impl Into<String>) -> Self {
        self.image_url = Some(image_url.into());
        self
    }

    pub fn thumb_url(mut self, thumb_url: impl Into<String>) -> Self {
        self.thumb_url = Some(thumb_url.into());
        self
    }

    pub fn thumb_width(mut self, thumb_width: u32) -> Self {
        self.thumb_width = Some(thumb_width);
        self
    }

    pub fn thumb_height(mut self, thumb_height: u32) -> Self {
        self.thumb_height = Some(thumb_height);
        self
    }

    pub fn service_name(mut self, service_name: impl Into<String>) -> Self {
        self.service_name = Some(service_name.into());
        self
    }

    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    pub fn footer_icon(mut self, footer_icon: impl Into<String>) -> Self {
        self.footer_icon = Some(footer_icon.into());
        self
    }

    pub fn ts(mut self, ts: AttachmentTs) -> Self {
        self.ts = Some(ts);
        self
    }

    pub fn mrkdwn_in(mut self, mrkdwn_in: Vec<String>) -> Self {
        self.mrkdwn_in = mrkdwn_in;
        self
    }

    pub fn blocks(mut self, blocks: Vec<Block>) -> Self {
        self.blocks = blocks;
        self
    }
}

/// A field shown in a table inside an attachment. <https://docs.slack.dev/legacy/legacy-messaging/legacy-secondary-message-attachments>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttachmentField {
    pub title: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short: Option<bool>,
}

impl AttachmentField {
    pub fn new(title: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            value: value.into(),
            short: None,
        }
    }

    pub fn short(mut self, short: bool) -> Self {
        self.short = Some(short);
        self
    }
}
