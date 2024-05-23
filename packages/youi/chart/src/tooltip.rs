use serde::{Deserialize, Serialize};
use crate::axis::AxisPointer;
use crate::style::TextStyle;

///
/// 提示框组件可以设置在多种地方：
///     可以设置在全局，即 tooltip
///     可以设置在坐标系中，即 grid.tooltip、polar.tooltip、single.tooltip
///     可以设置在系列中，即 series.tooltip
///     可以设置在系列的每个数据项中，即 series.data.tooltip
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tooltip {

    #[serde(skip_serializing_if = "Option::is_none")]
    trigger:Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_pointer:Option<AxisPointer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    formatter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_formatter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_style: Option<TextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_css_text: Option<String>,
}
