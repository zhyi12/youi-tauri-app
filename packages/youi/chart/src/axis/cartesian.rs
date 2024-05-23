
use serde::{Serialize, Deserialize};
use crate::axis::AxisBase;
use crate::data::OrdinalSortInfo;

///
/// 笛卡尔轴
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CartesianAxis{
    #[serde(flatten)]
    base:AxisBase,

    #[serde(skip_serializing_if = "Option::is_none")]
    grid_index:Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    grid_id:Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    offset:Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position:Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    category_sort_info:Option<OrdinalSortInfo>,

}
