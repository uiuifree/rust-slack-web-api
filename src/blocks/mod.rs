//! Slack Block Kit types (blocks, elements, composition objects), plus message attachments, views and message metadata.

mod attachment;
mod block;
mod chart;
mod composition;
mod element;
mod metadata;
mod rich_text;
mod select;
mod table;
mod tag;
mod view;

#[cfg(test)]
mod tests;

pub use attachment::{Attachment, AttachmentField, AttachmentTs};
pub use block::{
    ActionsBlock, AlertBlock, AlertLevel, Block, CardBlock, CarouselBlock, ContainerBlock,
    ContainerWidth, ContextActionsBlock, ContextBlock, DividerBlock, FileBlock, HeaderBlock,
    ImageBlock, InputBlock, MarkdownBlock, PlanBlock, PlanTask, SectionBlock, TaskCardBlock,
    TaskStatus, VideoBlock,
};
pub use chart::{
    AreaChart, AxisConfig, BarChart, Chart, DataPoint, DataSeries, DataVisualizationBlock,
    LineChart, PieChart, Segment,
};
pub use composition::{
    ButtonStyle, ConfirmationDialog, ConversationFilter, ConversationType, DispatchActionConfig,
    InputParameter, Mrkdwn, OptionGroup, OptionObject, PlainText, SlackFile, SlackIcon, TextObject,
    Trigger, TriggerAction, Workflow,
};
pub use element::{
    ActionsElement, ButtonElement, CheckboxesElement, ContextActionsElement, ContextElement,
    DatePickerElement, DatetimePickerElement, EmailInputElement, FeedbackButton,
    FeedbackButtonsElement, FileInputElement, IconButtonElement, ImageElement, InputElement,
    NumberInputElement, OverflowElement, PlainTextInputElement, RadioButtonsElement,
    RichTextInputElement, SectionAccessory, TimePickerElement, UrlInputElement, UrlSourceElement,
    WorkflowButtonElement,
};
pub use metadata::MessageMetadata;
pub use rich_text::{
    AttachmentMentionElement, BroadcastElement, BroadcastRange, CanvasElement,
    CanvasMessageUnfurlElement, CanvasUserMentionElement, ChannelElement, CitationDetails,
    CitationElement, ColorElement, DateElement, EmojiElement, ExternalCitation, FileCitation,
    FileElement, LinkElement, ListRecordElement, MemoryCitation, MessageCitation,
    MessageMentionElement, PreformattedElement, RichTextBlock, RichTextElement, RichTextList,
    RichTextListStyle, RichTextObject, RichTextPreformatted, RichTextQuote, RichTextSection,
    RichTextStyle, SalesforceDataFieldElement, TagColor, TagElement, TeamElement, TextElement,
    UserElement, UsergroupElement, WebCitation, WorkObjectMentionElement, WorkflowMentionElement,
};
pub use select::{
    ChannelsSelectElement, ConversationsSelectElement, ExternalSelectElement,
    MultiChannelsSelectElement, MultiConversationsSelectElement, MultiExternalSelectElement,
    MultiStaticSelectElement, MultiUsersSelectElement, StaticSelectElement, UsersSelectElement,
};
pub use table::{
    ActionCell, ActionCellFallback, ColumnAlign, ColumnSetting, DataTableBlock, DataTableCell,
    RawNumberCell, RawTextCell, TableBlock, TableCell,
};
pub use view::{View, ViewState, ViewType};
