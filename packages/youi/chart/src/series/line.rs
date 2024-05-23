use serde::{Serialize,Deserialize};
use crate::series::{LineEndLabel, SeriesDatasetInfo, SeriesInfo, SeriesLabel, XyAxisSeriesInfo};
use crate::series::stack::Stack;
use crate::style::{AreaStyle, Emphasis, ItemStyle, LineStyle};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineSeries{

    #[serde(flatten)]
    info:SeriesInfo,

    #[serde(flatten)]
    axis:XyAxisSeriesInfo,

    #[serde(flatten)]
    dataset_info:SeriesDatasetInfo,

    #[serde(flatten)]
    stack:Stack,

    #[serde(flatten)]
    state:LineState,

    ///是否平滑曲线显示。
    ///如果是 boolean 类型，则表示是否开启平滑处理。如果是 number 类型（取值范围 0 到 1），表示平滑程度，越小表示越接近折线段，反之则反。设为 true 时相当于设为 0.5。
    #[serde(skip_serializing_if = "Option::is_none")]
    smooth:Option<serde_json::Value>,

    ///
    /// 折线图在数据量远大于像素点时候的降采样策略，开启后可以有效的优化图表的绘制效率，默认关闭，也就是全部绘制不过滤数据点。
    /// 可选：
    /// 'lttb' 采用 Largest-Triangle-Three-Bucket 算法，可以最大程度保证采样后线条的趋势，形状和极值。
    /// 'average' 取过滤点的平均值
    /// 'min' 取过滤点的最小值
    /// 'max' 取过滤点的最大值
    /// 'minmax' 取过滤点绝对值的最大极值 (从 v5.5.0 开始支持)
    /// 'sum' 取过滤点的和
    #[serde(skip_serializing_if = "Option::is_none")]
    sampling:Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    area_style:Option<AreaStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    show_symbol:Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    emphasis:Option<Emphasis>,

    #[serde(skip_serializing_if = "Option::is_none")]
    line_style:Option<LineStyle>

}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineState{

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style:Option<ItemStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label:Option<SeriesLabel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_label: Option<LineEndLabel>

}