use serde::{Serialize,Deserialize};
use crate::series::{SeriesInfo};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RadarData{
    value:Vec<f64>,
    name:String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarSeries{

    #[serde(flatten)]
    info:SeriesInfo,

    /// 雷达图所使用的 radar 组件的 index。
    #[serde(skip_serializing_if = "Option::is_none")]
    radar_index:Option<usize>,

    ///
    /// 雷达图的数据是多变量（维度）的
    /// 其中的value项数组是具体的数据，每个值跟 radar.indicator 一一对应。
    ///
    data:Vec<RadarData>,
}