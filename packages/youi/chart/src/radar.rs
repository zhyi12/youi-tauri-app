//!
//! 雷达图坐标系组件，只适用于雷达图。
//!
//!
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Indicator{

    name:String,
    #[serde(skip_serializing_if = "Option::is_none")]
    max:Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min:Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color:Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Radar{

    ///
    ///
    ///
    indicator:Vec<Indicator>,

    ///
    /// 中心（圆心）坐标，数组的第一项是横坐标，第二项是纵坐标。
    /// 支持设置成百分比，设置成百分比时第一项是相对于容器宽度，第二项是相对于容器高度。
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    center:Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    radius:Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_angle:Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name_gap:Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    split_number:Option<usize>,

}