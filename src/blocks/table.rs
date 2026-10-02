use serde::{Deserialize, Serialize};

use super::element::ButtonElement;
use super::rich_text::RichTextBlock;
use super::tag::{tagged_enum, Tag, Tagged};

/// A table. <https://docs.slack.dev/reference/block-kit/blocks/table-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableBlock {
    r#type: Tag<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    pub rows: Vec<Vec<TableCell>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub column_settings: Vec<Option<ColumnSetting>>,
}

impl Tagged for TableBlock {
    const TAG: &'static str = "table";
}

impl TableBlock {
    pub fn new(rows: Vec<Vec<TableCell>>) -> Self {
        Self {
            r#type: Tag::default(),
            block_id: None,
            rows,
            column_settings: Vec::new(),
        }
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }

    pub fn column_settings(mut self, column_settings: Vec<Option<ColumnSetting>>) -> Self {
        self.column_settings = column_settings;
        self
    }
}

tagged_enum! {
    /// A cell in a table block. <https://docs.slack.dev/reference/block-kit/blocks/table-block>
    pub enum TableCell("type") {
        RawText(RawTextCell),
        RawNumber(RawNumberCell),
        RichText(RichTextBlock),
    }
}

/// Settings for a table column (use `None` for a column that keeps the defaults, sent as `null`). <https://docs.slack.dev/reference/block-kit/blocks/table-block>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ColumnSetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<ColumnAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_wrapped: Option<bool>,
}

impl ColumnSetting {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn align(mut self, align: ColumnAlign) -> Self {
        self.align = Some(align);
        self
    }

    pub fn is_wrapped(mut self, is_wrapped: bool) -> Self {
        self.is_wrapped = Some(is_wrapped);
        self
    }
}

/// A table with pagination, sorting and filtering. <https://docs.slack.dev/reference/block-kit/blocks/data-table-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataTableBlock {
    r#type: Tag<Self>,
    pub rows: Vec<Vec<DataTableCell>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    pub caption: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_header_column_index: Option<u32>,
}

impl Tagged for DataTableBlock {
    const TAG: &'static str = "data_table";
}

impl DataTableBlock {
    pub fn new(rows: Vec<Vec<DataTableCell>>, caption: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            rows,
            block_id: None,
            page_size: None,
            caption: caption.into(),
            row_header_column_index: None,
        }
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn row_header_column_index(mut self, row_header_column_index: u32) -> Self {
        self.row_header_column_index = Some(row_header_column_index);
        self
    }
}

tagged_enum! {
    /// A cell in a data table block. <https://docs.slack.dev/reference/block-kit/blocks/data-table-block>
    pub enum DataTableCell("type") {
        RawText(RawTextCell),
        RawNumber(RawNumberCell),
        RichText(RichTextBlock),
        ActionCell(ActionCell),
    }
}

/// A plain text cell. <https://docs.slack.dev/reference/block-kit/blocks/data-table-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawTextCell {
    r#type: Tag<Self>,
    pub text: String,
}

impl Tagged for RawTextCell {
    const TAG: &'static str = "raw_text";
}

impl RawTextCell {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            r#type: Tag::default(),
            text: text.into(),
        }
    }
}

/// A numeric cell. <https://docs.slack.dev/reference/block-kit/blocks/data-table-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawNumberCell {
    r#type: Tag<Self>,
    pub value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl Tagged for RawNumberCell {
    const TAG: &'static str = "raw_number";
}

impl RawNumberCell {
    pub fn new(value: f64) -> Self {
        Self {
            r#type: Tag::default(),
            value,
            text: None,
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
}

/// A cell holding a button (data table blocks only). <https://docs.slack.dev/reference/block-kit/blocks/data-table-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionCell {
    r#type: Tag<Self>,
    pub element: Box<ButtonElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ActionCellFallback>,
}

impl Tagged for ActionCell {
    const TAG: &'static str = "action_cell";
}

impl ActionCell {
    pub fn new(element: ButtonElement) -> Self {
        Self {
            r#type: Tag::default(),
            element: Box::new(element),
            fallback: None,
        }
    }

    pub fn fallback(mut self, fallback: impl Into<ActionCellFallback>) -> Self {
        self.fallback = Some(fallback.into());
        self
    }
}

tagged_enum! {
    /// The cell shown instead of the button on clients that do not support action cells. <https://docs.slack.dev/reference/block-kit/blocks/data-table-block>
    pub enum ActionCellFallback("type") {
        RawText(RawTextCell),
        RawNumber(RawNumberCell),
    }
}

/// Column alignment. Unknown values go to `Other`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnAlign {
    Left,
    Center,
    Right,
    #[serde(untagged)]
    Other(String),
}
