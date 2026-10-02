use serde::{Deserialize, Serialize};

use super::chart::DataVisualizationBlock;
use super::composition::{SlackFile, SlackIcon, TextObject};
use super::element::{
    ActionsElement, ButtonElement, ContextActionsElement, ContextElement, ImageElement,
    InputElement, SectionAccessory, UrlSourceElement,
};
use super::rich_text::RichTextBlock;
use super::table::{DataTableBlock, TableBlock};
use super::tag::{tagged_enum, Tag, Tagged};

tagged_enum! {
    /// A layout block. <https://docs.slack.dev/reference/block-kit/blocks>
    pub enum Block("type") {
        Actions(ActionsBlock),
        Alert(AlertBlock),
        Card(CardBlock),
        Carousel(CarouselBlock),
        Container(ContainerBlock),
        Context(ContextBlock),
        ContextActions(ContextActionsBlock),
        DataTable(DataTableBlock),
        DataVisualization(DataVisualizationBlock),
        Divider(DividerBlock),
        File(FileBlock),
        Header(HeaderBlock),
        Image(ImageBlock),
        Input(InputBlock),
        Markdown(MarkdownBlock),
        Plan(PlanBlock),
        RichText(RichTextBlock),
        Section(SectionBlock),
        Table(TableBlock),
        TaskCard(TaskCardBlock),
        Video(VideoBlock),
    }
}

/// A block that holds interactive elements. <https://docs.slack.dev/reference/block-kit/blocks/actions-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionsBlock {
    r#type: Tag<Self>,
    pub elements: Vec<ActionsElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Tagged for ActionsBlock {
    const TAG: &'static str = "actions";
}

impl ActionsBlock {
    pub fn new(elements: Vec<ActionsElement>) -> Self {
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

/// A block that shows an alert, warning or informational message (modals only). <https://docs.slack.dev/reference/block-kit/blocks/alert-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertBlock {
    r#type: Tag<Self>,
    pub text: TextObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<AlertLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Tagged for AlertBlock {
    const TAG: &'static str = "alert";
}

impl AlertBlock {
    pub fn new(text: impl Into<TextObject>) -> Self {
        Self {
            r#type: Tag::default(),
            text: text.into(),
            level: None,
            block_id: None,
        }
    }

    pub fn level(mut self, level: AlertLevel) -> Self {
        self.level = Some(level);
        self
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }
}

/// A card. <https://docs.slack.dev/reference/block-kit/blocks/card-block>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CardBlock {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hero_image: Option<ImageElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<TextObject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<ButtonElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_icon: Option<SlackIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtext: Option<TextObject>,
}

impl Tagged for CardBlock {
    const TAG: &'static str = "card";
}

impl CardBlock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }

    pub fn hero_image(mut self, hero_image: ImageElement) -> Self {
        self.hero_image = Some(hero_image);
        self
    }

    pub fn icon(mut self, icon: ImageElement) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn title(mut self, title: impl Into<TextObject>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn subtitle(mut self, subtitle: impl Into<TextObject>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn body(mut self, body: impl Into<TextObject>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn actions(mut self, actions: Vec<ButtonElement>) -> Self {
        self.actions = actions;
        self
    }

    pub fn slack_icon(mut self, slack_icon: SlackIcon) -> Self {
        self.slack_icon = Some(slack_icon);
        self
    }

    pub fn subtext(mut self, subtext: impl Into<TextObject>) -> Self {
        self.subtext = Some(subtext.into());
        self
    }
}

/// A horizontally scrolling row of cards. <https://docs.slack.dev/reference/block-kit/blocks/carousel-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CarouselBlock {
    r#type: Tag<Self>,
    pub elements: Vec<CardBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Tagged for CarouselBlock {
    const TAG: &'static str = "carousel";
}

impl CarouselBlock {
    pub fn new(elements: Vec<CardBlock>) -> Self {
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

/// A container that groups child blocks. <https://docs.slack.dev/reference/block-kit/blocks/container-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerBlock {
    r#type: Tag<Self>,
    pub title: TextObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text_title: Option<RichTextBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<TextObject>,
    pub child_blocks: Vec<Block>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<ContainerWidth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_collapsible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_collapsed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_header_divider: Option<bool>,
}

impl Tagged for ContainerBlock {
    const TAG: &'static str = "container";
}

impl ContainerBlock {
    pub fn new(title: impl Into<TextObject>, child_blocks: Vec<Block>) -> Self {
        Self {
            r#type: Tag::default(),
            title: title.into(),
            rich_text_title: None,
            subtitle: None,
            child_blocks,
            block_id: None,
            width: None,
            icon: None,
            is_collapsible: None,
            default_collapsed: None,
            has_header_divider: None,
        }
    }

    pub fn rich_text_title(mut self, rich_text_title: RichTextBlock) -> Self {
        self.rich_text_title = Some(rich_text_title);
        self
    }

    pub fn subtitle(mut self, subtitle: impl Into<TextObject>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }

    pub fn width(mut self, width: ContainerWidth) -> Self {
        self.width = Some(width);
        self
    }

    pub fn icon(mut self, icon: ImageElement) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn is_collapsible(mut self, is_collapsible: bool) -> Self {
        self.is_collapsible = Some(is_collapsible);
        self
    }

    pub fn default_collapsed(mut self, default_collapsed: bool) -> Self {
        self.default_collapsed = Some(default_collapsed);
        self
    }

    pub fn has_header_divider(mut self, has_header_divider: bool) -> Self {
        self.has_header_divider = Some(has_header_divider);
        self
    }
}

/// A block that shows supplementary info in small text and images. <https://docs.slack.dev/reference/block-kit/blocks/context-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextBlock {
    r#type: Tag<Self>,
    pub elements: Vec<ContextElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Tagged for ContextBlock {
    const TAG: &'static str = "context";
}

impl ContextBlock {
    pub fn new(elements: Vec<ContextElement>) -> Self {
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

/// A block that shows feedback buttons and icon buttons as contextual actions. <https://docs.slack.dev/reference/block-kit/blocks/context-actions-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextActionsBlock {
    r#type: Tag<Self>,
    pub elements: Vec<ContextActionsElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Tagged for ContextActionsBlock {
    const TAG: &'static str = "context_actions";
}

impl ContextActionsBlock {
    pub fn new(elements: Vec<ContextActionsElement>) -> Self {
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

/// A divider line. <https://docs.slack.dev/reference/block-kit/blocks/divider-block>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DividerBlock {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Tagged for DividerBlock {
    const TAG: &'static str = "divider";
}

impl DividerBlock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }
}

/// A remote file (only appears in retrieved messages). <https://docs.slack.dev/reference/block-kit/blocks/file-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileBlock {
    r#type: Tag<Self>,
    pub external_id: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Tagged for FileBlock {
    const TAG: &'static str = "file";
}

impl FileBlock {
    pub fn new(external_id: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            external_id: external_id.into(),
            source: source.into(),
            block_id: None,
        }
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }
}

/// A header. <https://docs.slack.dev/reference/block-kit/blocks/header-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeaderBlock {
    r#type: Tag<Self>,
    pub text: TextObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,
}

impl Tagged for HeaderBlock {
    const TAG: &'static str = "header";
}

impl HeaderBlock {
    pub fn new(text: impl Into<TextObject>) -> Self {
        Self {
            r#type: Tag::default(),
            text: text.into(),
            block_id: None,
            level: None,
        }
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }

    pub fn level(mut self, level: u8) -> Self {
        self.level = Some(level);
        self
    }
}

/// An image block (either `image_url` or `slack_file`). <https://docs.slack.dev/reference/block-kit/blocks/image-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageBlock {
    r#type: Tag<Self>,
    pub alt_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_file: Option<SlackFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Tagged for ImageBlock {
    const TAG: &'static str = "image";
}

impl ImageBlock {
    pub fn new(image_url: impl Into<String>, alt_text: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            alt_text: alt_text.into(),
            image_url: Some(image_url.into()),
            slack_file: None,
            title: None,
            block_id: None,
        }
    }

    pub fn from_slack_file(slack_file: SlackFile, alt_text: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            alt_text: alt_text.into(),
            image_url: None,
            slack_file: Some(slack_file),
            title: None,
            block_id: None,
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

    pub fn title(mut self, title: impl Into<TextObject>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }
}

/// An input field. <https://docs.slack.dev/reference/block-kit/blocks/input-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputBlock {
    r#type: Tag<Self>,
    pub label: TextObject,
    pub element: InputElement,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatch_action: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

impl Tagged for InputBlock {
    const TAG: &'static str = "input";
}

impl InputBlock {
    pub fn new(label: impl Into<TextObject>, element: impl Into<InputElement>) -> Self {
        Self {
            r#type: Tag::default(),
            label: label.into(),
            element: element.into(),
            dispatch_action: None,
            block_id: None,
            hint: None,
            optional: None,
        }
    }

    pub fn dispatch_action(mut self, dispatch_action: bool) -> Self {
        self.dispatch_action = Some(dispatch_action);
        self
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }

    pub fn hint(mut self, hint: impl Into<TextObject>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn optional(mut self, optional: bool) -> Self {
        self.optional = Some(optional);
        self
    }
}

/// A block that renders standard Markdown. <https://docs.slack.dev/reference/block-kit/blocks/markdown-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkdownBlock {
    r#type: Tag<Self>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Tagged for MarkdownBlock {
    const TAG: &'static str = "markdown";
}

impl MarkdownBlock {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            text: text.into(),
            block_id: None,
        }
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }
}

/// A collection of related tasks. <https://docs.slack.dev/reference/block-kit/blocks/plan-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanBlock {
    r#type: Tag<Self>,
    pub title: String,
    pub tasks: Vec<PlanTask>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Tagged for PlanBlock {
    const TAG: &'static str = "plan";
}

impl PlanBlock {
    pub fn new(title: impl Into<String>, tasks: Vec<PlanTask>) -> Self {
        Self {
            r#type: Tag::default(),
            title: title.into(),
            tasks,
            block_id: None,
        }
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }
}

/// A task inside a plan block (has no `type`). <https://docs.slack.dev/reference/block-kit/blocks/plan-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanTask {
    pub task_id: String,
    pub title: String,
    pub status: TaskStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<RichTextBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<RichTextBlock>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<UrlSourceElement>,
}

impl PlanTask {
    pub fn new(task_id: impl Into<String>, title: impl Into<String>, status: TaskStatus) -> Self {
        Self {
            task_id: task_id.into(),
            title: title.into(),
            status,
            details: None,
            output: None,
            sources: Vec::new(),
        }
    }

    pub fn details(mut self, details: RichTextBlock) -> Self {
        self.details = Some(details);
        self
    }

    pub fn output(mut self, output: RichTextBlock) -> Self {
        self.output = Some(output);
        self
    }

    pub fn sources(mut self, sources: Vec<UrlSourceElement>) -> Self {
        self.sources = sources;
        self
    }
}

/// The basic block for text, optionally alongside an element. <https://docs.slack.dev/reference/block-kit/blocks/section-block>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SectionBlock {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessory: Option<SectionAccessory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<bool>,
}

impl Tagged for SectionBlock {
    const TAG: &'static str = "section";
}

impl SectionBlock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: impl Into<TextObject>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }

    pub fn fields(mut self, fields: Vec<TextObject>) -> Self {
        self.fields = fields;
        self
    }

    pub fn accessory(mut self, accessory: impl Into<SectionAccessory>) -> Self {
        self.accessory = Some(accessory.into());
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }
}

/// A single task. <https://docs.slack.dev/reference/block-kit/blocks/task-card-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskCardBlock {
    r#type: Tag<Self>,
    pub task_id: String,
    pub title: String,
    pub status: TaskStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<RichTextBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<RichTextBlock>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<UrlSourceElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<SlackIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_title: Option<bool>,
}

impl Tagged for TaskCardBlock {
    const TAG: &'static str = "task_card";
}

impl TaskCardBlock {
    pub fn new(task_id: impl Into<String>, title: impl Into<String>, status: TaskStatus) -> Self {
        Self {
            r#type: Tag::default(),
            task_id: task_id.into(),
            title: title.into(),
            status,
            details: None,
            output: None,
            sources: Vec::new(),
            block_id: None,
            icon: None,
            hide_title: None,
        }
    }

    pub fn details(mut self, details: RichTextBlock) -> Self {
        self.details = Some(details);
        self
    }

    pub fn output(mut self, output: RichTextBlock) -> Self {
        self.output = Some(output);
        self
    }

    pub fn sources(mut self, sources: Vec<UrlSourceElement>) -> Self {
        self.sources = sources;
        self
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }

    pub fn icon(mut self, icon: SlackIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn hide_title(mut self, hide_title: bool) -> Self {
        self.hide_title = Some(hide_title);
        self
    }
}

/// An embedded video. <https://docs.slack.dev/reference/block-kit/blocks/video-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoBlock {
    r#type: Tag<Self>,
    pub alt_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    pub title: TextObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_url: Option<String>,
    pub thumbnail_url: String,
    pub video_url: String,
}

impl Tagged for VideoBlock {
    const TAG: &'static str = "video";
}

impl VideoBlock {
    pub fn new(
        alt_text: impl Into<String>,
        title: impl Into<TextObject>,
        thumbnail_url: impl Into<String>,
        video_url: impl Into<String>,
    ) -> Self {
        Self {
            r#type: Tag::default(),
            alt_text: alt_text.into(),
            author_name: None,
            block_id: None,
            description: None,
            provider_icon_url: None,
            provider_name: None,
            title: title.into(),
            title_url: None,
            thumbnail_url: thumbnail_url.into(),
            video_url: video_url.into(),
        }
    }

    pub fn author_name(mut self, author_name: impl Into<String>) -> Self {
        self.author_name = Some(author_name.into());
        self
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }

    pub fn description(mut self, description: impl Into<TextObject>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn provider_icon_url(mut self, provider_icon_url: impl Into<String>) -> Self {
        self.provider_icon_url = Some(provider_icon_url.into());
        self
    }

    pub fn provider_name(mut self, provider_name: impl Into<String>) -> Self {
        self.provider_name = Some(provider_name.into());
        self
    }

    pub fn title_url(mut self, title_url: impl Into<String>) -> Self {
        self.title_url = Some(title_url.into());
        self
    }
}

/// The level of an alert block. Unknown values go to `Other`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertLevel {
    Default,
    Info,
    Warning,
    Error,
    Success,
    #[serde(untagged)]
    Other(String),
}

/// The width of a container block. Unknown values go to `Other`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContainerWidth {
    Narrow,
    Standard,
    Wide,
    Full,
    #[serde(untagged)]
    Other(String),
}

/// The status of a task. Unknown values go to `Other`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Complete,
    Error,
    #[serde(untagged)]
    Other(String),
}
