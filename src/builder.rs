use crate::api::{SlackApiChatPostMessageRequest, SlackMessageAttachment};
use crate::SlackBlock;

pub struct SlackMessageBuilder {
    pub(crate) inner: SlackApiChatPostMessageRequest,
}

impl From<SlackMessageBuilder> for SlackApiChatPostMessageRequest {
    fn from(value: SlackMessageBuilder) -> Self {
        value.inner
    }
}

impl SlackMessageBuilder {
    pub fn new() -> SlackMessageBuilder {
        SlackMessageBuilder {
            inner: SlackApiChatPostMessageRequest::default(),
        }
    }

    pub fn channel<S: Into<String>>(mut self, channel: S) -> SlackMessageBuilder {
        let mut inner = self.inner;
        inner.channel = channel.into();
        SlackMessageBuilder { inner }
    }
    pub fn icon_emoji<S: Into<String>>(mut self, icon_emoji: S) -> SlackMessageBuilder {
        let mut inner = self.inner;
        inner.icon_emoji = Some(icon_emoji.into());
        SlackMessageBuilder { inner }
    }
    pub fn username<S: Into<String>>(mut self, username: S) -> SlackMessageBuilder {
        let mut inner = self.inner;
        inner.username = Some(username.into());
        SlackMessageBuilder { inner }
    }
    pub fn attachments<A: Into<SlackMessageAttachment>>(mut self, attachments: Vec<A>) -> SlackMessageBuilder {
        let mut inner = self.inner;
        let mut values = vec![];
        for attachment in attachments {
            values.push(attachment.into());
        }
        inner.attachments = Some(values);
        SlackMessageBuilder { inner }
    }

}

pub struct SlackAttachmentBuilder {
    pub(crate) inner: SlackMessageAttachment,
}

impl From<SlackAttachmentBuilder> for SlackMessageAttachment {
    fn from(value: SlackAttachmentBuilder) -> Self {
        value.inner
    }
}

impl SlackAttachmentBuilder {
    pub fn new() -> SlackAttachmentBuilder {
        SlackAttachmentBuilder {
            inner: SlackMessageAttachment::default(),
        }
    }
    pub fn color<S: Into<String>>(mut self, color: S) -> SlackAttachmentBuilder {
        let mut inner = self.inner;
        inner.color = Some(color.into());
        SlackAttachmentBuilder { inner }
    }

    pub fn block<B: Into<SlackBlock>>(mut self, block: B) -> SlackAttachmentBuilder {
        let mut inner = self.inner;
        let mut blocks = inner.blocks.unwrap_or_default();
        blocks.push(block.into());
        inner.blocks = Some(blocks);
        SlackAttachmentBuilder { inner }
    }
    pub fn blocks<B: Into<SlackBlock>>(mut self, blocks: Vec<B>) -> SlackAttachmentBuilder {
        let mut inner = self.inner;
        let mut blocks = blocks;
        let mut values = vec![];
        for blocks in blocks {
            values.push(blocks.into());
        }
        inner.blocks = Some(values);
        SlackAttachmentBuilder { inner }
    }
}
