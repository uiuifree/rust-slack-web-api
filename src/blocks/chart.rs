use serde::{Deserialize, Serialize};

use super::tag::{tagged_enum, Tag, Tagged};

/// A chart. <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataVisualizationBlock {
    r#type: Tag<Self>,
    pub title: String,
    pub chart: Chart,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl Tagged for DataVisualizationBlock {
    const TAG: &'static str = "data_visualization";
}

impl DataVisualizationBlock {
    pub fn new(title: impl Into<String>, chart: impl Into<Chart>) -> Self {
        Self {
            r#type: Tag::default(),
            title: title.into(),
            chart: chart.into(),
            block_id: None,
        }
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }
}

tagged_enum! {
    /// The chart payload for each chart type. <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>
    pub enum Chart("type") {
        Pie(PieChart),
        Bar(BarChart),
        Area(AreaChart),
        Line(LineChart),
    }
}

/// A pie chart. <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PieChart {
    r#type: Tag<Self>,
    pub segments: Vec<Segment>,
}

impl Tagged for PieChart {
    const TAG: &'static str = "pie";
}

impl PieChart {
    pub fn new(segments: Vec<Segment>) -> Self {
        Self {
            r#type: Tag::default(),
            segments,
        }
    }
}

/// A bar chart. <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BarChart {
    r#type: Tag<Self>,
    pub series: Vec<DataSeries>,
    pub axis_config: AxisConfig,
}

impl Tagged for BarChart {
    const TAG: &'static str = "bar";
}

impl BarChart {
    pub fn new(series: Vec<DataSeries>, axis_config: AxisConfig) -> Self {
        Self {
            r#type: Tag::default(),
            series,
            axis_config,
        }
    }
}

/// An area chart. <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AreaChart {
    r#type: Tag<Self>,
    pub series: Vec<DataSeries>,
    pub axis_config: AxisConfig,
}

impl Tagged for AreaChart {
    const TAG: &'static str = "area";
}

impl AreaChart {
    pub fn new(series: Vec<DataSeries>, axis_config: AxisConfig) -> Self {
        Self {
            r#type: Tag::default(),
            series,
            axis_config,
        }
    }
}

/// A line chart. <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineChart {
    r#type: Tag<Self>,
    pub series: Vec<DataSeries>,
    pub axis_config: AxisConfig,
}

impl Tagged for LineChart {
    const TAG: &'static str = "line";
}

impl LineChart {
    pub fn new(series: Vec<DataSeries>, axis_config: AxisConfig) -> Self {
        Self {
            r#type: Tag::default(),
            series,
            axis_config,
        }
    }
}

/// One slice of a pie chart. <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Segment {
    pub label: String,
    pub value: f64,
}

impl Segment {
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}

/// A data series in a chart. <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSeries {
    pub name: String,
    pub data: Vec<DataPoint>,
}

impl DataSeries {
    pub fn new(name: impl Into<String>, data: Vec<DataPoint>) -> Self {
        Self {
            name: name.into(),
            data,
        }
    }
}

/// One data point in a series. <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataPoint {
    pub label: String,
    pub value: f64,
}

impl DataPoint {
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}

/// X-axis categories and axis labels. <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxisConfig {
    pub categories: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_label: Option<String>,
}

impl AxisConfig {
    pub fn new(categories: Vec<String>) -> Self {
        Self {
            categories,
            x_label: None,
            y_label: None,
        }
    }

    pub fn x_label(mut self, x_label: impl Into<String>) -> Self {
        self.x_label = Some(x_label.into());
        self
    }

    pub fn y_label(mut self, y_label: impl Into<String>) -> Self {
        self.y_label = Some(y_label.into());
        self
    }
}
