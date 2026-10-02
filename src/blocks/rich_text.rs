use serde::{Deserialize, Serialize};

use super::tag::{tagged_enum, Tag, Tagged};

/// A rich text block. <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RichTextBlock {
    r#type: Tag<Self>,
    pub elements: Vec<RichTextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Tagged for RichTextBlock {
    const TAG: &'static str = "rich_text";
}

impl RichTextBlock {
    pub fn new(elements: Vec<RichTextObject>) -> Self {
        Self {
            r#type: Tag::default(),
            elements,
            block_id: None,
        }
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }
}

tagged_enum! {
    /// A top-level element of a rich text block. <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block>
    pub enum RichTextObject("type") {
        Section(RichTextSection),
        List(RichTextList),
        Preformatted(RichTextPreformatted),
        Quote(RichTextQuote),
    }
}

/// A rich text section. <https://docs.slack.dev/reference/block-kit/block-elements/rich-text-section-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RichTextSection {
    r#type: Tag<Self>,
    pub elements: Vec<RichTextElement>,
}

impl Tagged for RichTextSection {
    const TAG: &'static str = "rich_text_section";
}

impl RichTextSection {
    pub fn new(elements: Vec<RichTextElement>) -> Self {
        Self {
            r#type: Tag::default(),
            elements,
        }
    }
}

/// A rich text list. <https://docs.slack.dev/reference/block-kit/block-elements/rich-text-list-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RichTextList {
    r#type: Tag<Self>,
    pub style: RichTextListStyle,
    pub elements: Vec<RichTextSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<u8>,
}

impl Tagged for RichTextList {
    const TAG: &'static str = "rich_text_list";
}

impl RichTextList {
    pub fn new(style: RichTextListStyle, elements: Vec<RichTextSection>) -> Self {
        Self {
            r#type: Tag::default(),
            style,
            elements,
            indent: None,
            offset: None,
            border: None,
        }
    }

    pub fn indent(mut self, indent: u32) -> Self {
        self.indent = Some(indent);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn border(mut self, border: u8) -> Self {
        self.border = Some(border);
        self
    }
}

/// A rich text preformatted (code) block. <https://docs.slack.dev/reference/block-kit/block-elements/rich-text-preformatted-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RichTextPreformatted {
    r#type: Tag<Self>,
    pub elements: Vec<PreformattedElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl Tagged for RichTextPreformatted {
    const TAG: &'static str = "rich_text_preformatted";
}

impl RichTextPreformatted {
    pub fn new(elements: Vec<PreformattedElement>) -> Self {
        Self {
            r#type: Tag::default(),
            elements,
            border: None,
            language: None,
        }
    }

    pub fn border(mut self, border: u8) -> Self {
        self.border = Some(border);
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }
}

/// A rich text quote. <https://docs.slack.dev/reference/block-kit/block-elements/rich-text-quote-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RichTextQuote {
    r#type: Tag<Self>,
    pub elements: Vec<RichTextElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<u8>,
}

impl Tagged for RichTextQuote {
    const TAG: &'static str = "rich_text_quote";
}

impl RichTextQuote {
    pub fn new(elements: Vec<RichTextElement>) -> Self {
        Self {
            r#type: Tag::default(),
            elements,
            border: None,
        }
    }

    pub fn border(mut self, border: u8) -> Self {
        self.border = Some(border);
        self
    }
}

tagged_enum! {
    /// An element allowed in a preformatted block (text and links). <https://docs.slack.dev/reference/block-kit/block-elements/rich-text-preformatted-element>
    pub enum PreformattedElement("type") {
        Text(TextElement),
        Link(LinkElement),
    }
}

tagged_enum! {
    /// An inline rich text element used in sections, lists and quotes. <https://docs.slack.dev/reference/block-kit/block-elements/rich-text-section-element>
    pub enum RichTextElement("type") {
        AttachmentMention(AttachmentMentionElement),
        Broadcast(BroadcastElement),
        Canvas(CanvasElement),
        CanvasUserMention(CanvasUserMentionElement),
        CanvasMessageUnfurl(CanvasMessageUnfurlElement),
        Channel(ChannelElement),
        Citation(CitationElement),
        Color(ColorElement),
        Date(DateElement),
        Emoji(EmojiElement),
        File(FileElement),
        Link(LinkElement),
        ListRecord(ListRecordElement),
        MessageMention(MessageMentionElement),
        SalesforceDataField(SalesforceDataFieldElement),
        Tag(TagElement),
        Team(TeamElement),
        Text(TextElement),
        User(UserElement),
        Usergroup(UsergroupElement),
        WorkObjectMention(WorkObjectMentionElement),
        WorkflowMention(WorkflowMentionElement),
    }
}

/// Styling for a rich text element. <https://docs.slack.dev/reference/block-kit/block-elements/text-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichTextStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_highlight: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlink: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<bool>,
}

impl RichTextStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = Some(italic);
        self
    }

    pub fn strike(mut self, strike: bool) -> Self {
        self.strike = Some(strike);
        self
    }

    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = Some(highlight);
        self
    }

    pub fn client_highlight(mut self, client_highlight: bool) -> Self {
        self.client_highlight = Some(client_highlight);
        self
    }

    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = Some(underline);
        self
    }

    pub fn unlink(mut self, unlink: bool) -> Self {
        self.unlink = Some(unlink);
        self
    }

    pub fn code(mut self, code: bool) -> Self {
        self.code = Some(code);
        self
    }
}

/// A reference to an app attachment or entity. <https://docs.slack.dev/reference/block-kit/block-elements/attachment-mention-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttachmentMentionElement {
    r#type: Tag<Self>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_size_preview_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_object_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
}

impl Tagged for AttachmentMentionElement {
    const TAG: &'static str = "attachment_mention";
}

impl AttachmentMentionElement {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            url: url.into(),
            text: None,
            app_id: None,
            entity_id: None,
            icon_url: None,
            channel_id: None,
            ts: None,
            full_size_preview_enabled: None,
            icon_name: None,
            reference_object_type: None,
            product_name: None,
            style: None,
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    pub fn entity_id(mut self, entity_id: impl Into<String>) -> Self {
        self.entity_id = Some(entity_id.into());
        self
    }

    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }

    pub fn channel_id(mut self, channel_id: impl Into<String>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    pub fn ts(mut self, ts: impl Into<String>) -> Self {
        self.ts = Some(ts.into());
        self
    }

    pub fn full_size_preview_enabled(mut self, full_size_preview_enabled: bool) -> Self {
        self.full_size_preview_enabled = Some(full_size_preview_enabled);
        self
    }

    pub fn icon_name(mut self, icon_name: impl Into<String>) -> Self {
        self.icon_name = Some(icon_name.into());
        self
    }

    pub fn reference_object_type(mut self, reference_object_type: impl Into<String>) -> Self {
        self.reference_object_type = Some(reference_object_type.into());
        self
    }

    pub fn product_name(mut self, product_name: impl Into<String>) -> Self {
        self.product_name = Some(product_name.into());
        self
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// A broadcast mention (@here, @channel or @everyone). <https://docs.slack.dev/reference/block-kit/block-elements/broadcast-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BroadcastElement {
    r#type: Tag<Self>,
    pub range: BroadcastRange,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
}

impl Tagged for BroadcastElement {
    const TAG: &'static str = "broadcast";
}

impl BroadcastElement {
    pub fn new(range: BroadcastRange) -> Self {
        Self {
            r#type: Tag::default(),
            range,
            style: None,
        }
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// A link to a canvas. <https://docs.slack.dev/reference/block-kit/block-elements/canvas-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanvasElement {
    r#type: Tag<Self>,
    pub file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_title: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_skill_invocation: Option<bool>,
}

impl Tagged for CanvasElement {
    const TAG: &'static str = "canvas";
}

impl CanvasElement {
    pub fn new(file_id: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            file_id: file_id.into(),
            label: None,
            hide_title: None,
            section_id: None,
            style: None,
            text: None,
            url: None,
            is_skill_invocation: None,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn hide_title(mut self, hide_title: bool) -> Self {
        self.hide_title = Some(hide_title);
        self
    }

    pub fn section_id(mut self, section_id: impl Into<String>) -> Self {
        self.section_id = Some(section_id.into());
        self
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn is_skill_invocation(mut self, is_skill_invocation: bool) -> Self {
        self.is_skill_invocation = Some(is_skill_invocation);
        self
    }
}

/// A user mention inside canvas content. <https://docs.slack.dev/reference/block-kit/block-elements/canvas-user-mention-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanvasUserMentionElement {
    r#type: Tag<Self>,
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
}

impl Tagged for CanvasUserMentionElement {
    const TAG: &'static str = "canvas_user_mention";
}

impl CanvasUserMentionElement {
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            user_id: user_id.into(),
            thread_id: None,
            style: None,
        }
    }

    pub fn thread_id(mut self, thread_id: impl Into<String>) -> Self {
        self.thread_id = Some(thread_id.into());
        self
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// An inline preview of a message inside a canvas. <https://docs.slack.dev/reference/block-kit/block-elements/canvas-message-unfurl-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanvasMessageUnfurlElement {
    r#type: Tag<Self>,
    pub root_message_ts: String,
    pub root_message_channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
}

impl Tagged for CanvasMessageUnfurlElement {
    const TAG: &'static str = "canvas_message_unfurl";
}

impl CanvasMessageUnfurlElement {
    pub fn new(
        root_message_ts: impl Into<String>,
        root_message_channel: impl Into<String>,
    ) -> Self {
        Self {
            r#type: Tag::default(),
            root_message_ts: root_message_ts.into(),
            root_message_channel: root_message_channel.into(),
            style: None,
        }
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// A channel mention. <https://docs.slack.dev/reference/block-kit/block-elements/channel-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelElement {
    r#type: Tag<Self>,
    pub channel_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_llm: Option<bool>,
}

impl Tagged for ChannelElement {
    const TAG: &'static str = "channel";
}

impl ChannelElement {
    pub fn new(channel_id: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            channel_id: channel_id.into(),
            tab_id: None,
            style: None,
            from_llm: None,
        }
    }

    pub fn tab_id(mut self, tab_id: impl Into<String>) -> Self {
        self.tab_id = Some(tab_id.into());
        self
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn from_llm(mut self, from_llm: bool) -> Self {
        self.from_llm = Some(from_llm);
        self
    }
}

/// An AI citation. <https://docs.slack.dev/reference/block-kit/block-elements/citation-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CitationElement {
    r#type: Tag<Self>,
    pub url: String,
    pub text: String,
    pub index: u32,
    pub details: CitationDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_llm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_slack_url: Option<bool>,
}

impl Tagged for CitationElement {
    const TAG: &'static str = "citation";
}

impl CitationElement {
    pub fn new(
        url: impl Into<String>,
        text: impl Into<String>,
        index: u32,
        details: impl Into<CitationDetails>,
    ) -> Self {
        Self {
            r#type: Tag::default(),
            url: url.into(),
            text: text.into(),
            index,
            details: details.into(),
            from_llm: None,
            is_slack_url: None,
        }
    }

    pub fn from_llm(mut self, from_llm: bool) -> Self {
        self.from_llm = Some(from_llm);
        self
    }

    pub fn is_slack_url(mut self, is_slack_url: bool) -> Self {
        self.is_slack_url = Some(is_slack_url);
        self
    }
}

tagged_enum! {
    /// Details of a citation source (the kind is set by `citation_type`). <https://docs.slack.dev/reference/block-kit/block-elements/citation-element>
    pub enum CitationDetails("citation_type") {
        File(FileCitation),
        External(ExternalCitation),
        Web(WebCitation),
        Message(MessageCitation),
        Memory(MemoryCitation),
    }
}

/// A file citation source. <https://docs.slack.dev/reference/block-kit/block-elements/citation-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FileCitation {
    citation_type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
}

impl Tagged for FileCitation {
    const TAG: &'static str = "file";
}

impl FileCitation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn descriptor(mut self, descriptor: impl Into<String>) -> Self {
        self.descriptor = Some(descriptor.into());
        self
    }

    pub fn file_id(mut self, file_id: impl Into<String>) -> Self {
        self.file_id = Some(file_id.into());
        self
    }
}

/// An external app citation source. <https://docs.slack.dev/reference/block-kit/block-elements/citation-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ExternalCitation {
    citation_type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_icon_url: Option<String>,
}

impl Tagged for ExternalCitation {
    const TAG: &'static str = "external";
}

impl ExternalCitation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_name(mut self, app_name: impl Into<String>) -> Self {
        self.app_name = Some(app_name.into());
        self
    }

    pub fn app_icon_url(mut self, app_icon_url: impl Into<String>) -> Self {
        self.app_icon_url = Some(app_icon_url.into());
        self
    }
}

/// A web page citation source. <https://docs.slack.dev/reference/block-kit/block-elements/citation-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebCitation {
    citation_type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}

impl Tagged for WebCitation {
    const TAG: &'static str = "web";
}

impl WebCitation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = Some(display_name.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn snippet(mut self, snippet: impl Into<String>) -> Self {
        self.snippet = Some(snippet.into());
        self
    }
}

/// A message citation source. <https://docs.slack.dev/reference/block-kit/block-elements/citation-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageCitation {
    citation_type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_ts: Option<String>,
}

impl Tagged for MessageCitation {
    const TAG: &'static str = "message";
}

impl MessageCitation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn channel(mut self, channel: impl Into<String>) -> Self {
        self.channel = Some(channel.into());
        self
    }

    pub fn message_ts(mut self, message_ts: impl Into<String>) -> Self {
        self.message_ts = Some(message_ts.into());
        self
    }
}

/// A memory citation source. <https://docs.slack.dev/reference/block-kit/block-elements/citation-element>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MemoryCitation {
    citation_type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_id: Option<String>,
}

impl Tagged for MemoryCitation {
    const TAG: &'static str = "memory";
}

impl MemoryCitation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn memory_id(mut self, memory_id: impl Into<String>) -> Self {
        self.memory_id = Some(memory_id.into());
        self
    }
}

/// A color swatch. <https://docs.slack.dev/reference/block-kit/block-elements/color-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorElement {
    r#type: Tag<Self>,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
}

impl Tagged for ColorElement {
    const TAG: &'static str = "color";
}

impl ColorElement {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            value: value.into(),
            style: None,
        }
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// A date formatted for the viewer's locale and time zone. <https://docs.slack.dev/reference/block-kit/block-elements/date-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateElement {
    r#type: Tag<Self>,
    pub timestamp: i64,
    pub format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
}

impl Tagged for DateElement {
    const TAG: &'static str = "date";
}

impl DateElement {
    pub fn new(timestamp: i64, format: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            timestamp,
            format: format.into(),
            timezone: None,
            url: None,
            fallback: None,
            style: None,
        }
    }

    pub fn timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = Some(timezone.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn fallback(mut self, fallback: impl Into<String>) -> Self {
        self.fallback = Some(fallback.into());
        self
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// An emoji. <https://docs.slack.dev/reference/block-kit/block-elements/emoji-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmojiElement {
    r#type: Tag<Self>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode: Option<String>,
}

impl Tagged for EmojiElement {
    const TAG: &'static str = "emoji";
}

impl EmojiElement {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            name: name.into(),
            unicode: None,
        }
    }

    pub fn unicode(mut self, unicode: impl Into<String>) -> Self {
        self.unicode = Some(unicode.into());
        self
    }
}

/// A link to a Slack file. <https://docs.slack.dev/reference/block-kit/block-elements/file-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileElement {
    r#type: Tag<Self>,
    pub file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_skill_invocation: Option<bool>,
}

impl Tagged for FileElement {
    const TAG: &'static str = "file";
}

impl FileElement {
    pub fn new(file_id: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            file_id: file_id.into(),
            style: None,
            text: None,
            url: None,
            is_skill_invocation: None,
        }
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn is_skill_invocation(mut self, is_skill_invocation: bool) -> Self {
        self.is_skill_invocation = Some(is_skill_invocation);
        self
    }
}

/// A hyperlink. <https://docs.slack.dev/reference/block-kit/block-elements/link-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkElement {
    r#type: Tag<Self>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#unsafe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_llm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_slack_url: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
}

impl Tagged for LinkElement {
    const TAG: &'static str = "link";
}

impl LinkElement {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            url: url.into(),
            text: None,
            r#unsafe: None,
            from_llm: None,
            is_slack_url: None,
            truncated: None,
            style: None,
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn r#unsafe(mut self, r#unsafe: bool) -> Self {
        self.r#unsafe = Some(r#unsafe);
        self
    }

    pub fn from_llm(mut self, from_llm: bool) -> Self {
        self.from_llm = Some(from_llm);
        self
    }

    pub fn is_slack_url(mut self, is_slack_url: bool) -> Self {
        self.is_slack_url = Some(is_slack_url);
        self
    }

    pub fn truncated(mut self, truncated: bool) -> Self {
        self.truncated = Some(truncated);
        self
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// A link to a Slack list record. <https://docs.slack.dev/reference/block-kit/block-elements/list-record-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListRecordElement {
    r#type: Tag<Self>,
    pub file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Tagged for ListRecordElement {
    const TAG: &'static str = "list_record";
}

impl ListRecordElement {
    pub fn new(file_id: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            file_id: file_id.into(),
            record_id: None,
            view_id: None,
            style: None,
            text: None,
            url: None,
        }
    }

    pub fn record_id(mut self, record_id: impl Into<String>) -> Self {
        self.record_id = Some(record_id.into());
        self
    }

    pub fn view_id(mut self, view_id: impl Into<String>) -> Self {
        self.view_id = Some(view_id.into());
        self
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

/// A link to a Slack message. <https://docs.slack.dev/reference/block-kit/block-elements/message-mention-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageMentionElement {
    r#type: Tag<Self>,
    pub channel_id: String,
    pub message_ts: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Tagged for MessageMentionElement {
    const TAG: &'static str = "message_mention";
}

impl MessageMentionElement {
    pub fn new(channel_id: impl Into<String>, message_ts: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            channel_id: channel_id.into(),
            message_ts: message_ts.into(),
            author_id: None,
            thread_ts: None,
            style: None,
            text: None,
            url: None,
        }
    }

    pub fn author_id(mut self, author_id: impl Into<String>) -> Self {
        self.author_id = Some(author_id.into());
        self
    }

    pub fn thread_ts(mut self, thread_ts: impl Into<String>) -> Self {
        self.thread_ts = Some(thread_ts.into());
        self
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

/// A reference to a Salesforce data field. <https://docs.slack.dev/reference/block-kit/block-elements/salesforce-data-field-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SalesforceDataFieldElement {
    r#type: Tag<Self>,
    pub salesforce_record_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salesforce_field_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salesforce_field_api_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salesforce_include_field_label: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
}

impl Tagged for SalesforceDataFieldElement {
    const TAG: &'static str = "salesforce_data_field";
}

impl SalesforceDataFieldElement {
    pub fn new(salesforce_record_id: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            salesforce_record_id: salesforce_record_id.into(),
            salesforce_field_label: None,
            salesforce_field_api_name: None,
            salesforce_include_field_label: None,
            style: None,
        }
    }

    pub fn salesforce_field_label(mut self, salesforce_field_label: impl Into<String>) -> Self {
        self.salesforce_field_label = Some(salesforce_field_label.into());
        self
    }

    pub fn salesforce_field_api_name(
        mut self,
        salesforce_field_api_name: impl Into<String>,
    ) -> Self {
        self.salesforce_field_api_name = Some(salesforce_field_api_name.into());
        self
    }

    pub fn salesforce_include_field_label(mut self, salesforce_include_field_label: bool) -> Self {
        self.salesforce_include_field_label = Some(salesforce_include_field_label);
        self
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// A colored tag. <https://docs.slack.dev/reference/block-kit/block-elements/tag-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagElement {
    r#type: Tag<Self>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<TagColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
}

impl Tagged for TagElement {
    const TAG: &'static str = "tag";
}

impl TagElement {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            text: text.into(),
            color: None,
            style: None,
        }
    }

    pub fn color(mut self, color: TagColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// A workspace (team) mention. <https://docs.slack.dev/reference/block-kit/block-elements/team-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamElement {
    r#type: Tag<Self>,
    pub team_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
}

impl Tagged for TeamElement {
    const TAG: &'static str = "team";
}

impl TeamElement {
    pub fn new(team_id: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            team_id: team_id.into(),
            style: None,
        }
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// Text. <https://docs.slack.dev/reference/block-kit/block-elements/text-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextElement {
    r#type: Tag<Self>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
}

impl Tagged for TextElement {
    const TAG: &'static str = "text";
}

impl TextElement {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            text: text.into(),
            style: None,
        }
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// A user mention. <https://docs.slack.dev/reference/block-kit/block-elements/user-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserElement {
    r#type: Tag<Self>,
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_llm: Option<bool>,
}

impl Tagged for UserElement {
    const TAG: &'static str = "user";
}

impl UserElement {
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            user_id: user_id.into(),
            style: None,
            from_llm: None,
        }
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn from_llm(mut self, from_llm: bool) -> Self {
        self.from_llm = Some(from_llm);
        self
    }
}

/// A user group mention. <https://docs.slack.dev/reference/block-kit/block-elements/usergroup-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsergroupElement {
    r#type: Tag<Self>,
    pub usergroup_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
}

impl Tagged for UsergroupElement {
    const TAG: &'static str = "usergroup";
}

impl UsergroupElement {
    pub fn new(usergroup_id: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            usergroup_id: usergroup_id.into(),
            style: None,
        }
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// A reference to a Work Object. <https://docs.slack.dev/reference/block-kit/block-elements/work-object-mention-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkObjectMentionElement {
    r#type: Tag<Self>,
    pub entity_id: String,
    pub app_id: String,
    pub text: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_size_preview_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
}

impl Tagged for WorkObjectMentionElement {
    const TAG: &'static str = "work_object_mention";
}

impl WorkObjectMentionElement {
    pub fn new(
        entity_id: impl Into<String>,
        app_id: impl Into<String>,
        text: impl Into<String>,
        url: impl Into<String>,
    ) -> Self {
        Self {
            r#type: Tag::default(),
            entity_id: entity_id.into(),
            app_id: app_id.into(),
            text: text.into(),
            url: url.into(),
            icon_url: None,
            full_size_preview_enabled: None,
            product_name: None,
            style: None,
        }
    }

    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }

    pub fn full_size_preview_enabled(mut self, full_size_preview_enabled: bool) -> Self {
        self.full_size_preview_enabled = Some(full_size_preview_enabled);
        self
    }

    pub fn product_name(mut self, product_name: impl Into<String>) -> Self {
        self.product_name = Some(product_name.into());
        self
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// A link to a workflow. <https://docs.slack.dev/reference/block-kit/block-elements/workflow-mention-element>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowMentionElement {
    r#type: Tag<Self>,
    pub workflow_id: String,
    pub function_trigger_id: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RichTextStyle>,
}

impl Tagged for WorkflowMentionElement {
    const TAG: &'static str = "workflow_mention";
}

impl WorkflowMentionElement {
    pub fn new(
        workflow_id: impl Into<String>,
        function_trigger_id: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            r#type: Tag::default(),
            workflow_id: workflow_id.into(),
            function_trigger_id: function_trigger_id.into(),
            text: text.into(),
            url: None,
            channel_id: None,
            ts: None,
            style: None,
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn channel_id(mut self, channel_id: impl Into<String>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    pub fn ts(mut self, ts: impl Into<String>) -> Self {
        self.ts = Some(ts.into());
        self
    }

    pub fn style(mut self, style: RichTextStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// The style of a rich text list. Unknown values go to `Other`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RichTextListStyle {
    Bullet,
    Ordered,
    #[serde(untagged)]
    Other(String),
}

/// The range of a broadcast mention. Unknown values go to `Other`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BroadcastRange {
    Here,
    Channel,
    Everyone,
    #[serde(untagged)]
    Other(String),
}

/// The color of a tag. Unknown values go to `Other`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TagColor {
    Gray,
    Brown,
    Purple,
    Indigo,
    Blue,
    Green,
    Yellow,
    Orange,
    Red,
    #[serde(untagged)]
    Other(String),
}
