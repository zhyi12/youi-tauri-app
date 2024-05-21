use serde::{Serialize,Deserialize};
use crate::series::{SeriesDatasetInfo, SeriesInfo};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PieSeries{

    #[serde(flatten)]
    info:SeriesInfo,

    #[serde(flatten)]
    dataset_info:SeriesDatasetInfo,

    ///
    /// 起始角度，支持范围[0, 360]。
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    start_angle: Option<usize> ,
    ///
    /// string number
    /// 结束角度，默认值是 'auto'。
    ///  当值为 'auto' 时，根据 startAngle 自动计算结束角度，以确保是一个完整的圆。
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    end_angle: Option<serde_json::Value> ,
    ///
    /// 最小的扇区角度（0 ~ 360），用于防止某个值过小导致扇区太小影响交互。
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    min_angle: Option<usize> ,
    ///
    /// 饼图扇区之间的间隔角度（0 ~ 360）。
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pad_angle: Option<usize> ,
    ///
    /// 小于这个角度（0 ~ 360）的扇区，不显示标签（label 和 labelLine）。
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    min_show_label_angle: Option<usize> ,
    ///
    /// boolean string
    /// 是否展示成南丁格尔图，通过半径区分数据大小。可选择两种模式：
    ///
    /// 'radius' 扇区圆心角展现数据的百分比，半径展现数据的大小。
    /// 'area' 所有扇区圆心角相同，仅通过半径展现数据大小。
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    rose_type: Option<serde_json::Value> ,
    ///
    /// 是否启用防止标签重叠策略，默认开启，在标签拥挤重叠的情况下会挪动各个标签的位置，防止标签间的重叠。
    /// 如果不需要开启该策略，例如圆环图这个例子中需要强制所有标签放在中心位置，可以将该值设为 false。
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    avoid_label_overlap: Option<bool> ,
    ///
    /// 是否在数据和为0（一般情况下所有数据为0） 的时候仍显示扇区。
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    still_show_zero_sum: Option<bool> ,
    ///
    /// 饼图百分比数值的精度，默认保留小数点后两位。
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    percent_precision: Option<usize> ,
}