use serde::{Serialize, Deserialize};
use crate::axis::AxisData;
use crate::axis::cartesian::CartesianAxis;
use crate::axis::label::AxisLabel;


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryAxis{

    #[serde(flatten)]
    axis:CartesianAxis,

    #[serde(skip_serializing_if = "Option::is_none")]
    boundary_gap:Option<bool>,

    ///
    /// 类目数据，在类目轴（type: 'category'）中有效。 Vec<OrdinalRawValue|AxisData>
    ///
    #[serde(skip_serializing_if = "Option::is_none",default="default_data")]
    data:Option<Vec<AxisData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_label:Option<AxisLabel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    deduplication:Option<bool>,

    // axisTick?: AxisBaseOptionCommon['axisTick']
}

fn default_data()->Option<Vec<AxisData>>{
    None
}
