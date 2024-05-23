// use std::collections::HashMap;
// use serde::{Serialize, Deserialize};
// use crate::animation::Animation;
// use crate::component::Component;
// use crate::data::OrdinalSortInfo;
// use crate::OrdinalRawValue;
// use crate::style::{Gap, LineStyle, Text, TextStyle, Truncate};
// use crate::de::gap_deserialize;
// use crate::tooltip::Tooltip;
//
//
//
//
// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct CategoryAxisBase{
//
//
// }
//
// ///
// ///
// ///
// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Axis{
//     ///
//     /// 组件 ID。默认不指定。指定则可用于在 option 或者 API 中引用组件。
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     id:Option<String>,
//     ///
//     /// 是否显示
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     show:Option<bool>,
//
//     ///
//     /// 轴所在的 grid 的索引，默认位于第一个 grid。
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     grid_index:Option<usize>,
//
//     ///
//     /// 在多个 x 轴为数值轴的时候，可以开启该配置项自动对齐刻度。只对'value'和'log'类型的轴有效。
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     align_ticks:Option<bool>,
//
//     ///
//     /// 轴的位置
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     position:Option<String>,
//
//     ///
//     /// 轴相对于默认位置的偏移，在相同的 position 上有多个轴的时候有用
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     offset:Option<f32>,
//
//     ///
//     ///'value' 数值轴，适用于连续数据。
//     /// 'category' 类目轴，适用于离散的类目数据。为该类型时类目数据可自动从 series.data 或 dataset.source 中取，或者可通过 xAxis.data 设置类目数据。
//     ///'time' 时间轴，适用于连续的时序数据，与数值轴相比时间轴带有时间的格式化，在刻度计算上也有所不同，例如会根据跨度的范围来决定使用月，星期，日还是小时范围的刻度
//     /// 'log' 对数轴。适用于对数数据。对数轴下的堆积柱状图或堆积折线图可能带来很大的视觉误差，并且在一定情况下可能存在非预期效果，应避免使用。
//     /// type
//     #[serde(rename = "type",skip_serializing_if = "Option::is_none")]
//     axis_type:Option<String>,
//
//     ///
//     /// 坐标轴名称。
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     name:Option<String>,
//
//     ///
//     /// 坐标轴名称显示位置
//     /// 可选：
//     /// 'start'
//     /// 'middle' 或者 'center'
//     /// 'end'
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     name_location:Option<String>,
//
//     #[serde(skip_serializing_if = "Option::is_none")]
//     name_text_style:Option<TextStyle>,
//     ///
//     /// 坐标轴名称与轴线之间的距离。
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     name_gap:Option<f32>,
//
//     ///
//     /// 坐标轴名字旋转，角度值。
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     name_rotate:Option<f32>,
//     ///
//     /// 坐标轴名字的截断。
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     name_truncate: Option<Truncate>,
//     ///
//     /// 是否是反向坐标轴。
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     inverse:Option<bool>,
//
//     ///
//     /// 坐标轴两边留白策略，类目轴和非类目轴的设置和表现不一样。
//     //
//     /// 类目轴中 boundaryGap 可以配置为 true 和 false。默认为 true，这时候刻度只是作为分隔线，标签和数据点都会在两个刻度之间的带(band)中间。
//     //
//     /// 非类目轴，包括时间，数值，对数轴，boundaryGap是一个两个值的数组，分别表示数据最小值和最大值的延伸范围，可以直接设置数值或者相对的百分比，在设置 min 和 max 后无效。 示例：
//     ///
//     #[serde(skip_serializing_if = "Option::is_none",default="default_boundary_gap",deserialize_with="gap_deserialize")]
//     boundary_gap:Option<Gap>,
//     ///
//     /// 坐标轴刻度最小值。
//     ///
//     /// 可以设置成特殊值 'dataMin'，此时取数据在该轴上的最小值作为最小刻度。
//     ///
//     /// 不设置时会自动计算最小值保证坐标轴刻度的均匀分布。
//     ///
//     /// 在类目轴中，也可以设置为类目的序数（如类目轴 data: ['类A', '类B', '类C'] 中，序数 2 表示 '类C'。也可以设置为负数，如 -3）。
//     ///
//     /// 当设置成 function 形式时，可以根据计算得出的数据最大最小值设定坐标轴的最小值。如：
//     ///
//     /// min: function (value) {
//     ///     return value.min - 20;
//     /// }
//     /// 其中 value 是一个包含 min 和 max 的对象，分别表示数据的最大最小值，这个函数可返回坐标轴的最小值，也可返回 null/undefined 来表示“自动计算最小值”（返回 null/undefined 从 v4.8.0 开始支持）。
//     #[serde(skip_serializing_if = "Option::is_none")]
//     min:Option<OrdinalRawValue>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     max:Option<OrdinalRawValue>,
//     ///
//     /// 只在数值轴中（type: 'value'）有效。
//     /// 是否是脱离 0 值比例。设置成 true 后坐标刻度不会强制包含零刻度。在双数值轴的散点图中比较有用。
//     /// 在设置 min 和 max 之后该配置项无效。
//     #[serde(skip_serializing_if = "Option::is_none")]
//     scale:Option<f32>,
//     /// 坐标轴的分割段数，需要注意的是这个分割段数只是个预估值，最后实际显示的段数会在这个基础上根据分割后坐标轴刻度显示的易读程度作调整。
//     /// 在类目轴中无效。
//     #[serde(skip_serializing_if = "Option::is_none")]
//     split_number:Option<f32>,
//     ///
//     /// 自动计算的坐标轴最小间隔大小。
//     /// 例如可以设置成1保证坐标轴分割刻度显示成整数。
//     /// 只在数值轴或时间轴中（type: 'value' 或 'time'）有效。
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     min_interval:Option<f32>,
//     ///
//     /// 自动计算的坐标轴最大间隔大小。
//     /// 例如，在时间轴（（type: 'time'））可以设置成 3600 * 24 * 1000 保证坐标轴分割刻度最大为一天。
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     max_interval:Option<f32>,
//     ///
//     /// 强制设置坐标轴分割间隔。
//     /// 因为 splitNumber 是预估的值，实际根据策略计算出来的刻度可能无法达到想要的效果，这时候可以使用 interval 配合 min、max 强制设定刻度划分，一般不建议使用。
//     /// 无法在类目轴中使用。在时间轴（type: 'time'）中需要传时间戳，在对数轴（type: 'log'）中需要传指数值。
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     interval:Option<f32>,
//     ///
//     /// 对数轴的底数，只在对数轴中（type: 'log'）有效。
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     log_base:Option<usize>,
//     ///
//     /// 坐标轴是否是静态无法交互。
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     silent:Option<bool>,
//     ///
//     /// 坐标轴的标签是否响应和触发鼠标事件，默认不响应。
//     ///
//     #[serde(skip_serializing_if = "Option::is_none")]
//     trigger_event:Option<bool>,
//
//     ///
//     /// 类目数据，在类目轴（type: 'category'）中有效。 Vec<OrdinalRawValue|AxisData>
//     ///
//     #[serde(skip_serializing_if = "Option::is_none",default="default_data")]
//     data:Option<Vec<AxisData>>,
// }
//
// fn default_data()->Option<Vec<AxisData>>{
//     None
// }
//
// fn default_boundary_gap()->Option<Gap>{
//     None
// }
// ///
// /// value: OrdinalRawValue;
// /// textStyle?: TextCommonOption;
// ///
// #[derive(Clone, Debug, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct AxisData{
//
//     ///
//     /// 轴线显示值
//     ///
//     pub(crate) value:OrdinalRawValue,
//
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub(crate) text_style:Option<TextStyle>,
// }
//
// impl AxisData {
//     pub fn new(value:OrdinalRawValue,text_style:Option<TextStyle>)->Self{
//         Self{
//             value,text_style
//         }
//     }
// }
