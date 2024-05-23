mod category;
mod cartesian;
mod numeric;
mod value;
mod label;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::animation::Animation;
use crate::axis::category::CategoryAxis;
use crate::axis::value::ValueAxis;
use crate::component::Component;
use crate::data::BoolOrString;
use crate::OrdinalRawValue;
use crate::style::{LineStyle, Text, TextStyle, Truncate};
use crate::tooltip::Tooltip;

///
/// declare type AxisBaseOption = ValueAxisBaseOption | LogAxisBaseOption | CategoryAxisBaseOption | TimeAxisBaseOption | AxisBaseOptionCommon;
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Axis{

    Category(CategoryAxis),

    Value(ValueAxis)

}


///
/// 坐标轴基本信息
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisBase{

    #[serde(flatten)]
    component:Component,

    #[serde(flatten)]
    animation:Animation,

    #[serde(rename = "type",skip_serializing_if = "Option::is_none")]
    axis_type:Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    inverse: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name_location:Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name_rotate: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name_truncate:Option<Truncate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name_text_style: Option<AxisNameTextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_gap:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    silent:Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_event:Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tooltip:Option<Tooltip>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_line:Option<AxisLine>,

//     axisLabel?: AxisLabelBaseOption;
//     axisPointer?: CommonAxisPointerOption;
//     axisTick?: AxisTickOption;
    #[serde(skip_serializing_if = "Option::is_none")]
    axis_tick:Option<AxisTick>,
//     minorTick?: MinorTickOption;
//     splitLine?: SplitLineOption;
//     minorSplitLine?: MinorSplitLineOption;
//     splitArea?: SplitAreaOption;
//     min?: ScaleDataValue | 'dataMin' | ((extent: {
//     min: number;
//     max: number;
// }) => ScaleDataValue);
// max?: ScaleDataValue | 'dataMax' | ((extent: {
// min: number;
// max: number;
// }) => ScaleDataValue);
}


///
///
///
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisData{

    ///
    /// 轴线显示值
    ///
    pub(crate) value:OrdinalRawValue,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) text_style:Option<Text>,
}

impl AxisData {
    pub fn new(value:OrdinalRawValue,text_style:Option<Text>)->Self{
        Self{
            value,text_style
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisNameTextStyle{

    #[serde(flatten)]
    text:Text,

    #[serde(skip_serializing_if = "Option::is_none")]
    rich:Option<HashMap<String,Text>>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AxisLine{
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool> ,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_zero: Option<bool> ,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_zero_axis_index: Option<usize> ,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol:Option<Vec<String>> ,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_size: Option<Vec<f32>> ,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_offset: Option<Vec<f32>> ,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_style:Option<LineStyle> ,
}

#[derive(Clone, Debug, Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisPointer{

    ///
    /// 指示器类型。
    /// 可选
    /// 'line' 直线指示器
    /// 'shadow' 阴影指示器
    /// 'none' 无指示器
    /// 'cross' 十字准星指示器。其实是种简写，表示启用两个正交的轴的 axisPointer。
    #[serde(rename = "type")]
    pointer_type:String,

    ///
    /// 指示器的坐标轴。
    /// 默认情况，坐标系会自动选择显示哪个轴的 axisPointer（默认取类目轴或者时间轴）。
    /// 可以是 'x', 'y', 'radius', 'angle'。
    #[serde(skip_serializing_if = "Option::is_none")]
    axis:Option<String>,

    ///
    /// 坐标轴指示器是否自动吸附到点上。默认自动判断。
    /// 这个功能在数值轴和时间轴上比较有意义，可以自动寻找细小的数值点。
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    snap:Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label:Option<TextStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    line_style:Option<LineStyle>,
}

#[derive(Clone, Debug, Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisTick{
    #[serde(skip_serializing_if = "Option::is_none")]
    show:Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inside:Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    length:Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_style:Option<LineStyle>,
}


// interface AxisTickOption {
// show?: boolean | 'auto';
// inside?: boolean;
// length?: number;
// lineStyle?: LineStyleOption;
// }